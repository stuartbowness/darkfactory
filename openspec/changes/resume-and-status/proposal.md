## Why

If the Claude Code session dies mid-mission (laptop sleeps, terminal closes, crash), all mission state is on disk but there's no way to pick it back up. And while a mission is running in tmux, there's no way to check progress from another session without reading raw files. This change adds `/mission resume` and `/mission-status`.

## What Changes

- `/mission resume` detects an existing mission directory with incomplete tasks, reconstructs the orchestrator plan, and picks up where the mission left off
- `/mission-status` reads the mission directory and presents a formatted progress summary
- Both are lightweight skills that read existing file state

## Capabilities

### New Capabilities
- `mission-resume`: The ability to resume an interrupted mission from file state in a new Claude Code session
- `mission-status`: A read-only skill that summarizes mission progress from the files on disk

### Modified Capabilities

(none -- these are new skills, they don't modify /mission)

## Impact

- New files: `.claude/skills/mission-resume/SKILL.md`, `.claude/skills/mission-status/SKILL.md`
- Dependencies: existing mission directory structure, OpenSpec artifacts on disk
