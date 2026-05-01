# Task 4 - Smoke test

- **Task:** Task 4 - Smoke test: verify terminology consistency
- **Commit SHA:** a9351bb5b2504baf00b5b65f2a8fe586260c0cc3
- **Approach:** Ran comprehensive grep verification across all in-scope files to confirm the "assertion" to "check" rename is complete and consistent. Classified every remaining "assertion" hit. Verified preserved files are unchanged. Searched for awkward phrasings introduced by the rename, specifically doubled "check" usage.
- **Commands run:**
  - `grep -rni "assertion" skills/mission/SKILL.md skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md` -- exit 0, 3 text hits + 1 binary, all in code-test context
  - `grep -rn "check [0-9]" skills/mission/prompts/scrutiny-validator.md` -- exit 1 (zero hits, as expected)
  - `grep -ni "assertion" skills/test-quality/SKILL.md` -- exit 0, 6 hits (code-test context, preserved correctly)
  - `grep -ni "assertion" skills/tdd/tests.md` -- exit 0, 1 hit ("one logical assertion per test", preserved correctly)
  - `grep -ni "assertion" skills/prd/SKILL.md` -- exit 0, 1 hit (general English usage, preserved correctly)
  - `grep -rni "check.*check" <all in-scope files>` -- exit 0, multiple hits analyzed for awkward phrasing
  - `grep -rni "check each check\|check the check\|..." <all in-scope files>` -- exit 0, 3 hits found
- **Concerns:** Two instances of awkward doubled "check" phrasing found (low severity):
  1. `skills/mission/SKILL.md:351` -- "should be able to check every check without guessing"
  2. `skills/mission/prompts/usertest-validator.md:19` -- "Check each contract check:"

  Both are clear in meaning but read somewhat clunky. Flagged as warnings in the smoke test report for optional cleanup. Neither blocks acceptance.
