# Worker Brief: Task 2 - Remove the async subsystem and engine status/control

## Your Task
In ONE compile-safe change, remove the async-dispatch subsystem and the engine's status/control machinery from `tools/darkfactory-dispatch/`, making the dispatcher synchronous-only, and add integration tests for the new observable behaviors.

### Remove
- **CLI (`cli.rs`, `main.rs`):** the `--async` flag and the `conflicts_with = "async_mode"` on `--json`; the `Commands` enum and its `status`/`wait`/`result` subcommands and the `command: Option<Commands>` field; the store helpers (`prepare_async_store`, `write_host_pid`, `print_async_started`, `write_result`, `wait_for_id`, `read_dispatch_status`, `dispatch_store_root`, `dispatch_dir`, `AsyncStarted`, and the status/result JSON writers); the async examples in the `--help` text; in `main.rs` the `async_mode` branch, `handle_lifecycle_command`, `HostPidGuard`, and the `Commands` import.
- **Engine (`engine.rs`):** the entire watchdog `select` arm (BOTH the `control.json` read AND the in-loop `status.json` write); `WATCHDOG_INTERVAL_SECS`; `read_control_file` + `ControlFile` + `abort_error`; BOTH `write_status_json_atomic` call sites plus `write_status_json_atomic` itself, `status_dir`, and the `cleanup_status_dir`/`status_dir` plumbing. Confirm the inline `heartbeat_interval` select arm REMAINS.
- **Spec/types:** drop `store_dir` from `DispatchSpec`; remove the `LiveStatus` type and all its imports/uses; `engine.rs`'s `.parent_death_watch(spec.store_dir.is_none())` becomes always-on (pass `true` / rely on the builder default).
- **Deps:** remove `tracing` and `tracing-subscriber` from `Cargo.toml`, then regenerate `Cargo.lock` (a `cargo build` will do it) so neither remains a direct dependency.

### Compile-safety constraint (critical)
`engine.rs` reads `spec.store_dir` for status-dir selection, cleanup, parent-death, and both status writes. So `DispatchSpec.store_dir` and ALL its engine readers MUST be removed together in this single change — never remove the field while a reader still references it. The crate must `cargo build` cleanly when you are done (one commit).

### Preserve the spawn-error JSON result (behavior preservation, not a new feature)
Today `main.rs` turns a dispatch `Err` into a `failed` JSON result ONLY on the async `store_dir` branch (`cli::failed_start_result`); the sync branch returns a bare `Err` with no JSON. After removing async, move that conversion into the unified synchronous path so a dispatch `Err` still prints exactly one `failed` `DispatchResult` on stdout and exits non-zero. Exit codes per the crate convention (`CLAUDE.md`): `completed` → 0; `failed`/`timed_out` → 1; CLI error including a missing engine binary (`DispatchError::BinaryNotFound`) → 2.

### Keep (do NOT remove)
Process-group isolation, two-phase timeout, heartbeat emission (`emit_heartbeat_with_interval`), parent-death detection (now always on), the structured JSON result on stdout, `--log`, and the `--grace`/`--heartbeat` flags. Keep `DEFAULT_HEARTBEAT_INTERVAL_SECS` in `types.rs`. Leave `types.rs`'s `#[expect(dead_code)]` and its `DispatchError` variants untouched (out of scope).

### Add integration tests for the new observable behaviors
In `tests/cli.rs` / `tests/engine.rs`, add tests asserting:
- `darkfactory-dispatch --async --prompt x` is rejected by the argument parser: exit code 2, an "unexpected argument"/usage message, no `async_started` or JSON on stdout, and no dispatch files created.
- `status X`, `wait X`, `result X` are each rejected as an unrecognized subcommand: exit code 2 (NOT a runtime "dispatch not found").
- After a normal `--json` dispatch with no user `--log`/`--output` under those paths, no files exist under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/`.
- A file seeded under each of those paths before a dispatch is byte-identical after it.
- A spawn failure (missing engine binary) yields one `failed` JSON result on stdout and exit code 2.
- The retained flags `--output`/`--cwd`/`--log`/`--grace`/`--heartbeat` are accepted; `--output P` writes the artifact and the result reports `output` `P` with `output_exists` true.
Use the existing test-helper/mock-engine pattern. Sandbox note: `/tmp` may refuse to exec copied scripts — set `TMPDIR=target/tmp` and a repo-local `CARGO_HOME` if needed (the Task 1 worker did this).

## Acceptance criteria
- No `--async`, `status`/`wait`/`result` subcommands, dispatch store, control-file read, `status.json` write, `store_dir`, `LiveStatus`, or `tracing`/`tracing-subscriber` remain in `src/`, `Cargo.toml`, or `Cargo.lock`.
- A spawn error still emits a `failed` JSON result; all supervision (timeout, heartbeat, process group, parent-death) is preserved.
- From `tools/darkfactory-dispatch/`: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.

## Project Context
`darkfactory-dispatch` supervises Codex workers. It is being made synchronous-only. This is the core removal task: the async lifecycle (a `--async` flag, `status`/`wait`/`result` subcommands, and a `~/.darkfactory/dispatches/<ulid>/` store) and the engine's control-file + status-file machinery are removed, leaving the single synchronous path production already uses.

## Architecture
Crate at `tools/darkfactory-dispatch/` (NO root `Cargo.toml`; run all `cargo` commands there). Modules: `main.rs` (entry), `cli.rs` (clap + `DispatchSpec`), `engine.rs` (`dispatch()` loop), `supervisor.rs` (process group, signals, parent-death), `event.rs` (emitter), `types.rs` (`DispatchSpec`, `DispatchResult`, `DispatchStatus`, `DispatchError`, ...). Follow the crate's `CLAUDE.md` rules (Gallant boundary: engine owns dispatch logic, CLI only parses/formats; `thiserror` not `anyhow`; `pub(crate)`; `#[expect]` not `#[allow]`; no `#[serde(flatten/untagged)]`; stdout=result JSON, stderr=diagnostics).

## Key Terms
`Dispatch` — spawning a supervised Codex subprocess. `Dispatch result` — the structured JSON object on stdout.

## Domain Types
- `DispatchSpec` — inputs to a dispatch (drop `store_dir`); owning module `types.rs`.
- `DispatchResult` — the structured result (status, exit code, output, output_exists, ...); `types.rs`.
- `DispatchStatus` — `completed` | `failed` | `timed_out`; `types.rs`.
- `DispatchError` — error type (`BinaryNotFound` → CLI error exit 2); `types.rs`.

## What Previous Tasks Built
### Task 1: Delete dead test-only code (commit 07833a0)
- Removed dead test-only APIs from `supervisor.rs` and `event.rs` (`graceful_stop`, `RunHandle::pid` + `RunState.pid`, `StdinMode::Piped`, singular `arg`, `CancellationToken`, `heartbeat_ticker`, `Emitter::emit_heartbeat`, event-local heartbeat const, `StreamMode::Silent`/`Verbose`) and their tests; resolved `#[expect(dead_code)]` in `event.rs` + `main.rs`.
- The live heartbeat path (`emit_heartbeat_with_interval` + the inline `heartbeat_interval` arm in `engine.rs`) is intact. `types.rs` `#[expect(dead_code)]` + `DispatchError` variants remain.

## Downstream Consumers
Task 3 converts the `/adversarial` skill to call this synchronous `--json` CLI (no `--async`/`wait`). Task 4 documents the final synchronous behavior. Task 5 verifies the preserved supervision. Keep the synchronous `--json` result schema (`status`, `output`, `output_exists`, exit codes) stable.

## Procedures
**Before writing any code**, invoke the `/tdd` skill and follow its workflow. Write the new integration tests for the observable behaviors first (red), then implement the removal to make them pass (green), keeping the existing supervision tests green.
- Run all `cargo` commands from `tools/darkfactory-dispatch/`.
- Commit in a single commit: `[task-2] Remove async subsystem and engine status/control`. Stage only files under `tools/darkfactory-dispatch/`. Do NOT `git add -A`; never stage anything under `openspec/changes/dispatch-sync-only/`.
- Write your handoff to `openspec/changes/dispatch-sync-only/mission/handoffs/task-2.md` (write the file; do not commit it).

## Handoff Report Format
- Task: Task 2 - Remove the async subsystem and engine status/control
- Commit SHA: <the full SHA>
- Approach: <what you removed, how you kept it compile-safe, how you moved failed_start_result>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
