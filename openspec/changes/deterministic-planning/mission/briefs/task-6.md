# Worker Brief: Task 6.1 - Foundation Audit

## Your Task

Add a foundation audit step to SKILL.md Phase 8 Step 7 that runs after task 1 passes both validators.

This task produces one change:
1. Insert a foundation audit step in SKILL.md Phase 8 Step 7

### Detailed requirements:

**SKILL.md Phase 8 Step 7** -- Find Step 7 ("Track Progress") in Phase 8. Between item 4 ("Update the plan: set Current Task to the next unchecked task") and item 5 ("If more tasks remain: Go back to Step 1 for the next task"), insert:

```
4a. **Foundation audit (after task 1 only).** If this is task 1 and it passed both validators, run a foundation check before proceeding to task 2:

   - Read task 1's Produces field from its topology metadata in tasks.md
   - Extract module names from the Produces field. Look up those modules in design.md's Module Inventory. Verify that corresponding directories or files exist in the committed code with matching names.
   - Extract type names from the Produces field. Look up those types in design.md's Type Catalog. Verify that corresponding type definitions exist in the committed code with matching names.

   If drift is detected (module structure or type names do not match the design for items task 1 declared it would produce):
   - Generate a corrective task 1a
   - Insert it in tasks.md between task 1 and task 2
   - Add assertions for it to contract.md
   - Present both to the human for approval
   - The corrective task goes through the normal build loop before task 2 starts

   If no drift: log to progress.md: `<timestamp> -- Foundation audit PASSED. Module structure and type names confirmed for task 1 produces.`

   This audit runs only after task 1. It does not run after any other task.
```

The numbering of subsequent items stays the same (item 5 "If more tasks remain" keeps its number).

IMPORTANT: This step must be clearly scoped to task 1 only. The condition "If this is task 1" must be explicit so the orchestrator does not accidentally run the audit after every task.

## Project Context

Darkfactory missions sometimes drift from the design during the very first task. In the v3/v5 benchmark, v5's first task established type names that didn't match the design, and every subsequent task inherited those wrong names. The foundation audit catches this drift at the earliest possible point.

## Architecture

SKILL.md Phase 8 Step 7 ("Track Progress") is a numbered list of items the orchestrator runs after each task passes validation. The current items are: 1. Mark task complete in tasks.md, 2. Generate codebase delta, 3. Append to progress.md, 4. Update the plan, 5. If more tasks remain → Step 1, 6. If all tasks done → Phase 9.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Foundation Audit**: A one-time check after task 1 passes validation, verifying committed code matches the design's Module Inventory and Type Catalog for task 1's declared produces
- **Task Topology**: Per-task metadata including the Produces field that lists surfaces the task creates
- **Type Catalog**: A design.md section listing domain types with name, purpose, and owning module
- **Module Inventory**: A design.md section listing modules with interface width, what they hide, and boundary rationale

## What Previous Tasks Built

### Task 1.1: Design Template and Architecture Scoring
- Created: skills/mission/templates/design-template.md (Type Catalog and Module Inventory formats defined)
- Modified: SKILL.md Phase 4.3 (scoring gate)

### Task 4.1: Task Strategy, Topology, Quality Standard
- Added: SKILL.md Phase 4.4 Task Topology subsection (fixed format with Produces field)

## Downstream Consumers

No downstream tasks depend on this. This is the last feature task.

## Procedures
- Commit all changes in a single commit with message: "[task-6.1] Add foundation audit after task 1 in Phase 8"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-6.md

## Handoff Report Format
- Task: Task 6.1 - Foundation Audit
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
