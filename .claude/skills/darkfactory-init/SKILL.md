---
name: darkfactory-init
description: Set up Darkfactory in any project. Checks for required skills, tells you how to install missing ones, initializes OpenSpec, and verifies CLI dependencies. Run from any directory.
---

# Darkfactory Init

Check if this project is ready for `/mission`. Report what's installed and what's missing.

---

## Step 1: Check core skills

Check if each core skill exists in `~/.claude/skills/`:

- `~/.claude/skills/mission/SKILL.md`
- `~/.claude/skills/mission-resume/SKILL.md`
- `~/.claude/skills/mission-status/SKILL.md`
- `~/.claude/skills/darkfactory-init/SKILL.md`

If any are missing, report:

```
Missing core skills. Install them:

  git clone https://github.com/layercodedev/darkfactory.git ~/.darkfactory
  ln -sf ~/.darkfactory/.claude/skills/mission ~/.claude/skills/mission
  ln -sf ~/.darkfactory/.claude/skills/mission-resume ~/.claude/skills/mission-resume
  ln -sf ~/.darkfactory/.claude/skills/mission-status ~/.claude/skills/mission-status
  ln -sf ~/.darkfactory/.claude/skills/darkfactory-init ~/.claude/skills/darkfactory-init

Then restart Claude Code and re-run /darkfactory-init.
```

If all exist, report "Core skills: OK" and continue.

---

## Step 2: Check supporting skills

Check if each required supporting skill exists in `~/.claude/skills/`:

- `~/.claude/skills/grill-me/SKILL.md`
- `~/.claude/skills/ubiquitous-language/SKILL.md`
- `~/.claude/skills/write-a-prd/SKILL.md`
- `~/.claude/skills/tdd/SKILL.md`

If any are missing, report only the missing ones:

```
Missing supporting skills. Install them:

  git clone https://github.com/layercodedev/skills.git ~/.darkfactory-skills
  ln -sf ~/.darkfactory-skills/grill-me ~/.claude/skills/grill-me
  ln -sf ~/.darkfactory-skills/ubiquitous-language ~/.claude/skills/ubiquitous-language
  ln -sf ~/.darkfactory-skills/write-a-prd ~/.claude/skills/write-a-prd
  ln -sf ~/.darkfactory-skills/tdd ~/.claude/skills/tdd

Then restart Claude Code and re-run /darkfactory-init.
```

If all exist, report "Supporting skills: OK" and continue.

Also check optional/built-in skills and report their status:
- `/review` -- built-in to Claude Code (always available)
- `/simplify` -- built-in to Claude Code (always available)
- `/security-review` -- check ~/.claude/skills/security-review/SKILL.md
- `/test-app` -- check ~/.claude/skills/ or .claude/skills/ in the current project
- `/demo-video` -- check ~/.claude/skills/ or .claude/skills/ in the current project

Implementation note: when running file checks in parallel Bash calls, use a pattern that always exits 0, such as `test -f <path> && echo "found" || echo "missing"`. In Claude Code, a non-zero exit code cancels sibling parallel Bash calls.

---

## Step 3: Initialize OpenSpec

Check if `openspec/config.yaml` exists in the current directory.

**If it exists:** Report "OpenSpec: already initialized."

**If it doesn't exist:** Run `openspec init --tools claude,codex --force`. The `--force` flag ensures non-interactive cleanup of legacy files. If `openspec` CLI is not available, report: "OpenSpec CLI not installed. Run: `npm install -g openspec`"

Note: After first-time `openspec init`, the user needs to restart Claude Code for `/opsx:` commands to register.

---

## Step 4: Verify CLI dependencies

Check each required CLI tool:

```bash
codex --version 2>/dev/null || echo "NOT_INSTALLED"
darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"
gh --version 2>/dev/null || echo "NOT_INSTALLED"
gh auth status 2>/dev/null || echo "NOT_INSTALLED"
openspec --version 2>/dev/null || echo "NOT_INSTALLED"
jq --version 2>/dev/null || echo "NOT_INSTALLED"
```

Implementation note: when running CLI checks in parallel Bash calls, use a pattern that always exits 0, such as `command --version 2>/dev/null || echo "NOT_INSTALLED"`. In Claude Code, a non-zero exit code cancels sibling parallel Bash calls.

For each tool, report:
- Installed and authenticated (ready)
- Installed but not authenticated (needs: `codex login` or `gh auth login`)
- Not installed (needs: install command)

Install commands if missing:
- codex: `npm install -g @openai/codex`
- darkfactory-dispatch: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
- gh: `brew install gh`
- openspec: `npm install -g @fission-ai/openspec`
- jq: `brew install jq`

---

## Step 5: Report status

```
Darkfactory Setup Report
========================

Core skills:
  [x] /mission
  [x] /mission-resume
  [x] /mission-status
  [x] /darkfactory-init

Supporting skills:
  [x] /grill-me
  [x] /ubiquitous-language
  [x] /write-a-prd
  [x] /tdd
  [x] /review (built-in)
  [x] /simplify (built-in)
  [x] /security-review (conditional)
  [ ] /test-app (optional)
  [ ] /demo-video (optional)

CLI tools:
  [x] codex (authenticated)
  [x] darkfactory-dispatch
  [x] gh (authenticated)
  [x] openspec
  [x] jq

OpenSpec:
  [x] Initialized

Ready: run /mission to start.
```

If anything required is missing, list the install commands at the bottom.
