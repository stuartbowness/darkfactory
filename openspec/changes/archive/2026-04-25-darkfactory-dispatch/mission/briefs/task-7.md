# Worker Brief: Task 7 - Mission skill integration

## Your Task
Update `.claude/skills/mission/SKILL.md` to use `darkfactory-dispatch` instead of raw `codex exec` calls. There are two bash blocks to replace:

### 1. Worker dispatch (~line 486)

**Current:**
```bash
codex exec \
  --full-auto \
  --sandbox workspace-write \
  -c 'sandbox_workspace_write.writable_roots=[".git"]' \
  -m gpt-5.5 \
  -c model_reasoning_effort="high" \
  -o "$HANDOFF_PATH" \
  "$BRIEF_CONTENT" \
  < /dev/null > "$LOG_PATH" 2>&1
```

**Replace with:**
```bash
DISPATCH_RESULT=$(darkfactory-dispatch \
  --model gpt-5.5 \
  --effort high \
  --timeout 1800 \
  --sandbox workspace-write \
  --config 'sandbox_workspace_write.writable_roots=[".git"]' \
  --prompt-file "$BRIEF_PATH" \
  --output "$HANDOFF_PATH" \
  --cwd "$(pwd)" \
  --log "$LOG_PATH" \
  --json)

DISPATCH_STATUS=$(echo "$DISPATCH_RESULT" | jq -r '.status')

if [ "$DISPATCH_STATUS" = "completed" ] && [ -f "$HANDOFF_PATH" ]; then
  echo "TASK_COMPLETE: handoff written"
else
  echo "TASK_FAILED: dispatch status $DISPATCH_STATUS; handoff $([ -f "$HANDOFF_PATH" ] && echo present || echo missing). Log: $LOG_PATH"
fi
```

Note: the prompt is passed via `--prompt-file "$BRIEF_PATH"` instead of `"$BRIEF_CONTENT"` to avoid shell argument length limits on large briefs.

### 2. Adversarial reviewer A dispatch (~line 745)

**Current:**
```bash
codex exec \
  --full-auto \
  --sandbox workspace-write \
  -m gpt-5.5 \
  -c model_reasoning_effort="high" \
  -o "$REPORT_PATH" \
  "$(cat "$PROMPT_PATH")" \
  < /dev/null > "$LOG_PATH" 2>&1
```

**Replace with:**
```bash
DISPATCH_RESULT=$(darkfactory-dispatch \
  --model gpt-5.5 \
  --effort high \
  --timeout 2700 \
  --sandbox workspace-write \
  --prompt-file "$PROMPT_PATH" \
  --output "$REPORT_PATH" \
  --cwd "$(pwd)" \
  --log "$LOG_PATH" \
  --json)

DISPATCH_STATUS=$(echo "$DISPATCH_RESULT" | jq -r '.status')
```

### 3. Update result parsing

The current skill checks for `TASK_COMPLETE` or `TASK_FAILED` strings on the final stdout line. Update the result handling to parse the JSON result:

- `status: "completed"` -> proceed (equivalent of TASK_COMPLETE)
- `status: "failed"` or `status: "timed_out"` -> trigger fallback (equivalent of TASK_FAILED)

The existing fallback logic (Claude worker fallback on first codex failure, human escalation on second failure) should be preserved. The trigger condition just changes from string matching to JSON status parsing.

### 4. Remove the `< /dev/null` and `EXIT=$?` patterns

These are no longer needed -- darkfactory-dispatch handles stdin closure and exit code detection internally. The JSON result contains all the information the orchestrator needs.

### 5. Update the human visibility note

Change from:
> To watch codex progress live, run `tail -f <log path>` in another terminal.

To:
> To watch dispatch progress live, run `tail -f <log path>` in another terminal. The dispatcher emits heartbeat events every 15 seconds to stderr showing elapsed time and current activity.

## Project Context
darkfactory-dispatch is the new Rust process supervisor that wraps codex exec with timeout, heartbeat, and structured results. The mission skill is the primary consumer.

## Key Terms
- **Dispatcher**: The darkfactory-dispatch binary sitting between orchestrator and codex
- **Dispatch result**: Structured JSON with status, exit_code, signal, tokens, activity
- **Two-phase timeout**: Soft warning then hard kill -- the dispatcher handles this automatically

## What Previous Tasks Built
### Tasks 1-6: darkfactory-dispatch binary
- Full Rust binary with supervisor, engine, event emitter, CLI, async mode
- 35 tests passing, clippy clean

## Procedures
- Read the current SKILL.md to understand the full context around the two bash blocks
- Make targeted replacements -- do not restructure the surrounding skill logic
- Preserve the existing fallback behavior (Claude fallback on codex failure)
- Commit: "[task-7] Update mission skill to use darkfactory-dispatch for codex workers"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-7.md

## Handoff Report Format
- Task: Task 7 - Mission skill integration
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
