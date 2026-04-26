use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::types::{DispatchResult, DispatchSpec, DispatchStatus};

const DEFAULT_MODEL: &str = "gpt-5";

#[derive(Parser)]
#[command(
    name = "darkfactory-dispatch",
    version,
    about = "Process supervisor for Codex CLI subprocesses"
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
        let prompt = match (self.prompt, self.prompt_file) {
            (Some(prompt), None) => prompt,
            (None, Some(path)) => fs::read_to_string(&path)
                .map_err(|error| format!("read prompt file {}: {error}", path.display()))?,
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
