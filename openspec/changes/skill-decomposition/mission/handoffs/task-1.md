Moved the three mission skills into repo-root `skills/` with `git mv` and committed the pure renames.

Commit:
`4f053d0ee9a58890324bae4e9c5a9625ad7b5480`

Handoff report:
[task-1.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/handoffs/task-1.md)

Verification run:
- `git diff --cached --summary` before commit showed 13 renames at `100%`
- `git ls-files .claude/skills/mission .claude/skills/mission-resume .claude/skills/mission-status` returns no output
- `skills/mission/SKILL.md` is a regular file, 825 lines, with phases 1 through 9
- Frontmatter for `mission-resume` and `mission-status` is intact

Note: the handoff report is written after the commit so it can contain the final commit SHA. Existing unrelated dirty worktree files were left untouched.