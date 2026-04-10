## MODIFIED Requirements

### Requirement: Step 6 placeholder replaced with user-test validation
The build phase Step 6 SHALL spawn a user-test validator instead of being a placeholder.

#### Scenario: User-test validation runs after scrutiny passes
- **WHEN** the scrutiny validator reports PASS for a task
- **THEN** the orchestrator spawns the user-test validator before marking the task complete

#### Scenario: Task only marked complete after both validators pass
- **WHEN** both scrutiny and user-test validators report PASS
- **THEN** the orchestrator marks the task complete and moves to the next task
