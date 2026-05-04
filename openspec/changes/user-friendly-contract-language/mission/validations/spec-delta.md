# Spec Delta Check

## Result: CLEAN

All 13 coverage audit items verified present in the codebase.

## Coverage Verification

| # | PRD Requirement | Verified | Evidence |
|---|----------------|----------|----------|
| 1 | Progress log says "12 checks in contract" | YES | `skills/mission/SKILL.md` line 491: `<N> tasks, <M> checks in contract.` |
| 2 | PR body says "Total checks: 15" | YES | `skills/mission/templates/pr-body.md` line 22: `- Total checks: <M>` |
| 3 | Validator reports use "Contract Checks" and "Failed Checks" headings | YES | `skills/mission/prompts/scrutiny-validator.md` lines 288-294: `## Contract Checks` and `## Failed Checks (if any)` headings. `prompts/usertest-validator.md` lines 44-50: `## Contract Checks` and `## Failed Checks (if any)` headings. |
| 4 | Contract template uses "check" for items | YES | `skills/mission/templates/contract.md` lines 5-6, 10-11: items use `<check capturing specific spec scenario detail>` |
| 5 | Validator steps called "gates" in scrutiny prompt | YES | `skills/mission/prompts/scrutiny-validator.md`: `## Gate Sequence` heading (line 109), `## Gate Results` heading (line 270), table rows use `Gate 1:` through `Gate 14:` (lines 273-286). Zero matches for `check [0-9]` pattern. |
| 6 | Three-way split documented in UBIQUITOUS_LANGUAGE.md | YES | `UBIQUITOUS_LANGUAGE.md` line 183: `**"check" / "assertion" / "gate"** had overlapping usage... **Resolved** with a three-way split: a **Gate** is a numbered step in the scrutiny validator's sequence... A **Check** is a single verifiable statement in a contract. An **Assertion** is reserved for code test assertions only.` Also defined in tables: Check (line 17), Gate (line 28). |
| 7 | Consistent terminology in all prompts and templates | YES | Grep of `skills/mission/SKILL.md`, `skills/mission/prompts/`, and `skills/mission/templates/` for "assertion" returns only 3 hits, all in code-test contexts: test presence gate mentions "test assertions" (scrutiny-validator.md:113), P0 tautological assertions (scrutiny-validator.md:251, 257). No "assertion" used for contract items. |
| 8 | Validator subagent sees "Contract Checks for This Task" | YES | `skills/mission/prompts/scrutiny-validator.md` line 7: `## Contract Checks for This Task`. `prompts/usertest-validator.md` line 7: `## Contract Checks for This Task`. |
| 9 | "Assertion" preserved for code test concepts | YES | `skills/test-quality/SKILL.md`: "assertion quality" (line 23), "tautological assertions" (line 89), and assertion examples throughout. `skills/tdd/tests.md` line 23: "One logical assertion per test". Term is consistently used only for code test assertions. |
| 10 | Docs use "contract checks" consistently | YES | `docs/design-decisions.md` uses "contract checks" (lines 37, 44, 140, 142) and "validation checks" (line 84). No instances of "assertion" for contract items. |
| 11 | Launch dashboard says "total contract checks" | YES | `skills/mission/SKILL.md` line 485: `**Mission overview** -- change name, total tasks (feature + closing), total contract checks` |
| 12 | Fix brief says "Do NOT copy check text" | YES | `skills/mission/templates/worker-fix-brief.md` line 4: `Do NOT copy check text from the contract. Describe what went wrong, not what the contract expected.` |
| 13 | Mission-resume describes "checks grouped by task" | YES | `skills/mission-resume/SKILL.md` line 34: `| Contract | contract.md | Checks grouped by task for validators |` |

## Approved Deviations

None.

## Notes

- Item 10: `docs/design-decisions.md` line 84 uses "validation checks" once (`contract.md is a new OpenSpec artifact (validation checks, written during planning)`). This is describing the contract artifact's purpose, not naming a concept, so it does not conflict with the terminology standard. All other doc references use "contract checks" or "checks" consistently.
