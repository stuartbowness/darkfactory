Implemented and committed Task 5.

Commit: `05a1c67a825cc6e3a455bde031fee7c48f55834c`

Changed:
- Added combined-log CLI test in [cli.rs](/home/ironbank/.darkfactory/tools/darkfactory-dispatch/tests/cli.rs)
- Added supervisor two-phase timeout and Linux parent-death tests in [supervisor.rs](/home/ironbank/.darkfactory/tools/darkfactory-dispatch/tests/supervisor.rs)
- Wrote uncommitted handoff to [task-5.md](/home/ironbank/.darkfactory/openspec/changes/dispatch-sync-only/mission/handoffs/task-5.md)

Checks passed from `tools/darkfactory-dispatch/` with the required `TMPDIR` and `CARGO_HOME` workaround:
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`

`make smoke` was not run because it launches live Codex and may not complete in this sandbox. I verified `tests/smoketest.sh` structurally: it uses synchronous `--json`, parses `.status`, has no `--async`, and `make smoke` invokes the script.