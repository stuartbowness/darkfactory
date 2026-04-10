## ADDED Requirements

### Requirement: Skill has valid SKILL.md frontmatter
The skill file SHALL have YAML frontmatter with name and description fields.

#### Scenario: Valid frontmatter
- **WHEN** the file is parsed
- **THEN** it has `name: mission` and a description that explains what the skill does

### Requirement: Skill covers all 8 phases
The skill SHALL include Phase 0 (plan mode), Phase 1 (grill), Phase 2 (ubiquitous language), Phase 3 (OpenSpec propose), Phase 4 (validation contract), Phase 5 (mission directory), Phase 6 (approval), Phase 7 (build loop), and Phase 8 (completion).

#### Scenario: All phases present
- **WHEN** the skill is read
- **THEN** each of the 8 phases is documented with clear instructions

### Requirement: Skill references all required skills
The skill SHALL reference: /grill-me, /ubiquitous-language, /opsx:propose, /review, /code-simplify, /security-review, /test-app, /demo-video, /pr, /opsx:archive.

#### Scenario: All skills referenced
- **WHEN** the skill is read
- **THEN** each of the 10 skills listed above appears in the instructions

### Requirement: Build phase includes Monitor-based worker dispatch
The skill SHALL include a Monitor script for dispatching Codex workers and watching for handoff completion.

#### Scenario: Monitor script present
- **WHEN** the build phase is read
- **THEN** it includes a bash script using Monitor to kick off codex exec and watch for the handoff file

### Requirement: Build phase includes scrutiny and user-test validation
The skill SHALL spawn both validator types sequentially after each worker completes.

#### Scenario: Both validators present
- **WHEN** the build phase is read
- **THEN** it includes instructions for spawning a scrutiny validator and a user-test validator with their full prompts

### Requirement: Build phase includes fix loop with circuit breaker
The skill SHALL handle validation failures with a fix loop that retries up to 3 times.

#### Scenario: Fix loop present
- **WHEN** validation fails
- **THEN** the skill documents assembling a fix brief, dispatching a fix worker, re-running both validators, and breaking after 3 attempts

### Requirement: Skill uses ubiquitous language terms correctly
The skill SHALL use terms from UBIQUITOUS_LANGUAGE.md consistently (Mission, Orchestrator, Worker, Validator, Contract, Brief, Handoff, etc.).

#### Scenario: Terms used correctly
- **WHEN** the skill references domain concepts
- **THEN** it uses the canonical terms, not aliases
