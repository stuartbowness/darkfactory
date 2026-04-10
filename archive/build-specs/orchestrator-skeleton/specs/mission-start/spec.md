## ADDED Requirements

### Requirement: User can start a mission with /mission
The system SHALL provide a `/mission` skill that accepts a goal description and runs the full spec phase.

#### Scenario: Start a mission with a goal
- **WHEN** user runs `/mission` with a goal description
- **THEN** the orchestrator enters plan mode and writes the durable operating manual

#### Scenario: Start a mission without a goal
- **WHEN** user runs `/mission` without a description
- **THEN** the orchestrator asks the user to describe what they want to build

### Requirement: Orchestrator grills the user for alignment
The system SHALL run `/grill-me` to reach shared understanding before creating any artifacts.

#### Scenario: Grill phase completes
- **WHEN** the orchestrator has reached shared understanding with the user
- **THEN** the orchestrator proceeds to establish shared language

#### Scenario: User wants to skip the grill
- **WHEN** the user explicitly asks to skip the grill phase
- **THEN** the orchestrator warns that skipping reduces contract quality but allows it

### Requirement: Orchestrator establishes shared language
The system SHALL run `/ubiquitous-language` to create or update UBIQUITOUS_LANGUAGE.md with terms from the grill conversation.

#### Scenario: Shared language created
- **WHEN** the grill phase is complete
- **THEN** the orchestrator runs `/ubiquitous-language` and a UBIQUITOUS_LANGUAGE.md file exists with domain terms

### Requirement: Orchestrator creates OpenSpec artifacts
The system SHALL run `/opsx:propose` to create proposal.md, design.md, and tasks.md for the change.

#### Scenario: OpenSpec artifacts created
- **WHEN** shared language is established
- **THEN** the orchestrator runs `/opsx:propose` and proposal.md, design.md, and tasks.md exist in the change directory

### Requirement: Tasks follow vertical slice best practice
Each task in tasks.md SHALL be a vertical slice -- a thin but complete path through all integration layers that is demoable and verifiable on its own.

#### Scenario: Tasks are vertical slices
- **WHEN** the orchestrator creates tasks.md
- **THEN** each task represents a complete end-to-end path, not a horizontal layer

### Requirement: Orchestrator writes the validation contract
The system SHALL create contract.md with assertions grouped by task after OpenSpec artifacts are created.

#### Scenario: Contract created
- **WHEN** OpenSpec artifacts exist
- **THEN** the orchestrator writes contract.md with verifiable assertions for each task

### Requirement: Orchestrator creates mission directory
The system SHALL create the mission/ directory with config.yaml and progress.md.

#### Scenario: Mission directory created
- **WHEN** contract.md is written
- **THEN** the mission/ directory exists with config.yaml (engine settings) and progress.md (empty log)

### Requirement: Orchestrator presents plan for approval
The system SHALL show the user the task breakdown, contract assertions, and engine config before proceeding.

#### Scenario: User approves
- **WHEN** the orchestrator presents the plan and the user approves
- **THEN** the orchestrator records approval in progress.md and the spec phase is complete

#### Scenario: User requests changes
- **WHEN** the user requests changes to the plan, contract, or tasks
- **THEN** the orchestrator updates the relevant artifacts and re-presents for approval
