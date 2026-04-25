Blocked by sandbox permissions on `.codex`.

What completed:
- Ran `openspec update --force` with `XDG_CONFIG_HOME` inside the repo.
- Regenerated the four `.claude/skills/openspec-*/SKILL.md` files to `generatedBy: "1.3.1"`.
- Removed temporary `.config` files and reverted out-of-scope OpenSpec command diffs.
- Wrote the handoff report: [task-2.md](/Users/stuart/.darkfactory/openspec/changes/openspec-v1-3-1-alignment/mission/handoffs/task-2.md)

What failed:
- OpenSpec and manual `cp` both failed to write under `.codex/skills/...` with `Operation not permitted`.
- Verification shows `.codex` files still have `generatedBy: "1.2.0"`.
- `diff` fails for all four Claude/Codex skill pairs.
- No commit was created because the required sync and verification did not pass.

Current working tree has the regenerated `.claude` skill files plus the untracked OpenSpec change directory.