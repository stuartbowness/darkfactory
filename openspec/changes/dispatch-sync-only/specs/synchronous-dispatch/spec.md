# synchronous-dispatch

## ADDED Requirements

### Requirement: Synchronous supervised dispatch with structured result
The dispatcher SHALL run a Codex worker to completion synchronously and emit exactly one structured JSON result object on stdout. In `--json` mode, stdout SHALL carry only that object; the worker's own output is consumed by the dispatcher (parsed as NDJSON events) and never forwarded to stdout, and all dispatcher events and diagnostics SHALL go to stderr. A `completed` result SHALL exit the dispatcher process 0; any `failed` or `timed_out` result SHALL exit it non-zero.

#### Scenario: Successful dispatch emits one JSON result
- **WHEN** a dispatch is run with `--json`, the worker exits 0, and (when `--output` was given) the output file exists
- **THEN** stdout contains exactly one JSON object with `status` equal to `"completed"` and exit code 0, and the dispatcher process exits 0

#### Scenario: Worker exits 0 but the requested output file is missing
- **WHEN** the worker exits 0 but `--output` was specified and the file was not created
- **THEN** the JSON result has `status` equal to `"failed"` and the dispatcher process exits non-zero

#### Scenario: Spawn error reported as failed
- **WHEN** the worker cannot be spawned (for example, the engine binary is missing)
- **THEN** the JSON result has `status` equal to `"failed"` and the dispatcher process exits non-zero

#### Scenario: Failed worker reported as failed
- **WHEN** the worker exits non-zero
- **THEN** the JSON result has `status` equal to `"failed"` and includes the worker's non-zero exit code, stdout still carries exactly one JSON object (callers read `.status`), and the dispatcher process exits non-zero

### Requirement: Two-phase timeout
The dispatcher SHALL enforce `--timeout` by sending SIGTERM at the soft deadline, waiting a grace period, then sending SIGKILL, and SHALL report the outcome as a timed-out status.

#### Scenario: Worker exceeds the timeout
- **WHEN** a worker runs past its `--timeout`
- **THEN** the dispatcher sends SIGTERM, waits the grace period, sends SIGKILL if the worker is still alive, the JSON result has `status` equal to `"timed_out"`, and the dispatcher process exits non-zero

### Requirement: Heartbeat emission
The dispatcher SHALL emit a heartbeat event to stderr at the heartbeat interval (default 15 seconds), carrying elapsed time and last activity, so a caller can distinguish a working worker from a dead one.

#### Scenario: Long-running dispatch emits heartbeats
- **WHEN** a dispatch runs longer than one heartbeat interval
- **THEN** stderr carries heartbeat events that include an `elapsed_s` value and a `last_activity` field, emitted at least once per interval

### Requirement: Process-group teardown
When the dispatcher terminates a worker, it SHALL also terminate the other processes in the worker's process group, leaving no orphaned processes from that group.

#### Scenario: Killing a worker kills its process group
- **WHEN** the dispatcher kills a timed-out worker that spawned child processes in its process group
- **THEN** those child processes are also terminated

### Requirement: Parent-death cleanup
The dispatcher SHALL terminate the worker if the dispatcher's own parent process exits, in all dispatches (this watch is always enabled).

#### Scenario: Launching process dies mid-dispatch
- **WHEN** the process that launched the dispatcher exits while a worker is running
- **THEN** the worker process is terminated rather than left running

### Requirement: Worker log capture
When `--log <path>` is given, the dispatcher SHALL write the worker's stdout and stderr to that path. Dispatcher events (heartbeats, diagnostics) are emitted on the dispatcher's own stderr.

#### Scenario: Caller tails a combined log
- **WHEN** a dispatch runs with `--log f` and the caller appends the dispatcher's stderr to the same file (`2>>f`)
- **THEN** `f` contains both the worker's output and the dispatcher's heartbeat events, and can be followed live with `tail -f`
