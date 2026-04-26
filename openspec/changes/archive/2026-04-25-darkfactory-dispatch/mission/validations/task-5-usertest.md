# Task 5.1 User-Test Validation

**Commit:** d243014  
**Date:** 2026-04-23  
**Test command:** `cargo test` -- 27 passed (3 suites, 6.66s)

## Execution Assertions

### [PASS] Mock child with codex-format NDJSON produces correct DispatchResult

Test: `successful_dispatch_tracks_tokens_and_activity` (tests/engine.rs:47)

The test installs a mock `codex` binary (`tests/test_codex_mock.sh` in `success` mode) that emits codex-format NDJSON events on stdout. The test asserts:

- `result.status == DispatchStatus::Completed`
- `result.exit_code == Some(0)`
- `result.tokens.input == 100`, `output == 50`, `reasoning == 20`
- `result.activity.files_changed == ["src/main.rs"]`
- `result.activity.files_read == ["README.md"]`
- `result.activity.commands_run == ["cargo test"]`
- `result.activity.tool_calls == 1`

### [PASS] Timeout dispatch kills child process

Test: `timeout_dispatch_stops_child_process` (tests/engine.rs:79)

The test uses the `sleep` mock mode (sleeps 300s) with a 1-second timeout and 1-second grace period. Asserts:

- `result.status == DispatchStatus::TimedOut`
- `result.error.retryable == true`
- The child pid (written to a file by the mock) no longer exists after dispatch returns (polled via `kill(pid, 0)` up to 40 times at 50ms intervals)

### [PASS] Heartbeat events appear on stderr during dispatch

Test: `heartbeat_events_are_emitted_during_running_dispatch` (tests/engine.rs:103)

Uses `delayed_success` mock mode (3-second sleep before output). The emitter writes to a shared buffer (standing in for stderr -- production `Emitter::new` defaults to `io::stderr()`). Asserts:

- At least 2 heartbeat events emitted
- All heartbeat events have `interval_s == 1` (test mode; production uses 15s per `HEARTBEAT_INTERVAL_SECS`)

The production Emitter writes to stderr by default (event.rs:105: `Emitter::new` calls `with_writer(..., io::stderr())`). The test substitutes a SharedBuffer to capture output deterministically. The heartbeat interval is 15s in production via `#[cfg(not(test))] const HEARTBEAT_INTERVAL_SECS: u64 = 15`.

### [PASS] Malformed NDJSON produces raw_passthrough events and increments parse error count

Test: `malformed_ndjson_is_counted_and_logged` (tests/engine.rs:132)

Uses `malformed` mock mode that emits `not json` and `{still not json` lines between valid NDJSON. Asserts:

- `result.activity.parse_errors == 2`
- Both malformed lines appear in the log file
- `parse_codex_event` returns `EventKind::RawPassthrough` for non-JSON lines (unit test at engine.rs:750)

## Output Assertions

### [PASS] Codex invocation format

Source: `build_codex_args` (engine.rs:156-187) and unit test `codex_args_include_expected_flags` (engine.rs:709-747).

The args are built as:
```
codex exec --json --full-auto -s <sandbox> -m <model> -c model_reasoning_effort="<effort>" [-c <config>...] -o <output> <prompt>
```

The unit test asserts the exact arg list: `["exec", "--json", "--full-auto", "-s", "workspace-write", "-m", "gpt-5", "-c", "model_reasoning_effort=\"high\"", "-c", "foo=bar", "-o", "out.txt", "do work"]`.

Note: no `--log` flag is passed to codex. The contract says "Codex is invoked with ... -o <output> <prompt>" which matches. The `--log` flag mentioned in the log-file assertion below is a dispatch-level feature, not a codex CLI flag.

### [PASS] Child stdin is closed immediately

Source: engine.rs:45-46 -- `.stdin(StdinMode::Null)` is passed to the SupervisorBuilder. In supervisor.rs:116-118, `StdinMode::Null` maps to `Stdio::null()`, which provides `/dev/null` as stdin (an empty reader, not a pipe).

### [PASS] status.json is written atomically (tmp file + rename) every 5 seconds

Source: `write_status_json_atomic` (engine.rs:636-665). Writes to `status.json.tmp.<pid>`, calls `sync_all()`, then `std::fs::rename` to `status.json`, then syncs the directory. The watchdog interval is `WATCHDOG_INTERVAL_SECS = 5` (engine.rs:22), triggered in the select loop at engine.rs:116-123.

### [PASS] Log file contains both child stdout and child stderr

Source: The `successful_dispatch_tracks_tokens_and_activity` test passes a log path and asserts:
- `log_text.contains("thread.started")` -- this is stdout content (NDJSON line)
- `log_text.contains("mock stderr line")` -- this is stderr content

Stdout lines are written via `write_log_line` (engine.rs:236-248) in the select loop. Stderr is captured by `capture_stderr` (engine.rs:209-234) which writes each line to the shared log file.

### [PASS] Grace period defaults to timeout/2 with minimum 1s

Source: `effective_grace_sec` (engine.rs:673-679):
```rust
fn effective_grace_sec(timeout_sec: u64, grace_sec: u64) -> u64 {
    if grace_sec == 0 {
        (timeout_sec / 2).max(1)
    } else {
        grace_sec
    }
}
```

When `grace_sec == 0`, it defaults to `timeout_sec / 2` floored to at least 1. When `grace_sec > 0`, the explicit value is used.

### [PASS] Hard timeout calls graceful_stop with a floor of 10 seconds

Source: engine.rs:109-112 in the hard timeout branch:
```rust
let exit = run_handle
    .graceful_stop(grace_sec.max(10))
    .await
    .map_err(map_supervisor_error)?;
```

`grace_sec.max(10)` ensures at least 10 seconds are passed to `graceful_stop`. The `graceful_stop` method (supervisor.rs:244-257) sends SIGTERM first, then waits up to the grace period before sending SIGKILL.

## Summary

| # | Assertion | Result |
|---|-----------|--------|
| 1 | Mock NDJSON dispatch produces correct DispatchResult | PASS |
| 2 | Timeout dispatch kills child and returns timed_out | PASS |
| 3 | Heartbeat events at ~15s intervals on stderr | PASS |
| 4 | Malformed NDJSON -> raw_passthrough + parse error count | PASS |
| 5 | Codex invocation format matches spec | PASS |
| 6 | Child stdin is /dev/null (not a pipe) | PASS |
| 7 | status.json written atomically every 5s | PASS |
| 8 | Log file contains stdout and stderr | PASS |
| 9 | Grace period defaults to timeout/2, min 1s | PASS |
| 10 | Hard timeout graceful_stop floor of 10s | PASS |

**Result: 10/10 PASS**
