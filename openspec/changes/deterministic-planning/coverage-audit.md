# Spec Coverage Audit

## Coverage Table

| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | Evaluate 2-3 architecture candidates against rubric | architecture-scoring / Architecture candidate evaluation / scenarios 1-3 | Task 1.1 | COVERED |
| 2 | Concrete per-level scoring criteria for each dimension | architecture-scoring / Four-dimension scoring rubric / scenarios 1-4 | Task 1.1 | COVERED |
| 3 | Hard floor of 1 on every scoring dimension | architecture-scoring / Hard floor enforcement / scenario 1 | Task 1.1 | COVERED |
| 4 | Tied candidates presented to human | architecture-scoring / Tie escalation to human / scenario 1 | Task 1.1 | COVERED |
| 5 | Scoring only for greenfield or explicit arch changes | architecture-scoring / Architecture candidate evaluation / scenarios 2-3 | Task 1.1 | COVERED |
| 6 | Design includes Module Inventory table | design-structure / Mandatory design sections / Module Inventory scenario | Task 1.1 | COVERED |
| 7 | Design includes Type Catalog | design-structure / Mandatory design sections / Type Catalog scenario | Task 1.1 | COVERED |
| 8 | Design includes Error Strategy | design-structure / Mandatory design sections / Error Strategy scenario | Task 1.1 | COVERED |
| 9 | Design includes Integration Points | design-structure / Mandatory design sections / Integration Points scenario | Task 1.1 | COVERED |
| 10 | Config rules inject design section requirements | design-structure / OpenSpec config rules injection / scenario 1 | Task 1.1 | COVERED |
| 11 | SKILL.md gate verifies five sections exist | design-structure / Design section gate / scenarios 1-4 | Task 1.1 | COVERED |
| 12 | Task strategy approval before tasks.md | task-strategy / Task strategy approval / scenarios 1-3 | Task 4.1 | COVERED |
| 13 | Approved strategy recorded in Deterministic Anchors | deterministic-anchors / Anchors populated incrementally / scenario 2 | Task 4.1 | COVERED |
| 14 | Each task includes topology metadata | task-topology / Per-task topology metadata / scenarios 1-2 | Task 4.1 | COVERED |
| 15 | Tasks >2 layers flagged for review | task-topology / Architectural layer flagging / scenarios 1-3 | Task 4.1 | COVERED |
| 16 | Contract assertions reference spec IDs | spec-traceability / Contract assertions reference spec identifiers / scenarios 1-2 | Task 2.1 | COVERED |
| 17 | Deterministic Anchors in plan.md with six fields | deterministic-anchors / Anchors section in plan template / scenario 1 | Task 3.1 | COVERED |
| 18 | Anchors populated after design approval | deterministic-anchors / Anchors populated incrementally / scenario 1 | Task 3.1 | COVERED |
| 19 | Foundation audit after task 1 | foundation-audit / Foundation audit after task 1 / scenarios 1-2 | Task 6.1 | COVERED |
| 20 | Audit checks against Module Inventory and Type Catalog | foundation-audit / Audit scoped to task 1's declared produces / scenarios 1-2 | Task 6.1 | COVERED |
| 21 | Drift triggers corrective task 1a | foundation-audit / Drift triggers corrective task / scenarios 1-2 | Task 6.1 | COVERED |
| 22 | Mandatory CLAUDE.md closing task | project-conventions / CLAUDE.md as mandatory closing task / scenario 1 | Task 5.1 | COVERED |
| 23 | CLAUDE.md template follows best practices | project-conventions / CLAUDE.md template follows best practices / scenarios 1-2 | Task 5.1 | COVERED |
| 24 | ARCH.md created from design.md if absent | project-conventions / ARCH.md created only if absent / scenarios 1-2 | Task 5.1 | COVERED |
| 25 | Existing CLAUDE.md and ARCH.md left untouched | project-conventions / CLAUDE.md created only if absent / scenario 2 + ARCH.md created only if absent / scenario 2 | Task 5.1 | COVERED |
| 26 | CLAUDE.md template has 8 sections with example | project-conventions / CLAUDE.md created only if absent / scenario 1 | Task 5.1 | COVERED |
| 27 | Closing task order: smoke, cleanup, shape, CLAUDE.md, docs, CI | project-conventions / CLAUDE.md as mandatory closing task / scenario 1 | Task 5.1 | COVERED |

## Gaps

None. All 27 PRD requirements map to spec requirements and tasks.

## Deviations Requiring Approval

None. All requirements are implemented as specified.

## Additional coverage from specs not in PRD

The following spec requirements were added during the grill and design review but are not explicit PRD user stories:

| Spec Requirement | Task | Rationale |
|-----------------|------|-----------|
| design-structure / Design template reference document | Task 1.1 | Needed to house the scoring rubrics and section formats (design D1) |
| design-structure / Architecture candidate scores zero (gate) | Task 1.1 | Enforcement mechanism for PRD #3 (hard floor) |
| design-structure / Architecture candidates tie (gate) | Task 1.1 | Enforcement mechanism for PRD #4 (tie escalation) |
| task-strategy / Task quality standard is strategy-aware | Task 4.1 | Required to resolve contradiction between Phase 4.3a strategy choice and Phase 4.4 vertical-slice rule |
| task-topology / Layers are logical groupings | Task 4.1 | Clarification needed for topology metadata to work with foundation audit |
| task-topology / Vertical strategy exemption from layer flag | Task 4.1 | Without this, vertical strategy tasks would all be flagged |
| deterministic-anchors / Strategy anchor populated after Phase 4.3a | Task 4.1 | Timing fix: strategy doesn't exist until 4.3a approval |

These are all implementation requirements derived from the PRD requirements, not scope additions.
