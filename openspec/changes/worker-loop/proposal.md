## Why

The `/mission` skill can produce a plan, contract, and mission directory, but it can't execute. There's no way for the orchestrator to dispatch work to a worker, wait for it to finish, and read the result. This change adds the build phase -- the core loop where the orchestrator iterates through tasks, dispatches workers, and tracks progress.

## What Changes

- The `/mission` skill gains build phase logic (Phases 7-10) that runs after plan approval
- The orchestrator assembles a brief file per task from the OpenSpec artifacts
- The orchestrator kicks off a Codex worker (or Claude, per config) using Monitor to watch for completion
- The worker implements the task, commits in a single commit, and writes a handoff report
- The orchestrator reads the handoff, records progress, and moves to the next task
- No validation in this change -- the orchestrator just runs workers sequentially and tracks completion

## Capabilities

### New Capabilities
- `worker-brief`: The orchestrator's ability to assemble a self-contained brief file from proposal, design, and task description, written to mission/briefs/task-N.md
- `worker-dispatch`: The orchestrator's ability to kick off a worker (Codex or Claude per config), wait for completion via Monitor, and read the handoff report
- `worker-handoff`: The structured report a worker writes after completing a task, including commit SHA, approach, commands run, and concerns
- `worker-progress`: The orchestrator's ability to mark tasks complete, update progress.md, and iterate to the next task

### Modified Capabilities

- `mission-start`: The /mission skill is extended with build phase logic after plan approval

## Impact

- Modified files: `.claude/skills/mission/SKILL.md` (add build phase)
- Dependencies: Codex CLI (`codex exec`), Claude Code Monitor tool, existing mission directory structure from orchestrator-skeleton change
- No new dependencies added
