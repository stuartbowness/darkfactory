use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant as StdInstant;

use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Instant, Sleep};

use crate::event::Emitter;
use crate::supervisor::{ExitResult, StdinMode, SupervisorBuilder, SupervisorError};
use crate::types::{
    Activity, DispatchError, DispatchErrorInfo, DispatchResult, DispatchSpec, DispatchStatus,
    EventKind, HarnessEvent, TokenUsage, DEFAULT_HEARTBEAT_INTERVAL_SECS,
};

const STDOUT_BUFFER_BYTES: usize = 4 * 1024 * 1024;
const DISPATCH_DEPTH_ENV: &str = "DARKFACTORY_DISPATCH_DEPTH";

pub(crate) async fn dispatch(
    spec: DispatchSpec,
    dispatch_id: String,
    emitter: Arc<Emitter>,
) -> Result<DispatchResult, DispatchError> {
    let started = StdInstant::now();
    let grace_sec = effective_grace_sec(spec.timeout_sec, spec.grace_sec);
    let heartbeat_sec = effective_heartbeat_sec(spec.heartbeat_sec);

    emitter
        .emit_dispatch_start("codex", spec.model.clone())
        .map_err(dispatch_event_error)?;

    let depth = dispatch_depth();
    let args = build_codex_args(&spec, depth);
    // Fail before spawning if the log destination is unusable. Otherwise this
    // setup error can leave a worker running outside the supervision loop.
    let log = open_log(spec.log.as_deref()).await?;
    let run_handle = SupervisorBuilder::new("codex")
        .args(args)
        .current_dir(&spec.cwd)
        .env(DISPATCH_DEPTH_ENV, (depth + 1).to_string())
        .stdin(StdinMode::Null)
        .parent_death_watch(true)
        .spawn()
        .map_err(map_supervisor_error)?;

    let stdout = run_handle.stdout().map_err(map_supervisor_error)?;
    let stderr = run_handle.stderr().map_err(map_supervisor_error)?;
    let stderr_task = tokio::spawn(capture_stderr(stderr, log.clone()));

    let mut stdout_reader = BufReader::with_capacity(STDOUT_BUFFER_BYTES, stdout).lines();
    let mut tracker = ActivityTracker::new();
    let output_path = spec.output.clone();

    let soft_sleep = time::sleep(Duration::from_secs(spec.timeout_sec));
    tokio::pin!(soft_sleep);
    let mut soft_armed = true;
    let mut stdout_open = true;
    let mut hard_sleep: Option<Pin<Box<Sleep>>> = None;
    let mut heartbeat_interval = time::interval_at(
        Instant::now() + Duration::from_secs(heartbeat_sec),
        Duration::from_secs(heartbeat_sec),
    );

    let (exit, timed_out, terminal_error): (ExitResult, bool, Option<DispatchErrorInfo>) = loop {
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
                // Drain remaining stdout with a 2s ceiling -- child may have
                // written final NDJSON lines (token counts, activity) just
                // before exiting. The ceiling prevents hanging when a
                // grandchild keeps stdout open.
                let drain_deadline = time::sleep(Duration::from_secs(2));
                tokio::pin!(drain_deadline);
                loop {
                    tokio::select! {
                        line = stdout_reader.next_line() => {
                            match line {
                                Ok(Some(line)) => {
                                    write_log_line(&log, &line).await?;
                                    if let Some(event) = parse_codex_event(&line) {
                                        tracker.apply(event, &emitter)?;
                                    }
                                }
                                _ => break,
                            }
                        }
                        () = &mut drain_deadline => break,
                    }
                }
                // If soft timeout already fired (SIGTERM sent), this is still a timeout
                break (result.map_err(map_supervisor_error)?, !soft_armed, None);
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
                // Drain remaining stdout with a 1s ceiling after kill
                let drain_deadline = time::sleep(Duration::from_secs(1));
                tokio::pin!(drain_deadline);
                loop {
                    tokio::select! {
                        line = stdout_reader.next_line() => {
                            match line {
                                Ok(Some(line)) => {
                                    write_log_line(&log, &line).await?;
                                    if let Some(event) = parse_codex_event(&line) {
                                        tracker.apply(event, &emitter)?;
                                    }
                                }
                                _ => break,
                            }
                        }
                        () = &mut drain_deadline => break,
                    }
                }
                break (exit, true, None);
            }

            _ = heartbeat_interval.tick() => {
                emitter
                    .emit_heartbeat_with_interval(
                        tracker.last_activity.clone(),
                        heartbeat_sec,
                    )
                    .map_err(dispatch_event_error)?;
            }
        }
    };

    let stderr = stderr_task
        .await
        .map_err(|error| DispatchError::Other(format!("stderr capture task failed: {error}")))??;

    // If the worker exited cleanly but never created the requested output file,
    // materialize it from the final agent message captured off the NDJSON stream.
    // This recovers analysis-only runs and sandbox-blocked writes. Never overwrite
    // a file the worker already wrote.
    let mut terminal_error = terminal_error;
    if terminal_error.is_none() && !timed_out && exit.code == Some(0) {
        terminal_error =
            materialize_missing_output(output_path.as_deref(), tracker.final_response.as_deref());
    }

    let result = finalize_result(ResultFinalization {
        dispatch_id,
        duration_ms: started.elapsed().as_millis() as u64,
        exit,
        timed_out,
        output: output_path,
        terminal_error,
        tracker,
        stderr,
    });
    emitter
        .emit_dispatch_end(result.status.as_str(), result.duration_ms)
        .map_err(dispatch_event_error)?;

    Ok(result)
}

fn build_codex_args(spec: &DispatchSpec, depth: u64) -> Vec<OsString> {
    let sandbox = if spec.sandbox.is_empty() {
        "workspace-write"
    } else {
        spec.sandbox.as_str()
    };
    let mut args = vec![
        "exec".into(),
        "--json".into(),
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

    let prompt = if depth > 0 {
        format!(
            "[DISPATCH CONTEXT] You are running inside a darkfactory-dispatch worker (depth={depth}). \
             Do not call darkfactory-dispatch or create nested dispatches. \
             Execute all work directly.\n\
             ---\n\
             {}", spec.prompt
        )
    } else {
        spec.prompt.clone()
    };
    args.push(prompt.into());

    args
}

fn dispatch_depth() -> u64 {
    std::env::var(DISPATCH_DEPTH_ENV)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
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
        _ => None,
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
            Some("read" | "read_file" | "Read")
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
            Some("write" | "write_file" | "Edit" | "Write")
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
    response_seen: bool,
    final_response: Option<String>,
}

impl ActivityTracker {
    fn new() -> Self {
        Self {
            activity: Activity::default(),
            tokens: TokenUsage::default(),
            last_activity: "dispatch started".to_string(),
            response_seen: false,
            final_response: None,
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
                self.response_seen = true;
                self.final_response = event.text;
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

struct ResultFinalization {
    dispatch_id: String,
    duration_ms: u64,
    exit: ExitResult,
    timed_out: bool,
    output: Option<PathBuf>,
    terminal_error: Option<DispatchErrorInfo>,
    tracker: ActivityTracker,
    stderr: String,
}

fn finalize_result(input: ResultFinalization) -> DispatchResult {
    let output_requested = input.output.is_some();
    let output_exists = input.output.as_deref().is_some_and(Path::exists);
    // A run with no requested output file is satisfied by a clean exit alone.
    let output_satisfied = !output_requested || output_exists;
    let mut status = if input.terminal_error.is_some() {
        DispatchStatus::Failed
    } else if input.timed_out {
        DispatchStatus::TimedOut
    } else if input.exit.code == Some(0) && output_satisfied {
        DispatchStatus::Completed
    } else {
        DispatchStatus::Failed
    };

    let mut error = if let Some(error) = input.terminal_error {
        Some(error)
    } else {
        match status {
            DispatchStatus::Completed => None,
            DispatchStatus::TimedOut => Some(DispatchErrorInfo {
                code: "timed_out".to_string(),
                message: "dispatch timed out".to_string(),
                retryable: true,
            }),
            DispatchStatus::Failed => Some(DispatchErrorInfo {
                code: if input.exit.code == Some(0) && output_requested && !output_exists {
                    "missing_output"
                } else {
                    "process_failed"
                }
                .to_string(),
                message: failure_message(
                    input.exit,
                    output_requested,
                    output_exists,
                    &input.stderr,
                ),
                retryable: true,
            }),
        }
    };

    if status == DispatchStatus::Completed
        && input.tracker.activity.parse_errors > 0
        && !input.tracker.response_seen
    {
        status = DispatchStatus::Failed;
        error = Some(output_parse_error(input.tracker.activity.parse_errors));
    }

    DispatchResult {
        status,
        dispatch_id: input.dispatch_id,
        duration_ms: input.duration_ms,
        exit_code: input.exit.code,
        signal: input.exit.signal,
        output: input.output.map(|path| path.display().to_string()),
        output_exists,
        error,
        tokens: input.tracker.tokens,
        activity: input.tracker.activity,
    }
}

fn output_parse_error(parse_errors: u64) -> DispatchErrorInfo {
    DispatchErrorInfo {
        code: "output_parse_error".to_string(),
        message: format!(
            "Harness output contained {parse_errors} parse error(s) and no final response could be trusted."
        ),
        retryable: true,
    }
}

fn failure_message(
    exit: ExitResult,
    output_requested: bool,
    output_exists: bool,
    stderr: &str,
) -> String {
    if exit.code == Some(0) && output_requested && !output_exists {
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

// Worker exited cleanly but the requested output file is absent. If a final agent
// message was captured off the NDJSON stream, write it to the output path from the
// supervisor (which is not subject to the worker's sandbox). Returns a terminal
// error only when the write itself fails.
fn materialize_missing_output(
    output: Option<&Path>,
    final_response: Option<&str>,
) -> Option<DispatchErrorInfo> {
    let (Some(path), Some(response)) = (output, final_response) else {
        return None;
    };
    // An empty final message is not a recovered output; let it fall through to
    // missing_output rather than writing a 0-byte file and reporting success.
    if response.is_empty() {
        return None;
    }
    match write_output_no_clobber(path, response) {
        Ok(()) => None,
        // The worker's own write wins. `create_new` fails with AlreadyExists when
        // the file already exists, so the supervisor never overwrites worker output.
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => None,
        Err(error) => Some(DispatchErrorInfo {
            code: "output_write_failed".to_string(),
            message: format!(
                "failed to write captured response to {}: {error}",
                path.display()
            ),
            retryable: true,
        }),
    }
}

// Write the captured response only if the output path does not already exist.
// `create_new` (O_EXCL) fuses the existence check and the create into one atomic
// step, so a file the worker wrote is never clobbered -- no time-of-check gap.
fn write_output_no_clobber(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(contents.as_bytes())
}

fn effective_grace_sec(timeout_sec: u64, grace_sec: u64) -> u64 {
    if grace_sec == 0 {
        (timeout_sec / 2).max(1)
    } else {
        grace_sec
    }
}

fn effective_heartbeat_sec(heartbeat_sec: u64) -> u64 {
    if heartbeat_sec == 0 {
        DEFAULT_HEARTBEAT_INTERVAL_SECS
    } else {
        heartbeat_sec
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
            model: "gpt-5.5".to_string(),
            effort: Some("high".to_string()),
            timeout_sec: 10,
            grace_sec: 0,
            heartbeat_sec: DEFAULT_HEARTBEAT_INTERVAL_SECS,
            sandbox: String::new(),
            configs: vec!["foo=bar".to_string()],
            prompt: "do work".to_string(),
            output: Some(PathBuf::from("out.txt")),
            cwd: PathBuf::from("."),
            log: None,
        };

        let args = build_codex_args(&spec, 0)
            .into_iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            args,
            vec![
                "exec",
                "--json",
                "-s",
                "workspace-write",
                "-m",
                "gpt-5.5",
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
    fn codex_args_prepend_dispatch_context_at_depth_1() {
        let spec = DispatchSpec {
            model: "gpt-5.5".to_string(),
            effort: None,
            timeout_sec: 10,
            grace_sec: 0,
            heartbeat_sec: DEFAULT_HEARTBEAT_INTERVAL_SECS,
            sandbox: "read-only".to_string(),
            configs: vec![],
            prompt: "review this artifact".to_string(),
            output: None,
            cwd: PathBuf::from("."),
            log: None,
        };

        let args = build_codex_args(&spec, 1)
            .into_iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        let prompt = args.last().expect("args should have a prompt");
        assert!(prompt.starts_with("[DISPATCH CONTEXT]"));
        assert!(prompt.contains("depth=1"));
        assert!(prompt.contains("Do not call darkfactory-dispatch"));
        assert!(prompt.ends_with("review this artifact"));
    }

    #[test]
    fn codex_args_no_preamble_at_depth_0() {
        let spec = DispatchSpec {
            model: "gpt-5.5".to_string(),
            effort: None,
            timeout_sec: 10,
            grace_sec: 0,
            heartbeat_sec: DEFAULT_HEARTBEAT_INTERVAL_SECS,
            sandbox: "read-only".to_string(),
            configs: vec![],
            prompt: "do work".to_string(),
            output: None,
            cwd: PathBuf::from("."),
            log: None,
        };

        let args = build_codex_args(&spec, 0)
            .into_iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        let prompt = args.last().expect("args should have a prompt");
        assert_eq!(prompt, "do work");
    }

    #[test]
    fn effective_grace_defaults_to_half_timeout() {
        assert_eq!(effective_grace_sec(900, 0), 450);
    }

    #[test]
    fn effective_grace_has_one_second_minimum() {
        assert_eq!(effective_grace_sec(1, 0), 1);
        assert_eq!(effective_grace_sec(0, 0), 1);
    }

    #[test]
    fn effective_grace_uses_explicit_value() {
        assert_eq!(effective_grace_sec(900, 30), 30);
    }

    #[test]
    fn malformed_json_is_raw_passthrough() {
        let event = parse_codex_event("not json").expect("event");
        assert_eq!(event.kind, EventKind::RawPassthrough);
        assert_eq!(event.text.as_deref(), Some("not json"));
    }

    #[test]
    fn parse_codex_event_table() {
        let cases = vec![
            (
                "thread.started",
                r#"{"type":"thread.started","thread_id":"test-abc-123"}"#,
                Some(EventKind::SessionStart),
            ),
            (
                "item.started tool_use",
                r#"{"type":"item.started","item":{"type":"tool_use","name":"shell"}}"#,
                Some(EventKind::ToolStart),
            ),
            (
                "item.started agent_message",
                r#"{"type":"item.started","item":{"type":"agent_message"}}"#,
                None,
            ),
            (
                "item.started command_execution",
                r#"{"type":"item.started","item":{"type":"command_execution","command":"cargo test"}}"#,
                Some(EventKind::CommandRun),
            ),
            (
                "item.completed agent_message",
                r#"{"type":"item.completed","item":{"type":"agent_message","text":"I created the file."}}"#,
                Some(EventKind::Response),
            ),
            (
                "item.completed tool_use",
                r#"{"type":"item.completed","item":{"type":"tool_use","name":"shell"}}"#,
                Some(EventKind::ToolEnd),
            ),
            (
                "item.completed command_execution",
                r#"{"type":"item.completed","item":{"type":"command_execution","command":"cargo test"}}"#,
                Some(EventKind::ToolEnd),
            ),
            (
                "item.completed file write",
                r#"{"type":"item.completed","item":{"type":"tool_use","name":"write","file_path":"src/main.rs"}}"#,
                Some(EventKind::FileWrite),
            ),
            (
                "item.completed file read",
                r#"{"type":"item.completed","item":{"type":"tool_use","name":"read","file_path":"Cargo.toml"}}"#,
                Some(EventKind::FileRead),
            ),
            (
                "turn.completed",
                r#"{"type":"turn.completed","usage":{"input_tokens":100,"output_tokens":50,"reasoning_tokens":20}}"#,
                Some(EventKind::TurnComplete),
            ),
            (
                "turn.failed",
                r#"{"type":"turn.failed","error":{"message":"rate limited"}}"#,
                Some(EventKind::TurnFailed),
            ),
            (
                "error",
                r#"{"type":"error","error":{"message":"connection reset"}}"#,
                Some(EventKind::Error),
            ),
            (
                "non-json line",
                "This is not JSON at all",
                Some(EventKind::RawPassthrough),
            ),
            (
                "unknown json type",
                r#"{"type":"unknown.future.event","data":"something"}"#,
                None,
            ),
            ("turn.started", r#"{"type":"turn.started"}"#, None),
            ("item.updated", r#"{"type":"item.updated","item":{}}"#, None),
            ("empty line", "", None),
        ];

        for (name, input, expected_kind) in cases {
            let result = parse_codex_event(input);
            match expected_kind {
                Some(kind) => {
                    let event = result.unwrap_or_else(|| panic!("{name}: should parse"));
                    assert_eq!(event.kind, kind, "{name}: wrong EventKind");
                }
                None => assert!(result.is_none(), "{name}: should return None"),
            }
        }
    }

    #[test]
    fn parse_codex_event_extracts_session_id() {
        let event = parse_codex_event(r#"{"type":"thread.started","thread_id":"test-abc-123"}"#)
            .expect("thread.started should parse");

        assert_eq!(event.session_id.as_deref(), Some("test-abc-123"));
    }

    #[test]
    fn parse_codex_event_extracts_tool_names() {
        let started = parse_codex_event(
            r#"{"type":"item.started","item":{"type":"tool_use","name":"shell"}}"#,
        )
        .expect("item.started tool_use should parse");
        let completed = parse_codex_event(
            r#"{"type":"item.completed","item":{"type":"tool_use","name":"shell"}}"#,
        )
        .expect("item.completed tool_use should parse");

        assert_eq!(started.tool.as_deref(), Some("shell"));
        assert_eq!(completed.tool.as_deref(), Some("shell"));
    }

    #[test]
    fn parse_codex_event_extracts_commands() {
        let started = parse_codex_event(
            r#"{"type":"item.started","item":{"type":"command_execution","command":"cargo test"}}"#,
        )
        .expect("command_execution start should parse");
        let completed = parse_codex_event(
            r#"{"type":"item.completed","item":{"type":"command_execution","command":"cargo test"}}"#,
        )
        .expect("command_execution completion should parse");

        assert_eq!(started.command.as_deref(), Some("cargo test"));
        assert_eq!(completed.command.as_deref(), Some("cargo test"));
    }

    #[test]
    fn parse_codex_event_extracts_response_and_error_text() {
        let response = parse_codex_event(
            r#"{"type":"item.completed","item":{"type":"agent_message","text":"I created the file."}}"#,
        )
        .expect("agent_message should parse");
        let failed =
            parse_codex_event(r#"{"type":"turn.failed","error":{"message":"rate limited"}}"#)
                .expect("turn.failed should parse");
        let error = parse_codex_event(r#"{"type":"error","error":{"message":"connection reset"}}"#)
            .expect("error should parse");

        assert_eq!(response.text.as_deref(), Some("I created the file."));
        assert_eq!(failed.text.as_deref(), Some("rate limited"));
        assert_eq!(error.text.as_deref(), Some("connection reset"));
    }

    #[test]
    fn parse_codex_event_extracts_file_paths() {
        let write = parse_codex_event(
            r#"{"type":"item.completed","item":{"type":"tool_use","name":"write","file_path":"src/main.rs"}}"#,
        )
        .expect("file write should parse");
        let read = parse_codex_event(
            r#"{"type":"item.completed","item":{"type":"tool_use","name":"read","file_path":"Cargo.toml"}}"#,
        )
        .expect("file read should parse");

        assert_eq!(write.file.as_deref(), Some("src/main.rs"));
        assert_eq!(read.file.as_deref(), Some("Cargo.toml"));
    }

    #[test]
    fn parse_codex_event_extracts_token_counts() {
        let event = parse_codex_event(
            r#"{"type":"turn.completed","usage":{"input_tokens":100,"output_tokens":50,"reasoning_tokens":20}}"#,
        )
        .expect("turn.completed should parse");
        let tokens = event.tokens.expect("turn.completed should include tokens");

        assert_eq!(
            tokens,
            TokenUsage {
                input: 100,
                output: 50,
                reasoning: 20,
            }
        );
    }
}
