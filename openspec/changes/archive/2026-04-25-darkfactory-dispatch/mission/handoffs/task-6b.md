Implemented the async dispatch and lifecycle work in the worktree.

Changed:
- [src/cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/cli.rs): async store helpers, `status`, `result`, `wait`, PID/orphan detection, atomic `result.json`, cleanup pruning.
- [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs): async startup flow, event log wiring, lifecycle routing.
- [src/engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs): optional `store_dir` for watchdog status and final status.
- [src/supervisor.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/supervisor.rs): parent-death watcher now actually disables Linux `PDEATHSIG`.
- [tests/cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/cli.rs): async flow, wait, orphan detection, cleanup pruning.

Verification passed:
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --check`
- `git diff --check -- ...`

Blocked by sandbox:
- Could not write `../../openspec/changes/darkfactory-dispatch/mission/handoffs/task-6b.md`.
- Could not commit because `.git/index.lock` is outside the writable sandbox.

The blocked git error was:
```text
fatal: Unable to create '/Users/stuart/.darkfactory/.git/index.lock': Operation not permitted
```

To finish outside this sandbox:
```bash
git add \
  tools/darkfactory-dispatch/src/cli.rs \
  tools/darkfactory-dispatch/src/engine.rs \
  tools/darkfactory-dispatch/src/main.rs \
  tools/darkfactory-dispatch/src/supervisor.rs \
  tools/darkfactory-dispatch/src/types.rs \
  tools/darkfactory-dispatch/tests/cli.rs \
  tools/darkfactory-dispatch/tests/engine.rs

git commit -m "[task-6b] Implement async dispatch mode and status/result/wait lifecycle subcommands"
```

Handoff content to save once a commit SHA exists:
```text
Task: Task 6.2 - Async dispatch and lifecycle subcommands
Commit SHA: <full SHA>
Approach: Added durable async dispatch stores under ~/.darkfactory/dispatches/<id>, with fsynced host.pid, initial/final status.json, events.jsonl, atomic result.json, and 7-day store pruning. Added status/result/wait lifecycle commands as CLI disk readers, and threaded an optional store_dir through DispatchSpec so the engine watchdog writes durable status for async dispatches. Disabled parent-death watching for async dispatches, including Linux pre_exec pdeathsig.
Commands run: cargo fmt; cargo fmt --check; cargo test; cargo clippy --all-targets -- -D warnings; git diff --check.
Concerns: OpenSpec CLI reported no active change named darkfactory-dispatch, so implementation followed the worker brief and project CLAUDE.md directly.
```