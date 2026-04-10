## Context

The /mission skill writes all state to disk: tasks.md tracks completion, progress.md logs events, handoffs and validations capture per-task results, and config.yaml holds engine settings. This means a new Claude Code session can reconstruct the full picture by reading these files.

## Goals / Non-Goals

**Goals:**
- `/mission resume` finds the active mission, reconstructs the plan, and continues the build loop
- `/mission-status` shows progress without interrupting the running mission
- Both work from any Claude Code session (not just the one running the mission)

**Non-Goals:**
- Resuming mid-worker (if Codex was in the middle of a task, that task restarts)
- Parallel mission management (one mission at a time per project)

## Decisions

### Resume reconstructs the plan from files
The resume skill reads: tasks.md (which tasks are done), progress.md (what happened), config.yaml (engine settings), contract.md (for validation), and the OpenSpec artifacts. It enters plan mode and writes a fresh orchestrator operating manual with all the same content as the original, but with the current state reflected (completed tasks, current task number).

**Alternative considered:** Saving the plan to a file so it can be restored literally. Rejected because the plan references dynamic state (current task number, completed list) that changes every cycle. Reconstructing from files is more reliable than restoring a potentially stale snapshot.

### Resume starts from the next unchecked task
If a worker was in progress when the session died, that task's handoff file won't exist. The resume skill treats it as not started and dispatches a new worker. The previous worker's partial work (uncommitted changes) is ignored -- the new worker starts from the last committed state.

### Status is a separate skill
`/mission-status` is a read-only skill that doesn't modify any files. It can run in a separate Claude Code session while the mission runs in tmux. It reads tasks.md, progress.md, and validation reports, and presents a formatted summary.

### Status detects the active change
The status skill looks for OpenSpec changes that have a `mission/` directory. If there's exactly one, it uses that. If there are multiple, it asks the user which one.

## Risks / Trade-offs

- **[Partial worker commits may exist]** -> The new worker starts clean from the last committed state. If the previous worker committed but didn't write the handoff, the resume skill checks git log for commits matching the task pattern and asks the human if it should count that task as done.
- **[Multiple abandoned missions]** -> Status and resume both ask the user to pick one if multiple mission directories exist.
