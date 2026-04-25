## Why

Darkfactory was built against OpenSpec v1.2.0. We upgraded the CLI to v1.3.1. Two bugs in darkfactory-init now block new project initialization due to CLI behavior changes, the generated openspec-* skill files carry stale v1.2.0 metadata and content, and the README misattributes where `/opsx:archive` comes from.

## What Changes

- Fix darkfactory-init parallel Bash cancellation: add implementation note about safe exit-code patterns (`test -f ... || echo missing`, `cmd || echo NOT_INSTALLED`) so non-zero exits do not cancel sibling parallel Bash calls in Claude Code.
- Fix darkfactory-init openspec init: add `--force` flag for non-interactive legacy file cleanup.
- Regenerate openspec-* skill files from v1.3.1 using `openspec update --force`, replacing stale v1.2.0-generated content in both `.claude/skills/` and `.codex/skills/`.
- Update README skill table: change `/opsx:archive` source from "Installed by `openspec init`" to "Bundled with Darkfactory".

## Capabilities

### New Capabilities

(None -- this change fixes and updates existing functionality.)

### Modified Capabilities

- `init-checks`: Darkfactory-init skill's file-existence checks and CLI version checks must use safe Bash patterns, and `openspec init` must include the `--force` flag.
- `skill-regeneration`: All eight openspec-* skill files (4 in `.claude/skills/`, 4 in `.codex/skills/`) must be regenerated from v1.3.1 and kept in sync.
- `documentation-accuracy`: README skill table must accurately describe where each tool/command comes from.

## Impact

- `.claude/skills/darkfactory-init/SKILL.md` -- behavior fix
- `.claude/skills/openspec-{propose,apply-change,archive-change,explore}/SKILL.md` -- regenerated content
- `.codex/skills/openspec-{propose,apply-change,archive-change,explore}/SKILL.md` -- regenerated content (copies)
- `README.md` -- one line in the skill table
- No application code, no schema changes, no API changes
