## ADDED Requirements

### Requirement: Orchestrator spawns fix worker on validation failure
The orchestrator SHALL spawn a new worker when the scrutiny validator reports a failure, providing the failure details and the broken commit SHA.

#### Scenario: Fix worker dispatched
- **WHEN** the scrutiny validator report has an overall FAIL status
- **THEN** the orchestrator writes failure details to `mission/issues/task-N-v[attempt].md` and dispatches a fix worker

### Requirement: Fix worker receives failure context but not contract
The fix worker brief SHALL include the original task description, specific failures from the validator report, and the commit SHA to fix. It SHALL NOT include contract.md content.

#### Scenario: Fix brief assembled
- **WHEN** the orchestrator assembles a fix brief
- **THEN** it contains what failed, the validator output, the commit SHA, and instructions to fix only the reported issues

### Requirement: Fix worker commits a single fix commit
The fix worker SHALL commit its fixes in a single commit with message format `[task-N] fix: description`.

#### Scenario: Fix committed
- **WHEN** the fix worker completes
- **THEN** exactly one new commit exists with a message matching `[task-N] fix: description`

### Requirement: Both validators re-run after a fix
After a fix worker completes, the orchestrator SHALL re-run both the scrutiny validator and the user-test validator (when available) against the new commit.

#### Scenario: Full re-validation
- **WHEN** a fix worker writes a successful handoff
- **THEN** the orchestrator spawns the scrutiny validator again with the new commit SHA (and user-test validator when implemented)

### Requirement: Circuit breaker after 3 attempts
The orchestrator SHALL stop retrying after 3 total attempts on the same task (original + 2 fixes) and ask the human for guidance.

#### Scenario: Circuit breaker triggered
- **WHEN** the third attempt on a task fails validation
- **THEN** the orchestrator logs all failure details to progress.md and asks the human whether to retry, skip, or abort

#### Scenario: Human chooses to skip
- **WHEN** the human says to skip the task
- **THEN** the orchestrator marks the task as skipped in progress.md and moves to the next task

#### Scenario: Human chooses to abort
- **WHEN** the human says to abort
- **THEN** the orchestrator stops the build phase and logs "Mission paused by human" to progress.md
