Implemented and committed Task 2.1.

Commit: `50e518958b166dceeb341abf651331ff0d73b08b`

Changed [scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:114) to make check 3 parse and fill `Test runner`, report structured test counts, group common failure messages, and warn when fewer than 90% of discovered tests actually ran.

Added the TDD shell test at [test-task-2.1.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-2.1.sh:1).

Wrote the handoff to [task-2.1.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-2.1.md:1).

Verification run:
- `test-task-2.1.sh` failed before implementation, then passed after
- `test-task-1.1.sh` passed
- `openspec validate scrutiny-test-health --strict` passed
- `git diff --check` and `git diff --cached --check` passed

Note: the handoff is written after the commit so it can include the final SHA, matching the prior task pattern.