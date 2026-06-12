use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use clap::{Parser, Subcommand};
use nix::libc;
use serde::Serialize;
use ulid::Ulid;

use crate::types::{
    DispatchErrorInfo, DispatchResult, DispatchSpec, DispatchStatus, LiveStatus,
    DEFAULT_HEARTBEAT_INTERVAL_SECS,
};

const DEFAULT_MODEL: &str = "gpt-5.5";
const DISPATCH_RETENTION_SECS: u64 = 7 * 24 * 60 * 60;

#[derive(Parser)]
#[command(
    name = "darkfactory-dispatch",
    version,
    about = "Process supervisor for Codex CLI subprocesses",
    after_help = "\
EXAMPLES:
    # Sync dispatch with JSON result on stdout
    darkfactory-dispatch \\
      --model gpt-5.5 --effort high --timeout 900 \\
      --sandbox workspace-write \\
      --prompt \"Create hello.py that prints hello world\" \\
      --json

    # Dispatch with prompt file, output path, and log
    darkfactory-dispatch \\
      --model gpt-5.5 --effort high --timeout 1800 \\
      --sandbox workspace-write \\
      --config 'sandbox_workspace_write.writable_roots=[\".git\"]' \\
      --prompt-file brief.md --output handoff.md \\
      --cwd /path/to/repo --log dispatch.log \\
      --json

    # Async dispatch with lifecycle management
    darkfactory-dispatch --async --prompt \"Refactor auth module\" --timeout 1800
    darkfactory-dispatch status <dispatch-id>
    darkfactory-dispatch wait <dispatch-id>
    darkfactory-dispatch result <dispatch-id>

RESULT JSON:
    Sync mode (--json) emits a single JSON object on stdout:
    {\"status\":\"completed|timed_out|failed\", \"dispatch_id\":\"...\",
     \"duration_ms\":N, \"exit_code\":N, \"signal\":null,
     \"output\":\"/path\", \"output_exists\":true,
     \"tokens\":{\"input\":N,\"output\":N,\"reasoning\":N},
     \"activity\":{\"files_changed\":[...],\"commands_run\":[...]}}
    Stdout carries ONLY this object; all events and diagnostics go to stderr.
    Do not merge the streams (2>&1) when parsing -- redirect stderr (e.g. 2>>log).

    Async mode (--async) emits {\"kind\":\"async_started\",\"dispatch_id\":\"...\"}
    then continues in the background. Use 'result <id>' to collect."
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    #[arg(long)]
    pub(crate) model: Option<String>,

    #[arg(long)]
    pub(crate) effort: Option<String>,

    #[arg(long, default_value = "900")]
    pub(crate) timeout: u64,

    #[arg(long)]
    pub(crate) grace: Option<u64>,

    #[arg(long, default_value_t = DEFAULT_HEARTBEAT_INTERVAL_SECS)]
    pub(crate) heartbeat: u64,

    #[arg(long, default_value = "workspace-write")]
    pub(crate) sandbox: String,

    #[arg(long = "config", num_args = 1)]
    pub(crate) configs: Vec<String>,

    #[arg(long, conflicts_with = "prompt_file")]
    pub(crate) prompt: Option<String>,

    #[arg(long, conflicts_with = "prompt")]
    pub(crate) prompt_file: Option<PathBuf>,

    #[arg(long)]
    pub(crate) output: Option<PathBuf>,

    #[arg(long)]
    pub(crate) cwd: Option<PathBuf>,

    #[arg(long)]
    pub(crate) log: Option<PathBuf>,

    #[arg(long, conflicts_with = "async_mode")]
    pub(crate) json: bool,

    #[arg(long = "async", conflicts_with = "json")]
    pub(crate) async_mode: bool,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Status { id: String },
    Result { id: String },
    Wait { id: String },
}

impl Cli {
    pub(crate) fn dispatch_spec(self) -> Result<DispatchSpec, String> {
        if self.heartbeat == 0 {
            return Err("--heartbeat must be greater than 0".to_string());
        }

        let prompt = match (self.prompt, self.prompt_file) {
            (Some(prompt), None) => {
                if prompt.trim().is_empty() {
                    return Err("prompt is empty: provide a non-empty --prompt".to_string());
                }
                prompt
            }
            (None, Some(path)) => {
                let prompt = fs::read_to_string(&path)
                    .map_err(|error| format!("read prompt file {}: {error}", path.display()))?;
                if prompt.trim().is_empty() {
                    return Err(format!("prompt file {} is empty", path.display()));
                }
                prompt
            }
            (None, None) => {
                return Err("missing prompt: provide --prompt or --prompt-file".to_string());
            }
            (Some(_), Some(_)) => {
                return Err("--prompt and --prompt-file cannot be used together".to_string());
            }
        };

        let cwd = match self.cwd {
            Some(cwd) => cwd,
            None => {
                std::env::current_dir().map_err(|error| format!("read current dir: {error}"))?
            }
        };

        Ok(DispatchSpec {
            model: self.model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            effort: self.effort,
            timeout_sec: self.timeout,
            grace_sec: self.grace.unwrap_or_else(|| (self.timeout / 2).max(1)),
            heartbeat_sec: self.heartbeat,
            sandbox: self.sandbox,
            configs: self.configs,
            prompt,
            output: self.output,
            cwd,
            log: self.log,
            store_dir: None,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct AsyncStarted<'a> {
    pub(crate) kind: &'a str,
    pub(crate) dispatch_id: &'a str,
}

pub(crate) struct HostPidGuard {
    path: PathBuf,
}

impl Drop for HostPidGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub(crate) fn dispatch_store_root() -> Result<PathBuf, String> {
    let home = std::env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    Ok(PathBuf::from(home).join(".darkfactory").join("dispatches"))
}

pub(crate) fn prepare_async_store(dispatch_id: &str) -> Result<PathBuf, String> {
    let root = dispatch_store_root()?;
    fs::create_dir_all(&root)
        .map_err(|error| format!("create dispatch store {}: {error}", root.display()))?;
    prune_old_dispatches(&root).map_err(|error| format!("prune dispatch store: {error}"))?;
    let store_dir = root.join(dispatch_id);
    fs::create_dir_all(&store_dir)
        .map_err(|error| format!("create dispatch dir {}: {error}", store_dir.display()))?;
    Ok(store_dir)
}

pub(crate) fn write_host_pid(store_dir: &Path) -> Result<HostPidGuard, String> {
    let path = store_dir.join("host.pid");
    write_and_sync(&path, std::process::id().to_string().as_bytes())
        .map_err(|error| format!("write host pid {}: {error}", path.display()))?;
    Ok(HostPidGuard { path })
}

pub(crate) fn open_events_log(store_dir: &Path) -> Result<File, String> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(store_dir.join("events.jsonl"))
        .map_err(|error| format!("open events log: {error}"))
}

pub(crate) fn write_initial_status(store_dir: &Path, dispatch_id: &str) -> Result<(), String> {
    let status = LiveStatus {
        state: "running".to_string(),
        elapsed_s: 0,
        last_activity: "initializing".to_string(),
        tools_used: 0,
        files_changed: 0,
        dispatch_id: dispatch_id.to_string(),
        ts: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    };
    write_json_atomic(&store_dir.join("status.json"), &status)
        .map_err(|error| format!("write initial status: {error}"))
}

pub(crate) fn print_async_started(dispatch_id: &str) -> Result<(), String> {
    serde_json::to_writer(
        std::io::stdout(),
        &AsyncStarted {
            kind: "async_started",
            dispatch_id,
        },
    )
    .map_err(|error| format!("serialize async start: {error}"))?;
    println!();
    std::io::stdout()
        .flush()
        .map_err(|error| format!("flush async start: {error}"))
}

pub(crate) fn write_result(store_dir: &Path, result: &DispatchResult) -> Result<(), String> {
    write_json_atomic(&store_dir.join("result.json"), result)
        .map_err(|error| format!("write dispatch result: {error}"))
}

pub(crate) fn print_status(id: &str) -> Result<(), String> {
    let status = read_dispatch_status(id)?;
    print_json(&status)
}

pub(crate) fn print_result_for_id(id: &str) -> Result<(), String> {
    let store_dir = dispatch_dir(id)?;
    loop {
        if let Some(result) = read_result_json(&store_dir)? {
            return print_json(&result);
        }
        match host_process_state(&store_dir)? {
            HostProcessState::Running => std::thread::sleep(Duration::from_secs(1)),
            HostProcessState::Dead => return Err(format!("dispatch {id} is orphaned")),
            HostProcessState::Missing => return Err(format!("dispatch not found: {id}")),
        }
    }
}

pub(crate) fn wait_for_id(id: &str) -> Result<u8, String> {
    let store_dir = dispatch_dir(id)?;
    loop {
        if let Some(result) = read_result_json(&store_dir)? {
            return Ok(exit_code_for_result(&result));
        }
        match host_process_state(&store_dir)? {
            HostProcessState::Dead => return Err(format!("dispatch {id} is orphaned")),
            HostProcessState::Missing => return Err(format!("dispatch not found: {id}")),
            HostProcessState::Running => {}
        }

        match read_live_status(&store_dir)? {
            Some(status) => eprintln!(
                "[{}s] {} | {} tools | {} files changed",
                status.elapsed_s, status.state, status.tools_used, status.files_changed
            ),
            None => eprintln!("[?] status unknown"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

pub(crate) fn print_result(result: &DispatchResult, json: bool) -> Result<(), String> {
    if json {
        serde_json::to_writer(std::io::stdout(), result)
            .map_err(|error| format!("serialize dispatch result: {error}"))?;
        println!();
        return Ok(());
    }

    println!("status: {}", status_name(&result.status));
    println!("duration_ms: {}", result.duration_ms);
    println!("output: {}", result.output.as_deref().unwrap_or("<none>"));
    Ok(())
}

pub(crate) fn exit_code_for_result(result: &DispatchResult) -> u8 {
    match result.status {
        DispatchStatus::Completed => 0,
        DispatchStatus::Failed | DispatchStatus::TimedOut => 1,
    }
}

fn status_name(status: &DispatchStatus) -> &'static str {
    match status {
        DispatchStatus::Completed => "completed",
        DispatchStatus::TimedOut => "timed_out",
        DispatchStatus::Failed => "failed",
    }
}

#[derive(Debug, Serialize)]
struct LifecycleStatus {
    state: String,
    dispatch_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_activity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools_used: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files_changed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<DispatchResult>,
}

enum HostProcessState {
    Running,
    Dead,
    Missing,
}

fn read_dispatch_status(id: &str) -> Result<LifecycleStatus, String> {
    let store_dir = dispatch_dir(id)?;
    if let Some(result) = read_result_json(&store_dir)? {
        return Ok(LifecycleStatus {
            state: status_name(&result.status).to_string(),
            dispatch_id: result.dispatch_id.clone(),
            elapsed_s: Some(result.duration_ms / 1000),
            last_activity: Some("done".to_string()),
            tools_used: Some(result.activity.tool_calls),
            files_changed: Some(result.activity.files_changed.len() as u64),
            result: Some(result),
        });
    }

    match host_process_state(&store_dir)? {
        HostProcessState::Running => {
            if let Some(status) = read_live_status(&store_dir)? {
                Ok(LifecycleStatus {
                    state: "running".to_string(),
                    dispatch_id: status.dispatch_id,
                    elapsed_s: Some(status.elapsed_s),
                    last_activity: Some(status.last_activity),
                    tools_used: Some(status.tools_used),
                    files_changed: Some(status.files_changed),
                    result: None,
                })
            } else {
                Ok(LifecycleStatus {
                    state: "running".to_string(),
                    dispatch_id: id.to_string(),
                    elapsed_s: None,
                    last_activity: None,
                    tools_used: None,
                    files_changed: None,
                    result: None,
                })
            }
        }
        HostProcessState::Dead => Ok(LifecycleStatus {
            state: "orphaned".to_string(),
            dispatch_id: id.to_string(),
            elapsed_s: None,
            last_activity: None,
            tools_used: None,
            files_changed: None,
            result: None,
        }),
        HostProcessState::Missing => Err(format!("dispatch not found: {id}")),
    }
}

fn dispatch_dir(id: &str) -> Result<PathBuf, String> {
    Ok(dispatch_store_root()?.join(id))
}

fn read_result_json(store_dir: &Path) -> Result<Option<DispatchResult>, String> {
    let path = store_dir.join("result.json");
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text)
            .map(Some)
            .map_err(|error| format!("read result {}: {error}", path.display())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("read result {}: {error}", path.display())),
    }
}

fn read_live_status(store_dir: &Path) -> Result<Option<LiveStatus>, String> {
    let path = store_dir.join("status.json");
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text)
            .map(Some)
            .map_err(|error| format!("read status {}: {error}", path.display())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("read status {}: {error}", path.display())),
    }
}

fn host_process_state(store_dir: &Path) -> Result<HostProcessState, String> {
    let path = store_dir.join("host.pid");
    let pid_text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(HostProcessState::Missing);
        }
        Err(error) => return Err(format!("read host pid {}: {error}", path.display())),
    };
    let pid = pid_text
        .trim()
        .parse::<i32>()
        .map_err(|error| format!("parse host pid {}: {error}", path.display()))?;
    if process_alive(pid) {
        Ok(HostProcessState::Running)
    } else {
        Ok(HostProcessState::Dead)
    }
}

fn process_alive(pid: i32) -> bool {
    if pid <= 0 {
        return false;
    }
    let rc = unsafe { libc::kill(pid, 0) };
    rc == 0 || io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

fn print_json(value: &impl Serialize) -> Result<(), String> {
    serde_json::to_writer(std::io::stdout(), value)
        .map_err(|error| format!("serialize: {error}"))?;
    println!();
    Ok(())
}

fn write_and_sync(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    sync_parent(path)?;
    Ok(())
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("path has no parent"))?;
    fs::create_dir_all(parent)?;
    let tmp_path = parent.join(format!(
        ".{}.tmp.{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("json"),
        std::process::id()
    ));
    {
        let mut file = File::create(&tmp_path)?;
        serde_json::to_writer(&mut file, value)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, path)?;
    sync_parent(path)?;
    Ok(())
}

fn sync_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        let dir = File::open(parent)?;
        dir.sync_all()?;
    }
    Ok(())
}

fn prune_old_dispatches(root: &Path) -> io::Result<()> {
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(DISPATCH_RETENTION_SECS))
        .unwrap_or(SystemTime::UNIX_EPOCH);
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }
        if dispatch_dir_is_older_than(&entry.path(), cutoff)? {
            fs::remove_dir_all(entry.path())?;
        }
    }
    Ok(())
}

fn dispatch_dir_is_older_than(path: &Path, cutoff: SystemTime) -> io::Result<bool> {
    if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
        if let Ok(ulid) = Ulid::from_string(name) {
            return Ok(ulid.datetime() < cutoff);
        }
    }
    Ok(path.metadata()?.modified()? < cutoff)
}

pub(crate) fn failed_start_result(dispatch_id: String, message: String) -> DispatchResult {
    DispatchResult {
        status: DispatchStatus::Failed,
        dispatch_id,
        duration_ms: 0,
        exit_code: None,
        signal: None,
        output: None,
        output_exists: false,
        error: Some(DispatchErrorInfo {
            code: "startup_failed".to_string(),
            message,
            retryable: true,
        }),
        tokens: Default::default(),
        activity: Default::default(),
    }
}
