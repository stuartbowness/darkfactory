Implemented and committed the validator update.

Commit: `43d68103f290813bc3b2a3b047e93b3cbf903d4f`

Changed [scrutiny-validator.md](/Users/stuart/.darkfactory/skills/mission/prompts/scrutiny-validator.md:113):
- Gate 1 is now methodology-aware for `/tdd` and `/frontend`.
- Added new Gate 7: Frontend depth review.
- Renumbered old gates 7-14 to 8-15.
- Updated the output table through Gate 15.

Wrote the handoff report at [task-2.md](/Users/stuart/.darkfactory/openspec/changes/frontend-architecture-skill/mission/handoffs/task-2.md:1). It is uncommitted because it needed the final commit SHA.

Checks run:
- Red text check before edit: expected fail.
- Sequential gate numbering assertion: pass.
- Required text assertions with `rg`: pass.
- `test -f skills/frontend/DEEP-COMPONENTS.md`: pass.
- `git diff --check`: pass after removing one trailing space.

Skipped full build/tests because this was a markdown-only validator prompt change.