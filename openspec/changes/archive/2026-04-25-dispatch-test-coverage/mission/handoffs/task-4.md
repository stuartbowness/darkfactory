Implemented Task 4.

Changed:
- [tests/cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/cli.rs): added unknown-id negative tests for `status`, `result`, and `wait`.
- [task-4.md](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-4.md): wrote the handoff.

Verified:
- `cargo fmt --check`
- `cargo test` passed: 65 tests total.

I could not create the commit because this repo’s Git dir is `/Users/stuart/.darkfactory/.git`, which is outside the writable sandbox roots. The commit command failed while creating `.git/index.lock`. The changes are still in the working tree and unstaged.

To commit from a normal shell:

```bash
git add \
  tools/darkfactory-dispatch/tests/cli.rs \
  tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-4.md

git commit -m \
  "[test-4] Add lifecycle negative path tests for unknown dispatch IDs"
```