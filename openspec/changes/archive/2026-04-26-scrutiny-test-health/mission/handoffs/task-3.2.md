Implemented Task 3.2 and committed code + test changes.

Commit: `130175f2ba743abdcb3a87cd5accb58ad1e17d46`

Changed:
- Added check 13 “Diff coverage” in [scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:221)
- Added the output table row for Diff coverage
- Added regression test [test-task-3.2.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-3.2.sh:1)
- Wrote handoff at [task-3.2.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-3.2.md:1)

Verification:
- Red test first: `test-task-3.2.sh` failed before implementation
- All mission prompt tests pass
- `git diff --check` passes
- `openspec validate scrutiny-test-health --strict` passes

Note: the handoff is post-commit and uncommitted so it can contain the final SHA, matching the existing mission pattern. Unrelated dirty/untracked files were left untouched.