# Task 7.1 User-Test Validation

**Commit:** 6abecb7
**Date:** 2026-04-23
**Validator:** Claude Opus 4.6

## Test Scope

This is a markdown skill file change. Validation is behavioral analysis of the instruction text, not runtime execution.

## Scenario 1: Normal codex worker dispatch succeeds

**Steps:**
1. Mission orchestrator reaches Step 2 with engine=codex.
2. Bash block at line 484 runs `darkfactory-dispatch` with `--json`.
3. Dispatch completes successfully. JSON output: `{"status": "completed", ...}`.
4. `jq` extracts `"completed"`. Condition `[ "$DISPATCH_STATUS" = "completed" ] && [ -f "$HANDOFF_PATH" ]` is true.
5. Stdout: `TASK_COMPLETE: handoff written`.
6. Step 3 reads `TASK_COMPLETE`, proceeds to Step 4.

**Verdict:** PASS -- Happy path works as before, now with structured JSON instead of exit code.

## Scenario 2: Codex worker times out

**Steps:**
1. Dispatch runs past 1800s timeout.
2. Dispatcher kills the codex process, returns JSON: `{"status": "timed_out", ...}`.
3. `jq` extracts `"timed_out"`. Condition fails (not `"completed"`).
4. Stdout: `TASK_FAILED: dispatch status timed_out; handoff missing. Log: ...`.
5. Step 3 reads `TASK_FAILED`, triggers Claude fallback (line 522).

**Verdict:** PASS -- Timeout is now handled automatically by the dispatcher instead of requiring human intervention.

## Scenario 3: Codex worker fails (non-zero exit)

**Steps:**
1. Codex exits non-zero. Dispatcher returns JSON: `{"status": "failed", ...}`.
2. `jq` extracts `"failed"`. Condition fails.
3. Stdout: `TASK_FAILED: dispatch status failed; handoff missing. Log: ...`.
4. Step 3 triggers Claude fallback.

**Verdict:** PASS -- Same fallback behavior as before, cleaner error message with status name.

## Scenario 4: Adversarial reviewer A succeeds

**Steps:**
1. Reviewer A dispatch at line 744 runs with `--timeout 2700`.
2. Reviewer completes. JSON: `{"status": "completed", ...}`.
3. Condition true, report file exists.
4. Stdout: `REVIEW_COMPLETE: report written`.
5. Orchestrator reads the review report.

**Verdict:** PASS -- Reviewer path works identically to worker path with appropriate timeout.

## Scenario 5: Adversarial reviewer A times out

**Steps:**
1. Review exceeds 2700s. Dispatcher kills it, returns `{"status": "timed_out", ...}`.
2. Stdout: `REVIEW_FAILED: dispatch status timed_out; report missing. Log: ...`.
3. Orchestrator handles the failure.

**Verdict:** PASS -- Previously the reviewer had no timeout at all. Now it has a 45-minute hard cap.

## Scenario 6: Human monitors dispatch progress

**Steps:**
1. Human runs `tail -f <log-path>` in another terminal during dispatch.
2. Heartbeat events appear every 15s on stderr showing elapsed time and activity.
3. If timeout is reached, the human sees the kill happen automatically.

**Verdict:** PASS -- Human visibility improved. No more need to manually `ps aux | grep codex` and kill stale processes.

## Scenario 7: Dispatch succeeds but handoff file missing

**Steps:**
1. JSON status is `"completed"` but handoff file was not written (codex bug).
2. Condition `[ "$DISPATCH_STATUS" = "completed" ] && [ -f "$HANDOFF_PATH" ]` fails on the file check.
3. Stdout: `TASK_FAILED`.
4. Fallback triggers correctly.

**Verdict:** PASS -- Dual condition preserved from old code. File existence check is defense-in-depth.

## Scenario 8: jq not installed or JSON malformed

**Steps:**
1. `jq` is not on PATH or dispatch returns non-JSON output.
2. `DISPATCH_STATUS` is empty or contains an error string.
3. Condition fails (empty string != "completed").
4. Stdout: `TASK_FAILED`.

**Verdict:** PASS (fail-safe) -- Degrades to failure path, which triggers fallback. Error message is slightly less informative but the system recovers. The `darkfactory-init` skill should verify `jq` presence to prevent this.

## Overall Verdict

**PASS** -- All user-facing scenarios work correctly. The change improves the operator experience by adding automatic timeouts and heartbeat visibility while maintaining the existing fallback-to-Claude safety net.
