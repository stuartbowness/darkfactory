---
name: darkfactory-init
description: Set up Darkfactory in any project. Installs mission skills globally, checks for supporting skills, initializes OpenSpec, verifies CLI dependencies, and reports status.
---

# Darkfactory Init

Set up the Darkfactory mission system so `/mission`, `/mission-resume`, and `/mission-status` work in this project.

**Run this from the darkfactory repo directory.** It installs skills globally and sets up the current project.

---

## Step 1: Locate the darkfactory repo

Check that the current directory contains the Darkfactory skills:

```bash
ls .claude/skills/mission/SKILL.md .claude/skills/mission-resume/SKILL.md .claude/skills/mission-status/SKILL.md 2>/dev/null
```

If these files don't exist, report: "This doesn't look like the darkfactory repo. Run this skill from the darkfactory directory." and stop.

---

## Step 2: Check and install mission skills globally

Check if each core skill already exists in `~/.claude/skills/`:

- `~/.claude/skills/mission/SKILL.md`
- `~/.claude/skills/mission-resume/SKILL.md`
- `~/.claude/skills/mission-status/SKILL.md`
- `~/.claude/skills/darkfactory-init/SKILL.md`

**Only install skills that are missing.** For each missing skill, copy it from the darkfactory repo:

```bash
# Example: only if missing
[ ! -f ~/.claude/skills/mission/SKILL.md ] && cp -r .claude/skills/mission ~/.claude/skills/
```

Report which skills were installed (new) and which were already present (skipped).

---

## Step 3: Check and install supporting skills

Check for required external skills in `~/.claude/skills/`:

**Required (used by /mission during spec and build phases):**
- `grill-me` -- from [layercodedev/skills](https://github.com/layercodedev/skills) (fork of mattpocock/skills)
- `ubiquitous-language` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `write-a-prd` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `prd-to-plan` -- from [layercodedev/skills](https://github.com/layercodedev/skills)
- `tdd` -- from [layercodedev/skills](https://github.com/layercodedev/skills)

**Security:** These are installed from our own fork, pinned to a known-good commit. This prevents supply chain attacks from upstream changes.

**Only install skills that are missing.** Check `~/.claude/skills/<skill-name>/SKILL.md` first. Skip any that already exist.

For each missing skill:

```bash
# Clone our fork and fetch the pinned commit
TMPDIR=$(mktemp -d)
git clone https://github.com/layercodedev/skills.git "$TMPDIR" 2>/dev/null
cd "$TMPDIR" && git checkout 651eab033bdf8f7fd535c274f8cbe839075aba5e 2>/dev/null
cp -r "$TMPDIR/<skill-name>" ~/.claude/skills/
rm -rf "$TMPDIR"
```

If git is not available, report the skill as missing with a manual install command:
```
Missing: grill-me -- install manually:
  git clone https://github.com/layercodedev/skills /tmp/layercode-skills
  cd /tmp/layercode-skills && git checkout 651eab033bdf8f7fd535c274f8cbe839075aba5e
  cp -r /tmp/layercode-skills/grill-me ~/.claude/skills/
```

**To update pinned skills:** Pull upstream changes into the layercodedev/skills fork, review them, then update the commit SHA in this skill file.

**Optional (project-specific, enhance validation but not required):**
- `/review` -- code review
- `/simplify` -- complexity check
- `/security-review` -- security audit
- `/test-app` -- browser tests
- `/demo-video` -- frontend demo recording

Check if these exist in `~/.claude/skills/` or in the target project's `.claude/skills/`. Report which are found and which are missing. Don't try to install these -- they're project-specific.

---

## Step 4: Initialize OpenSpec in the target project

**Important:** This step should run in the target project, not in the darkfactory repo. Ask the user which project to initialize if unclear. If the user provides a path as an argument to `/darkfactory-init`, use that.

Check if `openspec/config.yaml` already exists in the target project:

```bash
ls <target-project>/openspec/config.yaml 2>/dev/null
```

**If it already exists:** Report "OpenSpec already initialized in <project>. Skipping."

**If it doesn't exist:** Run `cd <target-project> && openspec init --tools claude,codex`. If `openspec` CLI is not available, report it as a missing dependency.

**If no target project specified and current directory is the darkfactory repo:** Report "OpenSpec init skipped -- run `openspec init --tools claude,codex` in your target project."

Note: After first-time `openspec init`, the user needs to restart their Claude Code session for the new `/opsx:` commands to register.

---

## Step 5: Verify CLI dependencies

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

## Step 6: Report status

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
  [ ] /review (optional -- project-specific)
  [ ] /simplify (built-in to Claude Code)
  ...

CLI tools:
  [x] codex 0.118.0 (authenticated)
  [x] gh 2.x.x (authenticated)
  [x] openspec 1.2.0

OpenSpec:
  [x] Initialized in <project> (or [ ] Not initialized -- run in target project)

Ready to use:
  Run /mission in any project to start a mission.
  Restart Claude Code if this is the first install (new skills need a session restart).
```

If anything is missing, list the specific commands to fix it at the bottom.
