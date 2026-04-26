# Contract

## Task 1.1: Parser table tests
<!-- Audit gap: parse_codex_event has tests for only 2 of ~10 event types -->
- [ ] [execution] `cargo test` includes tests for parse_codex_event covering: thread.started, item.started (tool_use), item.started (agent_message), item.completed (agent_message with text), item.completed (tool_use), turn.completed (with token counts), turn.failed, error, non-JSON line, unknown JSON type
- [ ] [execution] Each parser test asserts the correct EventKind variant and at least one extracted field (e.g., session_id for thread.started, tokens for turn.completed)
- [ ] [output] Parser tests are inline `#[cfg(test)]` in `src/engine.rs`

## Task 2.1: Engine error path tests
<!-- Audit gap: no tests for spawn failure, failed child, missing output, SIGTERM-exit -->
- [ ] [execution] A test with codex binary absent from PATH produces a spawn error (not a hang)
- [ ] [execution] A test with a mock script that exits non-zero produces status `Failed` with the exit code in the result
- [ ] [execution] A test with a mock script that exits 0 without writing the output file produces status `Failed` with error code `missing_output`
- [ ] [execution] A test with a short timeout where the child respects SIGTERM produces status `TimedOut` and the child process is dead

## Task 3.1: Timeout math and grace tests
<!-- Audit gap: effective_grace_sec has no unit test -->
- [ ] [execution] A test verifies grace=0 with timeout=900 produces grace=450
- [ ] [execution] A test verifies grace=0 with timeout=1 produces grace=1 (minimum)
- [ ] [execution] A test verifies explicit grace=30 passes through unchanged

## Task 4.1: Lifecycle negative path tests
<!-- Audit gap: no tests for unknown ID, corrupt files -->
- [ ] [execution] `darkfactory-dispatch status nonexistent-id` exits non-zero
- [ ] [execution] `darkfactory-dispatch result nonexistent-id` exits non-zero
- [ ] [execution] `darkfactory-dispatch wait nonexistent-id` exits non-zero

## Task 5.1: Supervisor edge case tests
<!-- Audit gap: no tests for missing binary, kill_soft, normal exit -->
- [ ] [execution] A test spawning a non-existent binary returns an error (not a panic)
- [ ] [execution] A test verifies kill_soft sends SIGTERM (child reports signal 15)
- [ ] [execution] A test verifies normal exit reports exit_code with signal=None

## Task 6.1: Smoke test script
<!-- Audit gap: no live-codex end-to-end test -->
- [ ] [output] `tests/smoketest.sh` exists and is executable
- [ ] [output] The script runs at least 3 dispatch tests: simple task, timeout, and prompt-file
- [ ] [output] The Makefile has a `smoke` target that runs the script
- [ ] [execution] `tests/smoketest.sh` runs without error when darkfactory-dispatch and codex are available

## Task 7.1: Cleanup
- [ ] [execution] `make check` passes (fmt + clippy + all tests)
- [ ] [execution] Total test count is higher than 35 (the pre-mission baseline)

## Task 7.2: Documentation
- [ ] [output] `CLAUDE.md` testing table reflects any new test file structure
