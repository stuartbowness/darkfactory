# Task 6.2 Validation: Async dispatch and lifecycle subcommands

**Commit:** a2be004
**Date:** 2026-04-23
**Verdict:** PASS

## Test Results

- `cargo test`: 35 passed (4 suites)
- `cargo clippy --all-targets -- -D warnings`: clean

## Assertion Results

- [x] **[execution] `--async --prompt "test"` emits `{"kind":"async_started","dispatch_id":"..."}` to stdout and returns immediately**
  Verified in code (`cli.rs:171-184`, `AsyncStarted` struct) and test (`async_dispatch_flow_reports_status_and_result`). The ack JSON is written to stdout before the dispatch loop begins. The test reads it from a spawned child's piped stdout while the dispatch is still running.

- [x] **[execution] `status <id>` returns JSON with state field after an async dispatch**
  Verified in code (`cli.rs:191-193`, `read_dispatch_status`) and test (`async_dispatch_flow_reports_status_and_result` lines 102-112). The `LifecycleStatus` struct always includes a `state` field. Test asserts `status_json["state"] == "running"` during a live dispatch.

- [x] **[execution] `result <id>` returns the full DispatchResult JSON after async dispatch completes**
  Verified in code (`cli.rs:196-208`, `print_result_for_id` polls for `result.json`) and test (lines 120-131). Test asserts `result_json["status"] == "completed"` and `result_json["dispatch_id"]` matches. The output is the raw `DispatchResult` struct, not a wrapper.

- [x] **[execution] `wait <id>` blocks until the async dispatch completes, then exits 0**
  Verified in code (`cli.rs:210-231`, `wait_for_id` polls with 2s sleep) and test (lines 114-118). Test calls `wait` during a running `delayed_success` mock dispatch and asserts exit 0.

- [x] **[output] Async dispatch creates `~/.darkfactory/dispatches/<ulid>/` with host.pid, status.json, events.jsonl**
  Verified in code (`main.rs:45-59`): `prepare_async_store` creates the directory, `write_host_pid` writes `host.pid`, `write_initial_status` writes `status.json`, `open_events_log` creates `events.jsonl`. Test asserts `events.jsonl` exists. Note: `host.pid` is cleaned up on normal exit by `HostPidGuard::drop`, which is correct behavior (the PID file should only persist if the process died unexpectedly).

- [x] **[output] Async dispatch writes result.json to the dispatch directory on completion**
  Verified in code (`main.rs:81-82`): `cli::write_result(store_dir, &result)` writes `result.json` atomically via rename. The `result` subcommand reads this file, confirming it exists after completion.

- [x] **[execution] Status reports `orphaned` when host.pid references a dead process and no result.json exists**
  Verified in code (`cli.rs:322-330`, `HostProcessState::Dead` maps to `"orphaned"`) and test (`status_reports_orphaned_when_async_dispatcher_dies`). Test spawns an async dispatch with `sleep` mock, kills the dispatcher, then asserts `status_json["state"] == "orphaned"`. Process liveness check uses `kill(pid, 0)` with ESRCH detection (`cli.rs:381-387`).

- [x] **[execution] Dispatch directories older than 7 days are pruned when a new async dispatch starts**
  Verified in code (`cli.rs:135-136`, `prune_old_dispatches` called from `prepare_async_store`; retention is `7 * 24 * 60 * 60` seconds) and test (`async_dispatch_prunes_old_store_directories`). Test creates a directory with a ULID timestamped 8 days ago, starts a new async dispatch, and asserts the old directory was removed before the dispatch even finishes.

## Notes

- The `parent_death_watch` mechanism (kqueue on macOS, prctl on Linux) is correctly disabled for async dispatches (`supervisor.rs:141-143`, `engine.rs:46`) since the codex child must outlive any potential parent restart.
- Error handling for failed startup writes a `failed` result to `result.json` so lifecycle subcommands can observe it (`main.rs:72-78`).
- All file writes use atomic rename via temp files to prevent partial reads.
- The `HostPidGuard` RAII pattern ensures `host.pid` is cleaned up on normal exit but persists on abnormal exit, enabling orphan detection.
