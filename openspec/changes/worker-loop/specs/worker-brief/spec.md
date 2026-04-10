## ADDED Requirements

### Requirement: Orchestrator assembles a brief per task
The orchestrator SHALL create a brief file at `mission/briefs/task-N.md` for each task before dispatching a worker.

#### Scenario: Brief created from artifacts
- **WHEN** the orchestrator is ready to dispatch a worker for task N
- **THEN** a file exists at `mission/briefs/task-N.md` containing the task description, relevant excerpts from proposal.md and design.md, and worker procedures

### Requirement: Brief is self-contained
The brief SHALL contain everything the worker needs to implement the task without reading any other files.

#### Scenario: Worker operates from brief alone
- **WHEN** a worker receives the brief
- **THEN** it can implement the task, commit, and write the handoff report without accessing proposal.md, design.md, or tasks.md directly

### Requirement: Brief never contains contract content
The brief SHALL NOT include any content from contract.md to maintain adversarial separation between workers and validators.

#### Scenario: Contract excluded from brief
- **WHEN** the orchestrator assembles a brief
- **THEN** the brief contains no assertions or content from contract.md

### Requirement: Brief specifies commit and handoff procedures
The brief SHALL instruct the worker to commit in a single commit with a specific message format and write the handoff report to a specific file path.

#### Scenario: Procedures are explicit
- **WHEN** a worker reads the brief
- **THEN** it knows the commit message format (`[task-N] description`), the handoff file path (`mission/handoffs/task-N.md`), and the handoff report format
