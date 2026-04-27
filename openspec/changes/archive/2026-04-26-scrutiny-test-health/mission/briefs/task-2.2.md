# Worker Brief: Task 2.2 - Enhance check 6 (module depth) with file size pre-pass

## Your Task

Modify check 6 (Module depth review) in `.claude/skills/mission/prompts/scrutiny-validator.md` to incorporate the file size results from the Deterministic Pre-passes section.

Currently check 6 evaluates module depth qualitatively. After this task, check 6 should also:

1. **Reference the `File sizes` named fact** from the pre-pass section. Before the LLM evaluates module depth, it should read the file size data.

2. **Inject file size context when any touched source file exceeds 900 lines.** When a file in the `File sizes` fact exceeds 900 lines, add explicit instructions for the reviewer to:
   - Name the file and its line count in the module depth evaluation
   - Cross-reference design.md for concern boundaries for that module
   - If design.md specifies concern boundaries that weren't followed: flag as FAIL
   - If design.md says "this is intentionally one file because X": accept as PASS
   - If design.md doesn't address concern boundaries for this module: flag as FAIL (design gap)

3. **Exclude standalone test files from the 900-line check.** Files in `tests/` directories, `*.test.ts`, `*.spec.ts`, `test_*.py`, `*_test.py` should NOT be flagged even if over 900 lines.

4. **Explicitly state that Rust `#[cfg(test)]` blocks inside production files count toward the 900-line threshold.** This is the anti-pattern we're catching -- mixed test/production code in one file.

Add these instructions into the existing check 6 section. Do not remove any existing module depth review guidance -- add to it.

## Project Context
The ironbench benchmark showed a 1,940-line monolith file (jsonrpc.rs) grew across 3 tasks without being flagged by module depth review. The LLM reviewer missed it because it was evaluating depth qualitatively. This enhancement injects a hard number (line count) so the reviewer can't overlook it.

## Architecture
The scrutiny validator prompt has a "Deterministic Pre-passes" section (added in task 1.1) that stores a `File sizes` named fact. This task modifies check 6 to read that fact and use it alongside the existing qualitative module depth evaluation.

## Key Terms
- **File size pre-pass**: A deterministic line count on touched files injected into the module depth review.
- **Self-gating check**: A scrutiny check that naturally skips when the diff has no relevant files.

## What Previous Tasks Built

### Task 1.1: Language detection and deterministic pre-passes
- Modified: .claude/skills/mission/prompts/scrutiny-validator.md (+100 lines)
- Added "## Deterministic Pre-passes" section with 4 named facts including `File sizes`
- File size count: counts lines on touched source files, excludes standalone test files

### Task 2.1: Enhance check 3 (tests) with test runner output parsing
- Modified: check 3 to report structured counts, group failures, warn on low run rate

## Downstream Consumers
No later tasks depend directly on the file size check output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-2.2] Enhance check 6 with file size pre-pass for module depth"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-2.2.md

## Handoff Report Format
- Task: Task 2.2 - Enhance check 6 (module depth) with file size pre-pass
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
