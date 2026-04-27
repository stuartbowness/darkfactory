# Worker Brief: Task 2.3 - Enhance check 7 (code standards) with error handling smell density

## Your Task

Modify check 7 (Code standards check) in `.claude/skills/mission/prompts/scrutiny-validator.md` to incorporate the error handling smell density results from the Deterministic Pre-passes section.

Currently check 7 lists specific anti-patterns to flag (backwards-compat code, fallback lookups, belt-and-suspenders, feature flags, defensive impossible states). After this task, check 7 should also:

1. **Reference the `Error handling smells` named fact** from the pre-pass section. Before reviewing code standards, read the smell data.

2. **Inject the smell count and locations into the review.** Tell the LLM to evaluate the smell count per KLOC with graduated thresholds:
   - Under 5 per KLOC: PASS for the smell density component
   - 5-10 per KLOC: WARNING
   - Over 10 per KLOC: FAIL

3. **Instruct the reviewer to examine each flagged location** and provide file:line references with specific remediation guidance. For example: "Replace `.unwrap()` at store.rs:42 with `.context("failed to open connection pool")?` or `.expect("pool initialized in main")`."

4. **Explicitly state that test files are excluded** from the smell count (the pre-pass already excludes them, but check 7 should confirm this so the reviewer doesn't double-count).

Add these instructions into the existing check 7 section, before or after the existing anti-pattern list. Do not remove any existing code standards guidance.

## Project Context
The ironbench benchmark showed irontodo-v2 had 115 unwrap/panic matches at 15.0 per KLOC. The scrutiny validator's code standards check is supposed to catch sloppy error handling, but it relies on LLM judgment which missed the high density. Injecting a hard count ensures the reviewer can't overlook it.

## Architecture
The scrutiny validator prompt has a "Deterministic Pre-passes" section (added in task 1.1) that stores an `Error handling smells` named fact with count, locations, and per-KLOC density. This task modifies check 7 to read that fact and use it alongside the existing anti-pattern review.

## Key Terms
- **Error handling smell**: A language-specific anti-pattern in error handling code counted per KLOC of changed code.

## What Previous Tasks Built

### Task 1.1: Language detection and deterministic pre-passes
- Added `Error handling smells` named fact with per-language patterns and safe-pattern exclusions

### Task 2.1: Enhance check 3 with test runner output parsing
- Check 3 now reports structured counts and groups failures

### Task 2.2: Enhance check 6 with file size pre-pass
- Check 6 now reads `File sizes` fact and flags files over 900 lines

## Downstream Consumers
No later tasks depend directly on this check's output.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-2.3] Enhance check 7 with error handling smell density"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-2.3.md

## Handoff Report Format
- Task: Task 2.3 - Enhance check 7 (code standards) with error handling smell density
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
