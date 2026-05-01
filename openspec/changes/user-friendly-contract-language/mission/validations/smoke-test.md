# Smoke Test: Terminology Consistency

Date: 2026-04-29

## 1. "assertion" hits in in-scope files

Command: `grep -rni "assertion" skills/mission/SKILL.md skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md`

| File | Line | Context | Classification |
|------|------|---------|----------------|
| `skills/mission/prompts/scrutiny-validator.md` | 113 | "zero new test assertions" | Code-test context (test presence gate) -- CORRECT |
| `skills/mission/prompts/scrutiny-validator.md` | 251 | "tautological assertions" | Code-test context (test quality review) -- CORRECT |
| `skills/mission/prompts/scrutiny-validator.md` | 257 | "tautological assertion in test_list_open" | Code-test context (example output) -- CORRECT |
| `docs/architecture.png` | binary | Binary file match | Binary image -- NOT APPLICABLE |

**Result: PASS** -- All 3 text hits are in code-test context. No stale contract-assertion references remain.

## 2. "check [0-9]" in scrutiny-validator.md

Command: `grep -rn "check [0-9]" skills/mission/prompts/scrutiny-validator.md`

**Result: PASS** -- Zero hits. Exit code 1 (no matches). The old "check 1", "check 2" numbering style is gone.

## 3. Preserved files unchanged

| File | Expected | Found | Status |
|------|----------|-------|--------|
| `skills/test-quality/SKILL.md` | Code-test assertion references | 6 hits (assertion quality, tautological assertions, etc.) | PASS |
| `skills/tdd/tests.md` | "one logical assertion per test" | 1 hit at line 23 | PASS |
| `skills/prd/SKILL.md` | General English usage | 1 hit at line 11 ("verify their assertions") | PASS |

**Result: PASS** -- Preserved files retain their original "assertion" usage untouched.

## 4. Awkward phrasing check

Searched for doubled "check" patterns (`check.*check`) across all in-scope files.

### Findings

**4a. Awkward doubled usage (2 instances):**

1. **`skills/mission/SKILL.md:351`** -- "A validator reading only the contract and the diff should be able to check every check without guessing."
   - Verb "check" immediately followed by noun "check". Reads awkwardly.
   - Severity: Low. The meaning is clear, but the phrasing is clunky.

2. **`skills/mission/prompts/usertest-validator.md:19`** -- "Check each contract check:"
   - Verb "Check" immediately followed by noun "check". Classic doubled usage.
   - Severity: Low. The meaning is clear, but reads like a stutter.

**4b. Acceptable "check...check" patterns (not awkward):**

- `skills/mission/SKILL.md:252` -- "shape re-check so the re-check does not flag" -- "re-check" is a compound noun used consistently; reads naturally.
- `skills/mission/SKILL.md:353` -- "put the check on the task where it can first be checked" -- verb and noun separated by enough distance; reads naturally.
- `skills/mission/SKILL.md:367` -- "how a validator should check it" -- refers to a single check; reads naturally.
- `skills/mission/SKILL.md:372` -- "Cap `[inspection]` checks at..." -- no doubled usage.
- `skills/mission/SKILL.md:395-397` -- "confirm the check describes..." -- reads naturally; verb is "confirm", not "check".
- `skills/mission/prompts/scrutiny-validator.md:182` -- "Mark each check PASS, FAIL, or SKIP" -- no doubled usage.
- `skills/mission/prompts/scrutiny-validator.md:185-188` -- "CLI flag check: run the binary with the flag, check output" -- the first "check" is a noun (heading), the second is a verb in the instruction; reads naturally due to the colon separator.
- `README.md:96` -- "a thorough quality check that runs a sequence of checks" -- two different senses (singular quality check vs. plural item checks); reads naturally.
- `README.md:135` -- "validators check the work...a thorough quality check" -- verb then noun; separated by distance; reads naturally.

**Result: WARN** -- 2 instances of awkward doubled "check" phrasing found. Both are low severity (meaning is clear). Flagged for optional cleanup.

## Summary

| Verification | Result |
|--------------|--------|
| No stale "assertion" in contract/mission context | PASS |
| No "check [0-9]" numbered style | PASS |
| Preserved files unchanged | PASS |
| No awkward phrasing | WARN (2 minor instances) |

**Overall: PASS with warnings.** The terminology rename is complete and consistent. Two instances of doubled "check" phrasing exist but do not impair readability.
