Implemented the requested tests and handoff.

Changes:
- Added `effective_grace_sec` unit tests in [src/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs:816).
- Added supervisor edge case tests in [tests/supervisor.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/supervisor.rs:15).
- Wrote handoff at [task-3.md](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-3.md:1).

Verification passed:
- `cargo fmt`
- `cargo test`

I could not create the commit because this checkout’s Git root is `/Users/stuart/.darkfactory`, and the sandbox cannot write `/Users/stuart/.darkfactory/.git/index.lock`. The failing step was `git add`, with `Operation not permitted`.

To commit from your shell:

```bash
git add -- \
  tools/darkfactory-dispatch/src/engine.rs \
  tools/darkfactory-dispatch/tests/supervisor.rs \
  tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-3.md

git commit -m "[test-3] Add grace math unit tests and supervisor edge case tests"
```