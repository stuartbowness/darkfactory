# Worker Brief: Task 3 - Update README skill table

## Your Task
Edit `README.md` line 197. Change the `/opsx:archive` row's source column from "Installed by `openspec init`" to "Bundled with Darkfactory".

The current line looks like:
```
| `/opsx:archive` | Installed by `openspec init` | Post-merge: moves the change's planning files into the OpenSpec archive directory |
```

Change it to:
```
| `/opsx:archive` | Bundled with Darkfactory | Post-merge: moves the change's planning files into the OpenSpec archive directory |
```

Do not change any other lines in the file.

## Project Context
The `/opsx:archive` command is defined in `.claude/commands/opsx/archive.md`, which ships with Darkfactory. It is not installed by `openspec init`. The README should accurately reflect this.

## Architecture
README.md has a skill table showing where each tool/command comes from. Other rows use short source descriptions like "Your global Claude skills", "Your project's Claude skills". "Bundled with Darkfactory" matches this style.

## Key Terms
- **Change**: An OpenSpec unit of work with its own proposal, design, tasks, and mission directory

## What Previous Tasks Built
### Task 1: Fix darkfactory-init skill
- Modified: `.claude/skills/darkfactory-init/SKILL.md`
- Added safe Bash pattern notes and --force flag

### Task 2: Regenerate openspec skills
- Modified: 8 files in `.claude/skills/openspec-*/` and `.codex/skills/openspec-*/`
- All now have `generatedBy: "1.3.1"`

## Downstream Consumers
Task 4.2 (cleanup) will grep for any remaining "Installed by `openspec init`" references.

## Procedures
- Edit `README.md` line 197 only
- Commit with message: "[task-3] Update README /opsx:archive source attribution"
- Write your handoff report to openspec/changes/openspec-v1-3-1-alignment/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Update README skill table
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
