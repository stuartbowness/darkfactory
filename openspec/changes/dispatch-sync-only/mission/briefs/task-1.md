# Worker Brief: Task 1 - Delete dead test-only code

## Your Task
Remove dead, test-only code from the Rust crate `tools/darkfactory-dispatch/` and delete the tests that only exercise it. This is the first, risk-free step of making the dispatcher synchronous-only.

Remove exactly these items (each has NO production caller — only tests reference them):
- `src/supervisor.rs`: `RunHandle::graceful_stop`; `RunHandle::pid` AND the now-dead `RunState.pid` field it backs; the `StdinMode::Piped` variant and its match arm; the singular `SupervisorBuilder::arg` method.
- `src/event.rs`: `CancellationToken`; `heartbeat_ticker`; the `StreamMode::Silent` and `StreamMode::Verbose` variants (production constructs only `StreamMode::Normal` — collapse `StreamMode` accordingly or remove unused variants); `Emitter::emit_heartbeat` (the no-interval one); and the event-local `DEFAULT_HEARTBEAT_INTERVAL_SECS` constant (distinct from the live one in `types.rs`).
- Delete the tests that exist only to exercise the above: in `src/event.rs` the `stream_mode_filtering` and `heartbeat_ticker_can_be_cancelled` tests; in `tests/supervisor.rs` the `graceful_stop_*` tests and the singular `.arg(...)` usages; in `tests/engine.rs` any use of `StreamMode::Verbose` (switch it to `StreamMode::Normal`).

### CRITICAL — do NOT touch the live heartbeat
The live 15s heartbeat is `Emitter::emit_heartbeat_with_interval`, driven by the inline `heartbeat_interval` arm in `engine.rs`'s select loop. Leave it and `DEFAULT_HEARTBEAT_INTERVAL_SECS` in `types.rs` completely untouched. Only the *dead* helpers above are removed.

### Build-driven annotation cleanup
`#[expect(dead_code)]` fails the build if there is no remaining dead code to expect. After the deletions, `cargo build` will flag the now-unfulfilled annotations in `src/event.rs` and `src/main.rs` — remove exactly those. DO NOT remove the `#[expect(dead_code)]` in `src/types.rs` or touch the `DispatchError` variants it covers — those are out of scope for this task.

## Acceptance criteria
- The named dead APIs are gone from `src/`; the live heartbeat path is unchanged.
- From `tools/darkfactory-dispatch/`: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.

## Project Context
`darkfactory-dispatch` is a Rust process supervisor for Codex workers. It is being made synchronous-only by removing incidental machinery (async subsystem, control-file channel, status writes) and dead code. This task removes the pre-existing dead test-only code first, in isolation, so later tasks start clean.

## Architecture
The crate lives at `tools/darkfactory-dispatch/` (there is NO root `Cargo.toml` — run all `cargo` commands from that directory). Modules: `main.rs` (entry), `cli.rs` (arg parsing), `engine.rs` (dispatch loop), `supervisor.rs` (process group, signals, parent-death), `event.rs` (event emitter), `types.rs` (data types). This task touches only `supervisor.rs`, `event.rs`, their tests, and the `#[expect(dead_code)]` annotations in `event.rs`/`main.rs`.

## Key Terms
None domain-specific for this task — it is internal cleanup.

## Domain Types
None for this task.

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
Task 2 (remove the async subsystem and engine status/control) builds on the cleaned crate. No later task consumes a specific API from this task; the integration point is simply that the crate still builds and its retained tests pass.

## Procedures
**Before writing any code**, invoke the `/tdd` skill and follow its workflow. For this removal task, "red-green-refactor" means: confirm the suite is green, remove the dead code and its tests, and keep the suite green — adding no new behavior.

- Run all `cargo` commands from `tools/darkfactory-dispatch/`.
- Commit your changes in a single commit with message: `[task-1] Delete dead test-only code`. Stage only the files you changed under `tools/darkfactory-dispatch/`. Do NOT run `git add -A`/`git add .`, and never stage anything under `openspec/changes/dispatch-sync-only/` — that is mission bookkeeping the orchestrator owns.
- Write your handoff report to `openspec/changes/dispatch-sync-only/mission/handoffs/task-1.md` (write the file; do not commit it).

## Handoff Report Format
- Task: Task 1 - Delete dead test-only code
- Commit SHA: <the full SHA>
- Approach: <what you removed and how you handled the annotations>
- Commands run: <commands and their exit codes>
- Concerns: <anything uncertain>
