Implemented Task 2.2 and committed the code/test changes.

Commit: `b1c48eaa0a5d76f4a8555b9c8f750be018220222`

Changed:
- [.claude/skills/mission/prompts/scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:134)
- [test-task-2.2.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-2.2.sh:1)

Wrote the handoff report to:
- [task-2.2.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-2.2.md:1)

Verification run:
- `test-task-2.2.sh` failed before implementation, then passed after
- `test-task-1.1.sh` passed
- `test-task-2.1.sh` passed
- `openspec validate scrutiny-test-health --strict` passed
- `git diff --check` and `git diff --cached --check` passed

I left the existing unrelated untracked/modified worktree files untouched.