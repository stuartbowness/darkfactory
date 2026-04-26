Implemented Task 1.

Changed [src/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs:352):
- Added table-driven `parse_codex_event` tests covering the requested NDJSON cases, plus ignored progress events and command execution branches.
- Added focused extraction tests for session id, tools, commands, text, file paths, and token counts.
- Updated parser behavior so lowercase `read` / `write` map to file read/write, and unknown JSON event types return `None`.

Wrote the handoff at [task-1.md](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/openspec/changes/dispatch-test-coverage/mission/handoffs/task-1.md:1).

Verification: `cargo fmt && cargo test` passes.

I could not create the requested commit because this checkout’s Git root is `/Users/stuart/.darkfactory`, and the sandbox cannot write `/Users/stuart/.darkfactory/.git/index.lock` (`Operation not permitted`). The changes are ready to commit with message:

`[test-1] Add parser table tests for all codex NDJSON event types`