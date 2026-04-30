# Worker Brief: Task 3.1 - Deterministic Anchors

## Your Task

Add a Deterministic Anchors section to the plan template and wire it into SKILL.md Phase 4.3 so the orchestrator populates 5 anchors after design approval.

This task produces two changes:
1. Add a new section to `skills/mission/templates/plan.md`
2. Add an instruction to SKILL.md Phase 4.3

### Detailed requirements:

**templates/plan.md** -- Add a "Deterministic Anchors" section after the "Engine Config" section and before the "Skills to Use" section. The section contains six one-liner pointer fields:

```
## Deterministic Anchors
Filled during Phase 4.3 (5 anchors) and Phase 4.3a (task strategy anchor). Every worker brief references these. They survive context compaction.

- Workspace layout: <e.g., "two-crate workspace: core lib + cli binary" (see design.md Architecture Decision)>
- Task decomposition strategy: <e.g., "layered, 7 feature tasks + 5 closing" (see Phase 4.3a approval)>
- Primary domain types: <e.g., "Todo, TodoStatus, IronTodoError" (see design.md Type Catalog)>
- Error handling strategy: <e.g., "thiserror in core, never anyhow" (see design.md Error Strategy)>
- Test strategy: <e.g., "unit + integration, serial with DB" (see design.md)>
- Chosen architecture: <e.g., "workspace-with-lib-and-bin, scored 10/12" (see design.md Architecture Decision)>
```

Each field is a one-liner with a placeholder value in angle brackets showing the expected format, and a parenthetical reference to the source document.

**SKILL.md Phase 4.3** -- After the existing gate block (which was added by Task 1.1 and ends with the tie-breaking rule), add an instruction:

"After the human approves design.md, populate 5 of the 6 Deterministic Anchors in the plan: workspace layout, primary domain types, error handling strategy, test strategy, and chosen architecture. Source values from design.md's Architecture Decision, Type Catalog, and Error Strategy sections. Each anchor is a one-liner pointer with a source reference. The sixth anchor (task decomposition strategy) is populated in Phase 4.3a after the human approves the task strategy."

This instruction goes BEFORE the "If approved, proceed to Phase 4.4" line.

## Project Context

Darkfactory is a multi-agent mission system. Deterministic Anchors are one-liner pointers to the six highest-leverage design decisions, positioned in the plan to survive context compaction. They ensure the orchestrator can recover key decisions without re-reading the full design.

## Architecture

The plan template is at `skills/mission/templates/plan.md` (76 lines). Phase 4.3 of SKILL.md now includes the design template reference and scoring gate (added by Task 1.1). The anchor population instruction should go after the gate block and before the Phase 4.4 transition.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Deterministic Anchors**: One-liner pointers in the plan to the six highest-leverage design decisions, positioned to survive context compaction
- **Plan mode**: The Claude Code plan that serves as the orchestrator's durable operating manual, guaranteed to survive context compaction

## What Previous Tasks Built

### Task 1.1: Design Template and Architecture Scoring
- Created: skills/mission/templates/design-template.md (88 lines)
- Modified: openspec/config.yaml (added rules.design)
- Modified: skills/mission/SKILL.md Phase 4.3 (template reference, 5-section gate, scoring gate)

### Task 2.1: Spec Traceability in Contract
- Modified: skills/mission/templates/contract.md (Specs: format)
- Modified: skills/mission/SKILL.md Phase 5 (spec reference instructions)

## Downstream Consumers

- Task 4.1 will add the Phase 4.3a instruction that populates the 6th anchor (task strategy)
- Worker briefs reference anchors for context during the build loop

## Procedures
- Commit all changes in a single commit with message: "[task-3.1] Add deterministic anchors to plan template and Phase 4.3"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3.1 - Deterministic Anchors
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
