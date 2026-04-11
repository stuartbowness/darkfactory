---
name: darkfactory-init
description: Set up Darkfactory in any project. Installs mission skills globally, checks for supporting skills, initializes OpenSpec, verifies CLI dependencies, and reports status. Run from any directory.
---

# Darkfactory Init

Set up the Darkfactory mission system so `/mission`, `/mission-resume`, and `/mission-status` work in this project.

**Run this from any directory.** It installs skills globally and sets up the current project.

---

## Step 1: Check and install core mission skills globally

Check if each core skill already exists in `~/.claude/skills/`:

- `~/.claude/skills/mission/SKILL.md`
- `~/.claude/skills/mission-resume/SKILL.md`
- `~/.claude/skills/mission-status/SKILL.md`
- `~/.claude/skills/darkfactory-init/SKILL.md`

**If all exist:** Report "Core skills already installed. Skipping." and proceed to Step 2.

**If any are missing:** Pull them from the darkfactory repo on GitHub:

```bash
TMPDIR=$(mktemp -d)
git clone https://github.com/layercodedev/darkfactory.git "$TMPDIR" --depth 1 2>/dev/null

[ ! -f ~/.claude/skills/mission/SKILL.md ] && cp -r "$TMPDIR/.claude/skills/mission" ~/.claude/skills/ && echo "Installed: mission"
[ ! -f ~/.claude/skills/mission-resume/SKILL.md ] && cp -r "$TMPDIR/.claude/skills/mission-resume" ~/.claude/skills/ && echo "Installed: mission-resume"
[ ! -f ~/.claude/skills/mission-status/SKILL.md ] && cp -r "$TMPDIR/.claude/skills/mission-status" ~/.claude/skills/ && echo "Installed: mission-status"
[ ! -f ~/.claude/skills/darkfactory-init/SKILL.md ] && cp -r "$TMPDIR/.claude/skills/darkfactory-init" ~/.claude/skills/ && echo "Installed: darkfactory-init"

rm -rf "$TMPDIR"
```

Report which skills were installed (new) and which were already present (skipped).

---

## Step 2: Check and install supporting skills

Check for required external skills in `~/.claude/skills/`:

**Required (used by /mission during spec and build phases):**
- `grill-me` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `ubiquitous-language` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `write-a-prd` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `prd-to-plan` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `tdd` -- from [layercodedev/skills](https://github.com/layercodedev/skills)

**Security:** These are installed from our own fork, pinned to a known-good commit. This prevents supply chain attacks from upstream changes.

**Only install skills that are missing.** Check each one individually:

```bash
TMPDIR=$(mktemp -d)
git clone https://github.com/layercodedev/skills.git "$TMPDIR" 2>/dev/null
cd "$TMPDIR" && git checkout 651eab033bdf8f7fd535c274f8cbe839075aba5e 2>/dev/null

[ ! -f ~/.claude/skills/grill-me/SKILL.md ] && cp -r "$TMPDIR/grill-me" ~/.claude/skills/ && echo "Installed: grill-me"
[ ! -f ~/.claude/skills/ubiquitous-language/SKILL.md ] && cp -r "$TMPDIR/ubiquitous-language" ~/.claude/skills/ && echo "Installed: ubiquitous-language"
[ ! -f ~/.claude/skills/write-a-prd/SKILL.md ] && cp -r "$TMPDIR/write-a-prd" ~/.claude/skills/ && echo "Installed: write-a-prd"
[ ! -f ~/.claude/skills/prd-to-plan/SKILL.md ] && cp -r "$TMPDIR/prd-to-plan" ~/.claude/skills/ && echo "Installed: prd-to-plan"
[ ! -f ~/.claude/skills/tdd/SKILL.md ] && cp -r "$TMPDIR/tdd" ~/.claude/skills/ && echo "Installed: tdd"

rm -rf "$TMPDIR"
```

Report which were installed (new) and which were already present (skipped).

**To update pinned skills:** Pull upstream changes into the layercodedev/skills fork, review them, then update the commit SHA in this skill file.

**Required for scrutiny validation (built-in or from your dotfiles):**
- `/review` -- code review (built-in to Claude Code)
- `/simplify` -- complexity check (built-in to Claude Code)
- `/security-review` -- security audit (conditional, from your dotfiles)

**Optional (project-specific, enhance validation but not required):**
- `/test-app` -- browser tests
- `/demo-video` -- frontend demo recording

Check if these exist in `~/.claude/skills/` or in the current project's `.claude/skills/`. Report which are found and which are missing. Don't try to install these -- they're project-specific.

---

## Step 3: Initialize OpenSpec in the current project

Check if `openspec/config.yaml` already exists in the current directory:

```bash
ls openspec/config.yaml 2>/dev/null
```

**If it already exists:** Report "OpenSpec already initialized. Skipping."

**If it doesn't exist:** Run `openspec init --tools claude,codex`. If `openspec` CLI is not available, report it as a missing dependency.

Note: After first-time `openspec init`, the user needs to restart their Claude Code session for the new `/opsx:` commands to register.

---

## Step 4: Verify CLI dependencies

Check each required CLI tool:

```bash
# Codex
codex --version 2>/dev/null

# GitHub CLI
gh --version 2>/dev/null
gh auth status 2>/dev/null

# OpenSpec
openspec --version 2>/dev/null
```

For each tool, report:
- Installed and authenticated (ready)
- Installed but not authenticated (needs: `codex login` or `gh auth login`)
- Not installed (needs: install command)

---

## Step 5: Report status

Print a summary:

```
Darkfactory Setup Report
========================

Core skills (installed to ~/.claude/skills/):
  [x] /mission
  [x] /mission-resume
  [x] /mission-status
  [x] /darkfactory-init

Supporting skills:
  [x] /grill-me
  [x] /ubiquitous-language
  [x] /write-a-prd
  [x] /prd-to-plan
  [x] /tdd
  [x] /review (built-in to Claude Code)
  [x] /simplify (built-in to Claude Code)
  [x] /security-review (conditional -- from your dotfiles)
  [ ] /test-app (optional -- project-specific)
  [ ] /demo-video (optional -- project-specific)

CLI tools:
  [x] codex 0.118.0 (authenticated)
  [x] gh 2.x.x (authenticated)
  [x] openspec 1.2.0

OpenSpec:
  [x] Initialized in current project

Ready to use:
  Run /mission in any project to start a mission.
  Restart Claude Code if this is the first install (new skills need a session restart).
```

If anything is missing, list the specific commands to fix it at the bottom.
