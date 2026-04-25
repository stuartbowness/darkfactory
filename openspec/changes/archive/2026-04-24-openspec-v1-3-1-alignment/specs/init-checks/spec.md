## ADDED Requirements

### Requirement: Safe file-existence checks
The darkfactory-init skill SHALL instruct the agent to use Bash patterns that always exit 0 when checking whether optional files exist. This prevents non-zero exit codes from cancelling sibling parallel Bash calls in Claude Code.

#### Scenario: Optional skill file missing
- **WHEN** darkfactory-init checks for an optional skill file that does not exist (e.g., `~/.claude/skills/security-review/SKILL.md`)
- **THEN** the check exits 0 and reports "missing" without cancelling other parallel checks

#### Scenario: Optional skill file present
- **WHEN** darkfactory-init checks for an optional skill file that exists
- **THEN** the check exits 0 and reports "found"

#### Scenario: CLI tool not installed
- **WHEN** darkfactory-init checks the version of a CLI tool that is not installed (e.g., `codex`, `gh`, `openspec`)
- **THEN** the check exits 0 and reports "NOT_INSTALLED" without cancelling other parallel checks

### Requirement: Non-interactive openspec init
The darkfactory-init skill SHALL pass the `--force` flag to `openspec init` so that legacy file cleanup happens automatically without an interactive prompt.

#### Scenario: Project with legacy OpenSpec files
- **WHEN** darkfactory-init runs `openspec init` in a project that has legacy OpenSpec files from a previous version
- **THEN** the command cleans up legacy files automatically and completes without prompting for user input

#### Scenario: Fresh project with no legacy files
- **WHEN** darkfactory-init runs `openspec init --force` in a project with no legacy files
- **THEN** the command initializes normally (the `--force` flag has no negative effect)
