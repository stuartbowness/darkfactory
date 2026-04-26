# Worker Brief: Task 4 - Lifecycle negative path tests

## Your Task
Add CLI integration tests in `tests/cli.rs` for lifecycle subcommand negative paths.

### Tests to write

1. **Status with unknown ID**: Run `darkfactory-dispatch status nonexistent-id-12345`. Assert exit code is non-zero (1 or 2) and stderr/stdout contains an error message about the dispatch not being found.

2. **Result with unknown ID**: Run `darkfactory-dispatch result nonexistent-id-12345`. Assert exit code is non-zero.

3. **Wait with unknown ID**: Run `darkfactory-dispatch wait nonexistent-id-12345`. Assert exit code is non-zero.

### Test pattern
Follow the existing CLI test patterns in tests/cli.rs using `assert_cmd`:

```rust
#[test]
fn status_unknown_id_exits_nonzero() {
    Command::cargo_bin("darkfactory-dispatch")
        .unwrap()
        .args(["status", "nonexistent-id-12345"])
        .assert()
        .failure();
}
```

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Use `assert_cmd` patterns from existing tests
- Verify `cargo test` passes

## What Previous Tasks Built
- 62 tests passing
- tests/cli.rs has existing tests for help, version, mutual exclusions, async flow, orphan detection, pruning

## Procedures
- Read tests/cli.rs for patterns
- Add the 3 negative path tests
- Verify `cargo test` passes
- Commit: "[test-4] Add lifecycle negative path tests for unknown dispatch IDs"
- Write handoff to openspec/changes/dispatch-test-coverage/mission/handoffs/task-4.md
