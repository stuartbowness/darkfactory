# Worker Brief: Task 5.1 - Add check 14 (test quality review) to scrutiny validator

## Your Task

Add a new check 14 titled "Test quality review" to `.claude/skills/mission/prompts/scrutiny-validator.md`. Place it after check 13 (Diff coverage), before the Output Format section.

Check 14 should:

1. **Invoke the `/test-quality-review` skill as a subagent.** Follow the same delegation pattern used by check 5 (`/review`), check 8 (`/simplify`), and check 9 (`/security-review`). The skill reads the full diff and outputs structured markdown findings.

2. **Read the skill's markdown findings.** The output contains findings grouped by severity (P0, P1, P2, P3) with file:line, evidence, and remediation for each.

3. **Apply severity-based fail behavior:**
   - Any P0 finding (tautological assertion): FAIL
   - 3 or more P1 findings (untested critical branches, behavioral changes without tests): FAIL
   - P2 and P3 findings only: WARNING (included in report, does not fail the task)
   - No findings: PASS

4. **Self-gating:** When the diff contains no test files, report `test-quality-review | SKIP | no test files in diff`.

5. **Output format:** `test-quality-review | PASS | no findings` or `test-quality-review | WARN | 2 P2, 1 P3 findings` or `test-quality-review | FAIL | 1 P0 finding (tautological assertion in test_list_open)`. Include a summary of findings in the notes column.

Also update the output format table at the bottom of the file to include a row for `Test quality review`.

## Project Context
This is the LLM-based semantic layer that catches what deterministic checks miss -- tests that run, cover code, but don't verify anything meaningful. Inspired by the compound-engineering-plugin's ce-testing-reviewer persona.

## Architecture
The /test-quality-review skill exists at ~/.claude/skills/test-quality-review/SKILL.md (created in task 4.1). The scrutiny validator invokes it as a subagent following the same pattern as /review and /simplify. The skill outputs structured markdown findings with P0-P3 severity.

## Key Terms
- **Test quality review**: An LLM-based semantic review that evaluates whether tests verify meaningful behavior.
- **Tautological assertion**: A test assertion that passes regardless of code correctness.

## What Previous Tasks Built

### Tasks 1.1-3.2: All deterministic checks and pre-passes
- The scrutiny validator now has checks 1-13 with pre-passes

### Task 4.1: Test quality review skill
- Created /test-quality-review skill in layercodedev/skills repo
- 5 evaluation dimensions, P0-P3 severity, structured markdown output

## Downstream Consumers
No later tasks depend directly on this check's output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-5.1] Wire test quality review into scrutiny validator as check 14"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-5.1.md

## Handoff Report Format
- Task: Task 5.1 - Add check 14 (test quality review)
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
