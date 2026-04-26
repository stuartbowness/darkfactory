#![cfg_attr(
    not(test),
    expect(dead_code, reason = "types are consumed by upcoming modules")
)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg(test)]
pub(crate) const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 1;
#[cfg(not(test))]
pub(crate) const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 15;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DispatchSpec {
    pub(crate) model: String,
    pub(crate) effort: Option<String>,
    pub(crate) timeout_sec: u64,
    pub(crate) grace_sec: u64,
    pub(crate) heartbeat_sec: u64,
    pub(crate) sandbox: String,
    pub(crate) configs: Vec<String>,
    pub(crate) prompt: String,
    pub(crate) output: Option<PathBuf>,
    pub(crate) cwd: PathBuf,
    pub(crate) log: Option<PathBuf>,
    pub(crate) store_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DispatchResult {
    pub(crate) status: DispatchStatus,
    pub(crate) dispatch_id: String,
    pub(crate) duration_ms: u64,
    pub(crate) exit_code: Option<i32>,
    pub(crate) signal: Option<i32>,
    pub(crate) output: Option<String>,
    pub(crate) output_exists: bool,
    pub(crate) error: Option<DispatchErrorInfo>,
    pub(crate) tokens: TokenUsage,
    pub(crate) activity: Activity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DispatchStatus {
    Completed,
    TimedOut,
    Failed,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum DispatchError {
    #[error("codex binary not found on PATH")]
    BinaryNotFound,
    #[error("process spawn failed: {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("dispatch timed out after {timeout_sec}s (grace {grace_sec}s)")]
    TimedOut { timeout_sec: u64, grace_sec: u64 },
    #[error("child process failed: exit code {exit_code:?}, signal {signal:?}")]
    ProcessFailed {
        exit_code: Option<i32>,
        signal: Option<i32>,
    },
    #[error("NDJSON parse error: {0}")]
    ParseError(String),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DispatchErrorInfo {
    pub(crate) code: String,
    pub(crate) message: String,
    pub(crate) retryable: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TokenUsage {
    pub(crate) input: u64,
    pub(crate) output: u64,
    pub(crate) reasoning: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Activity {
    pub(crate) files_changed: Vec<String>,
    pub(crate) files_read: Vec<String>,
    pub(crate) commands_run: Vec<String>,
    pub(crate) tool_calls: u64,
    #[serde(default)]
    pub(crate) parse_errors: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HarnessEvent {
    pub(crate) kind: EventKind,
    pub(crate) tool: Option<String>,
    pub(crate) file: Option<String>,
    pub(crate) command: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) tokens: Option<TokenUsage>,
    pub(crate) session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EventKind {
    SessionStart,
    ToolStart,
    ToolEnd,
    FileWrite,
    FileRead,
    CommandRun,
    Response,
    TurnComplete,
    TurnFailed,
    Error,
    RawPassthrough,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LiveStatus {
    pub(crate) state: String,
    pub(crate) elapsed_s: u64,
    pub(crate) last_activity: String,
    pub(crate) tools_used: u64,
    pub(crate) files_changed: u64,
    pub(crate) dispatch_id: String,
    pub(crate) ts: String,
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn dispatch_result_json_round_trips() {
        let result = DispatchResult {
            status: DispatchStatus::Failed,
            dispatch_id: "01JY0000000000000000000000".to_string(),
            duration_ms: 1234,
            exit_code: Some(1),
            signal: Some(15),
            output: Some("/tmp/dispatch-output.md".to_string()),
            output_exists: true,
            error: Some(DispatchErrorInfo {
                code: "process_failed".to_string(),
                message: "child process failed".to_string(),
                retryable: true,
            }),
            tokens: TokenUsage {
                input: 100,
                output: 50,
                reasoning: 25,
            },
            activity: Activity {
                files_changed: vec!["src/main.rs".to_string()],
                files_read: vec!["README.md".to_string()],
                commands_run: vec!["cargo test".to_string()],
                tool_calls: 3,
                parse_errors: 0,
            },
        };

        let json = serde_json::to_string(&result).expect("serialize dispatch result");
        let round_tripped: DispatchResult =
            serde_json::from_str(&json).expect("deserialize dispatch result");

        assert_eq!(round_tripped, result);
    }

    #[test]
    fn dispatch_status_serializes_as_snake_case() {
        assert_eq!(
            serde_json::to_string(&DispatchStatus::Completed).expect("serialize completed"),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&DispatchStatus::TimedOut).expect("serialize timed out"),
            "\"timed_out\""
        );
        assert_eq!(
            serde_json::to_string(&DispatchStatus::Failed).expect("serialize failed"),
            "\"failed\""
        );
    }

    #[test]
    fn dispatch_error_display_messages_are_stable() {
        assert_eq!(
            DispatchError::BinaryNotFound.to_string(),
            "codex binary not found on PATH"
        );
        assert_eq!(
            DispatchError::SpawnFailed(io::Error::other("permission denied")).to_string(),
            "process spawn failed: permission denied"
        );
        assert_eq!(
            DispatchError::TimedOut {
                timeout_sec: 900,
                grace_sec: 30
            }
            .to_string(),
            "dispatch timed out after 900s (grace 30s)"
        );
        assert_eq!(
            DispatchError::ProcessFailed {
                exit_code: Some(1),
                signal: None
            }
            .to_string(),
            "child process failed: exit code Some(1), signal None"
        );
        assert_eq!(
            DispatchError::ParseError("bad line".to_string()).to_string(),
            "NDJSON parse error: bad line"
        );
        assert_eq!(
            DispatchError::Other("unexpected error".to_string()).to_string(),
            "unexpected error"
        );
    }

    #[test]
    fn default_values_are_zeroed_and_empty() {
        let tokens = TokenUsage::default();
        assert_eq!(tokens.input, 0);
        assert_eq!(tokens.output, 0);
        assert_eq!(tokens.reasoning, 0);

        let activity = Activity::default();
        assert!(activity.files_changed.is_empty());
        assert!(activity.files_read.is_empty());
        assert!(activity.commands_run.is_empty());
        assert_eq!(activity.tool_calls, 0);
        assert_eq!(activity.parse_errors, 0);
    }
}
