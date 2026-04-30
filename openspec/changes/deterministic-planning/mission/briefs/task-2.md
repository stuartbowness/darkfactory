# Worker Brief: Task 2.1 - Spec Traceability in Contract

## Your Task

Update the contract template and SKILL.md Phase 5 instructions to use spec capability/scenario references instead of PRD row numbers.

This task produces two changes:
1. Update `skills/mission/templates/contract.md` to use `<!-- Specs: capability/N -->` format
2. Update SKILL.md Phase 5 instructions to reference spec capability/scenario IDs

### Detailed requirements:

**templates/contract.md** -- Change the comment format from:
```
<!-- PRD Requirements: #1, #3 -->
```
to:
```
<!-- Specs: capability/N, capability/M -->
```

Where `capability` is the spec folder name (e.g., `add-todo`, `search`) and `N` is the scenario number within that spec file. Update both example task sections in the template.

Make sure NO references to "PRD Requirements" remain in the contract template.

**SKILL.md Phase 5** -- Update the contract writing instructions:
1. Where the instructions currently say to reference "PRD requirements" in the contract comments, change to "spec capability/scenario identifiers"
2. The cross-referencing section should direct the orchestrator to trace each assertion back to a specific spec capability and scenario number
3. Update the contract template reference format description to match the new `<!-- Specs: -->` format
4. Check ALL of Phase 5 for any remaining "PRD Requirements" references and update them to use "Specs" instead

IMPORTANT: The coverage audit (Phase 4a) and its template are NOT changed by this task. The coverage audit still maps PRD requirements to specs and tasks. Only the contract template and Phase 5 instructions change.

## Project Context

Darkfactory is a multi-agent mission system. This mission adds deterministic constraints to reduce planning variance. The contract is the validation document that workers never see -- it defines "done" for each task with assertions grouped by task.

## Architecture

The contract template is at `skills/mission/templates/contract.md` (12 lines). Phase 5 of `skills/mission/SKILL.md` handles contract writing (starts around line 290). The Phase 5 section includes: Contract Rules, Cross-Referencing the PRD, Contract Template description, Contract Review, and Contract Testability Pre-flight.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Spec Reference**: A capability/scenario identifier (e.g., add-todo/1) used to trace contract assertions back to formal spec requirements
- **Contract**: A set of assertions defining "done" independently of implementation, consumed only by validators
- **Assertion**: A single verifiable statement in a contract that a validator can check from outside the code

## What Previous Tasks Built

### Task 1.1: Design Template and Architecture Scoring
- Created: skills/mission/templates/design-template.md (88 lines, 5 mandatory sections with scoring rubrics)
- Modified: openspec/config.yaml (added rules.design with 5 entries)
- Modified: skills/mission/SKILL.md Phase 4.3 (added template reference, 5-section gate, architecture scoring gate with hard floor and tie escalation)

## Downstream Consumers

- Phase 5 contract writing uses this format for every future mission
- Validators read contract assertions with the new spec reference comments

## Procedures
- Commit all changes in a single commit with message: "[task-2.1] Update contract template and Phase 5 to use spec references"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2.1 - Spec Traceability in Contract
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
