# Worker Brief: Task 3 - Fork tdd and test-quality skills

## Your Task

This task has three parts:

### Part A: Fork tdd skill
Copy `skills/tdd/` from the source directory including SKILL.md and all 5 companion files. Update only the frontmatter `name` field.

Source directory: `/Users/stuart/Code/dotfiles/.claude/skills/layercode-skills/tdd/`

Files to copy:
- `SKILL.md` → `skills/tdd/SKILL.md` (change `name: tdd` — source already uses this name, verify)
- `deep-modules.md` → `skills/tdd/deep-modules.md` (unchanged)
- `interface-design.md` → `skills/tdd/interface-design.md` (unchanged)
- `mocking.md` → `skills/tdd/mocking.md` (unchanged)
- `refactoring.md` → `skills/tdd/refactoring.md` (unchanged)
- `tests.md` → `skills/tdd/tests.md` (unchanged)

Also add `user-invokable: true` to the SKILL.md frontmatter (after `name:`).

### Part B: Fork test-quality skill
Copy one file with frontmatter name change:

Source: `/Users/stuart/Code/dotfiles/.claude/skills/layercode-skills/test-quality-review/SKILL.md`
Destination: `skills/test-quality/SKILL.md`

- Change `name: test-quality-review` → `name: test-quality`
- Add `user-invokable: true` to frontmatter (after `name:`)
- Leave body content unchanged

### Part C: Update scrutiny-validator.md reference
In `skills/mission/prompts/scrutiny-validator.md`, replace `/test-quality-review` with `/test-quality`. There are currently 3 occurrences (lines 246, 248, 257). Replace all of them.

**Acceptance criteria:**
- `skills/tdd/SKILL.md` exists as a regular file, `grep "^name:" skills/tdd/SKILL.md` returns `name: tdd`
- All 5 companion files exist: `skills/tdd/deep-modules.md`, `skills/tdd/interface-design.md`, `skills/tdd/mocking.md`, `skills/tdd/refactoring.md`, `skills/tdd/tests.md`
- `skills/test-quality/SKILL.md` exists as a regular file, `grep "^name:" skills/test-quality/SKILL.md` returns `name: test-quality`
- `grep -c "/test-quality-review" skills/mission/prompts/scrutiny-validator.md` → 0
- `grep -c "/test-quality" skills/mission/prompts/scrutiny-validator.md` → ≥1
- `grep "user-invokable: true" skills/tdd/SKILL.md skills/test-quality/SKILL.md` → 2 matching lines

## Project Context

Darkfactory is consolidating all skills into `skills/` at the repo root. The tdd and test-quality-review skills are being forked from `~/Code/dotfiles` with updated names. The scrutiny-validator.md update ensures the validator invokes `/test-quality` (the new name) instead of `/test-quality-review`.

## Architecture

Same as Tasks 1 and 2: `skills/<name>/SKILL.md` directories at repo root. The tdd skill is special — it has 5 companion markdown files alongside SKILL.md that the skill reads during execution. All companion files move unchanged. The test-quality skill is a single SKILL.md.

The scrutiny-validator.md is at `skills/mission/prompts/scrutiny-validator.md` — it's a prompt template read by the mission orchestrator. Updating the reference here is the only functional change required.

## Key Terms

- **Owned skill**: A skill whose source file lives in `skills/` within this repo.
- **Companion files**: Supporting markdown files read by the tdd skill (deep-modules.md, interface-design.md, mocking.md, refactoring.md, tests.md).

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md, skills/mission/templates/, skills/mission/prompts/
- Created: skills/mission-resume/SKILL.md, skills/mission-status/SKILL.md

### Task 1a: Update mission skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — /grill-me→/grill, /write-a-prd→/prd, /ubiquitous-language→/language; /mission-init call added to Phase 0

### Task 2: Fork grill, prd, language (commits 66500d39 + 2ad413df)
- Created: skills/grill/SKILL.md (name: grill, user-invokable: true)
- Created: skills/prd/SKILL.md (name: prd, user-invokable: true)
- Created: skills/language/SKILL.md (name: language, user-invokable: true)

## Downstream Consumers

- Task C3 (module shape) verifies `skills/tdd/` has SKILL.md + 5 companions, and `skills/test-quality/` exists.
- The updated `scrutiny-validator.md` is used immediately by the scrutiny validator for all subsequent tasks in this mission.

## Procedures

- Do NOT use /tdd — these are .md file copies and a text substitution
- Commit all changes (6 tdd files + 1 test-quality file + 1 scrutiny-validator.md edit) in a single commit with message: `[task-3] fork tdd and test-quality skills, update scrutiny-validator ref`
- Write your handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-3.md`

## Handoff Report Format

- Task: Task 3 - Fork tdd and test-quality skills
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
