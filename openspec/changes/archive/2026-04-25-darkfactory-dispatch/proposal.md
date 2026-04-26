## Why

Darkfactory dispatches Codex workers as raw `codex exec` subprocesses with no timeout, no heartbeat, and no process lifecycle management. When Codex stalls (stdin EOF hang, long reasoning silence, subprocess wedge), the orchestrator waits forever. The human must manually detect and kill hung processes. We already applied a `< /dev/null` quickfix for the stdin stall, but the broader problem -- no supervision of Codex subprocesses -- remains.

## What Changes

- Build `darkfactory-dispatch`, a Rust CLI binary that supervises Codex subprocesses with process group isolation, two-phase timeout (soft warning then hard kill), NDJSON heartbeat events, and structured JSON results.
- Update the mission skill to use `darkfactory-dispatch` instead of raw `codex exec` calls for both worker dispatch and adversarial reviewer dispatch.
- Update darkfactory-init to check for `darkfactory-dispatch` as a required dependency.
- Codex-only -- Claude subagents continue using Claude Code's built-in Agent tool, which already provides its own lifecycle management.

## Capabilities

### New Capabilities

- `process-supervision`: Spawn Codex as a subprocess in its own process group with two-phase timeout, heartbeat emission, parent-death detection, and structured result reporting.
- `async-dispatch`: Dispatch a Codex subprocess asynchronously, returning a dispatch ID immediately, with status/result/wait subcommands for lifecycle management.
- `dispatch-cli`: A single Rust binary (`darkfactory-dispatch`) with flags for model, effort, timeout, grace period, sandbox, config pass-through, prompt/prompt-file, output path, working directory, and log path.

### Modified Capabilities

- `mission-worker-dispatch`: The mission skill's Codex worker dispatch (SKILL.md ~line 486) and adversarial reviewer dispatch (~line 745) switch from raw `codex exec` bash blocks to `darkfactory-dispatch` calls.
- `init-checks`: darkfactory-init adds `darkfactory-dispatch` as a required CLI dependency check.

## Impact

- New directory: `tools/darkfactory-dispatch/` (Rust crate, Makefile, tests)
- Modified: `.claude/skills/mission/SKILL.md` (two bash blocks replaced)
- Modified: `.claude/skills/darkfactory-init/SKILL.md` (add dependency check)
- New binary: `darkfactory-dispatch` installed to `~/.local/bin/`
- New runtime directory: `~/.darkfactory/dispatches/` (async dispatch store)
- Reference implementation: `~/Code/agent-mux` (buildoak/agent-mux, 200+ commits)
