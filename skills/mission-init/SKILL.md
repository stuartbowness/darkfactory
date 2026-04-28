---
name: mission-init
user-invokable: true
description: Verify that all Darkfactory skills are installed and required CLI tools are available. Run from any project before starting a mission.
---

# Mission Init

Verify the current project is ready to run `/mission`. Check skill symlinks, initialize OpenSpec if needed, verify required CLI tools, detect the coverage tool, and print one setup report.

## Step 1: Check Skill Symlinks

Check these 17 owned skills:

```bash
mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch qa module-design domain-model arch-review write-a-skill
```

For each skill, verify:

1. `$HOME/.claude/skills/<name>` exists as a symlink, not a regular file or directory.
2. The symlink target exists.
3. `readlink "$HOME/.claude/skills/<name>"` equals `$HOME/.darkfactory/skills/<name>`.

Use this status format:

```text
  [x] /mission - symlink OK
  [ ] /adversarial - missing (symlink does not exist)
  [!] /dispatch - broken (symlink exists but target missing)
  [!] /prd - wrong target
```

Use a Bash pattern that always exits 0:

```bash
for skill in mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch qa module-design domain-model arch-review write-a-skill; do
  link="$HOME/.claude/skills/$skill"
  expected="$HOME/.darkfactory/skills/$skill"

  if [ ! -L "$link" ]; then
    echo "[ ] /$skill - missing (symlink does not exist)"
  elif [ ! -e "$link" ]; then
    echo "[!] /$skill - broken (symlink exists but target missing)"
  else
    target="$(readlink "$link" 2>/dev/null || true)"
    if [ "$target" != "$expected" ]; then
      echo "[!] /$skill - wrong target"
    else
      echo "[x] /$skill - symlink OK"
    fi
  fi
done
```

If any required skill is missing, broken, or points to the wrong target, instruct the user to run:

```bash
~/.darkfactory/install.sh
```

Then tell them to restart Claude Code and re-run `/mission-init`. Stop after reporting the missing skill fixes.

If all 17 skills are present and resolve correctly, report:

```text
All 17 skills: OK
```

Then continue.

## Step 2: Initialize OpenSpec

Check for `openspec/config.yaml` in the current directory.

If it exists, report:

```text
OpenSpec: already initialized.
```

If it does not exist, first check whether the `openspec` CLI is available:

```bash
openspec --version 2>/dev/null || echo "NOT_INSTALLED"
```

If `openspec` is not installed, report:

```text
OpenSpec CLI not installed. Run: `npm install -g @fission-ai/openspec`
```

If it is installed, run:

```bash
openspec init --tools claude,codex --force
```

After a first-time init, tell the user to restart Claude Code so `/opsx:` commands can register.

## Step 3: Verify CLI Dependencies

Run these checks. Each command must always exit 0 so parallel Bash calls do not cancel each other:

```bash
codex --version 2>/dev/null || echo "NOT_INSTALLED"
darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"
gh --version 2>/dev/null || echo "NOT_INSTALLED"
gh auth status 2>/dev/null || echo "NOT_INSTALLED"
openspec --version 2>/dev/null || echo "NOT_INSTALLED"
jq --version 2>/dev/null || echo "NOT_INSTALLED"
```

Report each tool as installed, not installed, or not authenticated.

- `codex`: if missing, fix with `npm install -g @openai/codex`. If installed but a Codex command reports an auth failure, fix with `codex login`.
- `darkfactory-dispatch`: if missing, fix with `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`.
- `gh`: if `gh --version` is missing, fix with `brew install gh`. If `gh auth status` fails while `gh` is installed, fix with `gh auth login`.
- `openspec`: if missing, fix with `npm install -g @fission-ai/openspec`.
- `jq`: if missing, fix with `brew install jq`.

## Step 4: Coverage Tool Detection

Detect the project language from build files at the project root:

- `Cargo.toml` means Rust.
- `package.json` means TypeScript/Node.
- `pyproject.toml` or `requirements.txt` means Python.

Check the matching coverage tool.

For Rust:

```bash
cargo tarpaulin --version 2>/dev/null || echo "NOT_INSTALLED"
```

If missing, suggest:

```bash
cargo install cargo-tarpaulin
```

For TypeScript/Node, check `package.json` for `c8`, `jest`, or `istanbul` in `dependencies` or `devDependencies`. Use an exit-0 pattern:

```bash
node -e 'const p=require("./package.json"); const d={...(p.dependencies||{}), ...(p.devDependencies||{})}; console.log(["c8","jest","istanbul"].some((k)=>d[k]) ? "FOUND" : "NOT_INSTALLED")' 2>/dev/null || echo "NOT_INSTALLED"
```

If missing, suggest:

```bash
npm install -D c8
```

For Python:

```bash
pip show pytest-cov 2>/dev/null || echo "NOT_INSTALLED"
```

If missing, suggest:

```bash
pip install pytest-cov
```

If no recognized build file exists, report:

```text
No recognized language detected - coverage tool check skipped.
```

## Step 5: Report Status

Print a structured report:

```text
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
  [x] /qa
  [x] /module-design
  [x] /domain-model
  [x] /arch-review
  [x] /write-a-skill

Optional project skills:
  [ ] /test-app (optional - project-specific)
  [ ] /demo-video (optional - project-specific)

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

Also check optional project skills at these locations:

- `$HOME/.claude/skills/test-app`
- `$HOME/.claude/skills/demo-video`
- `.claude/skills/test-app`
- `.claude/skills/demo-video`

Optional skills never block readiness.

If any required skill or tool is missing, list the needed fix commands at the bottom under:

```text
Fix commands:
```
