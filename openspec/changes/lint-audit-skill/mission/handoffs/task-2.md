Implemented and committed Task 2.

Commit: `3057c01d02d80fa9228693938ddc32cb02535c4c`

Changed:
- [audit.sh](/Users/stuart/.darkfactory/skills/lint-audit/audit.sh)
- [audit-sh.sh](/Users/stuart/.darkfactory/skills/lint-audit/tests/audit-sh.sh)
- [task-2.md](/Users/stuart/.darkfactory/openspec/changes/lint-audit-skill/mission/handoffs/task-2.md) written after commit so it can name the final SHA, matching the task-1 pattern

Checks run:
- `skills/lint-audit/tests/audit-sh.sh` -> pass
- `sh -n skills/lint-audit/audit.sh` -> pass
- `sh -n skills/lint-audit/tests/audit-sh.sh` -> pass
- `skills/lint-audit/tests/validate-rules-json.sh` -> pass
- `git diff --cached --check` -> pass before commit

I left unrelated existing changes alone: `UBIQUITOUS_LANGUAGE.md`, `skills/language/SKILL.md`, and the pre-existing untracked OpenSpec folder content.