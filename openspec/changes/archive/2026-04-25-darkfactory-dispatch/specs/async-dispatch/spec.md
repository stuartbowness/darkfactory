## ADDED Requirements

### Requirement: Async dispatch mode
The dispatcher SHALL support an async mode that returns a dispatch ID immediately and continues running in the background.

#### Scenario: Async dispatch starts
- **WHEN** `--async` is passed
- **THEN** the dispatcher emits `{"kind":"async_started","dispatch_id":"<ulid>"}` to stdout and continues running the child in the background

#### Scenario: Async dispatch store
- **WHEN** an async dispatch is running
- **THEN** the dispatcher writes `host.pid`, `status.json`, and `events.jsonl` to `~/.darkfactory/dispatches/<ulid>/`
- **AND** writes `result.json` on completion

### Requirement: Dispatch status query
The dispatcher SHALL provide a `status` subcommand to query the state of an async dispatch.

#### Scenario: Running dispatch
- **WHEN** `darkfactory-dispatch status <id>` is called for a running dispatch
- **THEN** stdout contains JSON with state `running`, elapsed time, and last activity

#### Scenario: Completed dispatch
- **WHEN** `darkfactory-dispatch status <id>` is called for a completed dispatch
- **THEN** stdout contains JSON with the terminal state (`completed`, `timed_out`, or `failed`)

#### Scenario: Orphaned dispatch
- **WHEN** `darkfactory-dispatch status <id>` is called and `host.pid` references a dead process with no `result.json`
- **THEN** stdout contains JSON with state `orphaned`

### Requirement: Dispatch result query
The dispatcher SHALL provide a `result` subcommand to retrieve the full dispatch result.

#### Scenario: Result available
- **WHEN** `darkfactory-dispatch result <id>` is called after the dispatch completed
- **THEN** stdout contains the full `DispatchResult` JSON

#### Scenario: Result not yet available
- **WHEN** `darkfactory-dispatch result <id>` is called while the dispatch is still running
- **THEN** the command blocks until the dispatch completes, then emits the result

### Requirement: Dispatch wait
The dispatcher SHALL provide a `wait` subcommand that blocks until an async dispatch completes.

#### Scenario: Wait for completion
- **WHEN** `darkfactory-dispatch wait <id>` is called for a running dispatch
- **THEN** the command blocks, emitting periodic status lines to stderr, until the dispatch completes
- **AND** exits with 0 on completion, non-zero on failure or timeout

### Requirement: Dispatch store cleanup
The dispatcher SHALL prune old dispatch directories to prevent unbounded disk growth.

#### Scenario: Old dispatches pruned
- **WHEN** a new async dispatch starts
- **THEN** dispatch directories older than 7 days are removed from `~/.darkfactory/dispatches/`

### Requirement: Stdout contract separation
`--json` and `--async` SHALL be mutually exclusive modes with different stdout contracts.

#### Scenario: Both flags provided
- **WHEN** both `--json` and `--async` are passed
- **THEN** the dispatcher exits with an error explaining they are mutually exclusive
