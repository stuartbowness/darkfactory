# Proposal: dispatch-sync-only

## Problem

`tools/darkfactory-dispatch` is a Rust process supervisor for Codex workers (2,990 LOC). Its supervision core is solid, but it carries roughly 1,000 lines of incidental machinery on top: an async-dispatch subsystem (`--async`, `status`/`wait`/`result` subcommands, and a `~/.darkfactory/dispatches/<ulid>/` store), a control-file abort channel, redundant `/tmp` status.json writes, and dead code that only its own tests exercise.

This machinery is not free. The async lifecycle has exactly one consumer, the `/adversarial` skill, and it uses it incorrectly: the dispatch id never escapes the background shell, so a review that actually passed is reported as failed. The control-file and status files sit in a world-readable `/tmp` path, so any local user can read a dispatch's progress or plant an abort file to kill it. And the async surface duplicates Claude Code's own `run_in_background` + `BashOutput` primitive, which the mission flow already uses for the same job.

Success means a synchronous-only supervisor that keeps every reliability feature, sheds the dead/broken/insecure machinery, fixes the `/adversarial` bug, closes the `/tmp` hole, and still builds and passes its full test suite.

## Approach

- Remove the async-dispatch subsystem: the `--async` flag, the `status`/`wait`/`result` subcommands, and the `~/.darkfactory/dispatches/<ulid>/` store (status.json, result.json, host.pid, events.jsonl), including its 7-day pruning. Pre-existing dispatch-store and `/tmp` dispatch directories from past runs are left untouched (they are ephemeral; no migration or one-time cleanup is performed).
- Remove the control-file abort channel (the `control.json` watchdog and its handling).
- Remove the now-dead automatic `/tmp` status.json writes and the watchdog interval that drove them, so the dispatcher no longer creates status or control files under its world-readable `/tmp` dispatch directory. (This concerns the dispatcher's own automatic writes, not user-specified `--log` paths, which are unchanged.)
- Remove dead test-only code (`graceful_stop`, `RunHandle::pid`, `StdinMode::Piped`, singular `SupervisorBuilder::arg`, `CancellationToken`, `heartbeat_ticker`, `StreamMode::Silent`/`Verbose`, `Emitter::emit_heartbeat`, the event-local heartbeat constant) and delete the tests that only exercise it. The heartbeat-named items here are the unused/superseded helpers; the live heartbeat path (`emit_heartbeat_with_interval`, driven by the inline heartbeat interval in the engine select loop) is NOT touched — see the preservation bullet below.
- Drop the `tracing` and `tracing-subscriber` dependencies, which are unused.
- Convert the `/adversarial` skill from `--async` + `wait <id>` to a synchronous `--json` dispatch run via `run_in_background` + `BashOutput`, mirroring the mission's Phase 9 reviewer dispatch.
- Preserve all load-bearing supervision unchanged: process-group isolation, two-phase SIGTERM/SIGKILL timeout with output draining, 15-second heartbeat emission (the live `emit_heartbeat_with_interval` path, independent of the removed helpers), parent-death detection (now always on), the structured JSON result on stdout, the `--log` flag, and the `--grace`/`--heartbeat` flags.
- Update `tools/darkfactory-dispatch/README.md` and its `CLAUDE.md` to drop async/lifecycle documentation while keeping the progress-monitoring (`tail -f`) guidance.

## Rejected alternatives

- **Keep async, just fix the `/adversarial` bug.** Rejected: the async lifecycle has a single consumer that is better served by Claude Code's own background primitive, and the store carries its own security and reliability problems. Patching the consumer would preserve dead weight and a `/tmp` attack surface.
- **Replace the binary with a shell script.** Rejected: the load-bearing robustness (process-group teardown, two-phase timeout while draining NDJSON, parent-death detection) is exactly what shell does unreliably. That fragility is why the binary exists.
- **Also fix the deferred core bugs (orphan-on-error / no `kill_on_drop`, unbounded stderr await) in this pass.** Rejected for scope: they live in the kept core and deserve their own focused change; bundling them widens the diff and the risk. (Removing async partially mitigates the Linux orphan case by leaving parent-death always on.)
- **Move the dispatch store to a secure user-private registry instead of deleting it.** Rejected for now: nothing consumes a cross-session by-id registry today. It can be added later, secured, if a real need appears.

## Capabilities

- `synchronous-dispatch` — supervised synchronous dispatch of a Codex worker: two-phase timeout, 15s heartbeat events, process-group teardown, parent-death cleanup, structured JSON result on stdout, and log streaming via `--log`.
- `dispatch-cli` — the command-line surface after the cut: the retained flags work; the removed `--async` flag and `status`/`wait`/`result` subcommands are rejected by the CLI parser with a non-zero exit and a usage error, producing no JSON result and no filesystem side effects; and a dispatch with no user-specified `--log` path under those locations writes no files under the old `/tmp` dispatch directory or `~/.darkfactory/dispatches/`.
- `adversarial-sync-review` — the `/adversarial` skill dispatches its reviewers synchronously and reports the correct pass/fail outcome (the bug fix).
