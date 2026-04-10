## MODIFIED Requirements

### Requirement: Build phase includes demo video step
The build phase SHALL include a demo video step between user-test validation and task marking for frontend tasks.

#### Scenario: Demo video runs before task is marked complete
- **WHEN** both validators pass on a frontend task
- **THEN** the orchestrator runs the demo video step before marking the task complete

### Requirement: Mission has a completion phase after build
The /mission skill SHALL include a completion phase that runs after all tasks are built and validated.

#### Scenario: Completion phase creates draft PR
- **WHEN** the build phase completes with all tasks validated
- **THEN** the orchestrator enters the completion phase and creates a draft PR
