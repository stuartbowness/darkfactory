## ADDED Requirements

### Requirement: User can resume an interrupted mission
The system SHALL provide a `/mission resume` skill that picks up a mission from file state.

#### Scenario: Resume with one active mission
- **WHEN** user runs `/mission resume` and exactly one OpenSpec change has a mission/ directory with unchecked tasks
- **THEN** the orchestrator reads the mission state, enters plan mode, reconstructs the operating manual, and continues the build loop from the next unchecked task

#### Scenario: Resume with multiple active missions
- **WHEN** user runs `/mission resume` and multiple changes have mission/ directories
- **THEN** the orchestrator lists them and asks the user to pick one

#### Scenario: Resume with no active mission
- **WHEN** user runs `/mission resume` and no changes have mission/ directories with unchecked tasks
- **THEN** the orchestrator reports "No active missions found"

### Requirement: Resume reconstructs the plan from files
The orchestrator SHALL read tasks.md, progress.md, config.yaml, contract.md, and the OpenSpec artifacts to reconstruct a complete operating manual in plan mode.

#### Scenario: Plan reflects current state
- **WHEN** the plan is reconstructed
- **THEN** it shows which tasks are completed, which is next, and references all correct file paths

### Requirement: Resume starts from next unchecked task
The orchestrator SHALL find the first unchecked task in tasks.md and dispatch a worker for it, ignoring any partial work from the interrupted session.

#### Scenario: Interrupted task restarts
- **WHEN** a task was in progress when the session died (no handoff file)
- **THEN** the orchestrator dispatches a new worker for that task from the last committed state

### Requirement: Resume detects orphaned commits
If a worker committed but didn't write a handoff, the orchestrator SHALL check git log for commits matching the task pattern and ask the human.

#### Scenario: Orphaned commit found
- **WHEN** git log shows a commit matching `[task-N]` but no handoff file exists
- **THEN** the orchestrator asks the human whether to count the task as done or re-run it
