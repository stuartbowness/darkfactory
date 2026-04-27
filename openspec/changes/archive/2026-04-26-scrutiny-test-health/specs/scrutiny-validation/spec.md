## MODIFIED Requirements

### Requirement: Test suite run (check 3) includes structured output parsing
The scrutiny validator SHALL capture stdout/stderr from the test command, parse the summary line to extract passed/failed/skipped/ignored counts, and group failures by common error message. The parsed counts SHALL be included in the check results table.

#### Scenario: Rust test output parsed
- **WHEN** `cargo test` produces output containing `test result: ok. 49 passed; 46 failed; 2 ignored`
- **THEN** the scrutiny validator reports `tests | FAIL | 49 passed, 46 failed, 2 ignored (51.5% pass rate)` and groups the 46 failures by common error message

#### Scenario: Jest test output parsed
- **WHEN** `jest` produces output containing `Tests: 5 failed, 12 passed, 17 total`
- **THEN** the scrutiny validator reports `tests | FAIL | 12 passed, 5 failed (70.6% pass rate)` and groups failures by describe block or error message

#### Scenario: Low actual run rate warning
- **WHEN** the test command exits 0 but the parsed output shows fewer than 90% of tests actually ran (many skipped/ignored, few passed)
- **THEN** the scrutiny validator adds a WARNING: "Only N of M tests actually ran. The rest were skipped/ignored."

### Requirement: Module depth review (check 6) includes file size pre-pass
Before the LLM evaluates module depth, the scrutiny validator SHALL count lines on every source file the task touched. If any source file exceeds 900 lines, the validator SHALL inject the file name, line count, and a cross-reference to design.md's concern boundaries into the module depth review prompt.

#### Scenario: File exceeds 900 lines with design.md boundaries specified
- **WHEN** a task leaves `jsonrpc.rs` at 1,292 lines and design.md specifies "protocol, dispatch, and handlers should be in separate files"
- **THEN** the module depth review receives the line count and design.md reference, and the reviewer flags the file as FAIL for not following the specified concern boundaries

#### Scenario: File exceeds 900 lines with no design.md boundaries
- **WHEN** a task leaves `store.rs` at 950 lines and design.md does not address concern boundaries for that module
- **THEN** the module depth review receives the line count and flags it as FAIL (design gap -- concern boundaries should have been specified)

#### Scenario: File exceeds 900 lines but design.md explicitly justifies it
- **WHEN** a task leaves `generated_schema.rs` at 1,100 lines and design.md says "schema file is intentionally a single generated file"
- **THEN** the module depth review receives the line count and design.md justification, and the reviewer marks it as PASS

#### Scenario: Standalone test files excluded
- **WHEN** a task leaves `tests/integration.rs` at 1,500 lines
- **THEN** the file size pre-pass does NOT flag it (standalone test files are excluded from the 900-line check)

### Requirement: Code standards check (check 7) includes error handling smell density
Before the LLM reviews code standards, the scrutiny validator SHALL run a deterministic grep for language-specific error handling anti-patterns on changed production lines (excluding test files). The count per KLOC and specific locations SHALL be injected into the code standards review.

#### Scenario: Rust error handling smells counted
- **WHEN** a task's diff adds 500 production lines containing 8 instances of `.unwrap()` and 2 instances of `panic!()`
- **THEN** the code standards check receives "10 error handling smells in 500 changed lines (20.0 per KLOC)" with each location listed, and the overall result is FAIL (over 10/KLOC)

#### Scenario: Safe Rust patterns excluded
- **WHEN** a task's diff contains `.unwrap_or_default()`, `.unwrap_or_else(...)`, and `.expect("connection pool must be initialized")`
- **THEN** none of these are counted as error handling smells

#### Scenario: TypeScript error handling smells counted
- **WHEN** a task's diff adds 300 production lines containing 2 instances of `catch {}` and 1 instance of `as any`
- **THEN** the code standards check receives "3 error handling smells in 300 changed lines (10.0 per KLOC)" with locations listed, and the overall result is WARN (exactly 10/KLOC boundary rounds to FAIL threshold)

#### Scenario: Python error handling smells counted
- **WHEN** a task's diff adds 400 production lines containing 1 instance of `except: pass` and 1 instance of `type: ignore` without a code
- **THEN** the code standards check receives "2 error handling smells in 400 changed lines (5.0 per KLOC)" with locations listed, and the overall result is WARN (at 5/KLOC boundary)

#### Scenario: Test files excluded from smell count
- **WHEN** a task's diff contains `.unwrap()` calls in test files only
- **THEN** the error handling smell count is 0 (test files are excluded)

#### Scenario: Graduated thresholds applied
- **WHEN** the error handling smell density is under 5 per KLOC
- **THEN** the code standards enhancement reports PASS for the smell density component
