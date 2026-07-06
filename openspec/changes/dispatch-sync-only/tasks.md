# Tasks: dispatch-sync-only

## 1. Feature Tasks

- [x] Task 1: Delete dead test-only code
  Depends on: none
  Consumes: existing crate (supervisor.rs, event.rs, types.rs)
  Produces: removed dead APIs (`graceful_stop`, `RunHandle::pid` + the `RunState.pid` field, `StdinMode::Piped`, singular `SupervisorBuilder::arg`, `CancellationToken`, `heartbeat_ticker`, `StreamMode::Silent|Verbose`, `Emitter::emit_heartbeat`, the event-local heartbeat constant) and the tests that only exercise them; `#[expect(dead_code)]` annotations resolved as `cargo build` flags them
  Layers: supervisor, event, types
  Skill: /tdd

- [x] Task 2: Remove the async subsystem and engine status/control in one compile-safe step
  Depends on: 1
  Consumes: cli.rs, main.rs, engine.rs, types.rs, Cargo.toml, Cargo.lock
  Produces: synchronous-only CLI (no `--async`; no `status`/`wait`/`result` subcommands; no store helpers; clap `conflicts_with` removed); engine with no watchdog arm, no `control.json` read, and no `status.json` writes (BOTH `write_status_json_atomic` sites), no `store_dir`/`status_dir`; `DispatchSpec` without `store_dir`; `LiveStatus` removed; `tracing` + `tracing-subscriber` deps dropped from `Cargo.toml` AND `Cargo.lock` regenerated so the removed direct deps are gone; `cli::failed_start_result` moved into the unified sync path (dispatch `Err` -> `failed` JSON on stdout, non-zero exit); parent-death always-on; integration tests added for the new observable behaviors — `--async` and `status`/`wait`/`result` rejected (non-zero exit, no JSON, no files), no files under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/` after a normal dispatch, a planted `control.json` is ignored, a spawn error yields a `failed` JSON result, pre-existing files seeded under `/tmp/darkfactory-dispatch/` and `~/.darkfactory/dispatches/` are left unchanged by a dispatch (no pruning/deletion), and the retained flags `--output`/`--cwd`/`--log`/`--grace`/`--heartbeat` are accepted and functional
  Layers: cli, engine, types, configuration
  Skill: /tdd

- [x] Task 3: Convert /adversarial to synchronous dispatch
  Depends on: 2
  Consumes: the synchronous `--json` CLI from Task 2
  Produces: `skills/adversarial/SKILL.md` dispatches its reviewer via synchronous `--json` + `run_in_background` + `BashOutput`, parsing `.status` from captured stdout and emitting `REVIEW_COMPLETE`/`REVIEW_FAILED`; no `--async` and no `darkfactory-dispatch` lifecycle subcommand (`status`/`wait`/`result`) or dispatch-id lookup
  Layers: skill
  Skill: /tdd

- [x] Task 4: Update the dispatch README and tool CLAUDE.md
  Depends on: 2, 3
  Consumes: the final synchronous behavior
  Produces: `tools/darkfactory-dispatch/README.md` with the async-mode, lifecycle-subcommand, and dispatch-store sections removed and the `tail -f` monitoring section kept; `tools/darkfactory-dispatch/CLAUDE.md` repo map updated (`main.rs` no longer lists status/wait/result; `cli.rs` no async routing; `engine.rs` no watchdog; `types.rs` no `LiveStatus`)
  Layers: docs
  Skill: /tdd

## 2. Closing Tasks

- [x] Task 5: Smoke test (mandatory)
  Depends on: 2
  Consumes: the synchronous dispatcher
  Produces: `tests/smoketest.sh` verified/extended to exercise the primary synchronous `--json` dispatch flow end-to-end with no async surface; the existing supervision tests — two-phase timeout, process-group teardown, and heartbeat emission in `tests/supervisor.rs` and `tests/engine.rs` — are retained and pass, verifying the preserved supervision (the synchronous-dispatch scenarios for timeout, process group, heartbeat, and log capture); the run is documented in the handoff
  Layers: tests
  Skill: /tdd

- [x] Task 6: Cleanup and closing checks (mandatory)
  Depends on: 1, 2, 3, 4, 5
  Consumes: the full change
  Produces: no orphaned imports, dead references, or leftover scaffolding; `make check` (fmt + clippy `-D warnings` + test) green from `tools/darkfactory-dispatch/`; a one-line module-shape re-check note (optional for <5 feature tasks) confirming the module inventory still matches design.md; `wc -l src/*.rs` totals under ~2,100 (informational target — the binding gate is `make check` green)
  Layers: tests, docs
  Skill: /tdd

## Closing-task coverage (mission mandatory set)

- Smoke test → Task 5.
- Cleanup → Task 6.
- Module-shape re-check → optional (<5 feature tasks); covered by the one-line note in Task 6.
- CLAUDE.md + ARCH.md generation → no-op: both already exist (the necessary tool-CLAUDE.md accuracy update is Task 4; the mission rule says do not regenerate an existing one).
- Documentation → covered by Task 4 (README + tool CLAUDE.md).
- CI configuration → not needed (no pipeline change for this removal).
