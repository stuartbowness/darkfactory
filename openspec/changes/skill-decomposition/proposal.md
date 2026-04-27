## Why

Darkfactory's skills are scattered across three locations, require two repo clones and 8+ manual symlinks to install, and include duplicate files. External skills are forked copies we can't improve. There are no standalone skills for adversarial review, codex dispatch, or OpenSpec planning outside a mission.

## What Changes

- Move all skills from .claude/skills/ to skills/ at repo root with bare names
- Fork 5 external skills (grill-me, write-a-prd, ubiquitous-language, tdd, security-review) into the repo
- Create 3 new standalone skills: /adversarial, /dispatch, /plan
- Create install.sh that symlinks skills to ~/.claude/skills/
- Update mission SKILL.md to reference new skill names
- Delete .codex/skills/ duplicates
- Simplify /mission-init to verify symlinks instead of checking multiple locations
- Update README with 3-command install

## Capabilities

### New Capabilities

- `adversarial-review`: Dual-agent (Claude + Codex) parallel code review as a standalone skill
- `dispatch-skill`: Skill wrapper for the darkfactory-dispatch binary
- `standalone-plan`: OpenSpec artifact creation independent of a mission
- `install-script`: One-command skill installation via install.sh

### Modified Capabilities

- `mission-orchestration`: Mission SKILL.md references updated from /grill-me to /grill, /write-a-prd to /prd, /ubiquitous-language to /language
- `setup-verification`: mission-init simplified to verify symlinks and CLI tools
- `skill-layout`: All skills moved from .claude/skills/ to skills/ at repo root

## Impact

- New directory: skills/ (12 skill directories)
- New file: install.sh
- Deleted: .claude/skills/ (entire directory)
- Deleted: .codex/skills/ (entire directory)
- Modified: README.md (install instructions)
- Modified: CLAUDE.md (if skill references exist)
- No changes to tools/darkfactory-dispatch/ or openspec/
