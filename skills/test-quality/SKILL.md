---
name: test-quality
user-invokable: true
description: LLM-based semantic review of test effectiveness. Evaluates assertion quality, implementation coupling, branch coverage, behavioral changes without tests, and edge case awareness. Use when scrutiny validator invokes it as a subagent, or when user wants to review test quality independently.
---

# Test Quality Review

Review the test code in the diff alongside the production code and evaluate whether the tests actually verify meaningful behavior. Output structured markdown findings.

## Input

You receive the full diff (production + test changes) as context. Separate production files from test files by file path patterns:

- Rust test files: files in `tests/`, files containing `#[test]` or `#[tokio::test]`
- TypeScript test files: `*.test.ts`, `*.spec.ts`, files in `__tests__/`
- Python test files: `test_*.py`, `*_test.py`, files in `tests/`

## Evaluation Dimensions

Evaluate the test diff against the production diff across these five dimensions. For each finding, assign a severity.

### 1. Assertion quality (P0 if tautological)

Flag tests that pass regardless of whether the code works correctly.

**Rust tautological patterns:**
- `assert!(true)`, `assert_eq!(x, x)`
- `let _ = func()` with no subsequent assertion
- Test function that calls production code but never asserts on the result

**TypeScript tautological patterns:**
- `expect(x).toBeDefined()` or `expect(x).toBeTruthy()` as the only assertion on a non-boolean value
- `expect(result).not.toBeNull()` without checking the actual value
- Test that awaits an async function but only checks it doesn't throw

**Python tautological patterns:**
- `assert result is not None` as the only assertion without checking the value
- `assert True`, `assert 1`
- Test that calls a function and only checks `isinstance(result, SomeType)` without verifying content

A P0 finding means the test gives false confidence -- it would pass even if the production code returned garbage.

### 2. Implementation coupling (P2)

Flag tests coupled to implementation details rather than observable behavior.

- Tests that mock internal (non-boundary) functions and assert call counts: Rust `mockall` checkpoint counts, TypeScript `jest.fn().toHaveBeenCalledTimes(N)` on internal helpers, Python `assert_called_once` on patched internals
- Tests that check exact internal state (private fields, internal data structures) rather than public output
- Tests that would break if the implementation changed but the behavior stayed the same

P2 because these tests create maintenance burden without catching bugs.

### 3. Branch coverage by inspection (P1 for critical paths)

Compare the production diff with the test diff. For new decision points (if/else, match arms, switch cases, try/catch), check whether both branches are exercised by tests.

- Flag untested branches in error handling paths (the catch/else/Err arm)
- Flag untested branches in validation logic (the rejection path)
- Flag untested match arms for new enum variants

P1 when the untested branch is in a critical path (error handling, validation, auth). P3 when it is in a non-critical path (display formatting, logging).

### 4. Behavioral changes without tests (P1)

Flag tasks where observable behavior changed but no corresponding test was added.

- New public function or method with no test
- Changed return type or error type with no test update
- Modified error handling (new error variant, changed error message) with no test
- New CLI flag, API endpoint, or config option with no test

P1 because untested behavioral changes are the primary source of regressions.

### 5. Edge case awareness (P3)

For public-facing functions that accept input (user input, API parameters, file contents), check whether tests cover at least one boundary condition.

- Flag functions where tests only use a single "happy path" input
- Suggest at least one of: empty input, null/None, boundary values (0, MAX, empty string), malformed input

P3 because missing edge cases are a quality concern, not a correctness failure.

## Output Format

Write findings as markdown. Each finding is a section:

```markdown
### P0: Tautological assertion in test_list_open

**File:** src/store.rs:245
**Evidence:** Test calls `list_open(&pool).await` and only asserts `assert!(result.is_ok())`. This passes even if the function returns an empty list, wrong data, or any Ok value. The test does not verify the actual content.
**Remediation:** Assert on the returned list contents: check length, verify specific fields of returned items match test fixtures.
```

Group findings by severity (P0 first, then P1, P2, P3). At the end, include a summary line: `N findings: X P0, Y P1, Z P2, W P3`.

## Self-gating

If the diff contains no test files, output: "No test files in diff. Skipping test quality review."

## What This Skill Does NOT Do

- Does not run tests or measure coverage (that's check 3 and check 13)
- Does not check for silent skips (that's check 12)
- Does not review production code quality (that's /review and /simplify)
- Only reviews test code quality and its relationship to the production changes
