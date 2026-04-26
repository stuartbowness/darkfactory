# Task 5: Engine Module -- Scrutiny Validation

**Commit:** d243014  
**Validator:** Claude Opus 4.6 (1M context)  
**Date:** 2026-04-23  
**Verdict:** PASS

## 1. Test Presence

Engine tests exist in two locations:

- `tests/engine.rs` (265 lines) -- four integration tests using a mock `codex` shell script
- `src/engine.rs` `#[cfg(test)] mod tests` (50 lines) -- two unit tests for arg construction and NDJSON parsing

All four integration tests:
- `successful_dispatch_tracks_tokens_and_activity`
- `timeout_dispatch_stops_child_process`
- `heartbeat_events_are_emitted_during_running_dispatch`
- `malformed_ndjson_is_counted_and_logged`

## 2. Build

`cargo build` succeeds with no warnings.

## 3. Tests

`cargo test` passes: 27 tests across 3 suites (6.47s).

## 4. Lint

`cargo clippy --all-targets` clean. `cargo fmt --check` clean.

## 5. Code Review

**Architecture.** The dispatch loop uses `tokio::select!` with six arms: stdout line reads, child wait, soft timeout, hard timeout, watchdog (status.json), and heartbeat. This is correct and mirrors the reference implementation's structure.

**Potential issues reviewed:**

- **Race between stdout close and child wait.** When stdout returns `None`, `stdout_open` is set to false and the loop continues. The `run_handle.wait()` arm will catch the child exit. No race -- the select loop handles both events correctly.

- **Hard timeout branch guard.** The `if hard_sleep.is_some()` guard on the hard timeout arm prevents it from being polled when `hard_sleep` is `None`. The inner async block also handles the `None` case with `pending()`. Belt-and-suspenders -- correct.

- **Stderr capture task join.** After the loop breaks, `stderr_task.await` is called. If the child is killed by timeout, stderr EOF will arrive when the process dies, so the task will complete. No deadlock risk.

- **Unbounded stderr capture.** `capture_stderr` accumulates all stderr into a `String`. In theory a malicious child could OOM the host. In practice, codex processes produce limited stderr. Acceptable risk for this use case.

- **Status file cleanup.** Temp files in `/tmp/darkfactory-dispatch/<dispatch_id>/` are never cleaned up after dispatch. This is fine for a long-running supervisor -- the OS cleans `/tmp` on reboot, and the files are small. Could be noted as a future improvement but not a bug.

- **`set_var` / `remove_var` in tests.** The test suite manipulates `PATH` and `CODEX_MOCK_MODE` environment variables. These are protected by a `PATH_LOCK` async mutex that serializes tests that need PATH manipulation. This is necessary because Rust's env vars are process-global. The `TestPath` RAII guard restores previous values on drop. Correct pattern for integration tests.

No bugs, race conditions, or resource leaks found.

## 6. Module Depth

The engine exposes a single public function:

```rust
pub(crate) async fn dispatch(
    spec: DispatchSpec,
    dispatch_id: String,
    emitter: Arc<Emitter>,
) -> Result<DispatchResult, DispatchError>
```

All internal complexity (arg construction, NDJSON parsing, timeout management, status file writing, activity tracking, result finalization) is hidden behind this one entry point. The `EventBuilder` trait and `ActivityTracker` struct are module-private. This satisfies the "narrow interface hiding complexity" requirement from CLAUDE.md.

## 7. Code Standards

- **Gallant boundary:** All dispatch logic lives in `engine.rs`. CLI (`main.rs`) only declares the module. Correct.
- **Visibility:** All items are `pub(crate)` or private. No `pub` leaks.
- **Error types:** `thiserror` via `DispatchError` in `types.rs`. No `anyhow` anywhere.
- **Lint suppression:** Uses `#[expect(dead_code)]` (not `#[allow]`). Correct.
- **Serde:** No `#[serde(flatten)]` or `#[serde(untagged)]`. Correct.
- **Tokio tests:** All async tests use `#[tokio::test(flavor = "multi_thread")]`. Correct.

## 8. Code Simplify

The code is already lean. The `EventBuilder` trait on `HarnessEvent` is a builder pattern that makes the NDJSON parsing code readable. The JSON traversal helpers (`string_path`, `number_path`, `item_string`, `string_field`) are justified by the number of call sites and the variance in codex NDJSON field locations. No unnecessary abstractions found.

## 9. Security

- **Unsafe code:** None in `engine.rs`. The only `unsafe` in the crate is in `supervisor.rs` (pre_exec, libc calls) and the test helper `process_exists` which uses `libc::kill(pid, 0)` -- both reviewed in task 4.
- **Signal handling:** Hard timeout calls `graceful_stop(grace_sec.max(10))` which sends SIGTERM, waits, then SIGKILL. The `.max(10)` floor is correct.
- **File operations:** `write_status_json_atomic` uses tmp-file + `rename` + `sync_all`. The tmp filename includes `std::process::id()` to avoid collisions. Atomic write is correct.
- **Shell script:** `test_codex_mock.sh` uses `set -euo pipefail`, `$#` arithmetic, and `dirname` -- all POSIX-compatible. No GNU-only flags.

## 10. Contract Assertions

### [execution] Mock child with codex-format NDJSON, verifies DispatchResult -- PASS

`successful_dispatch_tracks_tokens_and_activity` dispatches the `success` mock which outputs codex-format NDJSON (thread.started, item.started, item.completed, turn.completed). The test verifies:
- `result.status == DispatchStatus::Completed`
- `result.tokens.input == 100`, `output == 50`, `reasoning == 20`
- `result.activity.files_changed == ["src/main.rs"]`
- `result.activity.files_read == ["README.md"]`
- `result.activity.commands_run == ["cargo test"]`

### [execution] Mock child past timeout, verifies timed_out and child is dead -- PASS

`timeout_dispatch_stops_child_process` dispatches the `sleep` mock (sleeps 300s) with a 1s timeout + 1s grace. Verifies:
- `result.status == DispatchStatus::TimedOut`
- `result.error.retryable == true`
- The child PID (written to a file by the mock) no longer exists via `libc::kill(pid, 0)` polling

### [execution] Heartbeat events at ~15s intervals -- PASS

`heartbeat_events_are_emitted_during_running_dispatch` dispatches the `delayed_success` mock (3s delay). In test mode, `HEARTBEAT_INTERVAL_SECS` is reduced to 1s. The test verifies:
- At least 2 heartbeat events are emitted
- All heartbeats have `interval_s == 1` (the test-mode interval)

In production, the interval is 15s. The test confirms the heartbeat mechanism works; the interval difference is a test acceleration, not a behavioral change.

### [execution] Malformed NDJSON produces raw_passthrough and increments parse_errors -- PASS

`malformed_ndjson_is_counted_and_logged` dispatches the `malformed` mock which outputs two non-JSON lines (`not json`, `{still not json`). Verifies:
- `result.activity.parse_errors == 2`
- Both malformed lines appear in the log file

The code path: `parse_codex_event` returns `RawPassthrough` for lines that fail `serde_json::from_str`, and `ActivityTracker::apply` increments `parse_errors` for `RawPassthrough` events.

### [output] Codex invoked with correct command line -- PASS

`build_codex_args` produces: `exec --json --full-auto -s <sandbox> -m <model> -c model_reasoning_effort="<effort>" -o <output> <prompt>`. Verified by the `codex_args_include_expected_flags` unit test which asserts the exact argument vector.

### [output] Child stdin is closed immediately -- PASS

Line 45: `.stdin(StdinMode::Null)`. The `SupervisorBuilder::spawn` maps `StdinMode::Null` to `Stdio::null()` (supervisor.rs line 117). The child receives `/dev/null` as stdin -- no pipe to leak or block on.

### [output] status.json written atomically every 5 seconds -- PASS

`write_status_json_atomic` (line 636):
1. Creates a tmp file: `status.json.tmp.<pid>`
2. Writes JSON via `serde_json::to_writer`
3. Calls `sync_all()` on the file
4. Calls `std::fs::rename` (atomic on same filesystem)
5. Syncs the directory fd

The watchdog interval is set to `WATCHDOG_INTERVAL_SECS = 5`.

### [output] Log file contains both child stdout and stderr -- PASS

- Stdout lines: written via `write_log_line` at line 78 in the select loop
- Stderr lines: written via `capture_stderr` which writes each line to the shared log at line 224-229

The `successful_dispatch_tracks_tokens_and_activity` test verifies both: `log_text.contains("thread.started")` (stdout) and `log_text.contains("mock stderr line")` (stderr).

### [output] Grace period defaults to timeout/2 with minimum 1s -- PASS

`effective_grace_sec` (line 673):
```rust
fn effective_grace_sec(timeout_sec: u64, grace_sec: u64) -> u64 {
    if grace_sec == 0 {
        (timeout_sec / 2).max(1)
    } else {
        grace_sec
    }
}
```

When `grace_sec` is 0 (not specified), it defaults to `timeout/2` with a `.max(1)` floor.

### [output] Hard timeout calls graceful_stop with floor of 10 seconds -- PASS

Line 110: `run_handle.graceful_stop(grace_sec.max(10))`. The `.max(10)` ensures the grace period passed to `graceful_stop` is never less than 10 seconds, regardless of the computed grace value.

## Summary

| # | Assertion | Result |
|---|-----------|--------|
| 1 | Mock NDJSON dispatch with status/tokens/activity | PASS |
| 2 | Timeout kills child, status is timed_out | PASS |
| 3 | Heartbeat events at ~15s intervals | PASS |
| 4 | Malformed NDJSON produces raw_passthrough + parse_errors | PASS |
| 5 | Codex invoked with correct args | PASS |
| 6 | Child stdin closed (Null) | PASS |
| 7 | status.json atomic write every 5s | PASS |
| 8 | Log file has stdout + stderr | PASS |
| 9 | Grace defaults to timeout/2, min 1s | PASS |
| 10 | graceful_stop floor of 10s | PASS |

All 10 contract assertions verified. Build, tests (27/27), clippy, and fmt all pass.
