# Worker Brief: Task 1 - Fix darkfactory-init skill

## Your Task
Edit `.claude/skills/darkfactory-init/SKILL.md` to fix two bugs:

1. **Safe Bash patterns for parallel calls.** Add an implementation note after Step 2's optional skill checks (around line 69) and after Step 4's CLI checks (around line 94) instructing the agent to use patterns that always exit 0:
   - For file checks: `test -f <path> && echo "found" || echo "missing"`
   - For CLI checks: `command --version 2>/dev/null || echo "NOT_INSTALLED"`
   
   The note should explain WHY: non-zero exit codes cancel sibling parallel Bash calls in Claude Code.

2. **Add --force flag to openspec init.** On line 81, change `openspec init --tools claude,codex` to `openspec init --tools claude,codex --force`. Add a brief explanation that `--force` ensures non-interactive cleanup of legacy files.

Do NOT rewrite the skill steps with exact bash commands. Keep the changes minimal -- add notes, don't restructure.

## Project Context
Darkfactory was built against OpenSpec v1.2.0. We upgraded to v1.3.1. The darkfactory-init skill has two bugs that block new project initialization: parallel Bash calls cancel each other when file checks exit non-zero, and `openspec init` now requires `--force` for non-interactive legacy file cleanup.

## Architecture
The darkfactory-init skill is a natural-language skill file (not code). It tells the Claude Code agent what to check and report, but does not prescribe exact bash commands. The fix preserves this flexibility by adding implementation notes rather than rewriting steps.

## Key Terms
- **Skill**: A reusable prompt template invoked with a slash command
- **Engine**: The configurable AI model/tool used for a given role

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
Task 4.1 (smoke test) will verify the darkfactory-init SKILL.md contains the safe Bash pattern note and --force flag.

## Procedures
- Read `.claude/skills/darkfactory-init/SKILL.md` first
- Make the two targeted edits
- Commit all changes in a single commit with message: "[task-1] Fix darkfactory-init parallel Bash cancellation and add --force to openspec init"
- Write your handoff report to openspec/changes/openspec-v1-3-1-alignment/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Fix darkfactory-init skill
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
