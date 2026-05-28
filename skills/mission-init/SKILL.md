---
name: mission-init
user-invokable: true
description: Verify that all Darkfactory skills are installed and required CLI tools are available. Run from any project before starting a mission.
---

# Mission Init

Verify the current project is ready to run `/mission`. Check skill symlinks, initialize OpenSpec if needed, verify required CLI tools, detect the coverage tool, and print one setup report.

## Step 1: Check Skill Symlinks

Check these 13 owned skills:

```bash
mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch artifact-review
```

For each skill, verify:

1. `$HOME/.claude/skills/<name>` exists as a symlink, not a regular file or directory.
2. The symlink target exists (not a broken symlink).
3. The symlink target contains a `SKILL.md` file (valid skill directory).
4. All skills resolve to the same Darkfactory repo root (no mixed sources).

Use this status format:

```text
  [x] /mission - symlink OK
  [ ] /adversarial - missing (symlink does not exist)
  [!] /dispatch - broken (symlink exists but target missing)
  [!] /prd - wrong target (no SKILL.md in target directory)
  [!] /tdd - wrong source (resolves to different repo root)
```

Use a Bash pattern that always exits 0:

```bash
df_root=""
for skill in mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch artifact-review; do
  link="$HOME/.claude/skills/$skill"

  if [ ! -L "$link" ]; then
    echo "[ ] /$skill - missing (symlink does not exist)"
  elif [ ! -e "$link" ]; then
    echo "[!] /$skill - broken (symlink exists but target missing)"
  elif [ ! -f "$link/SKILL.md" ]; then
    echo "[!] /$skill - wrong target (no SKILL.md in target directory)"
  else
    root="$(cd "$(readlink "$link")/../.." 2>/dev/null && pwd -P)"
    if [ -z "$df_root" ]; then
      df_root="$root"
      echo "[x] /$skill - symlink OK"
    elif [ "$root" != "$df_root" ]; then
      echo "[!] /$skill - wrong source (resolves to $root, expected $df_root)"
    else
      echo "[x] /$skill - symlink OK"
    fi
  fi
done
```

If any required skill is missing, broken, or has a wrong target, instruct the user to run `install.sh` from their Darkfactory clone directory. Then tell them to restart Claude Code and re-run `/mission-init`. Stop after reporting the missing skill fixes.

If all 13 skills are present and resolve correctly, report:

```text
All 13 skills: OK
```

Then continue.

## Step 1b: Check Codex Skill Symlinks

Workers dispatched via `darkfactory-dispatch` run inside `codex exec`, which discovers skills from `~/.codex/skills/`. Verify the skills that workers must invoke are also symlinked there.

Check these skills in `$HOME/.codex/skills/`:

```bash
for skill in frontend tdd ui-system react-perf; do
  link="$HOME/.codex/skills/$skill"
  if [ ! -L "$link" ]; then
    echo "[ ] /$skill - missing in ~/.codex/skills/"
  elif [ ! -e "$link" ]; then
    echo "[!] /$skill - broken symlink in ~/.codex/skills/"
  else
    resolved="$(cd "$link" 2>/dev/null && pwd -P)"
    expected="$(cd "$HOME/.claude/skills/$skill" 2>/dev/null && pwd -P)"
    if [ "$resolved" != "$expected" ]; then
      echo "[!] /$skill - wrong target in ~/.codex/skills/"
    else
      echo "[x] /$skill - symlink OK in ~/.codex/skills/"
    fi
  fi
done
```

If any are missing, the fix is the same as Step 1: run `install.sh` from the Darkfactory clone directory.

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
- `darkfactory-dispatch`: if missing, fix with `cd <darkfactory-clone>/tools/darkfactory-dispatch && make install` (where `<darkfactory-clone>` is the directory where the user cloned the Darkfactory repo).
- `gh`: if `gh --version` is missing, fix with `brew install gh`. If `gh auth status` fails while `gh` is installed, fix with `gh auth login`.
- `openspec`: if missing, fix with `npm install -g @fission-ai/openspec`.
- `jq`: if missing, fix with `brew install jq`.

## Step 4: Browser Tools Check (conditional)

If `browser_tools: true` in the mission's config.yaml (or if any task in the current mission has `Skill: /frontend`), check chrome-devtools-mcp in **both** Claude Code and Codex environments.

### 4a: Claude Code (orchestrator)

Check whether any tool names starting with `mcp__chrome-devtools__` or `chrome-devtools` appear in the available tool list. If none are found, the server is not registered.

If chrome-devtools-mcp is not registered in Claude Code:
- Report: `[!] chrome-devtools-mcp - not registered in Claude Code (required when browser_tools: true)`
- Fix: Add chrome-devtools-mcp to Claude Code's MCP settings
- This blocks mission launch for missions with `/frontend` tasks

### 4b: Codex (dispatch workers)

Workers dispatched via `darkfactory-dispatch` run in `codex exec`. They need chrome-devtools-mcp configured in Codex's global config. Check `~/.codex/config.toml` for the MCP server entry:

```bash
grep -q '\[mcp_servers\.chrome-devtools\]' "$HOME/.codex/config.toml" 2>/dev/null && echo "CONFIGURED" || echo "NOT_CONFIGURED"
```

If not configured:
- Report: `[!] chrome-devtools-mcp - not configured in Codex (workers cannot run visual verification)`
- Fix: Add to `~/.codex/config.toml`:
  ```toml
  [mcp_servers.chrome-devtools]
  command = "npx"
  args = ["-y", "chrome-devtools-mcp@latest"]
  ```
- This blocks mission launch for missions with `/frontend` tasks

### 4c: Dispatch probe (optional, run when both 4a and 4b pass)

Run a lightweight dispatch to verify chrome-devtools-mcp actually works inside `codex exec`:

```bash
darkfactory-dispatch \
  --model gpt-5.5 \
  --effort low \
  --timeout 120 \
  --sandbox read-only \
  --prompt "List browser pages using the chrome-devtools MCP tool (list_pages). Report the result or any error. Write PROBE_OK if the tool responded, PROBE_FAIL if it was unavailable." \
  --cwd "$(pwd)" \
  --json
```

Parse the result. If the dispatch completes and the output or activity suggests the MCP tool was called, report:
- `[x] chrome-devtools-mcp - verified in dispatch worker`

If the probe fails or times out, report:
- `[!] chrome-devtools-mcp - configured but not working in dispatch (check npx and Node availability)`
- Do not block on probe failure if 4a and 4b both passed — report as a warning.

If `browser_tools: false` or no `/frontend` tasks exist, skip all of Step 4.

## Step 5: Coverage Tool Detection

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

For TypeScript/Node, check `package.json` for `vitest`, `@vitest/coverage-v8`, `@vitest/coverage-istanbul`, `c8`, `jest`, or `istanbul` in `dependencies` or `devDependencies`. Use an exit-0 pattern:

```bash
node -e 'const p=require("./package.json"); const d={...(p.dependencies||{}), ...(p.devDependencies||{})}; console.log(["vitest","@vitest/coverage-v8","@vitest/coverage-istanbul","c8","jest","istanbul"].some((k)=>d[k]) ? "FOUND" : "NOT_INSTALLED")' 2>/dev/null || echo "NOT_INSTALLED"
```

If missing, suggest (Vitest preferred for new projects):

```bash
npm install -D vitest @vitest/coverage-v8
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

## Step 6: Report Status

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
  [x] /artifact-review

Optional standalone skills:
  [ ] /arch-review (optional - standalone)
  [ ] /domain-model (optional - standalone)
  [ ] /module-design (optional - standalone)
  [ ] /qa (optional - standalone)
  [ ] /write-a-skill (optional - standalone)
  [ ] /skill-review (optional - standalone)

Optional project skills:
  [ ] /test-app (optional - project-specific)
  [ ] /demo-video (optional - project-specific)

CLI tools:
  [x] codex (authenticated)
  [x] darkfactory-dispatch
  [x] gh (authenticated)
  [x] openspec
  [x] jq

Codex worker skills (via ~/.codex/skills/ symlinks):
  [x] /frontend
  [x] /tdd
  [x] /ui-system
  [x] /react-perf

Browser tools:
  [-] chrome-devtools-mcp in Claude Code (skipped - no /frontend tasks)
  [-] chrome-devtools-mcp in Codex config (skipped - no /frontend tasks)
  [-] chrome-devtools-mcp dispatch probe (skipped - no /frontend tasks)

Coverage:
  [x] cargo-tarpaulin (Rust detected)

OpenSpec:
  [x] Initialized

Ready: run /mission to start.
```

Also check optional standalone skills at these locations:

- `$HOME/.claude/skills/arch-review`
- `$HOME/.claude/skills/domain-model`
- `$HOME/.claude/skills/module-design`
- `$HOME/.claude/skills/qa`
- `$HOME/.claude/skills/write-a-skill`
- `$HOME/.claude/skills/skill-review`
- `$HOME/.claude/skills/frontend`
- `$HOME/.claude/skills/ui-system`
- `$HOME/.claude/skills/react-perf`

And optional project skills at these locations:

- `$HOME/.claude/skills/test-app`
- `$HOME/.claude/skills/demo-video`
- `.claude/skills/test-app`
- `.claude/skills/demo-video`

Optional skills never block readiness.

If any required skill or tool is missing, list the needed fix commands at the bottom under:

```text
Fix commands:
```
