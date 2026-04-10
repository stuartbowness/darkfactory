# Worker Brief: Task 2 - Mission Resume Skill

## Your Task

Generate `darkfactoryV2/.claude/skills/mission-resume/SKILL.md` -- a skill that resumes an interrupted Darkfactory mission from file state. When a Claude Code session dies mid-mission (laptop sleep, terminal close, crash), all state is on disk. This skill finds it, reconstructs the orchestrator plan, and continues the build loop.

## Project Context

Darkfactory is a lightweight multi-agent mission system. Missions run as a Claude Code session in plan mode. All state is written to disk (tasks.md checkboxes, progress.md log, handoff reports, validation reports). If the session dies, a new session can resume by reading these files.

The mission directory lives at `openspec/changes/<change-name>/mission/` and contains: config.yaml, progress.md, briefs/, handoffs/, validations/, issues/.

## Architecture

The resume skill must:

1. **Find active missions:** Search for OpenSpec changes that have a `mission/` directory with unchecked tasks. Look for `openspec/changes/*/mission/config.yaml` and check the corresponding `tasks.md` for `- [ ]` items.

2. **Handle zero, one, or multiple results:**
   - No active missions: report "No active missions found" and stop
   - One active mission: use it, report "Resuming mission: [name]"
   - Multiple: list them with progress (N/M tasks complete), ask user to pick

3. **Read mission state:** Read tasks.md, mission/progress.md, mission/config.yaml, contract.md, proposal.md, and design.md from the active change directory.

4. **Check for orphaned commits:** For each unchecked task, check `git log --oneline --grep="\[task-N\]" -1`. If a commit exists but no handoff file, ask the human: count as done or re-run?

5. **Reconstruct the plan:** Enter Claude Code plan mode and write a complete orchestrator operating manual reflecting current state (which tasks done, which is next, all file paths, engine config, mission flow, rules).

6. **Log the resume:** Append "Mission resumed in new session. Picking up from task [N]." to progress.md.

7. **Continue the build loop:** Follow the same Phase 7 logic from the /mission skill (brief assembly, worker dispatch, validation, fix loop, progress tracking).

## Key Terms
- Mission, Orchestrator, Worker, Validator, Contract, Brief, Handoff, Monitor, Plan mode, Progress log, Mission directory, Circuit breaker, Fix loop

## Procedures
- Create the directory `darkfactoryV2/.claude/skills/mission-resume/` if it doesn't exist
- Write the complete SKILL.md with YAML frontmatter (name and description)
- Commit all changes in a single commit with message: "[task-2] Generate mission-resume skill for darkfactoryV2"
- Write your handoff report to openspec/changes/darkfactory-v2/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2 - Mission Resume Skill
- Commit SHA: [the full SHA]
- Approach: [brief explanation of what you did and why]
- Commands run: [any commands and their exit codes]
- Concerns: [anything you're uncertain about]
