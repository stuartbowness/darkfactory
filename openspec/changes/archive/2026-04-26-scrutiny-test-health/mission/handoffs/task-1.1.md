Implemented and committed Task 1.1.

Commit: `3e61ba593aac4186248a69b54e41790b3517bb3e`

Changed:
- Added `## Deterministic Pre-passes` before `## Check Sequence` in [.claude/skills/mission/prompts/scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:9)
- Added focused TDD shell test at [test-task-1.1.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-1.1.sh:1)
- Marked task 1.1 complete in [tasks.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/tasks.md:3)
- Wrote handoff to [task-1.1.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-1.1.md:1)

Verification run:
- `bash openspec/changes/scrutiny-test-health/mission/tests/test-task-1.1.sh` -> pass
- `openspec validate scrutiny-test-health --strict` -> pass
- `git diff --check` -> pass
- `git diff --cached --check` -> pass

Note: the handoff was written after the commit so it can contain the final commit SHA.