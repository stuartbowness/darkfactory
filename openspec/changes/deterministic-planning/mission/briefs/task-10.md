# Worker Brief: Task 10.1 - Documentation

## Your Task

Update README.md and UBIQUITOUS_LANGUAGE.md to reflect all changes made by this mission.

### README.md changes:

In the "Stage 1: You and the orchestrator plan together" section (~line 67-84):

1. After the **Design** bullet point (~line 77), add a sentence: "The design evaluates 2-3 architecture candidates against a four-dimension rubric (separation of concerns, testability, task independence, module depth) and picks the highest-scoring option. A mandatory gate verifies the design includes an architecture decision, module inventory, type catalog, error strategy, and integration points."

2. After the **Tasks** bullet point (~line 78), add: "Before generating tasks, the orchestrator presents a decomposition strategy (layered, vertical, or hybrid) with a target task count for your approval. Each task includes topology metadata -- which prior tasks it depends on, what it consumes, and what it produces."

3. After the coverage audit paragraph (~line 82), add: "Contract assertions trace back to specific spec capability and scenario identifiers for end-to-end traceability."

4. In the closing tasks paragraph (~line 80), after "documentation task", add "a CLAUDE.md + ARCH.md generation task (creates project convention files if they don't already exist),"

In the "Stage 2" section (~line 86-107):

5. After the first paragraph about workers, add: "After the first task passes validation, the orchestrator runs a foundation audit -- checking that the committed code's module structure and type names match the design. This catches naming drift before it cascades into subsequent tasks."

In the "Skill structure" section (~line 190-194):

6. Add `design-template.md` and `claude-md.md` to the templates list.

### UBIQUITOUS_LANGUAGE.md changes:

1. Update the **Vertical slice** definition (line 20) from the unconditional "each task should be a thin tracer-bullet path through all layers" to reflect the strategy-aware standard: "The default task quality standard. Each task must produce a concrete, testable output. When the approved strategy is vertical, tasks cross all layers. When layered or hybrid, tasks may be scoped to a single layer with a clear done state and test surface."

2. In the Tasks definition (line 71), change "(should follow the vertical slice best practice)" to "(should follow the approved task quality standard)"

3. Verify all new terms added in Phase 2 (Deterministic Anchors, Architecture Candidate Scoring, Task Independence, Task Topology, Foundation Audit, Type Catalog, Module Inventory, Design Template, Spec Reference) are still accurate after implementation.

## Project Context

This is the documentation closing task for a mission that added deterministic planning constraints to the darkfactory mission orchestrator.

## Architecture

README.md is 301 lines. UBIQUITOUS_LANGUAGE.md is ~155 lines (updated in Phase 2 with new terms). Both are at the repo root.

## Procedures
- Commit all changes (README.md, UBIQUITOUS_LANGUAGE.md, and CLAUDE.md which was modified by a linter) in a single commit with message: "[task-10.1] Update documentation for deterministic planning changes"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-10.md

## Handoff Report Format
- Task: Task 10.1 - Documentation
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
