# Contract

All `cargo`/`make` commands run from `tools/darkfactory-dispatch/` (the crate is there, not the repo root).

## Task 1: Delete dead test-only code
<!-- Specs: none (internal cleanup — no observable behavior change; verified by build health + removal grep) -->
- [ ] [output] `grep -rnE "fn graceful_stop|fn heartbeat_ticker|struct CancellationToken|fn emit_heartbeat\b|StreamMode::(Silent|Verbose)|StdinMode::Piped|fn pid\b" src/` returns no matches (the named dead APIs are gone).
- [ ] [execution] From `tools/darkfactory-dispatch/`, `cargo build` and `cargo test` both succeed.
- [ ] [execution] From `tools/darkfactory-dispatch/`, `cargo clippy -- -D warnings` passes (no unfulfilled `#[expect(dead_code)]`, no dead-code warnings).

## Task 2: Remove the async subsystem and engine status/control
<!-- Specs: dispatch-cli/1, dispatch-cli/2, dispatch-cli/3, dispatch-cli/4, dispatch-cli/5, dispatch-cli/6, synchronous-dispatch/2, synchronous-dispatch/3, synchronous-dispatch/4 -->
- [ ] [execution] `darkfactory-dispatch --async --prompt x` is rejected by the argument parser: exit code 2, an "unexpected argument"/usage message on stderr, no `async_started` and no JSON object on stdout, and no dispatch files created. <!-- dispatch-cli/1 -->
- [ ] [execution] `darkfactory-dispatch status X`, `wait X`, and `result X` are each rejected by the parser as an unrecognized subcommand: exit code 2 with an "unrecognized subcommand"/usage message (NOT a runtime "dispatch not found"), and no JSON result. <!-- dispatch-cli/2 -->
- [ ] [execution] After a normal `--json` dispatch with no user `--log`/`--output` under those paths, no files exist under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/` created by that run. <!-- dispatch-cli/5 -->
- [ ] [execution] A file seeded under `/tmp/darkfactory-dispatch/` and one under `~/.darkfactory/dispatches/` before a dispatch are byte-identical after it (no pruning, deletion, or mutation). <!-- dispatch-cli/5 -->
- [ ] [output] `grep -rnE "control\.json|write_status_json_atomic|status_dir|store_dir|LiveStatus|enum Commands|async_mode" tools/darkfactory-dispatch/src/` returns no matches — the async/status/control internals are gone from the source. This directly verifies the status.json/store/control removal (the behavioral "no files after a dispatch" check alone can pass against the OLD code, which writes `/tmp/darkfactory-dispatch/<id>/status.json` mid-run and deletes it on exit) and the control-file channel removal (its path is keyed by a per-run dispatch id and cannot be pre-planted). <!-- dispatch-cli/6 -->
- [ ] [execution] A spawn failure where the engine binary cannot be found produces exactly one JSON result on stdout with `status` `"failed"` and exit code 2 (a missing binary is a CLI error per the crate convention); any other dispatch failure yields a `failed` result with exit code 1. <!-- synchronous-dispatch/3 -->
- [ ] [execution] A worker that exits 0 without creating the requested `--output` file yields a JSON result with `status` `"failed"`. <!-- synchronous-dispatch/2 -->
- [ ] [execution] A worker that exits non-zero yields a JSON result with `status` `"failed"` carrying the worker's exit code, and the dispatcher process exits 1. <!-- synchronous-dispatch/4 -->
- [ ] [execution] A dispatch with `--output P` writes the worker artifact to `P` and the JSON result reports `output` `P` with `output_exists` true; `--cwd`, `--log`, `--grace`, and `--heartbeat` are accepted without a parse error. <!-- dispatch-cli/4, dispatch-cli/3 -->
- [ ] [output] `Cargo.toml` declares no `tracing` or `tracing-subscriber` dependency, and neither appears as a direct dependency in `Cargo.lock`.
- [ ] [execution] From `tools/darkfactory-dispatch/`, `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.

## Task 3: Convert /adversarial to synchronous dispatch
<!-- Specs: adversarial-sync-review/1, adversarial-sync-review/2, adversarial-sync-review/3 -->
- [ ] [output] `grep -E "\-\-async|darkfactory-dispatch (status|wait|result)|async_started" skills/adversarial/SKILL.md` returns no matches. <!-- adversarial-sync-review/1 -->
- [ ] [output] `skills/adversarial/SKILL.md` dispatches its reviewer with `--json` run via `run_in_background` and collected with `BashOutput`. <!-- adversarial-sync-review/1 -->
- [ ] [output] `skills/adversarial/SKILL.md` reads the dispatch result `.status` from the captured output and emits a `REVIEW_COMPLETE` marker on `completed` and a `REVIEW_FAILED` marker on `failed`/`timed_out`. <!-- adversarial-sync-review/2, adversarial-sync-review/3 -->

## Task 4: Update the dispatch README and tool CLAUDE.md
<!-- Specs: dispatch-cli/7 -->
- [ ] [execution] `darkfactory-dispatch --help` lists no `--async` flag and no `status`/`wait`/`result` subcommands. <!-- dispatch-cli/7 -->
- [ ] [output] `tools/darkfactory-dispatch/README.md` contains no async-mode section, no `status`/`wait`/`result` lifecycle documentation, and no dispatch-store section, and it still contains the progress-monitoring (`tail -f`) guidance.
- [ ] [output] `tools/darkfactory-dispatch/CLAUDE.md` repo map no longer references the `status`/`wait`/`result` subcommands, async routing, the watchdog, or `LiveStatus`.

## Task 5: Smoke test and preserved supervision
<!-- Specs: synchronous-dispatch/1, synchronous-dispatch/5, synchronous-dispatch/6, synchronous-dispatch/7, synchronous-dispatch/8, synchronous-dispatch/9 -->
- [ ] [execution] `tests/smoketest.sh` (or `make smoke`) exercises the synchronous `--json` dispatch flow end-to-end and passes. <!-- synchronous-dispatch/1 -->
- [ ] [output] `tests/supervisor.rs` still contains the two-phase-timeout and process-group-teardown test functions (and the parent-death handling test), and `tests/engine.rs` still contains the heartbeat-emission test function — the retained supervision tests were not deleted. <!-- synchronous-dispatch/5, synchronous-dispatch/6, synchronous-dispatch/7, synchronous-dispatch/8 -->
- [ ] [execution] `cargo test` runs and passes the full `tests/supervisor.rs` and `tests/engine.rs` suites, which exercise the two-phase timeout, process-group teardown, heartbeat emission, and parent-death watch. <!-- synchronous-dispatch/5, synchronous-dispatch/6, synchronous-dispatch/7, synchronous-dispatch/8 -->
- [ ] [execution] A dispatch run as `darkfactory-dispatch ... --log f 2>>f` produces a file `f` containing both the worker's output and the dispatcher's heartbeat events. <!-- synchronous-dispatch/9 -->

## Task 6: Cleanup and closing checks
<!-- Specs: none (hygiene) -->
- [ ] [execution] From `tools/darkfactory-dispatch/`, `make check` (fmt `--check` + clippy `-D warnings` + test) is green, confirming no orphaned imports, dead references, or leftover scaffolding.
