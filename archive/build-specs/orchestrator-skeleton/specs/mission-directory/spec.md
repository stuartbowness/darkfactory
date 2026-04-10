## ADDED Requirements

### Requirement: Mission directory nests inside the OpenSpec change
The mission/ directory SHALL be created at `openspec/changes/<change-name>/mission/`.

#### Scenario: Directory location
- **WHEN** the orchestrator creates the mission directory
- **THEN** it exists at the path `openspec/changes/<change-name>/mission/`

### Requirement: Mission directory contains config.yaml
The config.yaml SHALL specify the engine for each role with sensible defaults.

#### Scenario: Default config created
- **WHEN** the mission directory is created
- **THEN** config.yaml exists with `worker: codex`, `scrutiny_validator: claude`, `usertest_validator: claude`, `demo_video: claude`

### Requirement: Mission directory contains progress.md
The progress.md SHALL serve as the orchestrator's running log of decisions and status.

#### Scenario: Progress log initialized
- **WHEN** the mission directory is created
- **THEN** progress.md exists with the mission name, start timestamp, and "Plan approved" entry

### Requirement: Mission directory contains subdirectories
The mission/ directory SHALL contain briefs/, handoffs/, validations/, and issues/ subdirectories.

#### Scenario: Subdirectories created
- **WHEN** the mission directory is created
- **THEN** briefs/, handoffs/, validations/, and issues/ directories exist (empty, ready for the worker loop)
