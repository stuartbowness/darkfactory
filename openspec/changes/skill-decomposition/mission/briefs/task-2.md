# Worker Brief: Task 2 - Fork grill, language, and prd skills

## Your Task

Copy three skill files from their source location into the `skills/` directory at the repo root, updating only the frontmatter `name` field in each. Content is otherwise unchanged.

**Exact copies to make:**

| Source | Destination | New name field |
|--------|-------------|----------------|
| `/Users/stuart/Code/dotfiles/.claude/skills/layercode-skills/grill-me/SKILL.md` | `skills/grill/SKILL.md` | `name: grill` |
| `/Users/stuart/Code/dotfiles/.claude/skills/layercode-skills/write-a-prd/SKILL.md` | `skills/prd/SKILL.md` | `name: prd` |
| `/Users/stuart/Code/dotfiles/.claude/skills/layercode-skills/ubiquitous-language/SKILL.md` | `skills/language/SKILL.md` | `name: language` |

**Steps:**
1. Create `skills/grill/`, `skills/prd/`, `skills/language/` directories
2. Copy the source SKILL.md to each destination
3. In each file, change only the frontmatter `name:` field to the new name (e.g., `name: grill-me` → `name: grill`)
4. Leave all other frontmatter fields (`description`, `user-invokable`, etc.) and all body content unchanged

**Acceptance criteria:**
- `skills/grill/SKILL.md` exists as a regular file (not a symlink), `grep "^name:" skills/grill/SKILL.md` returns `name: grill`
- `skills/prd/SKILL.md` exists as a regular file, `grep "^name:" skills/prd/SKILL.md` returns `name: prd`
- `skills/language/SKILL.md` exists as a regular file, `grep "^name:" skills/language/SKILL.md` returns `name: language`
- `grep "user-invokable: true" skills/grill/SKILL.md skills/prd/SKILL.md skills/language/SKILL.md` returns three matching lines
- Body content of each file matches its source (only the frontmatter name field changed)

## Project Context

Darkfactory is consolidating all skills into `skills/` at the repo root. Six external skills are being forked into the repo so it becomes self-contained — no dependency on a second external repo. This task forks three of them. "Fork" means: copy the file, update the frontmatter name to the new bare name, leave everything else unchanged.

## Architecture

Each skill is a self-contained directory under `skills/<name>/` containing a `SKILL.md` with YAML frontmatter. The frontmatter `name` field is what Claude Code uses to identify the skill when invoked as `/<name>`. After this task and `install.sh` (Task 8), running `install.sh` will symlink `~/.claude/skills/grill` → `~/.darkfactory/skills/grill`, making the skill available as `/grill`.

## Key Terms

- **Owned skill**: A skill whose source file lives in `skills/` within this repo.
- **Fork**: Copy a skill from an external source, update the frontmatter name field, preserve all content.

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md (825-line monolith, all phases 1-9)
- Created: skills/mission/templates/ — contract.md, coverage-audit.md, plan.md, pr-body.md, worker-brief.md, worker-fix-brief.md
- Created: skills/mission/prompts/ — adversarial-reviewer.md, scrutiny-validator.md, spec-delta-checker.md, usertest-validator.md
- Created: skills/mission-resume/SKILL.md (frontmatter: name: mission-resume)
- Created: skills/mission-status/SKILL.md (frontmatter: name: mission-status)
- Removed from git: .claude/skills/mission/, .claude/skills/mission-resume/, .claude/skills/mission-status/

### Task 1a: Update mission skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — /grill-me→/grill, /write-a-prd→/prd, /ubiquitous-language→/language substitutions; /mission-init call added to Phase 0

## Downstream Consumers

- Task 8 (install.sh) will loop over `skills/*/` and symlink each skill directory. These three directories need to exist with correct names.
- Task C2 (cleanup) checks that no old skill names (`grill-me`, `write-a-prd`, `ubiquitous-language`) are referenced in `skills/`.
- Task C3 (module shape) verifies 12 skill directories exist, including `grill`, `prd`, `language`.

## Procedures

- Do NOT use /tdd — these are .md file copies with a single field change, no testable code
- Commit all three files in a single commit with message: `[task-2] fork grill, prd, language skills into skills/`
- Write your handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-2.md`

## Handoff Report Format

- Task: Task 2 - Fork grill, language, and prd skills
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
