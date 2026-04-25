## ADDED Requirements

### Requirement: Skills regenerated from current CLI version
All openspec-generated skill files SHALL be regenerated from the installed OpenSpec CLI version (v1.3.1) using `openspec update --force`, replacing stale content from v1.2.0.

#### Scenario: Regeneration updates metadata
- **WHEN** `openspec update --force` runs in the Darkfactory directory
- **THEN** all four `.claude/skills/openspec-*/SKILL.md` files have `generatedBy: "1.3.1"` in their frontmatter

#### Scenario: Regeneration updates content
- **WHEN** `openspec update --force` runs in the Darkfactory directory
- **THEN** the skill file content matches what OpenSpec v1.3.1 generates (any v1.2.0-specific text is replaced)

### Requirement: Codex skill copies stay in sync
The `.codex/skills/openspec-*/SKILL.md` files SHALL be identical to their `.claude/skills/openspec-*/SKILL.md` counterparts after regeneration.

#### Scenario: Codex copies match Claude copies
- **WHEN** skill regeneration is complete
- **THEN** `diff .claude/skills/openspec-<name>/SKILL.md .codex/skills/openspec-<name>/SKILL.md` returns no differences for all four skills (propose, apply-change, archive-change, explore)
