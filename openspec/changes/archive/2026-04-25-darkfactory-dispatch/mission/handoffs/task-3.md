Implemented task 3 in [src/types.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/types.rs:1) and wired it from [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs:8).

Added:
- `DispatchSpec`, `DispatchResult`, `DispatchStatus`
- `DispatchError` with `thiserror`
- `DispatchErrorInfo`, `TokenUsage`, `Activity`
- `HarnessEvent`, `EventKind`, `LiveStatus`
- Inline tests for JSON round-trip, status serialization, error display, and defaults

Verification passed:
- `cargo fmt --check` exit 0
- `cargo test` exit 0
- `cargo clippy --all-targets -- -D warnings` exit 0

Blocked:
- Could not update `openspec/changes/darkfactory-dispatch/tasks.md` or write `mission/handoffs/task-3.md`: sandbox blocks writes outside `tools/darkfactory-dispatch`.
- Could not commit: `git add` failed because Git needs `/Users/stuart/.darkfactory/.git/index.lock`, which is outside the writable sandbox.

Handoff report content:

```md
Task: Task 3 - Types module
Commit SHA: unavailable; git index write blocked by sandbox
Approach: Implemented the shared dispatcher data structures in src/types.rs with serde derives, thiserror for DispatchError, snake_case DispatchStatus serialization, defaults for token/activity tracking, and focused inline tests. Updated main.rs to include mod types.
Commands run:
- cargo fmt --check: exit 0
- cargo test: exit 0
- cargo clippy --all-targets -- -D warnings: exit 0
Concerns: OpenSpec task checkbox, handoff file, and commit could not be completed because the sandbox cannot write outside tools/darkfactory-dispatch or create the repo index lock.
```