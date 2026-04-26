# Task 7.1 Scrutiny Validation

**Commit:** 6abecb7
**Date:** 2026-04-23
**Validator:** Claude Opus 4.6

## Exempt Gates

- **Test presence:** Exempt (markdown skill file, not testable code)
- **Build/test/lint:** Exempt (no compiled code changed)

## Contract Assertions

### 1. Worker dispatch block calls `darkfactory-dispatch` instead of raw `codex exec`
**PASS**

Line 484 of `SKILL.md` now reads:
```
DISPATCH_RESULT=$(darkfactory-dispatch \
```
The old `codex exec` invocation is fully removed. Grep confirms zero matches for `codex exec` in the file.

### 2. Adversarial reviewer A block calls `darkfactory-dispatch` instead of raw `codex exec`
**PASS**

Line 744 of `SKILL.md` now reads:
```
DISPATCH_RESULT=$(darkfactory-dispatch \
```
Same replacement pattern as the worker dispatch. Old `codex exec` invocation fully removed.

### 3. Mission skill parses JSON result for `status` field instead of checking for `TASK_COMPLETE`/`TASK_FAILED` strings
**PASS**

Both dispatch blocks now capture `darkfactory-dispatch` output into `$DISPATCH_RESULT` and extract the status with:
```bash
DISPATCH_STATUS=$(echo "$DISPATCH_RESULT" | jq -r '.status')
```
The old pattern (`EXIT=$?` followed by `if [ $EXIT -eq 0 ]`) is fully removed. The `TASK_COMPLETE`/`TASK_FAILED` strings still appear but only as echo output to stdout for the orchestrator's Step 3 logic -- the parsing mechanism itself is now JSON-based.

### 4. Fallback logic triggers on `status: "failed"` or `status: "timed_out"` from the JSON result
**PASS**

The condition `if [ "$DISPATCH_STATUS" = "completed" ]` with an else branch means any non-`"completed"` status (including `"failed"` and `"timed_out"`) echoes `TASK_FAILED`, which triggers the fallback path in Step 3 (line 521-522). The else-branch design is more robust than explicitly listing two status values -- it also catches unexpected statuses like `"error"` or malformed JSON.

## Additional Checks

### No remnant `codex exec` calls
**PASS** -- `grep -n 'codex exec' SKILL.md` returns zero matches.

### `< /dev/null` removed
**PASS** -- `grep -n '< /dev/null' SKILL.md` returns zero matches. The dispatcher handles stdin closure internally.

### Human visibility note updated
**PASS** -- The old "If codex hangs" paragraph (describing manual `ps aux | grep codex` workaround and no hard timeout) is replaced with a single paragraph at line 510 explaining:
- `tail -f` still works for live progress
- Heartbeat events every 15 seconds to stderr
- Automatic timeout with soft warning then hard kill after grace period

The "no hard timeout" footgun is eliminated.

### Fallback logic compatibility
**PASS** -- The Claude worker fallback path (Step 3, lines 520-526) is unchanged and still works correctly. The bash blocks translate JSON status into the same `TASK_COMPLETE`/`TASK_FAILED` stdout protocol that Step 3 consumes. The fallback triggers on `TASK_FAILED` as before.

### Timeout values
**NOTE** -- Worker dispatch uses `--timeout 1800` (30 min), adversarial reviewer uses `--timeout 2700` (45 min). The reviewer gets more time, which makes sense since it reviews the full mission output. Both are reasonable values.

### Missing `--config` on adversarial reviewer
**NOTE** -- The worker dispatch includes `--config 'sandbox_workspace_write.writable_roots=[".git"]'` but the adversarial reviewer does not. This is correct: the reviewer writes a report file (via `--output`), not git commits. No `.git` write access needed.

## Code Review

The diff is clean. 28 lines added, 28 removed. The new `darkfactory-dispatch` invocations use a consistent flag pattern (`--model`, `--effort`, `--timeout`, `--sandbox`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json`). The `--json` flag ensures structured output for `jq` parsing.

One minor observation: the `jq` dependency is implicit. If `jq` is not installed, `DISPATCH_STATUS` will be empty and the else branch triggers `TASK_FAILED`. This is fail-safe behavior (worker retries or escalates) but the error message would say `dispatch status ; handoff missing` which is slightly confusing. This is acceptable -- `jq` is a standard tool and `darkfactory-init` should verify its presence.

## Verdict

**PASS** -- All 4 contract assertions satisfied. No remnant patterns. Fallback logic intact.
