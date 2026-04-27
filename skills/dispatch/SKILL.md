---
name: dispatch
user-invokable: true
description: Run a coding task via darkfactory-dispatch. Accepts a task description inline or prompts for one. Parses the structured JSON result and presents outcome clearly.
---

# Dispatch

Run one coding task through `darkfactory-dispatch` and report the structured result. This is a thin user-facing wrapper around the binary.

Do not call the Codex CLI directly. Always use `darkfactory-dispatch`.

## Startup Config

At startup, read `default_model` from `~/.darkfactory/config.yaml`. If the file or key is absent, use `gpt-5.5`.

```bash
DEFAULT_MODEL=$(awk -F: '/^[[:space:]]*default_model[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$HOME/.darkfactory/config.yaml" 2>/dev/null)
: "${DEFAULT_MODEL:=gpt-5.5}"
```

## Input

Accept the task description in either mode:

- Inline: `/dispatch Fix the failing auth tests in src/auth.test.ts`
- No argument: ask the user `What task should I dispatch?` and wait for their description.

If the final task description is empty, stop and ask again.

## Step 1: Write A Prompt File

Write the task description to a temp file:

```bash
PROMPT_PATH=$(mktemp /tmp/dispatch-task.XXXXXX)
cat > "$PROMPT_PATH" << 'TASK'
<task description>
TASK
```

## Step 2: Run darkfactory-dispatch

Run the dispatcher from the current project directory and request structured JSON output:

```bash
darkfactory-dispatch \
  --model "$DEFAULT_MODEL" \
  --effort high \
  --timeout 1800 \
  --sandbox workspace-write \
  --config 'sandbox_workspace_write.writable_roots=[".git"]' \
  --prompt-file "$PROMPT_PATH" \
  --cwd "$(pwd)" \
  --json
```

Capture stdout. Parse the JSON result with `jq`.

Useful fields:

- `.status`
- `.duration_ms`
- `.activity.files_changed`
- `.output`
- `.error.message`
- `.log`

Always remove the temp prompt file after dispatch completes or fails:

```bash
rm -f "$PROMPT_PATH"
```

## Step 3: Present Outcome

If `.status` is `completed`, present:

```text
Dispatch complete.

Status: completed
Duration: <elapsed time>
Files written: <list from result, or "none reported">
Output: <output path if --output was set, otherwise "see above">
```

For files written, use `.activity.files_changed` when it exists and is non-empty. Otherwise write `none reported`.

For duration, prefer a human-readable value derived from `.duration_ms`.

For output, use `.output` when present and non-empty. Otherwise write `see above`.

If `.status` is `failed`, `.status` is `timed_out`, the command exits non-zero, JSON parsing fails, or any other error occurs, present:

```text
Dispatch failed.

Status: <status>
Error: <error.message from JSON, or "no error message">
Log: <log path if available>

The task did not complete. Review the error above and retry or adjust the task description.
```

Do not suppress failures. Always show the error clearly.
