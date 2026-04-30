Implemented and committed Task 4.1.

Commit: `0195f82d883c2d7f09f8354d88944425b60a98ac`

Changed:
- [SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md): added Phase 4.3a, updated Phase Index, rewrote Phase 4.4 task quality standard, and added Task Topology metadata rules.
- [config.yaml](/Users/stuart/.darkfactory/openspec/config.yaml): updated `rules.tasks` to match the strategy-aware task standard.

Handoff written to [task-4.md](/Users/stuart/.darkfactory/openspec/changes/deterministic-planning/mission/handoffs/task-4.md). It is uncommitted so it can record the final commit SHA, matching the existing Task 3 handoff convention.

Checks run:
- `git diff --check`
- YAML parse via Ruby
- `rg` verification for the new phase, topology fields, and updated task standard
- `git show --stat --oneline HEAD`

The worktree still has pre-existing unrelated dirty/untracked files; I left them untouched.