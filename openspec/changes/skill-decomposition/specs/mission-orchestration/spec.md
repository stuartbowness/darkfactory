## MODIFIED Requirements

### Requirement: scrutiny-validator.md references updated skill name
The scrutiny validator's check 14 (test quality review) SHALL invoke `/test-quality`, not `/test-quality-review`.

#### Scenario: /test-quality-review reference replaced with /test-quality
- **WHEN** `skills/mission/prompts/scrutiny-validator.md` is inspected
- **THEN** it contains no reference to `/test-quality-review`
- **AND** check 14 references `/test-quality` wherever it invokes the test quality skill

### Requirement: No other mission behavior changes
The mission SKILL.md SHALL remain otherwise unchanged: same phases, same rules, same 825-line monolithic structure.

#### Scenario: Mission file is not internally decomposed
- **WHEN** `skills/mission/SKILL.md` is inspected
- **THEN** all phases (1 through 9) are present in the single file
- **AND** the file does not delegate phases to separate skill files via sub-skill invocations

### Requirement: Mission templates and prompts remain nested
Template and prompt files SHALL stay under `skills/mission/templates/` and `skills/mission/prompts/` respectively.

#### Scenario: Templates nested under skills/mission/
- **WHEN** inspecting `skills/mission/templates/`
- **THEN** all template files from the original `.claude/skills/mission/templates/` exist there

#### Scenario: Prompts nested under skills/mission/
- **WHEN** inspecting `skills/mission/prompts/`
- **THEN** all prompt files from the original `.claude/skills/mission/prompts/` exist there
