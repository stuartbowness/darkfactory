Implemented Task 6.

Created [SKILL.md](/Users/stuart/.darkfactory/skills/adversarial/SKILL.md) and committed it as:

`1ac6f1c791b331fcb85f44e3bd9fdb0104f06a6d`
`[task-6] write adversarial skill`

Wrote the handoff to [task-6.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/handoffs/task-6.md).

Verification run:
`name: adversarial`, `gh pr diff`, `darkfactory-dispatch`, `adversarial-codex.md`, and severity terms all pass the required grep checks. The skill also now explicitly uses `darkfactory-dispatch --async`.

I left the existing unrelated worktree changes untouched.