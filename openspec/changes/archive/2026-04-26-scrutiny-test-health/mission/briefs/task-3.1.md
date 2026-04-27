# Worker Brief: Task 3.1 - Add check 12 (silent skip detection)

## Your Task

Add a new check 12 titled "Silent skip detection" to `.claude/skills/mission/prompts/scrutiny-validator.md`. Place it after check 11 (Behavioral conformance), before the Output Format section.

Check 12 should:

1. **Grep test files in the diff** for language-specific patterns that cause tests to report PASS while testing nothing. Use the `Detected language` named fact from the pre-pass to select the correct patterns, but also check file extensions in the diff for mixed-language projects.

2. **Rust patterns to flag:**
   - `else { return; }` or `else { return }` inside functions annotated with `#[test]` or `#[tokio::test]`
   - `if env::var("X").is_err() { return; }` or similar conditional returns at the top of test functions based on environment variables

3. **Rust patterns to NOT flag:**
   - `#[ignore]` attribute on test functions

4. **TypeScript/Node patterns to flag:**
   - `if (!process.env.X) return` inside `it()`, `test()`, or `describe()` blocks
   - `if (!process.env.X) { return; }` inside test blocks

5. **TypeScript/Node patterns to NOT flag:**
   - `it.skip()`, `describe.skip()`, `@jest.skip`

6. **Python patterns to flag:**
   - `if not os.environ.get("X"): return` inside `test_` functions
   - bare `return` at the top of `test_` functions conditional on environment variables

7. **Python patterns to NOT flag:**
   - `@pytest.mark.skip`, `pytest.skip("reason")` with explicit message

8. **Fail behavior:** Any silent skip found is a FAIL. The rationale: silent skips give false confidence by hiding missing coverage.

9. **Output format:** `silent-skip-detection | FAIL | N silent skips found`. Details should list each instance with file path, line number, test function name, and remediation guidance: "Either fail explicitly with `panic!("requires X")` or mark as `#[ignore]` / `@skip` so the test runner reports it as skipped, not passed."

10. **Self-gating:** When the diff contains no test files, report `silent-skip-detection | SKIP | no test files in diff`. Test file detection uses the same language-specific patterns as the pre-pass file size count exclusions (Rust: files in `tests/` or containing `#[test]`; TypeScript: `*.test.ts`, `*.spec.ts`, `__tests__/`; Python: `test_*.py`, `*_test.py`, `tests/`).

Also update the output format table at the bottom of the file to include a row for `Silent skip detection`.

## Project Context
In the irontodo-v2 benchmark, 2 CLI integration tests silently returned success via `let Some(database_url) = setup_test_database() else { return; }` -- reporting PASS while testing nothing. This check catches that pattern.

## Architecture
This is a new check in the scrutiny validator's check sequence. It uses the `Detected language` named fact from the pre-pass but runs its own grep on test files in the diff. It follows the self-gating pattern where the check skips itself when no relevant files are in the diff.

## Key Terms
- **Silent skip**: A test that conditionally returns early without testing anything but reports PASS to the test runner.
- **Self-gating check**: A scrutiny check that naturally skips itself when the diff contains no relevant files.

## What Previous Tasks Built

### Task 1.1: Language detection and deterministic pre-passes
- Added `Detected language` named fact
- Defined test file exclusion patterns per language

### Tasks 2.1-2.3: Enhanced existing checks 3, 6, 7
- Check 3 now parses test runner output
- Check 6 now flags files over 900 lines
- Check 7 now reports error handling smell density

## Downstream Consumers
No later tasks depend directly on this check's output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-3.1] Add silent skip detection check to scrutiny validator"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-3.1.md

## Handoff Report Format
- Task: Task 3.1 - Add check 12 (silent skip detection)
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
