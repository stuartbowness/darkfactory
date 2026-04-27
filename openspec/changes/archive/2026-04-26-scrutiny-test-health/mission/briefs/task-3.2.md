# Worker Brief: Task 3.2 - Add check 13 (diff coverage)

## Your Task

Add a new check 13 titled "Diff coverage" to `.claude/skills/mission/prompts/scrutiny-validator.md`. Place it after check 12 (Silent skip detection), before the Output Format section.

Check 13 should:

1. **Use the `Detected language` named fact** from the pre-pass to select the coverage tool.

2. **Run the language-appropriate coverage tool:**
   - Rust: `cargo tarpaulin --out json` (primary), `cargo llvm-cov --json` (fallback)
   - TypeScript/Node: `npx jest --coverage --json` or `npx c8 --reporter json`
   - Python: `pytest --cov --cov-report json`

3. **Filter the coverage report to added lines only.** Run `git diff <commit-sha>^ <commit-sha> --name-only` to get changed files. Then from the coverage JSON output, extract coverage data only for those files. Within those files, consider only added lines (lines that appear as additions in the diff, not the entire file). This is "diff coverage" -- measuring coverage on new code only.

4. **Apply graduated thresholds:**
   - 80%+ coverage on added lines: PASS
   - 70-79%: WARNING (included in report, does not fail the task)
   - Below 70%: FAIL (triggers fix loop)

5. **Handle missing coverage tools:**
   - If the coverage tool is not installed: report `diff-coverage | SKIP | <tool> not installed` as a WARNING
   - Then attempt auto-install as fallback (e.g., `cargo install cargo-tarpaulin`)
   - If auto-install succeeds, retry the coverage run
   - If auto-install fails, keep the SKIP result

6. **Self-gating:** When the diff contains no production source files (only markdown, config, test files, etc.), report `diff-coverage | SKIP | no production source files in diff`.

7. **Multi-language projects:** Run coverage for the primary language only (the language with the most files in the diff). Do not run two coverage tools.

8. **Output format:** `diff-coverage | PASS | 85% coverage on changed lines` or `diff-coverage | WARN | 74% coverage on changed lines (target: 80%)` or `diff-coverage | FAIL | 55% coverage on changed lines (minimum: 70%)`. Include details listing under-covered files with line ranges that lack coverage.

Also update the output format table at the bottom of the file to include a row for `Diff coverage`.

## Project Context
The ironbench benchmark showed irontodo-v2 had only 50.5% of its test suite actually running, and coverage on new code was likely even lower. This check catches tasks where large sections of new code are completely untested. The 70% hard fail aligns with Chromium's per-CL gate.

## Architecture
This is a new check in the scrutiny validator's check sequence. It uses the `Detected language` named fact from the pre-pass and runs external coverage tools. The coverage tool availability is verified by darkfactory-init (task 6.1, not yet implemented), but this check handles missing tools gracefully with skip-then-attempt-install.

## Key Terms
- **Diff coverage**: Line coverage measured only on added lines in a task's git diff, not the entire codebase.
- **Self-gating check**: A scrutiny check that naturally skips itself when the diff contains no relevant files.

## What Previous Tasks Built

### Task 1.1: Language detection and deterministic pre-passes
- Added `Detected language` named fact

### Tasks 2.1-2.3: Enhanced existing checks 3, 6, 7

### Task 3.1: Silent skip detection (check 12)
- Added check 12 after check 11

## Downstream Consumers
No later tasks depend directly on this check's output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-3.2] Add diff coverage check to scrutiny validator"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-3.2.md

## Handoff Report Format
- Task: Task 3.2 - Add check 13 (diff coverage)
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
