# Task 6.2 Scrutiny Validation: Async dispatch and lifecycle subcommands

**Commit:** a2be004  
**Validator:** Claude Opus 4.6 (scrutiny)  
**Date:** 2026-04-23  
**Verdict:** PASS

## 1. Test presence

Tests added in `tests/cli.rs`:
- `async_dispatch_flow_reports_status_and_result` -- end-to-end async flow covering ack, status, wait, result
- `status_reports_orphaned_when_async_dispatcher_dies` -- kills dispatcher, verifies orphaned state
- `async_dispatch_prunes_old_store_directories` -- creates 8-day-old ULID directory, verifies pruning
- `json_and_async_are_mutually_exclusive` -- flag conflict check

One new test in `tests/engine.rs`: `store_dir: None` added to existing spec fixture.

Total: 35 tests pass across 4 suites.

## 2. Build

`cargo build` -- clean, no warnings.

## 3. Tests

`cargo test` -- 35 passed (10.89s). No failures, no flaky tests.

## 4. Lint

- `cargo clippy --all-targets -- -D warnings` -- no issues.
- `cargo fmt --check` -- clean.

## 5. Code review

### Async lifecycle

The async dispatch flow in `main.rs` follows a clear sequence: prepare store, write PID, write initial status, open events log, print ack, run dispatch, write result. The `HostPidGuard` uses RAII (Drop impl) to clean up `host.pid` after the process completes. Result is written atomically via rename before the guard drops, so lifecycle queries never see a state where result.json is missing but host.pid is also gone.

### PID handling

`process_alive()` uses `kill(pid, 0)` with correct ESRCH handling. The EPERM case (process exists but different user) correctly returns `true`. PID is parsed from the file with error handling. Negative/zero PIDs return `false`.

### Orphan detection

`read_dispatch_status` checks result.json first, then host process state. When host PID references a dead process and no result exists, state is `"orphaned"`. The ordering prevents false orphan reports when the result was just written but the PID file hasn't been cleaned up yet.

### Atomic writes

`write_json_atomic` uses write-to-temp + rename + fsync pattern. Both the file and parent directory are synced. The temp file name includes the PID to avoid collisions.

### Parent death watch

Correctly disabled for async mode (`parent_death_watch(spec.store_dir.is_none())`). In sync mode the child dies with the parent. In async mode the child survives, which is the intended behavior -- the async dispatcher stays alive to write the result.

### Event logging

In async mode, the emitter writes to the events.jsonl file instead of stderr. The file is opened in append mode.

### Error path

When `engine::dispatch` fails in async mode, a `failed_start_result` is written to result.json so lifecycle queries can find the terminal state.

## 6. Module depth (Gallant boundary)

cli.rs grew from ~130 to 479 lines. The new code is lifecycle query logic (reading files, checking PIDs, polling) which is I/O formatting -- not dispatch logic. The engine still owns the actual dispatch loop. The public interface added to cli.rs is narrow: `prepare_async_store`, `write_host_pid`, `write_initial_status`, `open_events_log`, `print_async_started`, `write_result`, `print_status`, `print_result_for_id`, `wait_for_id`, `failed_start_result`. All private helpers stay private. No boundary violation.

## 7. Standards

- No new dependencies added.
- All new items use `pub(crate)` visibility.
- Error propagation uses `map_err` to String at CLI boundary, typed errors in engine.
- No `anyhow`, no `#[allow(...)]`, no `#[serde(flatten)]`, no `#[serde(untagged)]`.
- Stdout used for machine-readable data, stderr for diagnostics (wait progress).
- Exit codes: 0 success, 2 for CLI errors via existing `cli_error`.

## 8. Simplify

No unnecessary abstractions. The `LifecycleStatus` struct is only used for JSON serialization of status output. `HostProcessState` enum is a clean three-state model. No premature generalization.

## 9. Security

- No secrets, tokens, or credential handling.
- Minor note: `dispatch_dir(id)` joins user-supplied `id` to the store root without validating it as a ULID. A path traversal string like `../../../etc` could read outside the dispatches directory. In practice this only reads `result.json`/`status.json`/`host.pid` from the constructed path, limiting the blast radius. The IDs are generated internally as ULIDs and not attacker-controlled in normal use.

## 10. Contract assertions

| # | Assertion | Verdict | Evidence |
|---|-----------|---------|----------|
| 1 | `--async --prompt "test"` emits `{"kind":"async_started","dispatch_id":"..."}` and returns immediately | PASS | `print_async_started` writes JSON with `kind` and `dispatch_id` fields. Tested in `async_dispatch_flow_reports_status_and_result`. |
| 2 | `status <id>` returns JSON with state field | PASS | `print_status` -> `read_dispatch_status` returns `LifecycleStatus` with `state` field. Tested. |
| 3 | `result <id>` returns full DispatchResult JSON | PASS | `print_result_for_id` reads and prints `result.json`. Tested. |
| 4 | `wait <id>` blocks until complete, exits 0 | PASS | `wait_for_id` polls with 2s sleep, returns `exit_code_for_result` (0 for Completed). Tested. |
| 5 | Async creates `~/.darkfactory/dispatches/<ulid>/` with host.pid, status.json, events.jsonl | PASS | `prepare_async_store` + `write_host_pid` + `write_initial_status` + `open_events_log`. Test verifies events.jsonl exists. |
| 6 | result.json written on completion | PASS | `cli::write_result(store_dir, &result)` in main.rs line 82. Atomic via rename. |
| 7 | Status reports `orphaned` for dead PID with no result | PASS | `host_process_state` returns `Dead` when `kill(pid, 0)` gives ESRCH. `read_dispatch_status` maps to `"orphaned"`. Tested. |
| 8 | Directories older than 7 days pruned on new async dispatch | PASS | `prune_old_dispatches` with `DISPATCH_RETENTION_SECS = 7 * 24 * 60 * 60`. Uses ULID timestamp. Tested with 8-day-old directory. |

## 11. Behavioral conformance

The implementation matches the contract. The async dispatch model is: print ack to stdout immediately, run dispatch in foreground process, write result.json atomically on completion, clean up PID file via RAII. Lifecycle subcommands (status/result/wait) are independent processes that read from the shared dispatch directory. The orphan detection correctly uses `kill(pid, 0)` to check process liveness. Pruning is opportunistic (runs on each new async dispatch start).

All 8 assertions pass. No blocking issues found.
