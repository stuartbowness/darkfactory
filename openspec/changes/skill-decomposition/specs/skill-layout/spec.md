## ADDED Requirements

### Requirement: Skills directory at repo root
All skills SHALL live in `skills/` at the Darkfactory repo root (`~/.darkfactory/skills/`), not in `.claude/skills/`.

#### Scenario: skills/ directory exists with correct structure
- **WHEN** a developer clones the Darkfactory repo
- **THEN** `~/.darkfactory/skills/` exists and contains exactly 12 subdirectories: `mission`, `grill`, `language`, `prd`, `adversarial`, `dispatch`, `tdd`, `test-quality`, `security`, `mission-resume`, `mission-status`, `mission-init`
- **AND** each subdirectory contains a `SKILL.md` file with valid frontmatter (name, description, user-invokable fields)

### Requirement: Legacy .claude/skills/ removed
The `.claude/skills/` directory inside the Darkfactory repo SHALL be deleted.

#### Scenario: No .claude/skills/ in repo
- **WHEN** the migration is complete
- **THEN** `~/.darkfactory/.claude/skills/` does not exist
- **AND** `git ls-files .claude/skills/` returns no output

### Requirement: .codex/skills/ duplicates removed
The `.codex/skills/` directory inside the Darkfactory repo SHALL be deleted.

#### Scenario: No .codex/skills/ in repo
- **WHEN** the migration is complete
- **THEN** `~/.darkfactory/.codex/skills/` does not exist
- **AND** `git ls-files .codex/skills/` returns no output

### Requirement: Forked skills present and self-contained
Six externally-sourced skills SHALL be forked into the repo as owned files, not symlinks.

#### Scenario: grill skill forked from grill-me
- **WHEN** inspecting `skills/grill/SKILL.md`
- **THEN** the file exists and is a regular file (not a symlink)
- **AND** the frontmatter name is `grill`
- **AND** the skill content matches the behavior of the original `grill-me` skill

#### Scenario: prd skill forked from write-a-prd
- **WHEN** inspecting `skills/prd/SKILL.md`
- **THEN** the file exists and is a regular file (not a symlink)
- **AND** the frontmatter name is `prd`

#### Scenario: language skill forked from ubiquitous-language
- **WHEN** inspecting `skills/language/SKILL.md`
- **THEN** the file exists and is a regular file (not a symlink)
- **AND** the frontmatter name is `language`

#### Scenario: tdd skill forked with companion files
- **WHEN** inspecting `skills/tdd/`
- **THEN** `SKILL.md` exists and is a regular file (not a symlink)
- **AND** all 5 companion files exist: `deep-modules.md`, `interface-design.md`, `mocking.md`, `refactoring.md`, `tests.md`
- **AND** the frontmatter name is `tdd`

#### Scenario: test-quality skill forked from test-quality-review
- **WHEN** inspecting `skills/test-quality/SKILL.md`
- **THEN** the file exists and is a regular file (not a symlink)
- **AND** the frontmatter name is `test-quality`
- **AND** the skill content matches the behavior of the original `test-quality-review` skill (except the frontmatter name field)

#### Scenario: security skill forked from security-review
- **WHEN** inspecting `skills/security/SKILL.md`
- **THEN** the file exists and is a regular file (not a symlink)
- **AND** the frontmatter name is `security`
- **AND** the skill contains no references to toyo-review or toyo-specific review sections

### Requirement: Mission-family skills moved to skills/
Three existing skills SHALL be moved to `skills/`. Two are moved unchanged; one is renamed.

#### Scenario: mission-resume skill moved
- **WHEN** inspecting `skills/mission-resume/SKILL.md`
- **THEN** the file exists with frontmatter name `mission-resume`
- **AND** the skill content is identical to the original `.claude/skills/mission-resume/SKILL.md`

#### Scenario: mission-status skill moved
- **WHEN** inspecting `skills/mission-status/SKILL.md`
- **THEN** the file exists with frontmatter name `mission-status`

#### Scenario: mission-init skill renamed from darkfactory-init
- **WHEN** inspecting `skills/mission-init/SKILL.md`
- **THEN** the file exists with frontmatter name `mission-init`

### Requirement: Mission skill preserved with updated references
The existing mission SKILL.md and its companion files SHALL be moved to `skills/mission/`.

#### Scenario: mission skill and companions moved
- **WHEN** inspecting `skills/mission/`
- **THEN** `SKILL.md` exists
- **AND** `templates/` subdirectory exists with all original template files
- **AND** `prompts/` subdirectory exists with all original prompt files
