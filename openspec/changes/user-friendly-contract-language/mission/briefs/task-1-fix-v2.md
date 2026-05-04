# Fix Brief: Task 1 - Rename scrutiny validator steps (Attempt 2)

## What Failed
The report template table in `skills/mission/prompts/scrutiny-validator.md` uses bare descriptive names for each row instead of numbered gate labels. The table rows say "Test presence", "Build", etc. but should say "Gate 1: Test presence", "Gate 2: Build", etc.

## Diagnostic Triage
Triage unavailable -- the issue is straightforward: the report template table rows need "Gate N:" prefixes added.

## Validator Output
The report template (around lines 273-286 of `skills/mission/prompts/scrutiny-validator.md`) currently looks like:

```
| Test presence | PASS/FAIL | ... |
| Build | PASS/FAIL | ... |
| Tests | PASS/FAIL | ... |
```

It should look like:

```
| Gate 1: Test presence | PASS/FAIL | ... |
| Gate 2: Build | PASS/FAIL | ... |
| Gate 3: Tests | PASS/FAIL | ... |
```

There are 14 gates total. The full mapping:
1. Test presence
2. Build
3. Tests
4. Lint
5. Code review
6. Module depth
7. Code standards
8. Code simplify
9. Security review
10. Contract checks
11. Behavioral conformance
12. Silent skip detection
13. Diff coverage
14. Test quality review

Also note: "Contract assertions" in the template (gate 10) should remain as-is for now -- that rename happens in Task 3, not this fix.

## Commit to Fix
5a6e6184246b7d318acfa06718fb9e123d44f5c1

## Procedures
- Update the report template table rows in `skills/mission/prompts/scrutiny-validator.md` to add "Gate N:" prefix to each row
- Do not change anything else
- Commit as a single commit with message: "[task-1] fix: add gate numbering to report template table rows"
- Write handoff report to openspec/changes/user-friendly-contract-language/mission/handoffs/task-1-fix-v2.md

## Handoff Report Format
- Task: Task 1 - Rename scrutiny validator steps (Fix attempt 2)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
