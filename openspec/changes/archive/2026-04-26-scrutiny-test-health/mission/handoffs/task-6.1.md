Implemented and committed Task 6.1.

Commit: `4bd9b7aaa80df29434b8997a4f2df4c5a200707c`

Changed:
- Updated [SKILL.md](/Users/stuart/.darkfactory/.claude/skills/darkfactory-init/SKILL.md) with `/test-quality-review` checks and coverage tool verification.
- Added TDD regression test [test-task-6.1.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-6.1.sh).
- Wrote handoff report at [task-6.1.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-6.1.md).

Verification:
- Red test failed before implementation as expected.
- New test passes.
- All `scrutiny-test-health` mission tests pass.
- `openspec validate scrutiny-test-health --strict` passes.
- `git diff --check` passes.

Note: the handoff report is post-commit so it can include the final SHA, matching the existing handoff pattern. Pre-existing dirty and untracked files were left untouched.