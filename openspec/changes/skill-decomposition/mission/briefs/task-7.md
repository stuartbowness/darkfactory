# Worker Brief: Task 7 - Write dispatch skill

## Your Task

Write `skills/dispatch/SKILL.md` from scratch. This is a thin user-facing wrapper around the `darkfactory-dispatch` binary.

**File to create:** `skills/dispatch/SKILL.md`

## Required frontmatter

```yaml
---
name: dispatch
user-invokable: true
description: Run a coding task via darkfactory-dispatch. Accepts a task description inline or prompts for one. Parses the structured JSON result and presents outcome clearly.
---
```

## Required behavior

### Input

The skill accepts a task description either:
- **Inline:** `/dispatch Fix the failing auth tests in src/auth.test.ts`
- **Prompted:** `/dispatch` with no argument — ask the user "What task should I dispatch?" and wait for their description

### Step 1: Write a prompt file

Write the task description to a temp file:
```bash
PROMPT_PATH=$(mktemp /tmp/dispatch-task.XXXXXX)
cat > "$PROMPT_PATH" << 'TASK'
<task description>
TASK
```

### Step 2: Run darkfactory-dispatch

```bash
darkfactory-dispatch \
  --model gpt-5.5 \
  --effort high \
  --timeout 1800 \
  --sandbox workspace-write \
  --config 'sandbox_workspace_write.writable_roots=[".git"]' \
  --prompt-file "$PROMPT_PATH" \
  --cwd "$(pwd)" \
  --json
```

Do NOT call `codex exec` directly — always use `darkfactory-dispatch`.

Parse the JSON result from stdout using `jq`.

### Step 3: Present outcome

**On success** (`status: "completed"`):
```
Dispatch complete.

Status: completed
Duration: <elapsed time>
Files written: <list from result, or "none reported">
Output: <output path if --output was set, otherwise "see above">
```

**On failure** (`status: "failed"` or `status: "timed_out"` or any error):
```
Dispatch failed.

Status: <status>
Error: <error.message from JSON, or "no error message">
Log: <log path if available>

The task did not complete. Review the error above and retry or adjust the task description.
```

Do not suppress failures. Always show the error clearly.

### Cleanup

Remove the temp prompt file after dispatch completes (success or failure):
```bash
rm -f "$PROMPT_PATH"
```

## Acceptance criteria

- `skills/dispatch/SKILL.md` exists and `grep "^name:" skills/dispatch/SKILL.md` returns `name: dispatch`
- `grep -c "darkfactory-dispatch" skills/dispatch/SKILL.md` returns at least `1`
- `grep -c "codex exec" skills/dispatch/SKILL.md` returns `0` (does NOT call codex exec directly)
- `grep -c "\-\-json" skills/dispatch/SKILL.md` returns at least `1` (structured output documented)
- `grep -icE "fail|timed_out|error" skills/dispatch/SKILL.md` returns at least `1` (failure path documented)
- Skill prompts for task description when no arguments given; presents result summary (status, duration, files written)

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

### Task 5: Write mission-init (commit 4a3ed785)
- Created: skills/mission-init/SKILL.md

### Task 6: Write adversarial (commit 1ac6f1c7)
- Created: skills/adversarial/SKILL.md

## Downstream Consumers

- Task C3 (module shape) verifies `skills/dispatch/` exists.

## Procedures

- Write the full SKILL.md following the behavior spec above
- Commit in a single commit with message: `[task-7] write dispatch skill`
- Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-7.md`

## Handoff Report Format

- Task: Task 7 - Write dispatch skill
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
