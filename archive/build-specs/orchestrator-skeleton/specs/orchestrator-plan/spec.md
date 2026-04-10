## ADDED Requirements

### Requirement: Plan mode is entered immediately
The orchestrator SHALL enter Claude Code plan mode as the first action after `/mission` is invoked, before any other work.

#### Scenario: Plan written before grill
- **WHEN** `/mission` is invoked
- **THEN** the orchestrator enters plan mode and writes the operating manual before starting the grill

### Requirement: Plan contains mission state references
The plan SHALL include file paths to all OpenSpec artifacts, the mission directory, and the engine config.

#### Scenario: Plan references correct paths
- **WHEN** the plan is written
- **THEN** it contains paths to proposal.md, design.md, tasks.md, contract.md, and the mission/ directory

### Requirement: Plan contains the full mission flow
The plan SHALL describe the step-by-step flow for each task: brief -> worker -> scrutiny -> user-test -> demo -> mark complete.

#### Scenario: Flow is documented
- **WHEN** the plan is written
- **THEN** it includes the numbered mission flow steps, rules, and skills to use

### Requirement: Plan survives context compaction
The plan SHALL contain all information needed for the orchestrator to continue operating after context compaction removes conversation history.

#### Scenario: Orchestrator recovers from compaction
- **WHEN** context compaction removes prior conversation
- **THEN** the orchestrator can read the plan and mission state from disk to know exactly what to do next
