## ADDED Requirements

### Requirement: Orchestrator dispatches Codex worker via Monitor
The orchestrator SHALL use the Claude Code Monitor tool to kick off Codex in the background and watch for the handoff file to appear.

#### Scenario: Codex worker dispatched
- **WHEN** the orchestrator dispatches a worker for task N
- **THEN** it starts a Monitor that launches `codex exec` in the background and watches for `mission/handoffs/task-N.md` to appear

#### Scenario: Claude goes idle during worker execution
- **WHEN** the Monitor is running and Codex is working
- **THEN** Claude burns no tokens while waiting

### Requirement: Orchestrator supports Claude subagent as fallback engine
If `mission/config.yaml` has `worker: claude`, the orchestrator SHALL use the Agent tool to spawn a Claude subagent instead of Codex.

#### Scenario: Claude worker dispatched
- **WHEN** config.yaml specifies `worker: claude`
- **THEN** the orchestrator spawns a Claude subagent with the brief content in its prompt, and the subagent writes the handoff report to the same file path

### Requirement: Monitor detects worker failure
The Monitor SHALL detect when Codex exits with a non-zero exit code and no handoff file was written.

#### Scenario: Worker fails
- **WHEN** Codex exits non-zero and `mission/handoffs/task-N.md` does not exist
- **THEN** the Monitor reports failure and the orchestrator logs the error to progress.md and asks the human for guidance

### Requirement: Monitor has a configurable timeout
The Monitor SHALL have a default timeout of 600 seconds (10 minutes).

#### Scenario: Worker exceeds timeout
- **WHEN** the Monitor reaches its timeout and no handoff file exists
- **THEN** the orchestrator logs a timeout to progress.md and asks the human for guidance
