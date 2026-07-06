**CRITICAL**

None found.

**HIGH**

- `skills/adversarial/SKILL.md:15` and `skills/adversarial/SKILL.md:87`: the converted `/adversarial` flow still depends on `$DEFAULT_MODEL` being available when the background dispatch Bash call runs. The startup snippet defines it separately, but shell variables don’t survive across separate Bash tool calls. If the agent runs Step 2 in a fresh shell, `--model "$DEFAULT_MODEL"` becomes an empty model and the Codex reviewer can fail. Put the startup config inside the same background Bash script, or resolve the model before generating the dispatch command.

- `tools/darkfactory-dispatch/src/supervisor.rs:321`: Linux parent-death does not kill the supervised process group. The child-side `PR_SET_PDEATHSIG` at `supervisor.rs:131` kills the direct `codex` process when the dispatcher dies, but the Linux `watch_parent_death(_pgid)` ignores `pgid` and never sends SIGKILL to `-pgid`. That means shell commands spawned by Codex can survive orchestrator death, unlike the timeout path at `supervisor.rs:233` and `supervisor.rs:237`, which does signal the whole group. This weakens the “prevent orphans” guarantee in `tools/darkfactory-dispatch/README.md:175`.

**MEDIUM**

- `tools/darkfactory-dispatch/src/engine.rs:38` and `tools/darkfactory-dispatch/src/engine.rs:49`: the worker is spawned before the log file is opened. If `open_log` fails, dispatch returns an error after the child already exists, outside the normal timeout and result finalization path. On Linux the direct child may be killed by parent-death when the dispatcher exits, but on macOS the watcher only watches the dispatcher’s parent, so the worker can keep running. Open the log before spawning, or explicitly kill/wait the `RunHandle` on post-spawn setup failure.

**LOW**

None found.

I did not run tests because the workspace is read-only. I reviewed the mission patch, current Rust source, tests, README, Cargo metadata, and `skills/adversarial/SKILL.md`.