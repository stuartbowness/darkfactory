# Worker Brief: Task 5 - Smoke test and preserved supervision

## Your Task
Ensure the crate's end-to-end smoke test exercises the synchronous-only dispatcher, confirm the load-bearing supervision tests are present and passing, and add a combined-log test.

### tests/smoketest.sh
- Verify it exercises the primary synchronous `--json` dispatch flow and references NO async surface (`--async`, `status`/`wait`/`result`). If it still references the removed surface, fix it to the synchronous `--json` form. Keep it runnable via `make smoke`.
- Environment: `make smoke` runs against a real `codex` on PATH and may not complete in this sandbox. If it cannot run here, verify its correctness by inspection (it uses the synchronous CLI, parses the JSON result `.status`, no async surface) and note that in the handoff.

### Preserved supervision tests
Confirm these retained tests exist and PASS (they verify behavior the mission must preserve). Add any that are missing:
- `tests/supervisor.rs`: a two-phase-timeout test, a process-group-teardown test, and a parent-death test. (If a parent-death test does not already exist, add a focused one.)
- `tests/engine.rs`: a heartbeat-emission test.

### Add a combined-log test
Add an integration test asserting that a dispatch run with `--log f` while the caller appends the dispatcher's stderr to the same file (`2>>f`) produces a file `f` containing BOTH the worker's output AND the dispatcher's heartbeat events. Use the existing mock-engine test helper.

### Running the tests in this sandbox
The mock `codex` used by `tests/cli.rs`/`tests/engine.rs` will not exec from `/tmp` in this sandbox. Run cargo with a repo-local temp and cargo home so the copied mock is executable, e.g.:
```
mkdir -p target/tmp
TMPDIR="$PWD/target/tmp" CARGO_HOME="$PWD/target/cargo-home" cargo test
```
Do NOT treat a `/tmp`-exec sandbox limitation as a test failure — use the workaround so the tests actually run.

## Acceptance criteria
- `tests/smoketest.sh` exercises the synchronous `--json` flow with no async surface (runs via `make smoke`, or verified structurally if real codex is unavailable here).
- `tests/supervisor.rs` contains passing two-phase-timeout, process-group-teardown, and parent-death tests; `tests/engine.rs` contains a passing heartbeat-emission test.
- A combined-log test (`--log f` + `2>>f` → worker output + heartbeat events in `f`) exists and passes.
- From `tools/darkfactory-dispatch/` (with the TMPDIR/CARGO_HOME workaround): `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` pass.

## Project Context
The dispatcher is now synchronous-only. This closing task verifies the primary user flow (smoke test) and that the preserved supervision (timeout, process group, heartbeat, parent-death) is actually covered by passing tests.

## Architecture
Crate at `tools/darkfactory-dispatch/` (NO root Cargo.toml). This task touches `tests/` (and possibly `tests/smoketest.sh`); no `src/` changes expected. Follow the crate CLAUDE.md testing rules (real subprocesses via mock helpers; `#[tokio::test(flavor = "multi_thread")]` for async tests).

## Key Terms
`Dispatch` — spawning a supervised Codex subprocess. Heartbeat — a periodic NDJSON event on the dispatcher's stderr with elapsed time and last activity.

## Domain Types
None new.

## What Previous Tasks Built
- Task 2 (eece9ea): synchronous-only dispatcher; live heartbeat is `emit_heartbeat_with_interval` (event.rs) driven by the inline `heartbeat_interval` arm (engine.rs); process group via `setpgid` (supervisor.rs); parent-death now always-on. Result JSON has `status`, `exit_code`, `output`, `output_exists`. Task 2 added CLI integration tests (`--async` rejection, no-store-files, seeded-files-unchanged, spawn-error, retained flags).
- Existing supervision tests in `tests/supervisor.rs` and `tests/engine.rs` were retained through the cut.

## Downstream Consumers
Task 6 runs final cleanup + `make check`.

## Procedures
**Before writing any change**, invoke the `/tdd` skill. Confirm the supervision tests pass first, then add the parent-death test (if missing) and the combined-log test, keeping everything green.
- Run cargo with the `TMPDIR`/`CARGO_HOME` workaround above.
- Commit in a single commit: `[task-5] Smoke test and preserved-supervision coverage`. Stage only files under `tools/darkfactory-dispatch/`. Do NOT `git add -A`; never stage anything under `openspec/changes/dispatch-sync-only/`.
- Write your handoff to `openspec/changes/dispatch-sync-only/mission/handoffs/task-5.md` (write the file; do not commit it), and in it state explicitly whether `make smoke` ran or was verified structurally, and which supervision tests already existed vs. were added.

## Handoff Report Format
- Task: Task 5 - Smoke test and preserved supervision
- Commit SHA: <the full SHA>
- Approach: <smoke-test status; which supervision tests existed/added; the combined-log test>
- Commands run: <commands and exit codes, including the TMPDIR/CARGO_HOME invocation>
- Concerns: <anything uncertain>
