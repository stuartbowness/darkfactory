Implemented and committed the Phase 8 Step 7 foundation audit.

Commit: `796084ab83f0fd345664a7b6a36bdf9f32a806f3`

Changed [SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md:697): added item `4a` between items 4 and 5. It is explicitly scoped to task 1 only and keeps item 5 numbered as requested.

Wrote handoff report to [task-6.md](/Users/stuart/.darkfactory/openspec/changes/deterministic-planning/mission/handoffs/task-6.md). It is uncommitted because it records the final commit SHA, matching the existing handoff pattern.

Checks run:
- `git diff --check -- skills/mission/SKILL.md`
- `git diff --check HEAD~1..HEAD`
- `git show --stat --oneline HEAD`

Existing unrelated dirty/untracked files were left untouched.