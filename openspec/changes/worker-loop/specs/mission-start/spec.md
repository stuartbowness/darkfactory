## MODIFIED Requirements

### Requirement: Mission skill runs build phase after plan approval
The /mission skill SHALL proceed to the build phase after the user approves the plan, iterating through tasks and dispatching workers.

#### Scenario: Build phase starts after approval
- **WHEN** the user approves the plan in Phase 6
- **THEN** the orchestrator begins the build phase, picking up the first unchecked task in tasks.md

#### Scenario: Build phase runs to completion
- **WHEN** the build phase starts
- **THEN** the orchestrator dispatches a worker for each task sequentially until all tasks are complete or a failure requires human input
