# Worker Brief: Task 1 - Move mission, mission-resume, mission-status

## Your Task

Move the three mission-related skills from `.claude/skills/` to a new `skills/` directory at the repo root. Do NOT change any skill content.

**Exact moves:**

1. Move `.claude/skills/mission/SKILL.md` → `skills/mission/SKILL.md`
2. Move `.claude/skills/mission/templates/` (all files) → `skills/mission/templates/`
3. Move `.claude/skills/mission/prompts/` (all files) → `skills/mission/prompts/`
4. Move `.claude/skills/mission-resume/SKILL.md` → `skills/mission-resume/SKILL.md`
5. Move `.claude/skills/mission-status/SKILL.md` → `skills/mission-status/SKILL.md`

The `skills/` directory does not exist yet — create it. Use `git mv` so git tracks the renames. After the moves, `.claude/skills/mission/`, `.claude/skills/mission-resume/`, and `.claude/skills/mission-status/` should be gone from git tracking.

**Do not change any file content.** This is a pure filesystem move.

**Required file structure after this task:**

```
skills/
  mission/
    SKILL.md
    templates/
      contract.md
      coverage-audit.md
      plan.md
      pr-body.md
      worker-brief.md
      worker-fix-brief.md
    prompts/
      adversarial-reviewer.md
      scrutiny-validator.md
      spec-delta-checker.md
      usertest-validator.md
  mission-resume/
    SKILL.md
  mission-status/
    SKILL.md
```

**Acceptance criteria:**
- `skills/mission/SKILL.md` is a regular file (not a symlink) with all content intact
- `skills/mission/templates/` contains all 6 template files listed above
- `skills/mission/prompts/` contains all 4 prompt files listed above
- `skills/mission-resume/SKILL.md` exists; its frontmatter has `name: mission-resume`
- `skills/mission-status/SKILL.md` exists; its frontmatter has `name: mission-status`
- `git ls-files .claude/skills/mission .claude/skills/mission-resume .claude/skills/mission-status` returns no output (removed from git tracking)
- `skills/mission/SKILL.md` contains all phases 1-9 (the file is a ~800+ line monolith)

## Project Context

Darkfactory's skills are currently split between `.claude/skills/` (inside the repo, picked up automatically by Claude Code) and an external `~/Code/dotfiles` repo. The goal of this mission is to consolidate all skills into a `skills/` directory at the repo root, then use `install.sh` to symlink them to `~/.claude/skills/`. This task is the first step: moving the 3 mission-related skills that are already owned by this repo.

## Architecture

After this refactor, `skills/` at the repo root becomes the single source of truth for all owned skills. The `.claude/skills/` directory inside the repo will eventually be emptied and removed. Each skill is a self-contained directory with a `SKILL.md` file. The `mission` skill is intentionally monolithic — one SKILL.md with all phases, plus companion `templates/` and `prompts/` subdirectories.

## Key Terms

- **Owned skill**: A skill whose source file lives in `skills/` within this repo (as opposed to a symlink to an external repo).
- **Skill directory**: A directory under `skills/<name>/` containing at minimum a `SKILL.md` file with valid YAML frontmatter (`name:`, `description:`).

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

- Task 1a reads `skills/mission/SKILL.md` to make 4 text substitutions.
- Task 3 reads `skills/mission/prompts/scrutiny-validator.md` to update one reference.
- All later tasks that verify `skills/` structure depend on this task having created the directory.

## Procedures

- Use `git mv` to move files (preserves git history)
- Commit all changes in a single commit with message: `[task-1] move mission skills to skills/ at repo root`
- Write your handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-1.md`

## Handoff Report Format

- Task: Task 1 - Move mission, mission-resume, mission-status
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
