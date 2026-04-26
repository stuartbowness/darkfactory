# Worker Brief: Task 5 - Engine module

## Your Task
Implement `src/engine.rs` -- the core dispatch loop. This is the biggest module. It consumes a `Supervisor::RunHandle` and an `Emitter`, drives the entire dispatch lifecycle, and produces a `DispatchResult`.

### What to implement

**Public interface:**
```rust
pub(crate) async fn dispatch(spec: DispatchSpec, dispatch_id: String, emitter: Arc<Emitter>) -> Result<DispatchResult, DispatchError>
```

### Implementation details

**1. Codex arg construction:**
Build the codex exec command:
```
codex exec --json --full-auto
  -s <spec.sandbox>           (default: "workspace-write")
  -m <spec.model>
  -c model_reasoning_effort="<spec.effort>"   (if effort is set)
  -c <key>=<value>            (for each spec.configs entry)
  -o <spec.output>            (if output path is set)
  <spec.prompt>               (positional arg -- the full prompt text)
```

Reference: `~/Code/agent-mux/internal/engine/adapter/codex.go:57-99`

**2. Process spawning:**
Use `SupervisorBuilder::new("codex", args)` with:
- `cwd`: spec.cwd
- `stdin`: Stdio::null() (immediate EOF -- prevents the "Reading additional input from stdin" stall)
- `stdout`: piped (for NDJSON parsing)
- `stderr`: piped (for capture)

Call `.spawn()` to get a `RunHandle`.

**3. stdout NDJSON parsing:**
Read stdout line-by-line with a `BufReader` (4MB buffer). Each line is parsed as JSON. Map codex events to `HarnessEvent`:

| Codex event type | HarnessEvent kind | Data extracted |
|---|---|---|
| `thread.started` | SessionStart | thread_id -> session_id |
| `item.started` (tool_use) | ToolStart | tool name |
| `item.started` (function_call) | CommandRun | command |
| `item.completed` (agent_message) | Response | text |
| `item.completed` (tool_use/function_call) | ToolEnd | tool name |
| `item.completed` (file write detected) | FileWrite | file path |
| `item.completed` (file read detected) | FileRead | file path |
| `turn.completed` | TurnComplete | tokens (input, output, reasoning) |
| `turn.failed` | TurnFailed | error message |
| `error` | Error | error message |
| Non-JSON line | RawPassthrough | raw text |

Reference: `~/Code/agent-mux/internal/engine/adapter/codex.go:174-337`

**4. stderr capture:**
Read stderr in a separate tokio task into a String buffer. Write both stdout and stderr to the log file (if --log specified). The log file is a debug artifact, not parsed by the dispatcher.

**5. Activity tracking:**
As events are parsed, accumulate:
- `files_changed: Vec<String>` -- from FileWrite events (deduplicated)
- `files_read: Vec<String>` -- from FileRead events (deduplicated)
- `commands_run: Vec<String>` -- from CommandRun events
- `tool_calls: u64` -- count of ToolStart events
- `tokens: TokenUsage` -- sum of all TurnComplete token events
- `last_activity: String` -- updated on every event for heartbeat context

**6. Two-phase timeout (the core select! loop):**
```rust
tokio::select! {
    // Branch 1: stdout line available
    line = stdout_reader.next_line() => { ... parse event, update activity ... }
    
    // Branch 2: process exited
    result = run_handle.wait() => { ... finalize result ... }
    
    // Branch 3: soft timeout
    _ = soft_timer => {
        emitter.emit_timeout_warning();
        // arm hard timer
    }
    
    // Branch 4: hard timeout
    _ = hard_timer => {
        run_handle.graceful_stop(max(grace_sec, 10)).await;
        // finalize as timed_out
    }
    
    // Branch 5: watchdog tick (every 5s)
    _ = watchdog_interval.tick() => {
        write_status_json_atomic(dispatch_id, elapsed, last_activity, ...);
    }
    
    // Branch 6: heartbeat tick (every 15s)
    _ = heartbeat_interval.tick() => {
        emitter.emit_heartbeat(last_activity);
    }
}
```

The soft timer is set to `spec.timeout_sec`. When it fires, emit `timeout_warning` and arm the hard timer at `spec.grace_sec`. The hard timer calls `graceful_stop` with a floor of 10 seconds.

Grace defaults: if `grace_sec == 0`, set it to `timeout_sec / 2` with minimum 1.

Reference: `~/Code/agent-mux/internal/engine/loop.go:787-907`

**7. Atomic status.json writes:**
Write `LiveStatus` to a temp file, fsync, rename to `status.json`. Use the dispatch store path if in async mode, otherwise a temp directory.

Reference: `~/Code/agent-mux/internal/dispatch/status.go:30-70`

**8. Result finalization:**
When the process exits (or is killed by timeout):
- Check `exit_code` and `signal` from `ExitResult`
- Check if `spec.output` file exists (`output_exists`)
- Classify status:
  - Exit 0 with output -> `Completed`
  - Killed by timeout -> `TimedOut` (partial=true, recoverable=true)
  - Exit non-zero or no output -> `Failed`
- Build `DispatchResult` with all accumulated data

Reference: `~/Code/agent-mux/internal/engine/loop.go:997-1058`

### Tests to write

Write tests in `tests/engine.rs`. Create a small test helper script that simulates codex NDJSON output:

```bash
#!/bin/bash
# test_codex_mock.sh -- simulates codex exec --json output
echo '{"type":"thread.started","thread_id":"test-123"}'
echo '{"type":"item.started","item":{"type":"tool_use","name":"shell"}}'
echo '{"type":"item.completed","item":{"type":"tool_use","name":"shell"}}'
echo '{"type":"turn.completed","usage":{"input_tokens":100,"output_tokens":50,"reasoning_tokens":20}}'
```

Tests:
1. **Successful dispatch**: Use the mock script. Verify DispatchResult has status `completed`, correct token sums, activity data.
2. **Timeout dispatch**: Use `sleep 300` as the child, set timeout to 2s. Verify status `timed_out`, child is dead.
3. **Heartbeat emission**: Capture stderr during dispatch, verify heartbeat events appear at ~15s intervals (use a short interval like 1s for testing).
4. **Malformed NDJSON**: Mock script outputs `not json` lines. Verify they produce RawPassthrough events and increment parse error count.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Gallant boundary: ALL dispatch logic lives here, not in CLI
- `pub(crate)` visibility
- `thiserror` errors
- Update `src/main.rs` to `mod engine;`

## What Previous Tasks Built
### Task 1: Scaffolding
- Cargo.toml with all dependencies

### Task 2: Supervisor
- src/supervisor.rs: SupervisorBuilder, RunHandle (wait, graceful_stop, kill, stdout, stderr, pid), ExitResult
- Process groups via setpgid, signal delivery to -pgid

### Task 3: Types
- src/types.rs: DispatchSpec, DispatchResult, DispatchStatus, DispatchError, DispatchErrorInfo, HarnessEvent, EventKind, TokenUsage, Activity, LiveStatus

### Task 4: Event emitter
- src/event.rs: Emitter (emit_dispatch_start/end, emit_heartbeat, emit_timeout_warning, emit_tool_start/end, emit_file_write/read, emit_command_run, emit_error), StreamMode, heartbeat_ticker

## Downstream Consumers
Task 6.1 (CLI) will call `engine::dispatch(spec, id, emitter)` and format the result as JSON.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Implement src/engine.rs
- Create test helper script(s) in tests/ directory
- Write tests in tests/engine.rs
- Update src/main.rs to include `mod engine;`
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit: "[task-5] Implement engine module with codex dispatch loop, NDJSON parsing, and two-phase timeout"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-5.md

## Handoff Report Format
- Task: Task 5 - Engine module
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
