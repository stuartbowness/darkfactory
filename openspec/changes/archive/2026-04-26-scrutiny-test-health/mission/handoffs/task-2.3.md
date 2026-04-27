Implemented Task 2.3 and committed it.

Commit: `aeb7e4da271529dacbd0e99a22be8af2fb0d0b9e`

Changed:
- Updated [.claude/skills/mission/prompts/scrutiny-validator.md](/Users/stuart/.darkfactory/.claude/skills/mission/prompts/scrutiny-validator.md:159) so check 7 reads `Error handling smells`, applies the requested KLOC thresholds, excludes test files, and requires file:line remediation guidance.
- Added [test-task-2.3.sh](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/tests/test-task-2.3.sh:1) using the requested TDD flow.
- Wrote the handoff at [task-2.3.md](/Users/stuart/.darkfactory/openspec/changes/scrutiny-test-health/mission/handoffs/task-2.3.md:1).

Verification run:
- `test-task-2.3.sh` failed before implementation, then passed after.
- `test-task-1.1.sh`, `test-task-2.1.sh`, `test-task-2.2.sh`, and `test-task-2.3.sh` all pass.
- `git diff --cached --check` passed before commit.

Note: the repo already had unrelated dirty/untracked files, including `UBIQUITOUS_LANGUAGE.md` and existing OpenSpec mission files. I only staged the prompt and new test for the commit.