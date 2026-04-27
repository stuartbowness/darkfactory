## ADDED Requirements

### Requirement: Evaluate assertion quality
The test quality review skill SHALL identify tautological assertions -- tests that pass regardless of whether the code works correctly.

#### Scenario: Tautological assertion in Rust
- **WHEN** the diff contains a test with `assert!(true)`, `assert_eq!(x, x)`, or `let _ = func()` with no subsequent assertion
- **THEN** the skill reports a P0 finding with file, line, evidence, and remediation ("replace with an assertion that checks the function's actual return value")

#### Scenario: Tautological assertion in TypeScript
- **WHEN** the diff contains a test with `expect(x).toBeDefined()` or `expect(x).toBeTruthy()` on a non-boolean value without checking the actual value
- **THEN** the skill reports a P0 finding with file, line, evidence, and remediation

#### Scenario: Tautological assertion in Python
- **WHEN** the diff contains a test with `assert result is not None` without checking the value, or bare `assert True`
- **THEN** the skill reports a P0 finding with file, line, evidence, and remediation

### Requirement: Detect implementation coupling
The test quality review skill SHALL flag tests that are coupled to implementation details rather than observable behavior.

#### Scenario: Mock-count assertion on internal function
- **WHEN** the diff contains a test that mocks an internal function and asserts it was called exactly N times (Rust: `mockall` checkpoint counts; TypeScript: `jest.fn().toHaveBeenCalledTimes(N)` on internal helpers; Python: `assert_called_once` on a patched internal function)
- **THEN** the skill reports a P2 finding noting the test will break on refactor without catching bugs

### Requirement: Identify untested branches by inspection
The test quality review skill SHALL compare the production diff with the test diff and flag new decision points (if/else, match, switch, try/catch) that lack corresponding test coverage for both branches.

#### Scenario: New if/else without both branches tested
- **WHEN** the production diff adds an `if/else` block and the test diff only exercises one branch
- **THEN** the skill reports a P1 finding identifying the untested branch with file and line

### Requirement: Flag behavioral changes without tests
The test quality review skill SHALL flag tasks where observable behavior changed (new public function, changed return type, modified error handling) without corresponding test additions.

#### Scenario: New public function with no test
- **WHEN** the production diff adds a new public function and the test diff contains no test for that function
- **THEN** the skill reports a P1 finding identifying the untested function

### Requirement: Check edge case awareness
The test quality review skill SHALL flag public-facing functions that accept input where tests cover only the happy path with no boundary conditions tested.

#### Scenario: Happy-path-only test for input-accepting function
- **WHEN** a new public function accepts user input (parameters, API payload, file contents) and the test diff only tests one valid input with no empty, null, overflow, or malformed input tests
- **THEN** the skill reports a P3 finding suggesting at least one boundary condition test

### Requirement: Structured markdown output
The skill SHALL output findings as simple markdown matching the pattern used by `/review`. Each finding includes title, severity (P0/P1/P2/P3), file and line, evidence, and remediation.

#### Scenario: Multiple findings output
- **WHEN** the skill identifies 3 findings across different dimensions
- **THEN** the output is a markdown document with each finding as a section containing title, severity, file:line, evidence paragraph, and remediation paragraph

### Requirement: Severity-based fail behavior
The scrutiny validator SHALL treat P0 findings as a hard FAIL. 3 or more P1 findings SHALL also be a FAIL. P2 and P3 findings are WARNINGs only.

#### Scenario: Single P0 fails scrutiny
- **WHEN** the test quality review reports one P0 finding (tautological assertion)
- **THEN** the scrutiny validator overall result for this check is FAIL

#### Scenario: Accumulated P1s fail scrutiny
- **WHEN** the test quality review reports 3 P1 findings (untested branches)
- **THEN** the scrutiny validator overall result for this check is FAIL

#### Scenario: P2 and P3 only produce warning
- **WHEN** the test quality review reports 2 P2 findings and 1 P3 finding with no P0 or P1 findings
- **THEN** the scrutiny validator overall result for this check is WARN

### Requirement: Test quality review is self-gating
The check SHALL skip itself when the diff contains no test files.

#### Scenario: No test files in diff
- **WHEN** a task's diff contains no test files
- **THEN** the scrutiny validator reports `test-quality-review | SKIP | no test files in diff`
