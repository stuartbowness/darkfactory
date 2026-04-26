# Worker Brief: Task 6.2 - Async dispatch and lifecycle subcommands

## Your Task
Add async dispatch mode and the status/result/wait lifecycle subcommands to the existing CLI. This makes darkfactory-dispatch usable for fire-and-forget dispatch with later collection.

### What to implement

**1. Async dispatch mode (`--async` flag):**

When `--async` is passed:
1. Generate a ULID dispatch ID
2. Create `~/.darkfactory/dispatches/<ulid>/` directory
3. Write `host.pid` (current process PID, fsynced)
4. Emit `{"kind":"async_started","dispatch_id":"<ulid>"}` to stdout
5. Continue running the dispatch in the current process (the caller is expected to have backgrounded it)
6. During dispatch, write `status.json` and `events.jsonl` to the dispatch directory
7. On completion, write `result.json` to the dispatch directory
8. Clean up `host.pid` on exit (defer/drop)

Reference: `~/Code/agent-mux/cmd/agent-mux/async.go:19-110`

**2. Dispatch store at `~/.darkfactory/dispatches/<ulid>/`:**

Files in the dispatch directory:
- `host.pid` -- PID of the dispatcher process (for orphan detection)
- `status.json` -- current LiveStatus (written atomically every 5s by the watchdog)
- `events.jsonl` -- append-only event log
- `result.json` -- final DispatchResult (written on completion)

**3. Status subcommand (`darkfactory-dispatch status <id>`):**

1. Resolve the dispatch directory at `~/.darkfactory/dispatches/<id>/`
2. If `result.json` exists: read it, report the terminal state (completed/timed_out/failed)
3. If `result.json` doesn't exist and `host.pid` exists:
   - Read the PID
   - Check if the process is alive: `libc::kill(pid, 0)` returns 0 -> running; returns ESRCH -> dead
   - If alive: read `status.json`, report state "running" with elapsed and last activity
   - If dead: report state "orphaned"
4. If neither exists: report error "dispatch not found"
5. Output as JSON to stdout

Reference: `~/Code/agent-mux/cmd/agent-mux/lifecycle.go:132-138`

**4. Result subcommand (`darkfactory-dispatch result <id>`):**

1. Resolve the dispatch directory
2. If `result.json` exists: read and emit to stdout
3. If dispatch is still running: poll every 1 second until `result.json` appears, then emit
4. If orphaned: report error

Reference: `~/Code/agent-mux/cmd/agent-mux/lifecycle.go:156-275`

**5. Wait subcommand (`darkfactory-dispatch wait <id>`):**

1. Poll every 2 seconds, checking `status.json` and `result.json`
2. Emit periodic status lines to stderr: `[60s] running | 3 tools | 1 files changed`
3. When terminal state reached: exit 0 for completed, exit 1 for failed/timed_out

Reference: `~/Code/agent-mux/cmd/agent-mux/lifecycle.go:635-744`

**6. Parent-death reaper disabled in async mode:**

When `--async` is set, do NOT start the parent-death watcher. The caller exits immediately, and the dispatcher must survive.

Reference: `~/Code/agent-mux/internal/engine/loop.go:474-476`

**7. Dispatch store cleanup:**

On each new async dispatch, scan `~/.darkfactory/dispatches/` and remove directories older than 7 days (by ULID timestamp or mtime).

**8. Update engine.rs to accept optional dispatch store path:**

The engine's watchdog needs to know where to write `status.json`. Add an optional `store_dir: Option<PathBuf>` to the dispatch function or DispatchSpec. When set, the watchdog writes status.json there. When not set (sync mode), it writes to a temp directory that's cleaned up after dispatch.

### Tests to write

Add tests in `tests/cli.rs`:

1. **Async dispatch flow**: Run `darkfactory-dispatch --async --prompt "echo test"` with a mock codex, capture stdout, verify `async_started` JSON with dispatch_id. Then run `darkfactory-dispatch status <id>` and verify it reports a state. Then run `darkfactory-dispatch result <id>` and verify the full DispatchResult.

2. **Orphan detection**: Start an async dispatch, kill the dispatcher process, then run `status <id>` and verify it reports "orphaned".

3. **Store cleanup**: Create fake dispatch directories with old timestamps, start a new async dispatch, verify old directories are pruned.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Gallant boundary: lifecycle subcommands read from disk and format output, no engine logic
- `pub(crate)` visibility
- Atomic file writes for result.json (same pattern as status.json)

## What Previous Tasks Built
### Task 1-5: Foundation + Engine
- Full supervisor, types, event emitter, engine with dispatch loop

### Task 6.1: CLI sync dispatch
- src/cli.rs: clap CLI with all flags, sync dispatch path, JSON/human output
- src/main.rs: argument parsing, ULID generation, emitter creation, engine::dispatch call
- 32 tests passing

## Downstream Consumers
Task 9.1 (smoke test) will test end-to-end async dispatch.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Add async dispatch mode to src/cli.rs and src/main.rs
- Implement status/result/wait subcommands
- Add dispatch store management
- Update engine to accept optional store_dir
- Write tests in tests/cli.rs
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit: "[task-6b] Implement async dispatch mode and status/result/wait lifecycle subcommands"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-6b.md

## Handoff Report Format
- Task: Task 6.2 - Async dispatch and lifecycle subcommands
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
