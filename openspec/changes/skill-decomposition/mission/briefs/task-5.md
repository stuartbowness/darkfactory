# Worker Brief: Task 5 - Write mission-init skill

## Your Task

Write `skills/mission-init/SKILL.md` from scratch. This replaces the old `darkfactory-init` skill.

The key change from the old skill: instead of checking two possible install locations and suggesting `ln -sf` commands, the new skill verifies that each of the 12 owned skills has a symlink at `~/.claude/skills/<name>` pointing to `~/.darkfactory/skills/<name>`. On missing or broken symlinks it suggests running `~/.darkfactory/install.sh`.

**File to create:** `skills/mission-init/SKILL.md`

## Required frontmatter

```yaml
---
name: mission-init
user-invokable: true
description: Verify that all Darkfactory skills are installed and required CLI tools are available. Run from any project before starting a mission.
---
```

## Required behavior (all steps must be in the skill)

### Step 1: Check skill symlinks

The 12 owned skills are: `mission`, `mission-resume`, `mission-status`, `mission-init`, `grill`, `prd`, `language`, `tdd`, `test-quality`, `security`, `adversarial`, `dispatch`

For each skill name, verify that:
1. `~/.claude/skills/<name>` exists as a symlink (not a regular file)
2. The symlink resolves to `~/.darkfactory/skills/<name>` (use `readlink`)

Report status for each:
- `[x] /mission` — symlink OK
- `[ ] /adversarial` — missing (symlink does not exist)
- `[!] /dispatch` — broken (symlink exists but target missing)

If any are missing or broken, instruct the user to run:
```
~/.darkfactory/install.sh
```
Then restart Claude Code and re-run `/mission-init`.

If all 12 are present and resolve correctly, report "All 12 skills: OK" and continue.

### Step 2: Initialize OpenSpec

Check if `openspec/config.yaml` exists in the current directory.

**If it exists:** Report "OpenSpec: already initialized."

**If it doesn't exist:** Run `openspec init --tools claude,codex --force`. The `--force` flag ensures non-interactive cleanup of legacy files. If the `openspec` CLI is not available, report: "OpenSpec CLI not installed. Run: `npm install -g @fission-ai/openspec`"

After first-time `openspec init`, the user needs to restart Claude Code for `/opsx:` commands to register.

### Step 3: Verify CLI dependencies

Check each required CLI tool using patterns that always exit 0 (to avoid cancelling parallel Bash calls in Claude Code):

```bash
codex --version 2>/dev/null || echo "NOT_INSTALLED"
darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"
gh --version 2>/dev/null || echo "NOT_INSTALLED"
gh auth status 2>/dev/null || echo "NOT_INSTALLED"
openspec --version 2>/dev/null || echo "NOT_INSTALLED"
jq --version 2>/dev/null || echo "NOT_INSTALLED"
```

For each tool, report installed/not-installed/not-authenticated. Install commands if missing:
- codex: `npm install -g @openai/codex`
- darkfactory-dispatch: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
- gh: `brew install gh`
- openspec: `npm install -g @fission-ai/openspec`
- jq: `brew install jq`

### Step 4: Coverage tool detection

Detect the project language from build files at the project root:
- `Cargo.toml` → Rust
- `package.json` → TypeScript/Node
- `pyproject.toml` or `requirements.txt` → Python

Check the corresponding coverage tool:
- Rust: `cargo tarpaulin --version 2>/dev/null || echo "NOT_INSTALLED"`. If missing: `cargo install cargo-tarpaulin`
- TypeScript/Node: Check `package.json` for c8, jest, istanbul in dependencies/devDependencies. Suggest `npm install -D c8` if missing.
- Python: `pip show pytest-cov 2>/dev/null || echo "NOT_INSTALLED"`. If missing: `pip install pytest-cov`

If no recognized build file exists: "No recognized language detected — coverage tool check skipped."

### Step 5: Report status

Print a structured report:

```
Darkfactory Setup Report
========================

Skills (via ~/.claude/skills/ symlinks):
  [x] /mission
  [x] /mission-resume
  [x] /mission-status
  [x] /mission-init
  [x] /grill
  [x] /prd
  [x] /language
  [x] /tdd
  [x] /test-quality
  [x] /security
  [x] /adversarial
  [x] /dispatch

Optional project skills:
  [ ] /test-app (optional — project-specific)
  [ ] /demo-video (optional — project-specific)

CLI tools:
  [x] codex (authenticated)
  [x] darkfactory-dispatch
  [x] gh (authenticated)
  [x] openspec
  [x] jq

Coverage:
  [x] cargo-tarpaulin (Rust detected)

OpenSpec:
  [x] Initialized

Ready: run /mission to start.
```

If any required skills or tools are missing, list the fix commands at the bottom.

## Acceptance criteria

- `skills/mission-init/SKILL.md` exists and `grep "^name:" skills/mission-init/SKILL.md` returns `name: mission-init`
- `grep -cE "grill-me|darkfactory-init|test-quality-review" skills/mission-init/SKILL.md` returns `0` (old names not referenced)
- `grep -c "install.sh" skills/mission-init/SKILL.md` returns at least `1` (suggests install.sh as fix)
- `grep -cE "darkfactory-dispatch|codex|openspec|jq" skills/mission-init/SKILL.md` returns at least `1` (CLI tool checks present)
- The skill verifies 12 symlinks and runs `openspec init --tools claude,codex --force` when config absent

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md, skills/mission/templates/, skills/mission/prompts/
- Created: skills/mission-resume/SKILL.md, skills/mission-status/SKILL.md

### Task 1a: Update mission skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — old names updated, /mission-init call added to Phase 0

### Task 2: Fork grill, prd, language (commits 66500d39 + 2ad413df)
- Created: skills/grill/SKILL.md, skills/prd/SKILL.md, skills/language/SKILL.md

### Task 3: Fork tdd and test-quality (commit 8c12e873)
- Created: skills/tdd/SKILL.md + 5 companions, skills/test-quality/SKILL.md

### Task 4: Fork security (commit c06149050)
- Created: skills/security/SKILL.md

## Procedures

- Write the full SKILL.md content following the behavior spec above
- Commit in a single commit with message: `[task-5] write mission-init skill`
- Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-5.md`

## Handoff Report Format

- Task: Task 5 - Write mission-init skill
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and exit codes>
- Concerns: <anything uncertain>
