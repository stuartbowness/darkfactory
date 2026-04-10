## MODIFIED Requirements

### Requirement: Build phase includes validation after each worker
The build phase SHALL run the scrutiny validator after each worker completes, before marking the task as done.

#### Scenario: Validation inserted in build loop
- **WHEN** a worker completes and the handoff is read
- **THEN** the orchestrator spawns the scrutiny validator before marking the task complete in tasks.md

#### Scenario: Task only marked complete after validation passes
- **WHEN** the scrutiny validator reports PASS
- **THEN** the orchestrator marks the task complete and moves to the next task

#### Scenario: Validation failure triggers fix loop
- **WHEN** the scrutiny validator reports FAIL
- **THEN** the orchestrator enters the fix loop instead of marking the task complete
