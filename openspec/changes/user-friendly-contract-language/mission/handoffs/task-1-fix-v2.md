Implemented and committed the fix.

Commit: `0ae81ca18db7c04f1a0d2d0f4cd6ece9fd5a2226`

Changed [scrutiny-validator.md](/Users/stuart/.darkfactory/skills/mission/prompts/scrutiny-validator.md:273) so all 14 report template rows now use `Gate N:` labels. `Gate 10: Contract assertions` was left as requested.

Wrote the handoff to [task-1-fix-v2.md](/Users/stuart/.darkfactory/openspec/changes/user-friendly-contract-language/mission/handoffs/task-1-fix-v2.md:1).

Checks run:
- `git diff --check -- skills/mission/prompts/scrutiny-validator.md` -> exit 0
- `grep -n "| Gate [0-9]" skills/mission/prompts/scrutiny-validator.md` -> exit 0
- `git log -1 --oneline --stat` -> exit 0

Note: `UBIQUITOUS_LANGUAGE.md` was already modified and was not touched. The handoff file was written after the commit so it could include the real commit SHA.