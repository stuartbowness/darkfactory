Implemented and committed Task 1.

Commit: `5a6e6184246b7d318acfa06718fb9e123d44f5c1`

Changed:
- [scrutiny-validator.md](/Users/stuart/.darkfactory/skills/mission/prompts/scrutiny-validator.md)
- [SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md)

Handoff written:
- [task-1.md](/Users/stuart/.darkfactory/openspec/changes/user-friendly-contract-language/mission/handoffs/task-1.md)

Verification run:
- `grep -n "check [0-9]" skills/mission/prompts/scrutiny-validator.md` -> exit 1, zero matches
- `grep -n "Check Sequence\|Check Results\|check sequence" skills/mission/prompts/scrutiny-validator.md` -> exit 1, zero matches
- `git diff --check -- ...` -> exit 0

Note: `UBIQUITOUS_LANGUAGE.md` was already modified before my work and was not touched.