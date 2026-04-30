# Worker Brief: Task 4.1 - Task Strategy, Topology, and Quality Standard

## Your Task

Add Phase 4.3a (task strategy approval), update Phase 4.4 (task topology + strategy-aware quality standard), update config.yaml rules.tasks, and update the Phase Index.

This task produces four changes:
1. New Phase 4.3a section in SKILL.md
2. Updated Phase 4.4 in SKILL.md (topology subsection + quality standard rewrite)
3. Updated `openspec/config.yaml` rules.tasks
4. Updated Phase Index in SKILL.md

### Detailed requirements:

**Phase 4.3a: Task Strategy Approval** -- Insert a new subsection in SKILL.md between the end of Phase 4.3 (after the anchor population instruction added by Task 3.1) and the start of Phase 4.4.

```
### Phase 4.3a: Task Strategy Approval

Before generating tasks.md, present a task decomposition strategy for human approval:

1. **Decomposition approach:** layered (types and domain logic first, then transport/CLI), vertical (each task cuts end-to-end through all layers), or hybrid (foundation layer first, then vertical slices). State which and why.
2. **Target task count range:** e.g., "6-8 feature tasks + 4-5 closing tasks." Justify the range based on the module inventory from design.md.
3. **Dependency ordering rationale:** which tasks must come first and why (e.g., "Task 1 defines core types that all subsequent tasks import").

The human approves this strategy before tasks.md is generated. If rejected, revise and re-present.

After approval, populate the task decomposition strategy anchor in the plan's Deterministic Anchors section.
```

**Phase 4.4 Task Topology** -- Add a new subsection called "Task Topology" after the existing "Task Quality Standard" subsection in Phase 4.4. Content:

```
#### Task Topology

Each task in tasks.md must include topology metadata in this fixed format:

```
- [ ] Task N: <name>
  Depends on: <prior task numbers, or "none">
  Consumes: <surfaces from prior tasks -- types, interfaces, endpoints>
  Produces: <surfaces this task creates -- types, modules, interfaces, endpoints>
  Layers: <logical architectural layers touched, e.g. domain, transport, storage>
```

Layers are logical groupings of modules (domain, transport, storage, configuration, etc.) defined in the design narrative, not columns from the Module Inventory table.

When the approved strategy (Phase 4.3a) is layered or hybrid and a task's Layers field lists more than 2 architectural layers, flag the task for human review with a recommendation to split or approve as-is. When the approved strategy is vertical, the layer count flag does not apply since vertical tasks are expected to cross all layers.
```

**Phase 4.4 Task Quality Standard** -- Replace the current text that says:

"Each task must be a **vertical slice**: a thin end-to-end path through all layers (schema, API, UI, tests) that delivers narrow but complete functionality."

With:

"Each task must produce a concrete, testable output. When the approved strategy (Phase 4.3a) is vertical, tasks cross all layers. When the strategy is layered or hybrid, tasks may be scoped to a single layer, provided each task still has a clear done state and a test surface."

Keep the rest of the Task Quality Standard subsection (the paragraph about "One task = one worker = one commit = one validation cycle" and the paragraph about public surface area) unchanged.

**openspec/config.yaml rules.tasks** -- Replace the first rule:

Old: `- Each task should be a vertical slice -- a thin but complete path through all layers that is demoable and verifiable on its own`

New: `- Each task must produce a concrete, testable output. When the approved strategy is vertical, tasks cross all layers. When layered or hybrid, tasks may be scoped to a single layer with a clear done state and test surface.`

Keep the second rule ("One task = one worker = one commit = one validation cycle") unchanged.

**Phase Index** -- Add `4.3a. **Task Strategy Approval** -- approve decomposition approach before generating tasks` after the line for Phase 4.3 (Create and Review OpenSpec Artifacts) or after 4a (Spec Coverage Audit), wherever the index currently lists phases. Find the Phase Index at the top of SKILL.md (around lines 22-34).

## Project Context

Darkfactory is a multi-agent mission system. This task makes three related changes: (1) the human explicitly approves how tasks are decomposed before they're generated, (2) each task carries dependency/surface metadata for reviewability, and (3) the task quality standard becomes strategy-aware instead of hardcoding vertical slices.

## Architecture

SKILL.md Phase 4.4 starts with "Generate `tasks.md`" followed by the "Task Quality Standard" subsection, then "Mandatory Closing Tasks." The new Task Topology subsection goes between Task Quality Standard and the paragraph about public surface area. Phase 4.3a is an entirely new section inserted between Phase 4.3 and Phase 4.4. The Phase Index is at the top of the file (look for "## Phase Index").

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Task Topology**: Per-task metadata declaring dependencies, consumed surfaces, produced surfaces, and layers touched
- **Task Independence**: How well an architecture allows tasks to be completed without cross-task coordination
- **Deterministic Anchors**: One-liner pointers in the plan to high-leverage decisions

## What Previous Tasks Built

### Task 1.1: Design Template and Architecture Scoring
- Created: skills/mission/templates/design-template.md (88 lines)
- Modified: openspec/config.yaml (added rules.design)
- Modified: SKILL.md Phase 4.3 (template reference, scoring gate)

### Task 2.1: Spec Traceability in Contract
- Modified: skills/mission/templates/contract.md (Specs: format)
- Modified: SKILL.md Phase 5 (spec reference instructions)

### Task 3.1: Deterministic Anchors
- Modified: skills/mission/templates/plan.md (added Deterministic Anchors section with 6 fields)
- Modified: SKILL.md Phase 4.3 (added anchor population instruction after gate, before "If approved, proceed to Phase 4.4")

## Downstream Consumers

- Task 6.1 (Foundation Audit) reads the Produces field from task topology metadata
- The build loop uses topology metadata for worker brief "Downstream Consumers" section

## Procedures
- Commit all changes in a single commit with message: "[task-4.1] Add Phase 4.3a, task topology, and strategy-aware quality standard"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-4.md

## Handoff Report Format
- Task: Task 4.1 - Task Strategy, Topology, and Quality Standard
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
