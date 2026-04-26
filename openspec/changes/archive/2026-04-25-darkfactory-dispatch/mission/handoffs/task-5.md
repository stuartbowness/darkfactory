Implemented Task 5 engine work in:

- [src/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs)
- [src/event.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/event.rs)
- [src/types.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/types.rs)
- [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs)
- [tests/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/engine.rs)
- [tests/test_codex_mock.sh](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/test_codex_mock.sh)

What changed:
- Added `engine::dispatch(...)` with Codex arg construction, supervisor spawning, NDJSON parsing, event emission, activity/token tracking, log teeing, heartbeat/watchdog timers, two-phase timeout, atomic `status.json`, and result finalization.
- Added engine integration tests for success, timeout, heartbeat, and malformed NDJSON.
- Aligned heartbeat default to 15s and added a parse error counter to `Activity`.

Verification:
- `cargo fmt --check`: exit 0
- `cargo test`: exit 0
- `cargo clippy --all-targets -- -D warnings`: exit 0
- `git diff --check`: exit 0

Blocked:
- Could not write `../../openspec/changes/darkfactory-dispatch/mission/handoffs/task-5.md` or mark `tasks.md`: sandbox blocks writes outside `tools/darkfactory-dispatch`.
- Could not commit: `git commit` failed with `fatal: Unable to create '/Users/stuart/.darkfactory/.git/index.lock': Operation not permitted`.

Commit SHA: unavailable because the git index write was blocked.