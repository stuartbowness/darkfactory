---
name: adversarial
user-invokable: true
description: Run a dual-agent adversarial code review. Dispatches a Codex reviewer in parallel with a Claude reviewer. Both review the same diff independently. Surfaces disagreements after both complete.
---

# Adversarial Review

Run two independent reviewers against the same diff, then compare their reports. This skill is standalone and does not depend on the mission system.

## Startup Config

At startup, resolve the Darkfactory repo root from the skill symlink and read `default_model` from its `config.yaml`. If the file or key is absent, use `gpt-5.5`.

```bash
DARKFACTORY_ROOT="$(cd "$(readlink "$HOME/.claude/skills/adversarial")/../.." 2>/dev/null && pwd -P)"
DEFAULT_MODEL=$(awk -F: '/^[[:space:]]*default_model[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$DARKFACTORY_ROOT/config.yaml" 2>/dev/null)
: "${DEFAULT_MODEL:=gpt-5.5}"
```

## Input Modes

Determine the mode from the optional argument:

- No argument: review `git diff HEAD` for all staged and unstaged working tree changes.
- PR number, such as `/adversarial 42`: review `gh pr diff 42`.
- File path, such as `/adversarial path/to/file.ts`: review `git diff HEAD -- path/to/file.ts`.

Treat an all-digit argument as a PR number. Treat any other non-empty argument as a file path.

## Report Paths

Before dispatching reviewers, choose report paths:

- If the repo contains a Darkfactory mission directory matching `openspec/changes/*/mission/`, excluding any path that contains `openspec/changes/archive/`, write reports under that mission:
  - `mission/validations/adversarial-codex.md`
  - `mission/validations/adversarial-claude.md`
- Otherwise, write reports to temp files and tell the user both paths:
  - `/tmp/adversarial-codex-<timestamp>.md`
  - `/tmp/adversarial-claude-<timestamp>.md`

When a mission path is used, create `mission/validations/` if needed.

## Step 1: Get The Diff

Run exactly one diff command based on the input mode:

```bash
git diff HEAD
gh pr diff <number>
git diff HEAD -- <path>
```

If the diff is empty, report `No changes to review` and stop.

Save the diff content once and pass the identical content to both reviewers.

## Step 2: Dispatch Codex Reviewer

Start Codex first and do not wait for it before launching Claude. Dispatch with `darkfactory-dispatch --async` and use the Bash tool with `run_in_background: true`. Capture the `dispatch_id` from the `async_started` JSON on stdout.

Prompt:

```text
You are an adversarial code reviewer. Review the following diff for bugs, logic errors, security issues, and missing edge cases. Group findings by severity: critical, warning, nit. Cite file:line for each finding. Write your report to: <codex-report-path>

<diff content>
```

Dispatch command:

```bash
PROMPT_PATH=$(mktemp /tmp/adversarial-prompt.XXXXXX)
cat > "$PROMPT_PATH" << 'PROMPT'
You are an adversarial code reviewer. Review the following diff for bugs, logic errors, security issues, and missing edge cases. Group findings by severity: critical, warning, nit. Cite file:line for each finding. Write your report to: <codex-report-path>

<diff content>
PROMPT

ASYNC_RESULT=$(darkfactory-dispatch \
  --async \
  --model "$DEFAULT_MODEL" \
  --effort high \
  --timeout 900 \
  --sandbox workspace-write \
  --prompt-file "$PROMPT_PATH" \
  --output "<codex-report-path>" \
  --cwd "$(pwd)")

DISPATCH_ID=$(printf '%s\n' "$ASYNC_RESULT" | jq -r 'select(.kind == "async_started") | .dispatch_id // empty')
if [ -z "$DISPATCH_ID" ]; then
  echo "Failed to start Codex reviewer: $ASYNC_RESULT"
  exit 1
fi
```

## Step 3: Dispatch Claude Reviewer

While Codex is running in the background, dispatch Claude via the Agent tool. Give Claude the same diff content and the same reviewer prompt, changing only the report path to `<claude-report-path>`.

Claude must write its report to `adversarial-claude.md` at the selected report path.

## Step 4: Collect Results

After Claude finishes, run `darkfactory-dispatch wait <dispatch-id>` using the dispatch ID captured in Step 2:

```bash
darkfactory-dispatch wait "$DISPATCH_ID"
rm -f "$PROMPT_PATH"
```

Only read both report files after `darkfactory-dispatch wait <dispatch-id>` completes successfully. If the wait command fails, report the dispatch failure and do not claim the Codex report completed.

For each report, count findings by severity:

- `critical`
- `warning`
- `nit`

Find disagreements:

- Findings that appear in the Codex report but not the Claude report.
- Findings that appear in the Claude report but not the Codex report.
- Findings both report but classify with different severity.

Compare findings by affected file, line, and issue description. Minor wording differences do not count as a disagreement when the same bug is clearly being described.

## User Output

Present the final result in this format:

```text
Adversarial review complete.

Codex reviewer: N critical, M warnings, P nits
  Full report: <path>

Claude reviewer: N critical, M warnings, P nits
  Full report: <path>

Disagreements (flagged by one reviewer only):
- [Codex only] <finding>
- [Claude only] <finding>

Review the full reports and decide what to address.
```

If there are no disagreements, write:

```text
Disagreements (flagged by one reviewer only):
- None
```
