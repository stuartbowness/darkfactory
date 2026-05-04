# Scrutiny Validation: Task 4

## Result: FAIL

## Gate Results
| Gate | Result | Notes |
|------|--------|-------|
| 1. Test presence | N/A | Closing task -- verification report only, no testable code |
| 2. Build | N/A | No application code |
| 3. Tests | N/A | No application code |
| 4. Lint | N/A | No application code |
| 5. Code review | PASS | Smoke test report ran all required verification steps; handoff documents commands and results |
| 6. Module depth | N/A | No application code |
| 7. Code standards | N/A | No application code |
| 8. Simplify | N/A | No application code |
| 9. Security | N/A | No application code |
| 10. Contract checks | FAIL | Check 5 fails -- awkward phrasings exist in modified files |
| 11. Behavioral conformance | FAIL | Check 5 commands confirm the awkward phrasings exist |
| 12. Silent skip | N/A | Skipped per instructions |
| 13. Diff coverage | N/A | Skipped per instructions |
| 14. Test quality | N/A | Skipped per instructions |

## Contract Checks
| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | Every "assertion" hit in in-scope files is in code-test context | PASS | execution | 3 text hits + 1 binary. scrutiny-validator.md:113 ("zero new test assertions"), :251 ("tautological assertions"), :257 ("tautological assertion in test_list_open") -- all code-test context. docs/architecture.png is binary, N/A. |
| 2 | `grep -rn "check [0-9]" skills/mission/prompts/scrutiny-validator.md` returns zero hits | PASS | execution | Exit code 1, zero matches. No "check 1", "check 2" numbered patterns. |
| 3 | `skills/test-quality/SKILL.md` contains "assertion quality" and "tautological assertion" unchanged | PASS | execution | 6 hits: line 4 "assertion quality", line 23 "Assertion quality (P0 if tautological)", line 29/33/38 usage examples, line 89 "Tautological assertion in test_list_open". All code-test references, unchanged. |
| 4 | `skills/tdd/tests.md` contains "one logical assertion per test" unchanged | PASS | execution | 1 hit at line 23: "One logical assertion per test". Unchanged. |
| 5 | No awkward phrasings like "check each check" exist in any modified file | **FAIL** | execution | Two instances found in files modified by this mission: (1) `skills/mission/SKILL.md:351` -- "should be able to check every check without guessing" (2) `skills/mission/prompts/usertest-validator.md:19` -- "Check each contract check:" Both are verb-noun doubled "check" constructions. |

## Failed Checks

### Check 5: Awkward phrasings in modified files

The contract states: "No awkward phrasings like 'check each check' exist in any modified file -- verb forms use 'verify' or 'mark' instead."

Two instances exist:

1. **`skills/mission/SKILL.md:351`** -- "A validator reading only the contract and the diff should be able to check every check without guessing."
   - Modified in task-2 commit `8d87791`.
   - Fix: "...should be able to verify every check without guessing."

2. **`skills/mission/prompts/usertest-validator.md:19`** -- "Check each contract check:"
   - Modified in task-3 commit `99037c9`.
   - Fix: "Verify each contract check:" or "Mark each contract check:"

The smoke test correctly identified these instances but classified them as "WARN" rather than recognizing they violate the contract. The contract check is unambiguous: these phrasings must not exist. The smoke test's own findings are the evidence of failure.

## Recommendations

1. Fix the two awkward phrasings by replacing the verb "check" with "verify" in both instances:
   - `skills/mission/SKILL.md:351`: "...should be able to **verify** every check without guessing."
   - `skills/mission/prompts/usertest-validator.md:19`: "**Verify** each contract check:"
2. Re-run the smoke test to confirm no other doubled "check" patterns remain.
