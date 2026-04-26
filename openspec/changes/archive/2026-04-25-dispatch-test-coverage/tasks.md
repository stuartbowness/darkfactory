## 1. Parser table tests

- [x] 1.1 Add table-driven tests for parse_codex_event covering all event types: thread.started, item.started (tool_use, function_call, agent_message), item.completed (agent_message, tool_use, file write, file read), turn.completed (with token usage), turn.failed, error, empty line, non-JSON line, unknown JSON type. Each test provides a real NDJSON line and asserts the correct EventKind and extracted fields. Add as inline #[cfg(test)] in src/engine.rs.

## 2. Engine error path tests

- [x] 2.1 Add engine integration tests for: (a) spawn failure with missing binary (set PATH to empty, expect BinaryNotFound or SpawnFailed), (b) failed child with non-zero exit (mock script that exits 1, assert status Failed with exit_code), (c) missing output (mock script that succeeds but doesn't write -o file, assert missing_output error), (d) soft timeout where child respects SIGTERM (mock with short timeout, assert TimedOut status and child is dead). Add to tests/engine.rs.

## 3. Timeout math and grace tests

- [x] 3.1 Add unit tests for effective_grace_sec: (a) grace=0 with timeout=900 -> 450, (b) grace=0 with timeout=1 -> 1 (minimum), (c) grace=0 with timeout=0 -> 1 (minimum), (d) explicit grace=30 passes through unchanged. Add as inline #[cfg(test)] in src/engine.rs.

## 4. Lifecycle negative path tests

- [x] 4.1 Add CLI integration tests for: (a) status with unknown dispatch ID exits non-zero, (b) result with unknown ID exits non-zero, (c) wait with unknown ID exits non-zero, (d) status with corrupt status.json handles gracefully, (e) result with corrupt result.json handles gracefully. Add to tests/cli.rs.

## 5. Supervisor edge case tests

- [x] 5.1 Add supervisor tests for: (a) spawn with non-existent binary returns error, (b) kill_soft sends SIGTERM (verify child receives signal 15), (c) normal exit reports exit_code with no signal. Add to tests/supervisor.rs.

## 6. Smoke test script

- [x] 6.1 Create tests/smoketest.sh: a POSIX shell script that builds the release binary, then runs 3 live-codex dispatches: (a) simple task (echo to file), verify completed status and output_exists, (b) timeout test (sleep with 10s timeout), verify timed_out status, (c) prompt-file test (write prompt to temp file, dispatch with --prompt-file), verify completed. Each test prints PASS/FAIL. Script exits non-zero if any test fails. Add a `smoke` target to the Makefile that runs this script.

## 7. Closing tasks

- [x] 7.1 Cleanup: verify `make check` passes (fmt + clippy + all tests). Remove any dead test helpers or unused imports introduced during test writing.
- [x] 7.2 Documentation: update CLAUDE.md testing table if test file structure changed. Verify test count increased from 35.
