# Task 7 Scrutiny Report — Write dispatch skill

**Commit:** b639abe1fc02811a48b429d6f5cc324caf48b170
**Date:** 2026-04-26
**Validator:** scrutiny

## Overall Result: PASS

All 6 contract assertions passed.

---

## Assertion Results

### A1 — `skills/dispatch/SKILL.md` exists with `name: dispatch`

```
test -f skills/dispatch/SKILL.md && echo PASS   → PASS
grep "^name:" skills/dispatch/SKILL.md           → name: dispatch
```

**PASS**

### A2 — `darkfactory-dispatch` appears at least once

```
grep -c "darkfactory-dispatch" skills/dispatch/SKILL.md → 5
```

Required: >= 1. Found: 5.

**PASS**

### A3 — `codex exec` must NOT appear

```
grep -c "codex exec" skills/dispatch/SKILL.md → 0
```

Required: 0. Found: 0. The skill explicitly states "Do not call the Codex CLI directly. Always use `darkfactory-dispatch`."

**PASS**

### A4 — `--json` flag documented

```
grep -c "\-\-json" skills/dispatch/SKILL.md → 1
```

Required: >= 1. Found: 1. Appears in the dispatch command block on line 46.

**PASS**

### A5 — Failure path documented (fail|timed_out|error)

```
grep -icE "fail|timed_out|error" skills/dispatch/SKILL.md → 8
```

Required: >= 1. Found: 8. Failure handling covers `.status` is `failed`, `.status` is `timed_out`, non-zero exit, JSON parse failure, and always-show-error instruction.

**PASS**

### A6 — Inspection: prompts for task when no args; presents result summary after dispatch

Inspected lines 15-20 (input section):

- No-argument path: "ask the user `What task should I dispatch?` and wait for their description."
- Empty-description guard: "If the final task description is empty, stop and ask again."

Inspected lines 66-97 (Step 3: Present Outcome):

On success, the skill presents:
- `Status:` (from `.status`)
- `Duration:` (human-readable from `.duration_ms`)
- `Files written:` (from `.activity.files_changed`)
- `Output:` (output path from `.output` or "see above")

On failure, the skill presents status, error message, and log path.

All four required fields (status, duration, files written, output path) are explicitly present.

**PASS**

---

## Notes

- The temp file lifecycle is properly handled: `mktemp /tmp/dispatch-task.XXXXXX` is portable (POSIX-compatible, no GNU-only flags), and cleanup runs after dispatch completes or fails.
- The `--model gpt-5.5 --effort high --timeout 1800` defaults are reasonable and well-chosen.
- No issues found.
