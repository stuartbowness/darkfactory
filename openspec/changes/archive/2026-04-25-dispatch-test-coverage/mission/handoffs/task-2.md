Implemented the four engine error-path tests in [tests/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/engine.rs:154), with new mock modes in [tests/test_codex_mock.sh](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/test_codex_mock.sh:45).

Verification passed:
- `cargo fmt`
- `cargo test`

I also wrote the handoff at [task-2.md](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-2.md:1).

I could not create the requested commit because Git’s index is at `/Users/stuart/.darkfactory/.git/index`, outside the writable sandbox. `git add` failed with: `Operation not permitted`. I also could not write the parent mission handoff under `/Users/stuart/.darkfactory/openspec/...` for the same sandbox reason.