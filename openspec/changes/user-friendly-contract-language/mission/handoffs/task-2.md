Implemented Task 2.

Renamed contract-item `assertion/assertions` language to `check/checks` in [skills/mission/SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md). Committed as:

`8d877919e4b541d3d2cb401ef99fa9b2aa2c66b1`

Wrote the handoff report to [task-2.md](/Users/stuart/.darkfactory/openspec/changes/user-friendly-contract-language/mission/handoffs/task-2.md).

Verification run:
- `grep -ni "assertion" skills/mission/SKILL.md` returned no matches
- `grep -ni "assert" skills/mission/SKILL.md` returned no matches
- `git diff --check -- skills/mission/SKILL.md` passed

I left the existing unrelated `UBIQUITOUS_LANGUAGE.md` change and untracked mission tree untouched, except for the required Task 2 handoff file.