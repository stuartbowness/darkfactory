# User-Test Validation: Task 1.1 -- Fix darkfactory-init skill

**Commit:** `08627c71bb33c6c69b6200c248851b3ae179491e`
**Validator:** user-test (blind -- no access to proposal or worker reasoning)
**Date:** 2026-04-23

## Test Strategy

This is an infrastructure-only change to a markdown skill file. Validation is done by reading the file content directly and running grep checks against the contract assertions. No runtime behavior to test -- the assertions are purely about file contents.

## Tests Run

| # | Test | Method | Result |
|---|------|--------|--------|
| 1 | SKILL.md contains `test -f` pattern for file-existence checks | `grep -n 'test -f' SKILL.md` | Found at line 71: `test -f <path> && echo "found" \|\| echo "missing"` |
| 2 | SKILL.md contains `\|\| echo "NOT_INSTALLED"` pattern for CLI version checks | `grep -n 'NOT_INSTALLED' SKILL.md` | Found at line 98: `command --version 2>/dev/null \|\| echo "NOT_INSTALLED"` |
| 3 | SKILL.md contains `openspec init --tools claude,codex --force` | `grep -n 'openspec init --tools claude,codex' SKILL.md` | Found at line 81 with `--force` flag present |
| 4 | SKILL.md does NOT contain bare `openspec init --tools claude,codex` without `--force` | `grep 'openspec init' SKILL.md \| grep -v '--force'` | Zero matches (exit code 1) -- no bare occurrence exists |

## Contract Assertions

| # | Assertion | Status |
|---|-----------|--------|
| 1 | SKILL.md contains a note instructing the agent to use `test -f` patterns (or equivalent always-exit-0 patterns) for file-existence checks | PASS |
| 2 | SKILL.md contains a note instructing the agent to use `\|\| echo "NOT_INSTALLED"` patterns (or equivalent always-exit-0 patterns) for CLI version checks | PASS |
| 3 | SKILL.md contains `openspec init --tools claude,codex --force` (with the --force flag) | PASS |
| 4 | SKILL.md does NOT contain `openspec init --tools claude,codex` without the --force flag | PASS |

## Verdict

**PASS** -- All 4 contract assertions satisfied.

The commit adds two implementation notes (lines 71 and 98) instructing the agent to use always-exit-0 patterns for parallel Bash calls, and updates the openspec init command on line 81 to include the `--force` flag. The old bare command without `--force` is fully replaced.
