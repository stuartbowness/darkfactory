# Worker Brief: Task 3 - Mission Status Skill

## Your Task

Generate `darkfactoryV2/.claude/skills/mission-status/SKILL.md` -- a read-only skill that shows the progress of an active Darkfactory mission. This lets a user check on a running mission from a separate Claude Code session without interrupting it.

## Project Context

Darkfactory missions run in a Claude Code session (often in tmux). All state is on disk. A user may want to check progress from another terminal without touching the running session. This skill reads the files and presents a formatted summary.

The mission directory lives at `openspec/changes/<change-name>/mission/` and contains: config.yaml, progress.md, briefs/, handoffs/, validations/, issues/.

## Architecture

The status skill must:

1. **Find active missions:** Search for OpenSpec changes with `mission/` directories. Look for `openspec/changes/*/mission/config.yaml`.

2. **Handle zero, one, or multiple results:**
   - No missions: report "No active missions found" and stop
   - One: use it
   - Multiple: list them and ask user which one

3. **Read state (read-only -- do NOT modify any files):**
   - tasks.md -- count checked vs unchecked tasks
   - mission/progress.md -- recent activity log
   - mission/config.yaml -- engine settings

4. **Gather validation results per completed task:**
   - Check for mission/validations/task-N-scrutiny.md -- read Overall line
   - Check for mission/validations/task-N-usertest.md -- read Overall line
   - Check for mission/validations/task-N-demo.mp4 -- check if exists
   - Check for mission/issues/task-N-*.md -- count fix attempts

5. **Display formatted progress:**
   ```
   Mission: [change name]
   Engine: worker=[engine], validators=[engine]
   Started: [timestamp from first progress.md entry]

   Progress: [completed]/[total] tasks
   [=========>          ] 45%

   Tasks:
     [x] Task 1: [name] - Scrutiny: PASS, User-test: PASS, Demo: yes
     [x] Task 2: [name] - Scrutiny: PASS, User-test: PASS (2 attempts)
     [ ] Task 3: [name] - (next)
     [ ] Task 4: [name]

   Recent activity:
     [last 5 entries from progress.md]
   ```

6. **Detect in-progress tasks:** If a task is unchecked but has a brief file and no handoff, note "IN PROGRESS (worker dispatched, awaiting handoff)".

## Key Terms
- Mission, Progress log, Mission directory, Mission config, Validator, Worker

## Procedures
- Create the directory `darkfactoryV2/.claude/skills/mission-status/` if it doesn't exist
- Write the complete SKILL.md with YAML frontmatter (name and description)
- The skill must be explicitly read-only -- it must NOT create, modify, or delete any files
- Commit all changes in a single commit with message: "[task-3] Generate mission-status skill for darkfactoryV2"
- Write your handoff report to openspec/changes/darkfactory-v2/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Mission Status Skill
- Commit SHA: [the full SHA]
- Approach: [brief explanation of what you did and why]
- Commands run: [any commands and their exit codes]
- Concerns: [anything you're uncertain about]
