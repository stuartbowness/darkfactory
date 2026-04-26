## Why

darkfactory-dispatch (PR #86) has 35 tests but both Claude and Codex independently audited the coverage and found critical gaps. The NDJSON event parser has table tests for only 2 of ~10 event types. Core error paths (missing binary, failed child, missing output) have no tests. The two-phase timeout math has no unit test. There is no shell-level smoke test against real codex.

This is mission-critical infrastructure -- every mission worker and adversarial reviewer runs through it. Untested code paths will silently break during missions.

## What Changes

- Add parse_codex_event table tests for all event types (thread.started, item.started, item.completed variants, turn.completed, turn.failed, error, empty lines, unknown JSON)
- Add engine error path tests (missing binary, non-zero exit, missing output, SIGTERM-exit timeout)
- Add effective_grace_sec unit test
- Add lifecycle negative path tests (unknown ID, corrupt files)
- Add supervisor builder edge case tests
- Add smoketest.sh for live-codex end-to-end testing
- No production code changes -- tests only

## Capabilities

### New Capabilities

- `parser-coverage`: Table-driven tests for every codex NDJSON event type the parser handles
- `error-path-coverage`: Tests for all engine error paths (spawn failure, failed child, missing output, soft timeout exit)
- `lifecycle-coverage`: Negative path tests for async lifecycle subcommands
- `smoketest`: Shell script for live-codex end-to-end verification

### Modified Capabilities

(None -- no production code changes)

## Impact

- Modified: `tools/darkfactory-dispatch/src/engine.rs` (inline tests only)
- Modified: `tools/darkfactory-dispatch/tests/engine.rs` (new integration tests)
- Modified: `tools/darkfactory-dispatch/tests/cli.rs` (lifecycle negative paths)
- Modified: `tools/darkfactory-dispatch/tests/supervisor.rs` (builder edge cases)
- New: `tools/darkfactory-dispatch/tests/smoketest.sh` (live-codex smoke test)
- No changes to src/ production code
