## ADDED Requirements

### Requirement: Detect silent test skips in Rust
The scrutiny validator SHALL grep test files in the diff for patterns that cause Rust tests to report PASS while testing nothing. Patterns include `else { return; }` and `if env::var(...).is_err() { return; }` inside functions annotated with `#[test]` or `#[tokio::test]`. The check SHALL NOT flag tests marked with `#[ignore]`.

#### Scenario: Silent early return in Rust test
- **WHEN** a task's diff contains a Rust test function with `else { return; }` conditional on an environment variable
- **THEN** the scrutiny validator reports FAIL with the file path, line number, test function name, and remediation guidance ("mark as `#[ignore]` or fail explicitly")

#### Scenario: Correctly ignored Rust test
- **WHEN** a task's diff contains a Rust test function marked with `#[ignore]`
- **THEN** the scrutiny validator does NOT flag it as a silent skip

### Requirement: Detect silent test skips in TypeScript
The scrutiny validator SHALL grep test files in the diff for patterns that cause TypeScript/Jest/Vitest tests to report PASS while testing nothing. Patterns include `if (!process.env.X) return` and `if (!process.env.X) { return; }` inside `it()`, `test()`, or `describe()` blocks. The check SHALL NOT flag `it.skip()`, `describe.skip()`, or `@jest.skip`.

#### Scenario: Silent early return in Jest test
- **WHEN** a task's diff contains a Jest `test()` block with `if (!process.env.DATABASE_URL) return` at the top
- **THEN** the scrutiny validator reports FAIL with the file path, line number, test name, and remediation guidance

#### Scenario: Correctly skipped Jest test
- **WHEN** a task's diff contains `it.skip("requires database", ...)` 
- **THEN** the scrutiny validator does NOT flag it as a silent skip

### Requirement: Detect silent test skips in Python
The scrutiny validator SHALL grep test files in the diff for patterns that cause Python tests to report PASS while testing nothing. Patterns include `if not os.environ.get("X"): return` and bare `return` at the top of `test_` functions conditional on environment variables. The check SHALL NOT flag `@pytest.mark.skip` or `pytest.skip("reason")` with an explicit message.

#### Scenario: Silent early return in pytest function
- **WHEN** a task's diff contains a `test_` function with `if not os.environ.get("DATABASE_URL"): return` at the top
- **THEN** the scrutiny validator reports FAIL with the file path, line number, test function name, and remediation guidance

#### Scenario: Correctly skipped pytest test
- **WHEN** a task's diff contains a test decorated with `@pytest.mark.skip(reason="requires database")`
- **THEN** the scrutiny validator does NOT flag it as a silent skip

### Requirement: Silent skip check is a hard FAIL
Any silent skip detected SHALL result in a FAIL for the scrutiny check. The rationale is that silent skips give false confidence by hiding missing coverage.

#### Scenario: Single silent skip fails the check
- **WHEN** the scrutiny validator detects exactly one silent skip pattern in the diff
- **THEN** the overall silent-skip-detection check result is FAIL

### Requirement: Silent skip check is self-gating
The check SHALL skip itself when the diff contains no test files. No explicit closing-task exemption is needed.

#### Scenario: No test files in diff
- **WHEN** a task's diff contains no files matching test file patterns (Rust: files in `tests/` or containing `#[test]`; TypeScript: `*.test.ts`, `*.spec.ts`, `__tests__/`; Python: `test_*.py`, `*_test.py`, `tests/`)
- **THEN** the scrutiny validator reports `silent-skip-detection | SKIP | no test files in diff`
