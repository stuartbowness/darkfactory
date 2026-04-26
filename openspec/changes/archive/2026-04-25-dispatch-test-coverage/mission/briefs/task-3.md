# Worker Brief: Task 3 - Timeout math and grace tests + Task 5 - Supervisor edge cases

## Your Task
Two small test additions in one commit:

### Part A: effective_grace_sec unit tests (src/engine.rs inline)

The function `effective_grace_sec(timeout_sec, grace_sec)` computes the grace period. Add inline `#[cfg(test)]` tests:

1. `grace=0, timeout=900` -> should return 450 (half of timeout)
2. `grace=0, timeout=1` -> should return 1 (minimum 1s)
3. `grace=0, timeout=0` -> should return 1 (minimum 1s)
4. `grace=30, timeout=900` -> should return 30 (explicit grace passes through)

### Part B: Supervisor edge case tests (tests/supervisor.rs)

1. **Spawn non-existent binary**: `SupervisorBuilder::new("nonexistent-binary-xyz").spawn()` should return an error, not panic.
2. **kill_soft sends SIGTERM**: Spawn `sleep 300`, call `kill_soft()`, wait. Verify `ExitResult.signal == Some(15)` (SIGTERM).
3. **Normal exit reports exit_code with no signal**: Spawn `true` (exits 0), wait. Verify `exit_code == Some(0)` and `signal == None`.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Grace tests: inline `#[cfg(test)]` in src/engine.rs
- Supervisor tests: in tests/supervisor.rs following existing patterns
- `#[tokio::test(flavor = "multi_thread")]` for async tests
- Verify `cargo test` passes

## What Previous Tasks Built
- 53 tests passing
- tests/supervisor.rs has 3 existing tests (process group kill, graceful stop, signal detection)
- src/engine.rs has inline parser tests and the effective_grace_sec function

## Procedures
- Read effective_grace_sec in src/engine.rs
- Add inline grace tests
- Read tests/supervisor.rs for patterns
- Add supervisor edge case tests
- Verify `cargo test` passes
- Commit: "[test-3] Add grace math unit tests and supervisor edge case tests"
- Write handoff to openspec/changes/dispatch-test-coverage/mission/handoffs/task-3.md
