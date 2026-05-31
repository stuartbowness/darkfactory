Done.

Committed Task 3 as `6e3929b1f749b8e0ceac2f90a325fca0ce9a233d`.

Created:
- `skills/lint-audit/SKILL.md`
- `skills/lint-audit/RULES.md`
- `openspec/changes/lint-audit-skill/mission/handoffs/task-3.md`

Checks run:
- `skills/lint-audit/tests/validate-rules-json.sh` -> pass
- `skills/lint-audit/tests/audit-sh.sh` -> pass
- `jq -r '.oxlint_version' skills/lint-audit/rules.json` -> `1.67.0`
- `git diff --cached --check` -> pass before commit

I left unrelated existing dirty files alone: `UBIQUITOUS_LANGUAGE.md`, `skills/language/SKILL.md`, and the pre-existing untracked OpenSpec mission files.