Implemented and committed Task 6.

Commit: `41b43e768e8fbb05da0256f2a805ef774c8e23f4`

Changed under `tools/darkfactory-dispatch/`:
- Added the two CLI tests in `tests/cli.rs`.
- Removed single-variant `StreamMode`.
- Dedupe status string handling into `DispatchStatus::as_str()`.
- Removed stale `#[expect(dead_code)]` and unused `DispatchError` variants.
- Cleaned the stale stream-mode note in `CLAUDE.md`.

Handoff written, uncommitted as requested:
`openspec/changes/dispatch-sync-only/mission/handoffs/task-6.md`

Checks run:
- `cargo test json_failed_dispatch_exits_1_and_reports_worker_exit_code`: exit 0
- `cargo test plain_dispatch_output_reports_status_duration_and_output`: exit 0
- `cargo fmt --check`: exit 0
- `cargo clippy --all-targets -- -D warnings`: exit 0
- `cargo test`: exit 0
- `make check`: exit 0

No checks skipped. `tools/darkfactory-dispatch/` has no remaining uncommitted diff.