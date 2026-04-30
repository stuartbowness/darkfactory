## ADDED Requirements

### Requirement: Anchors section in plan template
The plan template SHALL include a Deterministic Anchors section with six fields.

#### Scenario: Plan template structure
- **WHEN** the plan is created from the template
- **THEN** the Deterministic Anchors section MUST contain fields for: workspace layout, task decomposition strategy with target task count, primary domain types, error handling strategy, test strategy, and chosen architecture

### Requirement: Anchors populated incrementally during planning
The orchestrator SHALL populate the Deterministic Anchors as decisions are approved.

#### Scenario: Design approved in Phase 4.3
- **WHEN** the human approves design.md
- **THEN** the orchestrator MUST fill 5 anchor fields (workspace layout, primary domain types, error handling strategy, test strategy, chosen architecture) as one-liner pointers with source references (e.g., "thiserror in core, never anyhow (see design.md Error Strategy)")

#### Scenario: Task strategy approved in Phase 4.3a
- **WHEN** the human approves the task decomposition strategy
- **THEN** the orchestrator MUST fill the task decomposition strategy anchor field with the approved approach and target task count

### Requirement: Anchors survive context compaction
The Deterministic Anchors SHALL remain readable in the plan after context compaction.

#### Scenario: Context compacts mid-mission
- **WHEN** context compaction occurs during the build loop
- **THEN** the Deterministic Anchors in the plan MUST still contain all six fields with their values, because the plan is the orchestrator's durable operating manual that survives compaction
