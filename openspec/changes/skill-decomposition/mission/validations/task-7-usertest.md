# Task 7 User-Test Report — Write dispatch skill

**Commit:** b639abe1fc02811a48b429d6f5cc324caf48b170
**Date:** 2026-04-26
**Validator:** user-test

## Overall Result: PASS

All contract assertions passed. Exempt from test presence gate (pure .md file).

---

## Assertion Summary

| # | Assertion | Result |
|---|-----------|--------|
| A1 | `skills/dispatch/SKILL.md` exists; `name: dispatch` | PASS |
| A2 | `darkfactory-dispatch` appears >= 1 times | PASS (5 occurrences) |
| A3 | `codex exec` does NOT appear | PASS (0 occurrences) |
| A4 | `--json` flag documented | PASS (1 occurrence) |
| A5 | Failure path documented (fail/timed_out/error) | PASS (8 occurrences) |
| A6 | Prompts for task when no args; presents result summary | PASS |

---

## User-Facing Behaviour Check (A6 Detail)

**No-argument prompt path:**

The skill explicitly instructs: ask the user "What task should I dispatch?" and wait. If the response is empty, ask again. This correctly gates on user input before proceeding.

**Result summary after dispatch:**

The success outcome block in Step 3 presents exactly the four fields required by the contract:

- Status — `.status` field
- Duration — derived from `.duration_ms`, formatted human-readable
- Files written — `.activity.files_changed`, falls back to "none reported"
- Output path — `.output`, falls back to "see above"

The failure outcome block presents status, error message (`.error.message`), and log path, with an explicit "The task did not complete" message and retry guidance.

**Correctness notes:**

- The skill correctly guards against both JSON parse failure and non-zero exit code as error conditions.
- `mktemp /tmp/dispatch-task.XXXXXX` is POSIX-portable (no `--suffix`, no GNU-only flags), satisfying the project's cross-platform shell requirement.
- Cleanup of the temp prompt file is unconditional (runs on success and failure).
- No `codex exec` call anywhere in the file; the prohibition is reinforced by an explicit prose instruction on line 11.

---

## Exemptions Applied

- Test presence gate: EXEMPT (pure .md skill file, no executable code)
- All other checks: N/A

---

## Conclusion

The dispatch skill satisfies all contract assertions. The implementation is clean, portable, and correctly handles both the happy path and error paths. No issues found.
