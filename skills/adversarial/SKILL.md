---
name: adversarial
user-invokable: true
description: Run a dual-agent adversarial code review. Dispatches a Codex reviewer in parallel with a Claude reviewer. Both review the same diff independently. Surfaces disagreements after both complete.
---

# Adversarial Review

Run two independent reviewers against the same diff, then compare their reports. This skill is standalone and does not depend on the mission system.

## Startup Config

At startup, resolve the Darkfactory repo root from the skill symlink and read `default_model` from its `config.yaml`. If the file or key is absent, use `gpt-5.6-terra`.

```bash
DARKFACTORY_ROOT="$(cd "$(readlink "$HOME/.claude/skills/adversarial")/../.." 2>/dev/null && pwd -P)"
DEFAULT_MODEL=$(awk -F: '/^[[:space:]]*default_model[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$DARKFACTORY_ROOT/config.yaml" 2>/dev/null)
: "${DEFAULT_MODEL:=gpt-5.6-terra}"
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
  - `mission/logs/adversarial-codex.log`
- Otherwise, write reports to temp files and tell the user both paths:
  - `/tmp/adversarial-codex-<timestamp>.md`
  - `/tmp/adversarial-claude-<timestamp>.md`
  - `/tmp/adversarial-codex-<timestamp>.log`

When a mission path is used, create `mission/validations/` and `mission/logs/` if needed.

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

Start Codex first and do not block before launching Claude. Dispatch with `darkfactory-dispatch --json` in a single Bash tool call with `run_in_background: true`. The backgrounded script must capture the dispatcher output, parse `.status`, remove the prompt file, and print a final stdout line beginning with `REVIEW_COMPLETE` or `REVIEW_FAILED`.

The backgrounded script runs in its own shell, so it must be self-contained: resolve the model inside it rather than relying on `$DEFAULT_MODEL` from the Startup Config call, which does not survive into a separate Bash tool call.

`--output "<codex-report-path>"` writes Codex's **final message** to that path. So the prompt asks Codex to return the report as its final message and not write any files itself -- if Codex wrote the report to that path with its own tools, its closing assistant message would overwrite it. (Claude in Step 3 has no `--output`, so it writes its file directly.)

Prompt:

```text
You are an adversarial code reviewer. Review the following diff for bugs, logic errors, security issues, and missing edge cases. Group findings by severity: critical, warning, nit. Cite file:line for each finding. Return the full report as your final message -- do not write any files.

<diff content>
```

Dispatch command:

```bash
# Resolve the model inside this backgrounded script (see note above).
DARKFACTORY_ROOT="$(cd "$(readlink "$HOME/.claude/skills/adversarial")/../.." 2>/dev/null && pwd -P)"
DEFAULT_MODEL=$(awk -F: '/^[[:space:]]*default_model[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$DARKFACTORY_ROOT/config.yaml" 2>/dev/null)
: "${DEFAULT_MODEL:=gpt-5.6-terra}"

REPORT_PATH="<codex-report-path>"
LOG_PATH="<codex-log-path>"
PROMPT_PATH=$(mktemp /tmp/adversarial-prompt.XXXXXX)
# Use >| : mktemp pre-creates the file, and zsh `noclobber` makes a bare `>` fail on it.
cat >| "$PROMPT_PATH" << 'PROMPT'
You are an adversarial code reviewer. Review the following diff for bugs, logic errors, security issues, and missing edge cases. Group findings by severity: critical, warning, nit. Cite file:line for each finding. Return the full report as your final message -- do not write any files.

<diff content>
PROMPT

DISPATCH_RESULT=$(darkfactory-dispatch \
  --model "$DEFAULT_MODEL" \
  --effort high \
  --timeout 900 \
  --sandbox workspace-write \
  --prompt-file "$PROMPT_PATH" \
  --output "$REPORT_PATH" \
  --cwd "$(pwd)" \
  --log "$LOG_PATH" \
  --json 2>>"$LOG_PATH")

rm -f "$PROMPT_PATH"

if [ -z "$DISPATCH_RESULT" ]; then
  echo "REVIEW_FAILED: dispatcher crashed with no output. Log: $LOG_PATH"
else
  DISPATCH_STATUS=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.status')
  DISPATCH_ERROR=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.error.message // empty')

  if [ "$DISPATCH_STATUS" = "completed" ] && [ -f "$REPORT_PATH" ]; then
    echo "REVIEW_COMPLETE: report written"
  else
    echo "REVIEW_FAILED: dispatch status $DISPATCH_STATUS${DISPATCH_ERROR:+; error: $DISPATCH_ERROR}; report $([ -f "$REPORT_PATH" ] && echo present || echo missing). Log: $LOG_PATH"
  fi
fi
```

## Step 3: Dispatch Claude Reviewer

While Codex is running in the background, dispatch Claude via the Agent tool with `subagent_type=darkfactory-claude`. Give Claude the same diff content and the same reviewer prompt, changing only the report path to `<claude-report-path>`.

Claude must write its report to `adversarial-claude.md` at the selected report path.

## Step 4: Collect Results

After Claude finishes, retrieve the backgrounded Codex Bash call's stdout with BashOutput. If the Codex shell task has not completed yet, collect it again after the task completion notification.

Treat a final stdout line beginning with `REVIEW_COMPLETE` as success. Only then read the Codex report file. Treat a final stdout line beginning with `REVIEW_FAILED`, or a dispatcher `.status` of `failed` or `timed_out`, as a Codex review failure. On failure, report the dispatch failure and log path instead of claiming the Codex report completed.

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
