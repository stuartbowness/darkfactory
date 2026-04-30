# Worker Brief: Task 1a - Update mission SKILL.md skill references

## Your Task

In `skills/mission/SKILL.md`, replace four old skill invocation names with their new names. This is a targeted text substitution — no other content changes.

**Substitutions to make (all occurrences):**

| Old | New |
|-----|-----|
| `/grill-me` | `/grill` |
| `/write-a-prd` | `/prd` |
| `/ubiquitous-language` | `/language` |
| `/darkfactory-init` | `/mission-init` |

**Acceptance criteria (verified by grep after the change):**
- `grep -c "/grill-me" skills/mission/SKILL.md` → 0
- `grep -c "/write-a-prd" skills/mission/SKILL.md` → 0
- `grep -c "/ubiquitous-language" skills/mission/SKILL.md` → 0
- `grep -c "/darkfactory-init" skills/mission/SKILL.md` → 0
- `grep -c "/grill" skills/mission/SKILL.md` → ≥1 (the replacement reference)
- `grep -c "/mission-init" skills/mission/SKILL.md` → ≥1 (the replacement reference)
- No other content changes in `skills/mission/SKILL.md` — all phases, rules, and behavior must remain intact

**Important:** Do NOT modify any other file. Only `skills/mission/SKILL.md` changes in this task.

## Project Context

Darkfactory is consolidating its skills into a `skills/` directory at the repo root with a new `install.sh` that creates symlinks. As part of this, several externally-hosted skills are being forked into the repo with shorter bare names (`grill-me` → `grill`, `write-a-prd` → `prd`, `ubiquitous-language` → `language`, `darkfactory-init` → `mission-init`). The mission SKILL.md refers to these skills by their old names — this task updates those references so the mission orchestrator invokes the correct new skill names.

## Architecture

`skills/mission/SKILL.md` is a monolithic ~825-line file containing all 9 phases of the mission orchestrator. It calls sub-skills at several points (e.g., `/grill-me` to run the grilling session in Phase 2). After this task, those references will use the new bare names. The skill file's behavior is otherwise unchanged.

## Key Terms

- **Owned skill**: A skill whose source file lives in `skills/` within this repo.

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md (825-line monolith, all phases 1-9)
- Created: skills/mission/templates/ — contract.md, coverage-audit.md, plan.md, pr-body.md, worker-brief.md, worker-fix-brief.md
- Created: skills/mission/prompts/ — adversarial-reviewer.md, scrutiny-validator.md, spec-delta-checker.md, usertest-validator.md
- Created: skills/mission-resume/SKILL.md (frontmatter: name: mission-resume)
- Created: skills/mission-status/SKILL.md (frontmatter: name: mission-status)
- Removed from git: .claude/skills/mission/, .claude/skills/mission-resume/, .claude/skills/mission-status/

## Downstream Consumers

No later task reads `skills/mission/SKILL.md` content directly. The rename substitutions are internal references that will resolve correctly once the new skills are installed via `install.sh` (Task 8).

## Procedures

- Make only the 4 substitutions listed above. Use sed or direct file editing — do NOT use /tdd since there is no code to test, only text substitutions in a .md file.
- After making the substitutions, verify with the grep commands above.
- Commit in a single commit with message: `[task-1a] update mission SKILL.md skill references to new names`
- Write your handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-1a.md`

## Handoff Report Format

- Task: Task 1a - Update mission SKILL.md skill references
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
