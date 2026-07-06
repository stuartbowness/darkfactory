# Worker Brief: Task 3 - Convert /adversarial to synchronous dispatch

## Your Task
Rewrite `skills/adversarial/SKILL.md` so it dispatches its Codex reviewer with a **synchronous `--json`** dispatch run via a single Bash `run_in_background: true` call and collected with `BashOutput` — instead of the current `--async` + `darkfactory-dispatch wait "$DISPATCH_ID"` pattern. The async lifecycle surface (`--async`, `status`/`wait`/`result`) no longer exists after Task 2, and the current pattern is also buggy (see below).

### Why the current pattern is broken
Step 2 dispatches with `--async` inside a `run_in_background` Bash call and captures `DISPATCH_ID` from the `async_started` JSON. Step 4 then runs `darkfactory-dispatch wait "$DISPATCH_ID"` in a **separate** Bash call — but a shell variable set inside a backgrounded Bash call does NOT survive into a later, separate Bash call, so `$DISPATCH_ID` is empty and the wait fails, reporting a *passing* review as failed.

### Target pattern (mirror the mission's Phase 9 Reviewer A script)
Do the whole Codex dispatch — result capture, status parse, and outcome marker — inside ONE `run_in_background: true` Bash call, so nothing needs to cross call boundaries. See `skills/mission/SKILL.md` Phase 9 "Reviewer A Dispatch Script" for the exact shape. Concretely:

- **Step 2 (Dispatch Codex reviewer):** remove `--async`; use `--json`; add `--log "<log-path>"` and redirect the dispatcher's stderr with `2>>"<log-path>"`. Capture the result and decide the outcome in the same backgrounded call, e.g.:
  ```bash
  DISPATCH_RESULT=$(darkfactory-dispatch \
    --model "$DEFAULT_MODEL" --effort high --timeout 900 \
    --sandbox workspace-write \
    --prompt-file "$PROMPT_PATH" --output "<codex-report-path>" \
    --cwd "$(pwd)" --log "<codex-log-path>" --json 2>>"<codex-log-path>")
  rm -f "$PROMPT_PATH"
  if [ -z "$DISPATCH_RESULT" ]; then
    echo "REVIEW_FAILED: dispatcher crashed with no output. Log: <codex-log-path>"
  else
    DISPATCH_STATUS=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.status')
    DISPATCH_ERROR=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.error.message // empty')
    if [ "$DISPATCH_STATUS" = "completed" ] && [ -f "<codex-report-path>" ]; then
      echo "REVIEW_COMPLETE: report written"
    else
      echo "REVIEW_FAILED: status $DISPATCH_STATUS${DISPATCH_ERROR:+; error: $DISPATCH_ERROR}. Log: <codex-log-path>"
    fi
  fi
  ```
  The final stdout line is `REVIEW_COMPLETE` or `REVIEW_FAILED`.
- Remove the "Do NOT pass `--json` alongside `--async`" note and the `async_started`/`DISPATCH_ID` capture. Keep the note that `--output` writes Codex's final message, so the prompt still asks Codex to return the report as its final message and write no files.
- **Step 3 (Claude reviewer):** unchanged (already synchronous via the Agent tool).
- **Step 4 (Collect Results):** replace `darkfactory-dispatch wait "$DISPATCH_ID"` with retrieving the backgrounded Bash call's stdout via **BashOutput**; treat `REVIEW_COMPLETE` as success and `REVIEW_FAILED` (or a `failed`/`timed_out` status) as a Codex review failure — read the Codex report only on `REVIEW_COMPLETE`, and on failure report the dispatch failure rather than claiming the report completed.

## Acceptance criteria
- `skills/adversarial/SKILL.md` contains no `--async`, no `async_started`, and no `darkfactory-dispatch` lifecycle subcommand (`status`/`wait`/`result`) or dispatch-id lookup.
- It dispatches the Codex reviewer with `--json` via `run_in_background` + `BashOutput`, parses the result `.status`, and reports `REVIEW_COMPLETE` on `completed` / `REVIEW_FAILED` on `failed`/`timed_out`.
- The parallelism is preserved: dispatch Codex first (non-blocking background), then run the Claude reviewer, then collect Codex via BashOutput.

## Project Context
This mission made `darkfactory-dispatch` synchronous-only (Task 2). `/adversarial` was the only skill using the removed async surface; this task converts it to the synchronous pattern so it keeps working and reports correct outcomes.

## Architecture
This is a change to one skill file: `skills/adversarial/SKILL.md` (Darkfactory skills live at the repo root under `skills/`). No Rust code changes. The reference implementation is the Phase 9 Reviewer A block in `skills/mission/SKILL.md`.

## Key Terms
`Dispatch` — spawning a supervised Codex subprocess via `darkfactory-dispatch`. The result is a JSON object on stdout with a `status` field (`completed`/`failed`/`timed_out`).

## Domain Types
None (skill-markdown change).

## What Previous Tasks Built
- Task 1 (07833a0): removed dead test-only code.
- Task 2 (eece9ea): made `darkfactory-dispatch` synchronous-only — removed `--async` and the `status`/`wait`/`result` subcommands. A synchronous `--json` dispatch prints one result object on stdout; run it in a backgrounded Bash call and read `.status`.

## Downstream Consumers
Task 4 documents the final state. No code consumes this skill.

## Procedures
**Before writing any change**, invoke the `/tdd` skill. This task edits a skill markdown file (no compilable code), so "test" here means: verify by inspection that the async surface is gone and the synchronous pattern (with `.status` parsing and the REVIEW_COMPLETE/REVIEW_FAILED markers) is present, and that the reference pattern matches the mission's Phase 9 Reviewer A block.
- Commit in a single commit: `[task-3] Convert /adversarial to synchronous dispatch`. Stage only `skills/adversarial/SKILL.md`. Do NOT `git add -A`; never stage anything under `openspec/changes/dispatch-sync-only/`.
- Write your handoff to `openspec/changes/dispatch-sync-only/mission/handoffs/task-3.md` (write the file; do not commit it).

## Handoff Report Format
- Task: Task 3 - Convert /adversarial to synchronous dispatch
- Commit SHA: <the full SHA>
- Approach: <what you changed in Steps 2 and 4>
- Commands run: <any>
- Concerns: <anything uncertain>
