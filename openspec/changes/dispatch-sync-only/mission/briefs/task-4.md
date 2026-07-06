# Worker Brief: Task 4 - Update the dispatch README and tool CLAUDE.md

## Your Task
Update the documentation of `tools/darkfactory-dispatch/` to reflect the now synchronous-only dispatcher. The async subsystem, the lifecycle subcommands, the dispatch store, and the control-file/status machinery no longer exist (removed in Task 2); the docs must not describe them.

### `tools/darkfactory-dispatch/README.md`
- Remove the async-mode documentation: any "Async dispatch"/async-mode section, the `--async` flag from the flag table, the `status`/`wait`/`result` lifecycle subcommands, and the "dispatch store" (`~/.darkfactory/dispatches/<id>/`, `status.json`/`result.json`/`host.pid`/`events.jsonl`, 7-day pruning) section.
- Remove or correct the async clause in the parent-death description (parent-death is now always on).
- KEEP the "Monitoring progress" guidance and the `tail -f` example prominent — heartbeats still stream to the dispatcher's stderr; capturing them in a tailable file needs the caller to redirect stderr (`2>>log`), so keep that accurate.
- Keep the description of the synchronous dispatch, the structured JSON result, the retained flags, and the "Dispatch result status" table (`completed`/`failed`/`timed_out`).

### `tools/darkfactory-dispatch/CLAUDE.md`
Update the repo map so it matches the code:
- `main.rs` no longer lists `status`/`result`/`wait` subcommands (it is now: parse args, dispatch synchronously, print result).
- `cli.rs` line no longer mentions "async routing".
- `engine.rs` line no longer mentions "watchdog".
- `types.rs` line no longer lists `LiveStatus`.
- If the "Reference implementation" table references `cli/async.go` for async/detached mode, drop that async row (there is no async mode anymore).
Leave the crate's Rules, Testing, Versioning, and Secrets sections unchanged except where they mention removed features.

## Acceptance criteria
- `darkfactory-dispatch --help` lists no `--async` flag and no `status`/`wait`/`result` subcommands. (This is produced by the built binary from Task 2 — you are only aligning the docs; do not change code.)
- `tools/darkfactory-dispatch/README.md` has no async-mode section, no lifecycle-subcommand docs, and no dispatch-store section, and still contains the `tail -f` monitoring guidance.
- `tools/darkfactory-dispatch/CLAUDE.md` repo map no longer references the `status`/`wait`/`result` subcommands, async routing, the watchdog, or `LiveStatus`.
- Any commands shown in the README that you change must reflect the real synchronous CLI.

## Project Context
The dispatcher was made synchronous-only. This task brings its docs in line so no reader is told about the removed async/lifecycle/store surface.

## Architecture
Docs-only change: `tools/darkfactory-dispatch/README.md` and `tools/darkfactory-dispatch/CLAUDE.md`. No source or test changes.

## Key Terms
`Dispatch` — spawning a supervised Codex subprocess. Result is a JSON object on stdout with a `status` field.

## Domain Types
None (docs change).

## What Previous Tasks Built
- Task 1 (07833a0): removed dead code.
- Task 2 (eece9ea): synchronous-only dispatcher — removed `--async`, the `status`/`wait`/`result` subcommands, the dispatch store, the control-file/status machinery, `store_dir`, `LiveStatus`, and the `tracing` deps. Retained flags: `--model/--effort/--timeout/--sandbox/--config/--prompt-file/--prompt/--output/--cwd/--log/--json/--grace/--heartbeat`. Result JSON: `status` (`completed`/`failed`/`timed_out`), plus `exit_code`, `output`, `output_exists`, tokens, activity. Exit codes: 0 completed, 1 failed/timed_out, 2 CLI error.
- Task 3 (ac41660): converted `/adversarial` to synchronous dispatch.

## Downstream Consumers
Task 6 runs the final cleanup + `make check`. No code consumes these docs.

## Procedures
**Before writing any change**, invoke the `/tdd` skill. This is a docs-only change (no compilable code), so verify by inspection that the async/lifecycle/store content is gone, the `tail -f` monitoring section remains, and the CLAUDE.md repo map matches the current source. You may run `darkfactory-dispatch --help` to confirm the doc matches the binary's real flags/subcommands.
- Commit in a single commit: `[task-4] Update dispatch README and CLAUDE.md for synchronous-only`. Stage only `tools/darkfactory-dispatch/README.md` and `tools/darkfactory-dispatch/CLAUDE.md`. Do NOT `git add -A`; never stage anything under `openspec/changes/dispatch-sync-only/`.
- Write your handoff to `openspec/changes/dispatch-sync-only/mission/handoffs/task-4.md` (write the file; do not commit it).

## Handoff Report Format
- Task: Task 4 - Update the dispatch README and tool CLAUDE.md
- Commit SHA: <the full SHA>
- Approach: <what sections you removed/kept>
- Commands run: <any>
- Concerns: <anything uncertain>
