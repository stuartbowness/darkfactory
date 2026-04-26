# Worker Brief: Task 3 - Types module

## Your Task
Implement `src/types.rs` -- all shared data structures for the dispatcher. These types are consumed by the engine, event emitter, and CLI modules.

### What to implement

**DispatchSpec** -- input to a dispatch:
```rust
pub(crate) struct DispatchSpec {
    pub(crate) model: String,
    pub(crate) effort: Option<String>,
    pub(crate) timeout_sec: u64,
    pub(crate) grace_sec: u64,
    pub(crate) sandbox: String,
    pub(crate) configs: Vec<String>,
    pub(crate) prompt: String,
    pub(crate) output: Option<PathBuf>,
    pub(crate) cwd: PathBuf,
    pub(crate) log: Option<PathBuf>,
}
```

**DispatchResult** -- output of a dispatch (serialized to JSON on stdout):
```rust
#[derive(Serialize, Deserialize)]
pub(crate) struct DispatchResult {
    pub(crate) status: DispatchStatus,
    pub(crate) dispatch_id: String,
    pub(crate) duration_ms: u64,
    pub(crate) exit_code: Option<i32>,
    pub(crate) signal: Option<i32>,
    pub(crate) output: Option<String>,      // path passed via --output
    pub(crate) output_exists: bool,
    pub(crate) error: Option<DispatchErrorInfo>,
    pub(crate) tokens: TokenUsage,
    pub(crate) activity: Activity,
}
```

**DispatchStatus** -- enum: `Completed`, `TimedOut`, `Failed` (serialize as lowercase strings):
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DispatchStatus {
    Completed,
    TimedOut,
    Failed,
}
```

**DispatchError** -- thiserror error enum for internal errors:
```rust
#[derive(thiserror::Error, Debug)]
pub(crate) enum DispatchError {
    #[error("codex binary not found on PATH")]
    BinaryNotFound,
    #[error("process spawn failed: {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("dispatch timed out after {timeout_sec}s (grace {grace_sec}s)")]
    TimedOut { timeout_sec: u64, grace_sec: u64 },
    #[error("child process failed: exit code {exit_code:?}, signal {signal:?}")]
    ProcessFailed { exit_code: Option<i32>, signal: Option<i32> },
    #[error("NDJSON parse error: {0}")]
    ParseError(String),
    #[error("{0}")]
    Other(String),
}
```

**DispatchErrorInfo** -- serializable error info for the result JSON (NOT the same as DispatchError):
```rust
#[derive(Serialize, Deserialize)]
pub(crate) struct DispatchErrorInfo {
    pub(crate) code: String,
    pub(crate) message: String,
    pub(crate) retryable: bool,
}
```

**TokenUsage**:
```rust
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct TokenUsage {
    pub(crate) input: u64,
    pub(crate) output: u64,
    pub(crate) reasoning: u64,
}
```

**Activity**:
```rust
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Activity {
    pub(crate) files_changed: Vec<String>,
    pub(crate) files_read: Vec<String>,
    pub(crate) commands_run: Vec<String>,
    pub(crate) tool_calls: u64,
}
```

**HarnessEvent** -- internal event parsed from codex NDJSON:
```rust
pub(crate) struct HarnessEvent {
    pub(crate) kind: EventKind,
    pub(crate) tool: Option<String>,
    pub(crate) file: Option<String>,
    pub(crate) command: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) tokens: Option<TokenUsage>,
    pub(crate) session_id: Option<String>,
}

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
```

**LiveStatus** -- written to status.json by the watchdog:
```rust
#[derive(Serialize, Deserialize)]
pub(crate) struct LiveStatus {
    pub(crate) state: String,
    pub(crate) elapsed_s: u64,
    pub(crate) last_activity: String,
    pub(crate) tools_used: u64,
    pub(crate) files_changed: u64,
    pub(crate) dispatch_id: String,
    pub(crate) ts: String,
}
```

### Tests to write

Write inline `#[cfg(test)]` tests in `src/types.rs`:

1. **DispatchResult JSON round-trip**: Create a DispatchResult with all fields populated, serialize to JSON, deserialize back, verify all fields match.
2. **DispatchStatus serialization**: Verify `Completed` serializes to `"completed"`, `TimedOut` to `"timed_out"`, `Failed` to `"failed"`.
3. **DispatchError display**: Verify error messages render correctly.
4. **Default values**: Verify TokenUsage::default() and Activity::default() produce zeroed/empty values.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md` first
- All items `pub(crate)` unless called from main.rs
- `thiserror` for DispatchError, no `anyhow`
- No `#[serde(flatten)]` or `#[serde(untagged)]`
- Update `src/main.rs` to `mod types;`
- Verify `cargo clippy --all-targets -- -D warnings` passes

## Project Context
darkfactory-dispatch is a Rust process supervisor for Codex CLI subprocesses. The types module defines all shared data structures used across modules.

## What Previous Tasks Built
### Task 1: Project scaffolding
- Created: tools/darkfactory-dispatch/ with Cargo.toml, Makefile, src/main.rs stub
- Dependencies declared and compiling

### Task 2: Supervisor module
- Created: src/supervisor.rs with process groups, signals, GracefulStop, parent-death
- Created: tests/supervisor.rs with 3 passing tests
- Public interface: SupervisorBuilder, RunHandle, ExitResult

## Downstream Consumers
- Task 4.1 (event emitter) will use EventKind and HarnessEvent
- Task 5.1 (engine) will use DispatchSpec, DispatchResult, HarnessEvent, TokenUsage, Activity, LiveStatus
- Task 6.1 (CLI) will use DispatchSpec, DispatchResult, DispatchStatus

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Implement src/types.rs with all types above
- Write inline tests
- Update src/main.rs to include `mod types;`
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit: "[task-3] Implement types module with DispatchSpec, DispatchResult, and all shared types"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Types module
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
