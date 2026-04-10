## ADDED Requirements

### Requirement: Worker writes structured handoff report
The worker SHALL write a handoff report to `mission/handoffs/task-N.md` after completing a task.

#### Scenario: Handoff report written
- **WHEN** a worker completes its task
- **THEN** a file exists at `mission/handoffs/task-N.md` containing: task reference, commit SHA, approach taken, commands run with exit codes, and any concerns

### Requirement: Worker commits in a single commit
The worker SHALL commit all changes for the task in exactly one commit with the message format `[task-N] description`.

#### Scenario: Single commit created
- **WHEN** a worker finishes implementation
- **THEN** exactly one new commit exists with a message matching `[task-N] description`

### Requirement: Orchestrator reads handoff and extracts commit SHA
The orchestrator SHALL read the handoff report and extract the commit SHA for use by validators in later changes.

#### Scenario: Commit SHA extracted
- **WHEN** the orchestrator reads `mission/handoffs/task-N.md`
- **THEN** it has the commit SHA and can pass it to validators
