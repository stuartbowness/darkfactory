## Why

Darkfactory's ts-quality.sh hook enforces TypeScript patterns at edit time via grep, but it cannot catch type-system-level issues like floating promises, misused promises, or implicit `any` from missing annotations. Those require the compiler and linter to be configured with strict rules. Most projects (including Toyo) run oxlint with defaults and have incomplete tsconfig strictness, leaving an enforcement gap between what the hook catches and what the build catches.

## What Changes

- Add a new `/lint-audit` darkfactory skill that audits a TypeScript project's oxlint config and tsconfig.json strictness against an agent-friendly ruleset
- The skill detects the project's current linter setup, compares it against recommended rules, reports gaps, and offers to apply fixes
- Oxlint only -- projects using ESLint without oxlint are skipped (not our company's projects)
- Includes an executable `audit.sh` script for detection and comparison logic, enabling programmatic testing
- RULES.md reference file defining the recommended agent-friendly ruleset with oxlint canonical rule names

## Capabilities

### New Capabilities
- `project-detection`: Detect project linter (oxlint vs ESLint), oxlint config format (`.oxlintrc.json`, `.oxlintrc.jsonc`, `oxlint.config.ts`), and all tsconfig.json files in a monorepo
- `ruleset-audit`: Compare detected oxlint rules and tsconfig settings against the agent-friendly ruleset, producing a structured findings report with per-rule status
- `config-apply`: Apply recommended rules to oxlint config and tsconfig.json files, with per-item accept/deny and apply-all/skip-all options

### Modified Capabilities

(none)

## Impact

- New skill directory: `skills/lint-audit/` with SKILL.md, RULES.md, audit.sh
- install.sh will symlink the new skill to `~/.claude/skills/lint-audit`
- No changes to existing skills, mission flow, or scrutiny validator -- Gate 4 already runs the project's linter, so added rules are automatically enforced
- README.md and ARCH.md updated to reflect the new skill
