# Worker Brief: Task 4 - Event emitter module

## Your Task
Implement `src/event.rs` -- the NDJSON event emitter for the dispatcher. This module writes structured events to stderr for monitoring and emits periodic heartbeats during dispatch.

### What to implement

**Event struct** (serialized to NDJSON on stderr):
```rust
#[derive(Serialize)]
pub(crate) struct DispatchEvent {
    #[serde(rename = "type")]
    pub(crate) event_type: String,
    pub(crate) dispatch_id: String,
    pub(crate) ts: String,  // RFC3339 timestamp
    // Optional fields depending on event type:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) elapsed_s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_activity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) interval_s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) message: Option<String>,
}
```

**Emitter** -- thread-safe event writer:
```rust
pub(crate) struct Emitter {
    dispatch_id: String,
    start: Instant,
    stream_mode: StreamMode,
}

pub(crate) enum StreamMode {
    Silent,   // only: dispatch_start, dispatch_end, timeout_warning, error
    Normal,   // all events
    Verbose,  // all events + raw harness lines
}
```

**Key methods:**
- `Emitter::new(dispatch_id, stream_mode)` -- create emitter
- `emit_dispatch_start(engine, model)` -- type: "dispatch_start"
- `emit_dispatch_end(status, duration_ms)` -- type: "dispatch_end"
- `emit_heartbeat(last_activity)` -- type: "heartbeat" with elapsed_s and interval_s
- `emit_timeout_warning()` -- type: "timeout_warning" with elapsed_s
- `emit_tool_start(tool)` -- type: "tool_start"
- `emit_tool_end(tool)` -- type: "tool_end"
- `emit_file_write(path)` -- type: "file_write"
- `emit_file_read(path)` -- type: "file_read"
- `emit_command_run(command)` -- type: "command_run"
- `emit_error(code, message)` -- type: "error"

All emit methods write one NDJSON line to stderr, gated by stream_mode.

**Heartbeat ticker:**
```rust
pub(crate) fn heartbeat_ticker(
    emitter: Arc<Emitter>,
    interval_secs: u64,
    last_activity: Arc<Mutex<String>>,
    cancel: CancellationToken,
)
```
Spawns a tokio task that emits heartbeat events at the configured interval until cancelled. The `last_activity` is updated by the engine module whenever it processes a harness event.

Reference: `~/Code/agent-mux/internal/event/event.go:107-214`

### Tests to write

Write inline `#[cfg(test)]` tests in `src/event.rs`:

1. **Heartbeat event format**: Create an emitter, call `emit_heartbeat("running: cargo test")`, capture stderr, verify the output is valid NDJSON with type "heartbeat", elapsed_s, last_activity, and interval_s fields.
2. **Stream mode filtering**: Verify that in Silent mode, `emit_tool_start` does NOT emit, but `emit_timeout_warning` does. In Normal mode, both emit.
3. **Event type coverage**: Verify each emit method produces the correct `type` field value.

Note: To capture stderr in tests, you may need to write events to a `Vec<u8>` writer instead of stderr directly. Consider making the Emitter generic over `Write` or providing a test constructor that writes to a buffer.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- All items `pub(crate)`
- `thiserror` for errors, no `anyhow`
- Update `src/main.rs` to `mod event;`
- Verify `cargo clippy --all-targets -- -D warnings` passes

## What Previous Tasks Built
### Task 1: Scaffolding
- Cargo.toml with tokio, serde, chrono, etc.

### Task 2: Supervisor
- src/supervisor.rs: process groups, signals, GracefulStop
- Public: SupervisorBuilder, RunHandle, ExitResult

### Task 3: Types
- src/types.rs: DispatchSpec, DispatchResult, DispatchError, HarnessEvent, EventKind, TokenUsage, Activity, LiveStatus
- All types with serde derives, 4 inline tests

## Downstream Consumers
Task 5.1 (engine) will use Emitter for heartbeats, timeout_warning, tool events during the dispatch loop.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Implement src/event.rs
- Write inline tests
- Update src/main.rs to include `mod event;`
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit: "[task-4] Implement event emitter module with heartbeat ticker and stream mode filtering"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-4.md

## Handoff Report Format
- Task: Task 4 - Event emitter module
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
