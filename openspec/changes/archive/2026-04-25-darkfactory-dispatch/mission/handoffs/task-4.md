Implemented Task 4 in [src/event.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/event.rs:1) and wired it from [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs:3).

What changed:
- Added `DispatchEvent`, `Emitter`, `StreamMode`, typed `EventError`, and heartbeat ticker.
- Added stream-mode filtering for Silent, Normal, and Verbose.
- Added inline tests for heartbeat format, filtering, event type coverage, and ticker cancellation.
- Used a crate-local cancellation token to avoid adding a new dependency in the offline sandbox.

Verification passed:
- `cargo fmt --check`: exit 0
- `cargo test`: exit 0
- `cargo clippy --all-targets -- -D warnings`: exit 0

Blocked:
- Could not write `openspec/changes/darkfactory-dispatch/mission/handoffs/task-4.md`: sandbox blocks writes outside `tools/darkfactory-dispatch`.
- Could not commit: `git add` failed creating `/Users/stuart/.darkfactory/.git/index.lock` with `Operation not permitted`.

Commit SHA is unavailable because the sandbox blocked the Git index write.