## ADDED Requirements

### Requirement: Process group isolation
The dispatcher SHALL spawn the Codex child process in its own process group so that killing the child also kills any grandchild processes.

#### Scenario: Child spawns grandchildren
- **WHEN** the Codex process spawns subprocesses (e.g., shell commands)
- **THEN** all grandchild processes belong to the same process group as the Codex process
- **AND** killing the process group terminates both the Codex process and all its grandchildren

### Requirement: Stdin closure
The dispatcher SHALL close stdin on the Codex child process immediately after spawning to prevent the "Reading additional input from stdin" stall.

#### Scenario: Codex receives empty stdin
- **WHEN** the dispatcher spawns `codex exec`
- **THEN** the child's stdin is an empty reader that returns EOF immediately
- **AND** Codex never blocks waiting for stdin input

### Requirement: Two-phase timeout
The dispatcher SHALL enforce a configurable two-phase timeout: a soft timeout emits a warning event and arms a hard timeout, which kills the process group.

#### Scenario: Soft timeout fires
- **WHEN** the configured timeout period elapses without the child exiting
- **THEN** the dispatcher emits a `timeout_warning` event to stderr
- **AND** arms the hard timeout with the configured grace period

#### Scenario: Hard timeout fires
- **WHEN** the grace period elapses after the soft timeout without the child exiting
- **THEN** the dispatcher sends SIGTERM to the process group, waits up to a floor of 10 seconds, then sends SIGKILL
- **AND** the dispatch result status is `timed_out`

#### Scenario: Grace period defaults
- **WHEN** no grace period is specified
- **THEN** the grace period defaults to half of the timeout, minimum 1 second

#### Scenario: Child exits before timeout
- **WHEN** the child exits before the soft timeout fires
- **THEN** no timeout events are emitted and the dispatch result reflects the child's actual exit status

### Requirement: Heartbeat emission
The dispatcher SHALL emit periodic NDJSON heartbeat events to stderr to signal liveness and current activity.

#### Scenario: Heartbeat during active dispatch
- **WHEN** a dispatch is running
- **THEN** the dispatcher emits a heartbeat event to stderr every 15 seconds
- **AND** each heartbeat includes elapsed time and the last observed activity from the child's NDJSON stream

### Requirement: NDJSON event parsing
The dispatcher SHALL parse the child's stdout as NDJSON events and translate them into unified dispatcher events.

#### Scenario: Codex emits tool events
- **WHEN** the Codex child emits `item.started`, `item.completed`, or `turn.completed` NDJSON events on stdout
- **THEN** the dispatcher translates them into `tool_start`, `tool_end`, `file_write`, `file_read`, `command_run`, and `response` events
- **AND** updates the last-activity tracker used by heartbeats

#### Scenario: Malformed NDJSON
- **WHEN** the child emits a non-JSON line on stdout
- **THEN** the dispatcher treats it as a `raw_passthrough` event and increments a parse error counter
- **AND** the line is written to the log file

### Requirement: Structured result
The dispatcher SHALL return a structured JSON result on stdout when the dispatch completes.

#### Scenario: Successful completion
- **WHEN** the child exits with code 0
- **THEN** stdout contains a JSON object with status `completed`, dispatch_id, duration_ms, exit_code 0, signal null, output path, output_exists boolean, token usage, and activity (file path lists)

#### Scenario: Failed completion
- **WHEN** the child exits with a non-zero code
- **THEN** stdout contains a JSON object with status `failed`, the actual exit_code, signal (if killed), and error details

#### Scenario: Timeout completion
- **WHEN** the two-phase timeout kills the child
- **THEN** stdout contains a JSON object with status `timed_out`, partial true, recoverable true, and the signal used to kill

### Requirement: Parent-death detection
The dispatcher SHALL detect when its parent process dies and kill the supervised child to prevent orphans.

#### Scenario: Parent dies on macOS
- **WHEN** the dispatcher's parent process exits (macOS)
- **THEN** the dispatcher detects this via kqueue EVFILT_PROC and sends SIGKILL to the child's process group

#### Scenario: Parent dies on Linux
- **WHEN** the dispatcher's parent process exits (Linux)
- **THEN** the dispatcher detects this via PR_SET_PDEATHSIG and the child receives SIGKILL

#### Scenario: Parent-death disabled in async mode
- **WHEN** the dispatcher is running in async mode
- **THEN** parent-death detection is disabled so the dispatcher survives the caller exiting

### Requirement: Log file capture
The dispatcher SHALL capture the child's stdout and stderr to a log file while parsing stdout for events.

#### Scenario: Log file routing
- **WHEN** `--log PATH` is specified
- **THEN** child stdout and child stderr are both written to the log file
- **AND** the dispatcher parses the child's stdout pipe directly (not the log file) for NDJSON events
- **AND** the dispatcher's own events (heartbeats, warnings) go to dispatcher stderr, not the log file
