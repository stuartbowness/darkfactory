Implemented and committed Task 2.

Commit: `eece9ea969c296baaf3a0baf170f5c9ee8bc424e`

Removed:
- CLI async flag, lifecycle subcommands, async store/status/result helpers.
- Engine watchdog control/status file machinery.
- `DispatchSpec.store_dir` and `LiveStatus`.
- Direct `tracing` / `tracing-subscriber` deps from `Cargo.toml` and `Cargo.lock`.

Preserved:
- Synchronous dispatch path.
- Heartbeats, timeout/grace handling, process group isolation, parent-death detection.
- Startup failure JSON result on stdout, including missing `codex` binary exit code `2`.

Added integration coverage for:
- `--async` rejection.
- `status` / `wait` / `result` parser rejection.
- No writes under old dispatch-store paths.
- Existing sentinel files under old store paths remain unchanged.
- Missing engine binary emits one failed JSON result.
- `--output`, `--cwd`, `--log`, `--grace`, `--heartbeat` still work.

Checks run from `tools/darkfactory-dispatch/`:
- `cargo build` -> pass
- `cargo test` -> pass
- `cargo clippy -- -D warnings` -> pass
- `cargo fmt --check` -> pass
- Forbidden-symbol scans -> no matches

Handoff written, not committed:
[task-2.md](/home/ironbank/.darkfactory/openspec/changes/dispatch-sync-only/mission/handoffs/task-2.md)