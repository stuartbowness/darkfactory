# Worker Brief: Task 1.1 - Design Template and Architecture Scoring

## Your Task

Create the design template reference document, add OpenSpec config rules for mandatory design sections, and update SKILL.md Phase 4.3 with the architecture scoring gate.

This task produces three outputs:
1. A new file `skills/mission/templates/design-template.md` defining mandatory design.md sections and scoring rubrics
2. New `rules.design` entries in `openspec/config.yaml`
3. Updates to SKILL.md Phase 4.3 referencing the template and adding the scoring gate

### Detailed requirements:

**design-template.md** must contain five mandatory sections:

1. **Architecture Decision** -- A four-dimension scoring rubric for evaluating 2-3 architecture candidates. Each dimension is scored 0-3 with concrete, language-agnostic per-level criteria:

   **Separation of concerns:**
   - 3: Each responsibility maps to exactly one unit. No unit mixes domain logic with transport, storage, or configuration.
   - 2: Mostly isolated but 1-2 units mix concerns pragmatically.
   - 1: Several units mix concerns. Domain logic tangled with transport or storage in more than one place.
   - 0: No clear separation. Business logic, I/O, and configuration interleaved throughout.

   **Testability:**
   - 3: Each module testable in isolation with swappable dependencies via interfaces or dependency injection.
   - 2: Most modules testable in isolation. One or two require a real external dependency.
   - 1: Tests require significant setup or multiple running services. Modules tightly coupled to dependencies.
   - 0: No practical way to test modules in isolation. Testing requires the full system running.

   **Task independence:**
   - 3: Separately buildable or deployable units. A task can modify one unit without touching others.
   - 2: Explicit interface boundaries within a single build unit with enforced contracts.
   - 1: Directory/namespace separation only. No enforcement beyond developer discipline.
   - 0: Flat structure. No meaningful boundaries. Every task touches shared state.

   **Module depth:**
   - 3: Modules expose small public API relative to implementation. Callers don't need to understand internals.
   - 2: Reasonable interface-to-implementation ratio. One or two modules expose more surface than they hide.
   - 1: Several modules are thin wrappers. Public API nearly as large as implementation.
   - 0: No meaningful abstraction. Public surface and implementation effectively the same.

2. **Module Inventory** -- Table format with columns: Module name, Interface width, What it hides, Why this boundary

3. **Type Catalog** -- Each type listed with: name (from UBIQUITOUS_LANGUAGE.md), purpose (one line), owning module. No field-level detail.

4. **Error Strategy** -- Error type hierarchy, propagation rules, which layer throws vs catches

5. **Integration Points** -- How modules call each other: caller, callee, method/interface, direction

**openspec/config.yaml** -- Add `rules.design` entries listing the five mandatory sections. The current file has `rules.tasks` entries; add a sibling `design` key under `rules`.

**SKILL.md Phase 4.3** -- Add three things:
1. A reference to `templates/design-template.md` by relative path, instructing the orchestrator to follow it
2. A gate before human review that checks all five section headings exist in design.md. If any are missing, regenerate.
3. Gate behavior for architecture scoring:
   - For greenfield missions or missions with explicit architecture changes: require the Architecture Decision section to contain 2-3 scored candidates
   - For existing codebase missions without architecture changes: require documentation of the current architecture and how changes fit, no scoring needed
   - Hard floor: no recommended candidate can score 0 on any dimension. If one does, flag it and recommend against it.
   - Ties: if candidates tie on total score, present both to the human for discussion and selection (no arbitrary tie-breaker)

The gate additions should be inserted AFTER the existing "Present it to the human with a recommendation" line in Phase 4.3, and BEFORE "If approved, proceed to Phase 4.4."

IMPORTANT: Preserve the existing "Module Depth" and "Existing Codebases" subsections in Phase 4.3 unchanged. The new content is additive.

## Project Context

Darkfactory is a multi-agent mission system built as Claude Code skills, markdown templates, and bash scripts. This mission is adding deterministic constraints to the planning pipeline to reduce variance between runs. All changes are to markdown files.

## Architecture

The mission skill lives at `skills/mission/SKILL.md` (837 lines). Templates are in `skills/mission/templates/`. The openspec config is at `openspec/config.yaml`. Phase 4.3 is the design generation and review phase. It currently has subsections for "Module Depth" and "Existing Codebases" that must be preserved.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Architecture Candidate Scoring**: Evaluating 2-3 architecture options against a four-dimension rubric to select deterministically
- **Task Independence**: A scoring dimension measuring how well an architecture allows tasks to be completed without cross-task coordination
- **Design Template**: A reference document defining mandatory design.md sections and scoring rubrics
- **Type Catalog**: A design.md section listing domain types with name, purpose, and owning module
- **Module Inventory**: A design.md section listing modules with interface width, what they hide, and boundary rationale

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

- Task 3.1 (Deterministic Anchors) will reference the design template's Type Catalog and Module Inventory outputs
- Task 4.1 (Task Strategy/Topology) will reference the Phase 4.3 scoring gate
- Task 6.1 (Foundation Audit) will read the Type Catalog and Module Inventory formats to check committed code

## Procedures
- Commit all changes in a single commit with message: "[task-1.1] Add design template, config rules, and architecture scoring gate"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1.1 - Design Template and Architecture Scoring
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
