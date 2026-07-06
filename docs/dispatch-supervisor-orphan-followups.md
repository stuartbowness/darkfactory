# darkfactory-dispatch: deferred supervisor orphan bugs

Two pre-existing orphan bugs in the `tools/darkfactory-dispatch/` supervisor core, surfaced by the Phase 9 adversarial review of the `dispatch-sync-only` change and **deliberately deferred** (the synchronous-only change was purely subtractive; these live in the kept supervision core and touch signal/process-group delivery, so they deserve their own focused, well-tested change). They are not regressions — the synchronous-only change did not introduce them, and it improved the Linux case by making parent-death always-on.

## 1. Parent-death kills only the direct child, not the process group (Linux)

Tracked in [#223](https://github.com/layercodedev/darkfactory/issues/223).


- Where: `src/supervisor.rs` — the child-side `PR_SET_PDEATHSIG` (arranges SIGKILL to the direct `codex` process on dispatcher death) vs. `watch_parent_death(_pgid)`, which ignores the `pgid` and never signals `-pgid`.
- Effect: when the orchestrator (dispatcher's parent) dies, `codex` is killed, but shell commands and subprocesses `codex` spawned in its group can survive as orphans. The timeout path already signals the whole group (`-pgid`); the parent-death path does not.
- Fix direction: on parent death, deliver SIGKILL to the whole process group (`-pgid`), matching the timeout path. Add a test that a grandchild in the group dies when the dispatcher's parent exits.

## 2. Worker spawned before the log file is opened (macOS orphan on error)

Tracked in [#224](https://github.com/layercodedev/darkfactory/issues/224).


- Where: `src/engine.rs` — the `SupervisorBuilder(...).spawn()` runs before `open_log(...)`. If `open_log` fails, dispatch returns an error after the child already exists, outside the normal timeout / result-finalization path.
- Effect: on Linux always-on parent-death may still reap the direct child, but on macOS (kqueue watches the dispatcher's parent) the worker can keep running. Same error-path-orphan class as the historical `findings.md` item 1.2 (no `kill_on_drop`).
- Fix direction: open the log before spawning, or explicitly kill/wait the `RunHandle` on any post-spawn setup failure.

Full reviewer detail: `openspec/changes/dispatch-sync-only/mission/validations/adversarial-codex.md`.
