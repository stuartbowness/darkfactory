Implemented and committed the skill fix.

Commit: `08627c71bb33c6c69b6200c248851b3ae179491e`

Changed [SKILL.md](/Users/stuart/.darkfactory/.claude/skills/darkfactory-init/SKILL.md:71):
- Added safe Bash notes for parallel file and CLI checks.
- Updated `openspec init --tools claude,codex` to include `--force`.
- Added the legacy cleanup explanation.

Wrote the handoff to [task-1.md](/Users/stuart/.darkfactory/openspec/changes/openspec-v1-3-1-alignment/mission/handoffs/task-1.md:1).

Verification run:
- `rg` checks for the new notes and `--force`
- `git show --stat --oneline HEAD`

No automated tests were relevant for this Markdown-only change. The handoff file is post-commit so it can contain the final commit SHA.