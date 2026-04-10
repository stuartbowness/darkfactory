## ADDED Requirements

### Requirement: Orchestrator records demo video for frontend tasks
The orchestrator SHALL spawn a demo video subagent for tasks that have a frontend component, after both validators pass.

#### Scenario: Frontend task gets demo video
- **WHEN** both validators pass and the orchestrator determines the task has a frontend component
- **THEN** a Claude subagent runs /demo-video and saves the recording to `mission/validations/task-N-demo.mp4`

#### Scenario: Backend-only task skips demo video
- **WHEN** both validators pass and the task has no frontend component
- **THEN** the demo video step is skipped

### Requirement: Demo video failure does not block progress
A failed demo recording SHALL be logged but SHALL NOT trigger the fix loop or block the task from being marked complete.

#### Scenario: Demo recording fails
- **WHEN** the demo video subagent fails to record
- **THEN** the orchestrator logs the failure to progress.md and continues to mark the task complete
