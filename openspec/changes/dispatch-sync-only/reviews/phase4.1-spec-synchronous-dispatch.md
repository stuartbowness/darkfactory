# Review Checkpoint — Phase 4.1: specs/synchronous-dispatch

Status: REVIEWED (clean after fixes)
Timestamp: 2026-06-18T19:10:00Z
Reviewer: Codex (gpt-5.5), 3 combined passes over all three specs.

## Findings resolved (this spec)
- R1 HIGH: vague "non-completed status" → named `status: "failed"` with the worker exit code.
- R1 HIGH: `--log f` + `2>f` truncation race → scenario now uses `2>>f` (append).
- R2 HIGH: `completed` also requires the `--output` file present → success scenario amended + missing-output scenario added.
- R3 HIGH: missing-output scenario didn't state the process exit → closed globally with the invariant "any `failed`/`timed_out` result exits non-zero."
- MEDIUM: process-group scoping (was "any child processes"); stdout purity (worker output consumed as NDJSON, not forwarded); spawn-error scenario added.

6 requirements, 9 scenarios. No CRITICAL/HIGH open.
