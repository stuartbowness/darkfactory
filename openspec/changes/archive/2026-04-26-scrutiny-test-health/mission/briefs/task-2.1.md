# Worker Brief: Task 2.1 - Enhance check 3 (tests) with test runner output parsing

## Your Task

Modify check 3 (Tests) in `.claude/skills/mission/prompts/scrutiny-validator.md` to incorporate the test runner output parsing results from the Deterministic Pre-passes section.

Currently check 3 just says: "Run the project's test command. Do all tests pass?"

After this task, check 3 should:

1. **Reference the `Test runner` named fact** from the pre-pass section. After running the test command, the validator should fill in the `Test runner` named fact by parsing the summary line.

2. **Report structured counts** in the output table row. Instead of just `PASS` or `FAIL`, report: `tests | FAIL | 49 passed, 46 failed, 2 ignored (51.5% pass rate)` or `tests | PASS | 234 passed, 0 failed, 0 ignored (100% pass rate)`.

3. **Group failures by common error message.** When tests fail, instruct the validator to look at the test output and group failures that share the same error message. For example: "46 failures in 1 group: 'connection refused to 127.0.0.1:5432' -- infrastructure-dependent tests running without infrastructure." This gives the fix worker actionable context.

4. **Add a WARNING condition** for low actual run rate. When the test command exits 0 but the parsed output shows fewer than 90% of tests actually ran (many skipped/ignored, few passed), add a WARNING note: "Only N of M tests actually ran. The rest were skipped/ignored." This catches suites where `cargo test` exits 0 because all failing tests are `#[ignore]` but only 10 tests actually ran.

The existing test command invocation stays the same. You are only changing how the results are reported and interpreted.

Also update the output format section at the bottom of the file -- the check results table row for "Tests" should show the new structured format.

## Project Context
Darkfactory missions produce code that passes the scrutiny validator but scores lower on test quality. The ironbench benchmark showed 49.5% of irontodo-v2's test suite either panicked or silently pretended to pass, but the scrutiny validator only reported "tests failed" with no detail about why. This enhancement gives the fix worker actionable information.

## Architecture
The scrutiny validator is a single markdown prompt file. Task 1.1 added a "Deterministic Pre-passes" section with a `Test runner` named fact that gets filled after check 3 runs the test command. This task modifies check 3 to fill that fact and use it for structured reporting.

## Key Terms
- **Scrutiny validator**: The first validator type that runs checks against the contract.
- **Self-gating check**: A scrutiny check that naturally skips itself when the diff contains no relevant files.

## What Previous Tasks Built

### Task 1.1: Language detection and deterministic pre-passes
- Modified: .claude/skills/mission/prompts/scrutiny-validator.md (+100 lines)
- Added section: "## Deterministic Pre-passes" before "## Check Sequence"
- Contains 4 named facts: Detected language, File sizes, Error handling smells, Test runner
- The `Test runner` named fact is declared but filled after check 3 runs the test command

## Downstream Consumers
- The test runner output parsing results are used by the fix worker when tests fail (provides grouped failure context).
- No other tasks directly depend on this output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-2.1] Enhance check 3 with test runner output parsing"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-2.1.md

## Handoff Report Format
- Task: Task 2.1 - Enhance check 3 (tests) with test runner output parsing
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
