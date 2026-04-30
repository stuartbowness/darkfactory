## Why

Identical PRDs produce materially different mission outputs. Benchmarking two runs against the same 53-user-story PRD showed a 12-point Tier 1 swing (74.9% vs 62.5%) and 5x more HIGH-severity findings. The divergence originates in the planning phases (design, task decomposition, convention establishment), where unconstrained model choices cascade into different architectures, naming, and code quality. The fix is reducing degrees of freedom at the source -- constrain high-leverage decisions through scored rubrics, structural templates, topology gates, and compaction-resistant anchors.

## What Changes

- Add architecture candidate scoring to Phase 4.3 with a four-dimension rubric (separation of concerns, testability, task independence, module depth), concrete per-level criteria, and a hard floor of 1 per dimension
- Add a design template reference document defining mandatory design.md sections (Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points) with openspec config.yaml rules injecting these requirements
- Add a SKILL.md gate verifying all five design sections exist before human review
- Add Phase 4.3a for task strategy approval (decomposition approach, target count, dependency rationale) before task generation
- Add task topology metadata to Phase 4.4 (depends-on, consumes, produces, layers) with a >2 layers flag for human review
- Add deterministic anchors to plan.md -- six one-liner pointers to high-leverage decisions that survive context compaction
- Update contract template to reference spec capability/scenario identifiers instead of row numbers
- Add foundation audit after task 1 checking committed code against design's Module Inventory and Type Catalog
- Add CLAUDE.md + ARCH.md as a mandatory closing task (create if absent, skip if exists) with a template following Claude Code best practices
- Update closing task order: smoke test, cleanup, module shape re-check, CLAUDE.md/ARCH.md generation, documentation, CI (optional)

## Capabilities

### New Capabilities
- `architecture-scoring`: Four-dimension rubric for evaluating architecture candidates with concrete per-level criteria, hard floor enforcement, and tie escalation to human
- `design-structure`: Mandatory design.md sections enforced via openspec config rules, design template reference document, and SKILL.md gate
- `task-strategy`: Explicit task decomposition strategy approval phase before task generation
- `task-topology`: Per-task dependency and surface metadata with architectural layer flagging
- `deterministic-anchors`: Compaction-resistant one-liner pointers in plan.md to high-leverage design decisions
- `spec-traceability`: Contract assertions reference spec capability/scenario identifiers for end-to-end traceability
- `foundation-audit`: Post-task-1 check verifying committed code matches design naming and structure
- `project-conventions`: CLAUDE.md + ARCH.md closing task with template following Claude Code best practices

### Modified Capabilities

(none -- all changes are additive to the mission skill)

## Impact

- `skills/mission/SKILL.md` -- phases 4.3, 4.4, 5, 8 modified; phase 4.3a added
- `skills/mission/templates/` -- plan.md and contract.md modified; design-template.md and claude-md.md added
- `openspec/config.yaml` -- design rules added
- `README.md` -- updated to reflect all changes
- `UBIQUITOUS_LANGUAGE.md` -- new terms added
- No changes to validator prompts, worker briefs (beyond scope), dispatch logic, or the OpenSpec schema
