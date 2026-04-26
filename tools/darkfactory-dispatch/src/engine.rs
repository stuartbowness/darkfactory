use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant as StdInstant;

use chrono::{SecondsFormat, Utc};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Instant, Sleep};

use crate::event::Emitter;
use crate::supervisor::{ExitResult, StdinMode, SupervisorBuilder, SupervisorError};
use crate::types::{
    Activity, DispatchError, DispatchErrorInfo, DispatchResult, DispatchSpec, DispatchStatus,
    EventKind, HarnessEvent, LiveStatus, TokenUsage,
};

const STDOUT_BUFFER_BYTES: usize = 4 * 1024 * 1024;
const WATCHDOG_INTERVAL_SECS: u64 = 5;

#[cfg(test)]
const HEARTBEAT_INTERVAL_SECS: u64 = 1;
#[cfg(not(test))]
const HEARTBEAT_INTERVAL_SECS: u64 = 15;

pub(crate) async fn dispatch(
    spec: DispatchSpec,
    dispatch_id: String,
    emitter: Arc<Emitter>,
) -> Result<DispatchResult, DispatchError> {
    let started = StdInstant::now();
    let grace_sec = effective_grace_sec(spec.timeout_sec, spec.grace_sec);

    emitter
        .emit_dispatch_start("codex", spec.model.clone())
        .map_err(dispatch_event_error)?;

    let args = build_codex_args(&spec);
    let run_handle = SupervisorBuilder::new("codex")
        .args(args)
        .current_dir(&spec.cwd)
        .stdin(StdinMode::Null)
        .parent_death_watch(spec.store_dir.is_none())
        .spawn()
        .map_err(map_supervisor_error)?;

    let stdout = run_handle.stdout().map_err(map_supervisor_error)?;
    let stderr = run_handle.stderr().map_err(map_supervisor_error)?;
    let log = open_log(spec.log.as_deref()).await?;
    let stderr_task = tokio::spawn(capture_stderr(stderr, log.clone()));

    let mut stdout_reader = BufReader::with_capacity(STDOUT_BUFFER_BYTES, stdout).lines();
    let mut tracker = ActivityTracker::new();
    let output_path = spec.output.clone();
    let cleanup_status_dir = spec.store_dir.is_none();
    let status_dir = spec
        .store_dir
        .clone()
        .unwrap_or_else(|| status_dir(&dispatch_id));

    let soft_sleep = time::sleep(Duration::from_secs(spec.timeout_sec));
    tokio::pin!(soft_sleep);
    let mut soft_armed = true;
    let mut stdout_open = true;
    let mut hard_sleep: Option<Pin<Box<Sleep>>> = None;
    let mut watchdog_interval = time::interval_at(
        Instant::now() + Duration::from_secs(WATCHDOG_INTERVAL_SECS),
        Duration::from_secs(WATCHDOG_INTERVAL_SECS),
    );
    let mut heartbeat_interval = time::interval_at(
        Instant::now() + Duration::from_secs(HEARTBEAT_INTERVAL_SECS),
        Duration::from_secs(HEARTBEAT_INTERVAL_SECS),
    );

    let (exit, timed_out) = loop {
        tokio::select! {
            line = stdout_reader.next_line(), if stdout_open => {
                match line {
                    Ok(Some(line)) => {
                        write_log_line(&log, &line).await?;
                        if let Some(event) = parse_codex_event(&line) {
                            tracker.apply(event, &emitter)?;
                        }
                    }
                    Ok(None) => {
                        stdout_open = false;
                    }
                    Err(error) => return Err(DispatchError::Other(format!("read child stdout: {error}"))),
                }
            }

            result = run_handle.wait() => {
                // Drain remaining stdout before finalizing -- child may have
                // written final NDJSON lines (token counts, activity) just
                // before exiting.
                while let Ok(Some(line)) = stdout_reader.next_line().await {
                    write_log_line(&log, &line).await?;
                    if let Some(event) = parse_codex_event(&line) {
                        tracker.apply(event, &emitter)?;
                    }
                }
                // If soft timeout already fired (SIGTERM sent), this is still a timeout
                break (result.map_err(map_supervisor_error)?, !soft_armed);
            }

            () = &mut soft_sleep, if soft_armed => {
                soft_armed = false;
                tracker.last_activity = format!("soft timeout after {}s -- sending SIGTERM", spec.timeout_sec);
                emitter.emit_timeout_warning().map_err(dispatch_event_error)?;
                // Send SIGTERM immediately at soft timeout (not just a warning)
                let _ = run_handle.kill_soft();
                hard_sleep = Some(Box::pin(time::sleep(Duration::from_secs(grace_sec))));
            }

            () = async {
                if let Some(timer) = hard_sleep.as_mut() {
                    timer.as_mut().await;
                } else {
                    std::future::pending::<()>().await;
                }
            }, if hard_sleep.is_some() => {
                tracker.last_activity = format!("hard timeout after {}s grace -- sending SIGKILL", grace_sec);
                // Hard timeout: SIGKILL directly, no second grace period
                run_handle.kill().map_err(map_supervisor_error)?;
                let exit = run_handle.wait().await.map_err(map_supervisor_error)?;
                break (exit, true);
            }

            _ = watchdog_interval.tick() => {
                let _ = write_status_json_atomic(
                    &status_dir,
                    "running",
                    &dispatch_id,
                    started.elapsed().as_secs(),
                    tracker.last_activity.clone(),
                    tracker.activity.tool_calls,
                    tracker.activity.files_changed.len() as u64,
                );
            }

            _ = heartbeat_interval.tick() => {
                emitter
                    .emit_heartbeat_with_interval(
                        tracker.last_activity.clone(),
                        HEARTBEAT_INTERVAL_SECS,
                    )
                    .map_err(dispatch_event_error)?;
            }
        }
    };

    let stderr = stderr_task
        .await
        .map_err(|error| DispatchError::Other(format!("stderr capture task failed: {error}")))??;

    let result = finalize_result(
        dispatch_id,
        started.elapsed().as_millis() as u64,
        exit,
        timed_out,
        output_path.as_deref(),
        tracker,
        stderr,
    );
    emitter
        .emit_dispatch_end(status_name(&result.status), result.duration_ms)
        .map_err(dispatch_event_error)?;
    let _ = write_status_json_atomic(
        &status_dir,
        status_name(&result.status),
        &result.dispatch_id,
        result.duration_ms / 1000,
        "done".to_string(),
        result.activity.tool_calls,
        result.activity.files_changed.len() as u64,
    );
    if cleanup_status_dir {
        let _ = std::fs::remove_dir_all(&status_dir);
    }

    Ok(result)
}

fn build_codex_args(spec: &DispatchSpec) -> Vec<OsString> {
    let sandbox = if spec.sandbox.is_empty() {
        "workspace-write"
    } else {
        spec.sandbox.as_str()
    };
    let mut args = vec![
        "exec".into(),
        "--json".into(),
        "--full-auto".into(),
        "-s".into(),
        sandbox.into(),
        "-m".into(),
        spec.model.clone().into(),
    ];

    if let Some(effort) = spec.effort.as_deref().filter(|effort| !effort.is_empty()) {
        args.push("-c".into());
        args.push(format!("model_reasoning_effort=\"{effort}\"").into());
    }
    for config in &spec.configs {
        args.push("-c".into());
        args.push(config.clone().into());
    }
    if let Some(output) = &spec.output {
        args.push("-o".into());
        args.push(output.as_os_str().to_os_string());
    }
    args.push(spec.prompt.clone().into());

    args
}

type SharedLog = Option<Arc<Mutex<tokio::fs::File>>>;

async fn open_log(path: Option<&Path>) -> Result<SharedLog, DispatchError> {
    let Some(path) = path else {
        return Ok(None);
    };
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(DispatchError::SpawnFailed)?;
    }
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await
        .map_err(DispatchError::SpawnFailed)?;
    Ok(Some(Arc::new(Mutex::new(file))))
}

async fn capture_stderr(
    stderr: tokio::process::ChildStderr,
    log: SharedLog,
) -> Result<String, DispatchError> {
    let mut reader = BufReader::new(stderr);
    let mut captured = String::new();
    loop {
        let mut line = String::new();
        let bytes = reader
            .read_line(&mut line)
            .await
            .map_err(|error| DispatchError::Other(format!("read child stderr: {error}")))?;
        if bytes == 0 {
            break;
        }
        if let Some(file) = &log {
            let mut file = file.lock().await;
            file.write_all(line.as_bytes())
                .await
                .map_err(DispatchError::SpawnFailed)?;
            file.flush().await.map_err(DispatchError::SpawnFailed)?;
        }
        captured.push_str(&line);
    }
    Ok(captured)
}

async fn write_log_line(log: &SharedLog, line: &str) -> Result<(), DispatchError> {
    if let Some(file) = log {
        let mut file = file.lock().await;
        file.write_all(line.as_bytes())
            .await
            .map_err(DispatchError::SpawnFailed)?;
        file.write_all(b"\n")
            .await
            .map_err(DispatchError::SpawnFailed)?;
        file.flush().await.map_err(DispatchError::SpawnFailed)?;
    }
    Ok(())
}

fn parse_codex_event(line: &str) -> Option<HarnessEvent> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let Ok(raw) = serde_json::from_str::<Value>(line) else {
        return Some(event(EventKind::RawPassthrough).text(line));
    };

    let event_type = raw.get("type").and_then(Value::as_str).unwrap_or_default();
    match event_type {
        "thread.started" => Some(
            event(EventKind::SessionStart)
                .session_id(string_field(&raw, &["thread_id", "session_id"])),
        ),
        "turn.started" | "item.updated" => None,
        "item.started" => parse_item_started(&raw),
        "item.completed" => parse_item_completed(&raw),
        "turn.completed" => Some(event(EventKind::TurnComplete).tokens(TokenUsage {
            input: number_path(&raw, &[&["usage", "input_tokens"], &["input_tokens"]]),
            output: number_path(&raw, &[&["usage", "output_tokens"], &["output_tokens"]]),
            reasoning: number_path(
                &raw,
                &[&["usage", "reasoning_tokens"], &["reasoning_tokens"]],
            ),
        })),
        "turn.failed" => Some(
            event(EventKind::TurnFailed).text(
                string_path(&raw, &[&["error", "message"], &["message"], &["error"]])
                    .unwrap_or_else(|| "turn failed".to_string()),
            ),
        ),
        "error" => Some(
            event(EventKind::Error).text(
                string_path(&raw, &[&["message"], &["error", "message"]])
                    .unwrap_or_else(|| "codex error".to_string()),
            ),
        ),
        _ => Some(event(EventKind::RawPassthrough).text(line)),
    }
}

fn parse_item_started(raw: &Value) -> Option<HarnessEvent> {
    let item_type = item_string(raw, "type").unwrap_or_default();
    match item_type.as_str() {
        "agent_message" => None,
        "function_call" | "command_execution" => {
            let command = item_string(raw, "command")
                .or_else(|| string_path(raw, &[&["item", "input", "command"], &["command"]]));
            Some(event(EventKind::CommandRun).command(command))
        }
        _ => {
            let tool = item_string(raw, "name").unwrap_or(item_type);
            Some(event(EventKind::ToolStart).tool(Some(tool)))
        }
    }
}

fn parse_item_completed(raw: &Value) -> Option<HarnessEvent> {
    let item_type = item_string(raw, "type").unwrap_or_default();
    let file = file_path(raw);
    if is_file_read(&item_type, raw) {
        return Some(event(EventKind::FileRead).file(file));
    }
    if is_file_write(&item_type, raw) {
        return Some(event(EventKind::FileWrite).file(file));
    }

    match item_type.as_str() {
        "agent_message" => Some(
            event(EventKind::Response).text(
                item_string(raw, "text")
                    .or_else(|| string_field(raw, &["content", "text", "message"]))
                    .unwrap_or_default(),
            ),
        ),
        "function_call" | "command_execution" => Some(
            event(EventKind::ToolEnd)
                .tool(Some(item_type))
                .command(item_string(raw, "command").or_else(|| string_field(raw, &["command"]))),
        ),
        "" => Some(event(EventKind::RawPassthrough).text(raw.to_string())),
        _ => {
            let tool = item_string(raw, "name").unwrap_or(item_type);
            Some(event(EventKind::ToolEnd).tool(Some(tool)))
        }
    }
}

fn is_file_read(item_type: &str, raw: &Value) -> bool {
    matches!(item_type, "file_read" | "read_file")
        || matches!(
            item_string(raw, "name").as_deref(),
            Some("read_file" | "Read")
        )
}

fn is_file_write(item_type: &str, raw: &Value) -> bool {
    matches!(item_type, "file_change" | "file_write" | "write_file")
        || raw
            .get("change_type")
            .and_then(Value::as_str)
            .is_some_and(|change| !change.is_empty())
        || matches!(
            item_string(raw, "name").as_deref(),
            Some("write_file" | "Edit" | "Write")
        )
}

fn file_path(raw: &Value) -> Option<String> {
    string_path(
        raw,
        &[
            &["file_path"],
            &["path"],
            &["item", "file_path"],
            &["item", "path"],
            &["item", "input", "file_path"],
            &["item", "input", "path"],
            &["item", "parameters", "file_path"],
            &["item", "parameters", "path"],
        ],
    )
}

fn item_string(raw: &Value, key: &str) -> Option<String> {
    raw.get("item")
        .and_then(|item| item.get(key))
        .and_then(Value::as_str)
        .or_else(|| raw.get(key).and_then(Value::as_str))
        .map(str::to_string)
}

fn string_field(raw: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| raw.get(*key).and_then(Value::as_str))
        .map(str::to_string)
}

fn string_path(raw: &Value, paths: &[&[&str]]) -> Option<String> {
    paths
        .iter()
        .find_map(|path| {
            path.iter()
                .try_fold(raw, |value, key| value.get(*key))
                .and_then(Value::as_str)
        })
        .map(str::to_string)
}

fn number_path(raw: &Value, paths: &[&[&str]]) -> u64 {
    paths
        .iter()
        .find_map(|path| {
            path.iter()
                .try_fold(raw, |value, key| value.get(*key))
                .and_then(Value::as_u64)
        })
        .unwrap_or(0)
}

fn event(kind: EventKind) -> HarnessEvent {
    HarnessEvent {
        kind,
        tool: None,
        file: None,
        command: None,
        text: None,
        tokens: None,
        session_id: None,
    }
}

trait EventBuilder {
    fn tool(self, tool: Option<String>) -> Self;
    fn file(self, file: Option<String>) -> Self;
    fn command(self, command: Option<String>) -> Self;
    fn text(self, text: impl Into<String>) -> Self;
    fn tokens(self, tokens: TokenUsage) -> Self;
    fn session_id(self, session_id: Option<String>) -> Self;
}

impl EventBuilder for HarnessEvent {
    fn tool(mut self, tool: Option<String>) -> Self {
        self.tool = tool;
        self
    }

    fn file(mut self, file: Option<String>) -> Self {
        self.file = file;
        self
    }

    fn command(mut self, command: Option<String>) -> Self {
        self.command = command;
        self
    }

    fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    fn tokens(mut self, tokens: TokenUsage) -> Self {
        self.tokens = Some(tokens);
        self
    }

    fn session_id(mut self, session_id: Option<String>) -> Self {
        self.session_id = session_id;
        self
    }
}

struct ActivityTracker {
    activity: Activity,
    tokens: TokenUsage,
    last_activity: String,
}

impl ActivityTracker {
    fn new() -> Self {
        Self {
            activity: Activity::default(),
            tokens: TokenUsage::default(),
            last_activity: "dispatch started".to_string(),
        }
    }

    fn apply(&mut self, event: HarnessEvent, emitter: &Emitter) -> Result<(), DispatchError> {
        match event.kind {
            EventKind::SessionStart => {
                self.last_activity = format!(
                    "session started: {}",
                    event.session_id.unwrap_or_else(|| "unknown".to_string())
                );
            }
            EventKind::ToolStart => {
                let tool = event.tool.unwrap_or_else(|| "tool".to_string());
                self.activity.tool_calls += 1;
                self.last_activity = format!("tool started: {tool}");
                emitter
                    .emit_tool_start(tool)
                    .map_err(dispatch_event_error)?;
            }
            EventKind::ToolEnd => {
                let tool = event.tool.unwrap_or_else(|| "tool".to_string());
                self.last_activity = format!("tool ended: {tool}");
                emitter.emit_tool_end(tool).map_err(dispatch_event_error)?;
            }
            EventKind::FileWrite => {
                if let Some(path) = event.file {
                    push_unique(&mut self.activity.files_changed, path.clone());
                    self.last_activity = format!("file changed: {path}");
                    emitter
                        .emit_file_write(path)
                        .map_err(dispatch_event_error)?;
                }
            }
            EventKind::FileRead => {
                if let Some(path) = event.file {
                    push_unique(&mut self.activity.files_read, path.clone());
                    self.last_activity = format!("file read: {path}");
                    emitter.emit_file_read(path).map_err(dispatch_event_error)?;
                }
            }
            EventKind::CommandRun => {
                if let Some(command) = event.command {
                    self.activity.commands_run.push(command.clone());
                    self.last_activity = format!("command: {command}");
                    emitter
                        .emit_command_run(command)
                        .map_err(dispatch_event_error)?;
                }
            }
            EventKind::Response => {
                self.last_activity = "agent response".to_string();
            }
            EventKind::TurnComplete => {
                if let Some(tokens) = event.tokens {
                    self.tokens.input += tokens.input;
                    self.tokens.output += tokens.output;
                    self.tokens.reasoning += tokens.reasoning;
                    self.last_activity = "turn completed".to_string();
                }
            }
            EventKind::TurnFailed => {
                let message = event.text.unwrap_or_else(|| "turn failed".to_string());
                self.last_activity = format!("turn failed: {message}");
                emitter
                    .emit_error("turn_failed", message)
                    .map_err(dispatch_event_error)?;
            }
            EventKind::Error => {
                let message = event.text.unwrap_or_else(|| "codex error".to_string());
                self.last_activity = format!("error: {message}");
                emitter
                    .emit_error("codex_error", message)
                    .map_err(dispatch_event_error)?;
            }
            EventKind::RawPassthrough => {
                self.activity.parse_errors += 1;
                self.last_activity = "raw passthrough".to_string();
            }
        }

        Ok(())
    }
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

fn finalize_result(
    dispatch_id: String,
    duration_ms: u64,
    exit: ExitResult,
    timed_out: bool,
    output: Option<&Path>,
    tracker: ActivityTracker,
    stderr: String,
) -> DispatchResult {
    let output_exists = output.is_some_and(Path::exists);
    let status = if timed_out {
        DispatchStatus::TimedOut
    } else if exit.code == Some(0) && output_exists {
        DispatchStatus::Completed
    } else {
        DispatchStatus::Failed
    };

    let error = match status {
        DispatchStatus::Completed => None,
        DispatchStatus::TimedOut => Some(DispatchErrorInfo {
            code: "timed_out".to_string(),
            message: "dispatch timed out".to_string(),
            retryable: true,
        }),
        DispatchStatus::Failed => Some(DispatchErrorInfo {
            code: if exit.code == Some(0) && !output_exists {
                "missing_output"
            } else {
                "process_failed"
            }
            .to_string(),
            message: failure_message(exit, output_exists, &stderr),
            retryable: true,
        }),
    };

    DispatchResult {
        status,
        dispatch_id,
        duration_ms,
        exit_code: exit.code,
        signal: exit.signal,
        output: output.map(|path| path.display().to_string()),
        output_exists,
        error,
        tokens: tracker.tokens,
        activity: tracker.activity,
    }
}

fn failure_message(exit: ExitResult, output_exists: bool, stderr: &str) -> String {
    if exit.code == Some(0) && !output_exists {
        return "child exited successfully but output file was not created".to_string();
    }
    let stderr = stderr.trim();
    if stderr.is_empty() {
        format!(
            "child process failed: exit code {:?}, signal {:?}",
            exit.code, exit.signal
        )
    } else {
        format!(
            "child process failed: exit code {:?}, signal {:?}: {}",
            exit.code, exit.signal, stderr
        )
    }
}

fn write_status_json_atomic(
    dir: &Path,
    state: &str,
    dispatch_id: &str,
    elapsed_s: u64,
    last_activity: String,
    tools_used: u64,
    files_changed: u64,
) -> io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let status = LiveStatus {
        state: state.to_string(),
        elapsed_s,
        last_activity,
        tools_used,
        files_changed,
        dispatch_id: dispatch_id.to_string(),
        ts: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };
    let final_path = dir.join("status.json");
    let tmp_path = dir.join(format!("status.json.tmp.{}", std::process::id()));
    {
        let mut file = std::fs::File::create(&tmp_path)?;
        serde_json::to_writer(&mut file, &status)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    std::fs::rename(&tmp_path, final_path)?;
    if let Ok(dir_file) = std::fs::File::open(dir) {
        let _ = dir_file.sync_all();
    }
    Ok(())
}

fn status_dir(dispatch_id: &str) -> PathBuf {
    std::env::temp_dir()
        .join("darkfactory-dispatch")
        .join(dispatch_id)
}

fn effective_grace_sec(timeout_sec: u64, grace_sec: u64) -> u64 {
    if grace_sec == 0 {
        (timeout_sec / 2).max(1)
    } else {
        grace_sec
    }
}

fn status_name(status: &DispatchStatus) -> &'static str {
    match status {
        DispatchStatus::Completed => "completed",
        DispatchStatus::TimedOut => "timed_out",
        DispatchStatus::Failed => "failed",
    }
}

fn map_supervisor_error(error: SupervisorError) -> DispatchError {
    match error {
        SupervisorError::Spawn { program, message }
            if program == "codex" && message.contains("No such file") =>
        {
            DispatchError::BinaryNotFound
        }
        other => DispatchError::Other(other.to_string()),
    }
}

fn dispatch_event_error(error: impl std::fmt::Display) -> DispatchError {
    DispatchError::Other(format!("emit dispatch event: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_args_include_expected_flags() {
        let spec = DispatchSpec {
            model: "gpt-5".to_string(),
            effort: Some("high".to_string()),
            timeout_sec: 10,
            grace_sec: 0,
            sandbox: String::new(),
            configs: vec!["foo=bar".to_string()],
            prompt: "do work".to_string(),
            output: Some(PathBuf::from("out.txt")),
            cwd: PathBuf::from("."),
            log: None,
            store_dir: None,
        };

        let args = build_codex_args(&spec)
            .into_iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            args,
            vec![
                "exec",
                "--json",
                "--full-auto",
                "-s",
                "workspace-write",
                "-m",
                "gpt-5",
                "-c",
                "model_reasoning_effort=\"high\"",
                "-c",
                "foo=bar",
                "-o",
                "out.txt",
                "do work",
            ]
        );
    }

    #[test]
    fn malformed_json_is_raw_passthrough() {
        let event = parse_codex_event("not json").expect("event");
        assert_eq!(event.kind, EventKind::RawPassthrough);
        assert_eq!(event.text.as_deref(), Some("not json"));
    }
}
