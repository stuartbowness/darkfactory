## MODIFIED Requirements

### Requirement: Codex worker dispatch uses darkfactory-dispatch
The mission skill's Codex worker dispatch SHALL use `darkfactory-dispatch` instead of raw `codex exec` bash blocks.

#### Scenario: Worker dispatch
- **WHEN** the orchestrator dispatches a Codex worker for a task
- **THEN** the mission skill calls `darkfactory-dispatch` with the appropriate model, effort, timeout, sandbox, config, prompt-file, output, cwd, log, and --json flags
- **AND** the orchestrator parses the JSON result for status, exit_code, output_exists, and token usage

#### Scenario: Adversarial reviewer dispatch
- **WHEN** the orchestrator dispatches Codex adversarial reviewer A
- **THEN** the mission skill calls `darkfactory-dispatch` with the reviewer prompt, output path, and --json flags
- **AND** parses the JSON result the same way as a worker dispatch

#### Scenario: Fallback behavior preserved
- **WHEN** darkfactory-dispatch returns status `failed` or `timed_out` for a worker task
- **THEN** the orchestrator's existing fallback logic (Claude worker fallback, circuit breaker) is triggered based on the structured result instead of checking for `TASK_COMPLETE`/`TASK_FAILED` strings
