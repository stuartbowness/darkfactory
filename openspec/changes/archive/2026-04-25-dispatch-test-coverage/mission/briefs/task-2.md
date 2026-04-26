# Worker Brief: Task 2 - Engine error path tests

## Your Task
Add integration tests in `tests/engine.rs` for four engine error paths that currently have no coverage.

### Tests to write

**1. Spawn failure with missing binary**
Set PATH to an empty/nonexistent directory so `codex` can't be found. Call `engine::dispatch(...)`. Assert it returns an error (DispatchError::SpawnFailed or similar), not a hang or panic.

Pattern: Use the existing `TestPath` helper if available, or temporarily set PATH. The key is that the dispatch function returns a clean error.

**2. Failed child with non-zero exit**
Create a mock script that exits with code 1:
```bash
#!/bin/bash
echo '{"type":"thread.started","thread_id":"fail-test"}'
echo '{"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":5,"reasoning_tokens":0}}'
exit 1
```
Dispatch with this mock as the "codex" binary. Assert:
- `result.status == DispatchStatus::Failed`
- `result.exit_code == Some(1)`
- `result.signal == None`

**3. Missing output file (exit 0 but no -o file)**
Create a mock script that exits 0 but doesn't write the output file:
```bash
#!/bin/bash
echo '{"type":"thread.started","thread_id":"no-output"}'
echo '{"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":5,"reasoning_tokens":0}}'
exit 0
```
Dispatch with `--output /tmp/nonexistent-output.md`. Assert:
- `result.status == DispatchStatus::Failed` (or whatever status the engine uses)
- `result.output_exists == false`
- `result.error` contains "missing_output" or similar

**4. Soft timeout where child respects SIGTERM**
Create a mock script that runs for a while but exits cleanly on SIGTERM:
```bash
#!/bin/bash
trap 'exit 0' TERM
sleep 300
```
Dispatch with timeout=1, grace=5. The child should receive SIGTERM at soft timeout and exit. Assert:
- `result.status == DispatchStatus::TimedOut`
- The child process is dead (verify with kill -0)

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Follow the existing test patterns in tests/engine.rs
- Use `#[tokio::test(flavor = "multi_thread")]` for async tests
- Verify `cargo test` passes after adding

## What Previous Tasks Built
- tests/engine.rs already has: successful_dispatch, timeout_dispatch, heartbeat, malformed_ndjson tests
- tests/test_codex_mock.sh is used as a mock codex binary
- The TestPath helper (if it exists) swaps the codex binary on PATH for tests

## Procedures
- Read existing tests/engine.rs to understand the test patterns and helpers
- Add the 4 new tests
- Create any needed mock scripts in tests/
- Verify `cargo test` passes
- Commit: "[test-2] Add engine error path tests for spawn failure, failed child, missing output, and SIGTERM timeout"
- Write handoff to openspec/changes/dispatch-test-coverage/mission/handoffs/task-2.md
