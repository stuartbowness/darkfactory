Implemented and committed Task 5.1.

Commit: `1c3d447ac3a2c034063f2c8bb1229a07116fa1e9`

Changed:
- Added check 14 “Test quality review” to [scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:246)
- Added the output table row for `Test quality review`
- Updated [README.md](/Users/stuart/.darkfactory/README.md:103)
- Added regression test [test-task-5.1.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-5.1.sh:1)
- Wrote handoff at [task-5.1.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-5.1.md:1)

Verification:
- Red test failed before implementation as expected
- New test passes
- All `scrutiny-test-health` mission prompt tests pass
- `openspec validate scrutiny-test-health --strict` passes
- `git diff --check` passes

The handoff is post-commit so it can include the final SHA. Pre-existing dirty/untracked files were left untouched.