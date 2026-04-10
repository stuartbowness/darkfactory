## ADDED Requirements

### Requirement: Orchestrator marks tasks complete after worker finishes
The orchestrator SHALL check off the task in tasks.md after reading a successful handoff report.

#### Scenario: Task marked complete
- **WHEN** the orchestrator reads a valid handoff report for task N
- **THEN** the corresponding checkbox in tasks.md changes from `- [ ]` to `- [x]`

### Requirement: Orchestrator updates progress log
The orchestrator SHALL append an entry to `mission/progress.md` after each task completes.

#### Scenario: Progress logged
- **WHEN** task N completes successfully
- **THEN** progress.md contains a timestamped entry with the task name, commit SHA, and status

### Requirement: Orchestrator iterates to next task
The orchestrator SHALL find the next unchecked task in tasks.md and repeat the dispatch cycle.

#### Scenario: Next task picked up
- **WHEN** task N is marked complete and unchecked tasks remain
- **THEN** the orchestrator assembles a brief for the next unchecked task and dispatches a worker

#### Scenario: All tasks complete
- **WHEN** the last task is marked complete
- **THEN** the orchestrator logs "All tasks complete" to progress.md and stops the build phase
