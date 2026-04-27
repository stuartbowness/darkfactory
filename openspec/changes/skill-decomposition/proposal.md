## Why

Darkfactory's skills are scattered across three locations, require two repo clones and 8+ manual symlinks to install, and include duplicate files. External skills are forked copies we can't improve. There are no standalone skills for adversarial review, codex dispatch, or OpenSpec planning outside a mission.

## What Changes

- Move all skills from .claude/skills/ to skills/ at repo root
- Fork 6 external skills (grill-me, write-a-prd, ubiquitous-language, tdd, test-quality-review, security-review) into the repo
- Create 2 new standalone skills: /adversarial, /dispatch
- Create install.sh that symlinks skills to ~/.claude/skills/
- Delete .codex/skills/ duplicates
- Simplify /mission-init to verify symlinks instead of checking multiple locations
- Update README with 3-command install

## Capabilities

### New Capabilities

- `adversarial-review`: Dual-agent (Claude + Codex) parallel code review as a standalone skill
- `dispatch-skill`: Skill wrapper for the darkfactory-dispatch binary
- `install-script`: One-command skill installation via install.sh

### Modified Capabilities

- `setup-verification`: mission-init simplified to verify symlinks and CLI tools
- `skill-layout`: All skills moved from .claude/skills/ to skills/ at repo root

## Impact

- New directory: skills/ (12 skill directories)
- New file: install.sh
- Modified: README.md (install instructions)
- No changes to tools/darkfactory-dispatch/ or openspec/
