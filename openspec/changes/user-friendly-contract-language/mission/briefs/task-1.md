# Worker Brief: Task 1 - Rename scrutiny validator steps from "check" to "gate"

## Your Task

Rename all references to numbered scrutiny validator steps from "check" to "gate" in two files:

**File 1: `skills/mission/prompts/scrutiny-validator.md`**
- Rename section header `## Check Sequence` to `## Gate Sequence`
- Rename section header `## Check Results` to `## Gate Results`
- Rename any inline references to the "check sequence" or specific numbered checks (e.g., "check 3") to "gate sequence" / "gate 3"
- The step name "Code standards check" (item 7) should become "Code standards gate"
- Do NOT rename "assertion" to "check" anywhere in this file -- that is a separate task

**File 2: `skills/mission/SKILL.md`**
- Line 594 says "The prompt runs 11 checks: test presence, build, tests, lint, code review, module depth, code standards, simplify, security (if applicable), contract assertions, behavioral conformance." Change "11 checks" to "14 gates" (the actual count in the validator prompt is 14, not 11) and list all 14: test presence, build, tests, lint, code review, module depth, code standards, simplify, security (if applicable), contract assertions, behavioral conformance, silent skip detection, diff coverage, test quality review.
- Change any other references to validator "checks" (in the validator-step sense) to "gates" in SKILL.md. Do NOT rename "assertion" to "check" in this file -- that is a separate task.

**What NOT to rename:**
- Do not rename "assertion" or "assertions" anywhere -- that is handled by Task 2 and Task 3
- Do not rename "check" when used as a verb (e.g., "check if", "check whether", "check for")
- Do not rename "check" in the phrase "re-check" (e.g., "module shape re-check")
- Do not rename "Final gates" -- that term already uses "gate"

**Verification:** After changes, `grep -n "check [0-9]" skills/mission/prompts/scrutiny-validator.md` should return zero results. `grep -n "Check Sequence\|Check Results\|check sequence" skills/mission/prompts/scrutiny-validator.md` should return zero results.

## Project Context

Darkfactory is renaming user-facing mission contract language. The word "check" was previously used for scrutiny validator steps, creating ambiguity with "check" as a contract item term. This task renames validator steps to "gate" to free up "check" for contract items in subsequent tasks.

## Architecture

The scrutiny validator prompt (`skills/mission/prompts/scrutiny-validator.md`) defines 14 numbered steps that a Claude subagent executes when validating a worker's output. Each step is pass/fail (with WARN, SKIP, N/A also possible). The prompt is consumed only by validator subagents, never by workers.

`skills/mission/SKILL.md` is the orchestrator's operating manual (~800 lines). It references validator steps when describing what the scrutiny validator does.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Gate**: A numbered step in the scrutiny validator's sequence (e.g., gate 3: tests, gate 10: contract checks). Gates report PASS, FAIL, WARN, SKIP, or N/A.
- **Check**: A single verifiable statement in a contract (NOT used in this task -- this task only renames validator steps to "gate")
- **Scrutiny validator**: The first validator type that runs build, test, lint, code review, code simplify, and test health gates against the contract

## Domain Types

- Check: A single verifiable statement in a contract (owning module: glossary) -- NOT relevant to this task
- Gate: A numbered step in the scrutiny validator's sequence (owning module: glossary) -- the term being introduced
- Contract: A set of checks grouped by task, consumed by validators (owning module: glossary)

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

Task 2 and Task 3 depend on this task completing first. They will rename "assertion" to "check" for contract items, which requires that validator steps no longer use "check" (to avoid collision). Minimum integration test: `grep -n "check [0-9]" skills/mission/prompts/scrutiny-validator.md` returns zero results after this task.

## Procedures
- Read the two files thoroughly before making changes
- Make all changes in a single commit with message: "[task-1] Rename scrutiny validator steps from check to gate"
- Write your handoff report to openspec/changes/user-friendly-contract-language/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Rename scrutiny validator steps from "check" to "gate"
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
