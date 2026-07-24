use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::types::{
    DispatchErrorInfo, DispatchResult, DispatchSpec, DispatchStatus,
    DEFAULT_HEARTBEAT_INTERVAL_SECS,
};

const DEFAULT_MODEL: &str = "gpt-5.6-terra";
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
      --model gpt-5.6-terra --effort high --timeout 900 \\
      --sandbox workspace-write \\
      --prompt \"Create hello.py that prints hello world\" \\
      --json

    # Dispatch with prompt file, output path, and log
    darkfactory-dispatch \\
      --model gpt-5.6-terra --effort high --timeout 1800 \\
      --sandbox workspace-write \\
      --config 'sandbox_workspace_write.writable_roots=[\".git\"]' \\
      --prompt-file brief.md --output handoff.md \\
      --cwd /path/to/repo --log dispatch.log \\
      --json

RESULT JSON:
    Sync mode (--json) emits a single JSON object on stdout:
    {\"status\":\"completed|timed_out|failed\", \"dispatch_id\":\"...\",
     \"duration_ms\":N, \"exit_code\":N, \"signal\":null,
     \"output\":\"/path\", \"output_exists\":true,
     \"tokens\":{\"input\":N,\"output\":N,\"reasoning\":N},
     \"activity\":{\"files_changed\":[...],\"commands_run\":[...]}}
    Stdout carries ONLY this object; all events and diagnostics go to stderr.
    Do not merge the streams (2>&1) when parsing -- redirect stderr (e.g. 2>>log)."
)]
pub(crate) struct Cli {
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

    #[arg(long)]
    pub(crate) json: bool,
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
        })
    }
}

pub(crate) fn print_result(result: &DispatchResult, json: bool) -> Result<(), String> {
    if json {
        serde_json::to_writer(std::io::stdout(), result)
            .map_err(|error| format!("serialize dispatch result: {error}"))?;
        println!();
        return Ok(());
    }

    println!("status: {}", result.status.as_str());
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
