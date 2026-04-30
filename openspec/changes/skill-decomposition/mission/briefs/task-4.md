# Worker Brief: Task 4 - Fork security skill

## Your Task

Copy one file with frontmatter name change and a `user-invokable: true` addition:

Source: `/Users/stuart/Code/dotfiles/.claude/skills/security-review/SKILL.md`
Destination: `skills/security/SKILL.md`

**Changes:**
- Change `name: security-review` → `name: security`
- Add `user-invokable: true` to frontmatter (after `name:`)
- Strip any toyo-specific sections (references to toyo's review conventions or toyo-specific severity levels) — **pre-verified: the source file contains zero references to "toyo" (case-insensitive grep returned 0). No toyo sections to strip.**
- Leave all other content unchanged

**Acceptance criteria:**
- `skills/security/SKILL.md` exists as a regular file (not a symlink)
- `grep "^name:" skills/security/SKILL.md` returns `name: security`
- `grep -ic "toyo" skills/security/SKILL.md` returns `0`
- `grep -icE "injection|auth|secret|vulnerabilit|OWASP|XSS|SQL" skills/security/SKILL.md` returns at least `1`
- `grep "user-invokable: true" skills/security/SKILL.md` returns 1 match

## Project Context

Same as Tasks 2 and 3: forking an external skill into the repo with a new bare name. The design says to strip toyo-specific sections from security-review, but pre-verification confirms there are none — this is effectively a name-only fork.

## Architecture

`skills/security/SKILL.md` — single file, no companions.

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md, skills/mission/templates/, skills/mission/prompts/
- Created: skills/mission-resume/SKILL.md, skills/mission-status/SKILL.md

### Task 1a: Update mission skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — old names updated, /mission-init call added to Phase 0

### Task 2: Fork grill, prd, language (commits 66500d39 + 2ad413df)
- Created: skills/grill/SKILL.md, skills/prd/SKILL.md, skills/language/SKILL.md (all with user-invokable: true)

### Task 3: Fork tdd and test-quality (commit 8c12e873)
- Created: skills/tdd/SKILL.md + 5 companions, skills/test-quality/SKILL.md (both with user-invokable: true)
- Modified: skills/mission/prompts/scrutiny-validator.md — /test-quality-review→/test-quality

## Downstream Consumers

- Task C3 (module shape) verifies `skills/security/` exists.

## Procedures

- Do NOT use /tdd — this is a .md file copy
- Commit in a single commit with message: `[task-4] fork security skill into skills/`
- Write your handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-4.md`

## Handoff Report Format

- Task: Task 4 - Fork security skill
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
