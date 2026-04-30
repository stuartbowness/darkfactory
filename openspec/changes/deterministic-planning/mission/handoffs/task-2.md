Implemented Task 2.1.

Committed the scoped template/SKILL changes:

`c152c840bc16907e24c0439e58c23ec9e667c554`

Changed:
- [contract.md](/Users/stuart/.darkfactory/skills/mission/templates/contract.md)
- [SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md)

Wrote the handoff report to [task-2.md](/Users/stuart/.darkfactory/openspec/changes/deterministic-planning/mission/handoffs/task-2.md). It is uncommitted so it can record the final commit SHA, matching the existing Task 1.1 handoff convention.

Checks run:
- `rg` checks for old `PRD Requirements` references
- `git diff --check`
- `git show --stat HEAD`

Skipped runtime tests because this was a docs/template-only change.