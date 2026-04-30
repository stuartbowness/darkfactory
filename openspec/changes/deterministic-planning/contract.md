# Contract

## Task 1.1: Design Template and Architecture Scoring
<!-- Specs: architecture-scoring/1, architecture-scoring/2, architecture-scoring/3, architecture-scoring/4, design-structure/1, design-structure/2, design-structure/3, design-structure/4 -->
- [ ] [output] `skills/mission/templates/design-template.md` exists and contains headings for all five mandatory sections: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points
- [ ] [output] The Architecture Decision section in the design template contains a four-dimension scoring rubric with per-level criteria (0-3) for: separation of concerns, testability, task independence, module depth
- [ ] [output] Each scoring dimension has concrete criteria for all four levels (0, 1, 2, 3) that reference observable structural properties, not subjective qualities
- [ ] [output] The Module Inventory section defines a table format with columns for module name, interface width, what it hides, and why this boundary exists
- [ ] [output] The Type Catalog section specifies that each type must include name (from UBIQUITOUS_LANGUAGE.md), purpose (one line), and owning module -- with no field-level detail
- [ ] [output] `openspec/config.yaml` contains `rules.design` entries that list the five mandatory design sections
- [ ] [output] SKILL.md Phase 4.3 references `templates/design-template.md` by path
- [ ] [output] SKILL.md Phase 4.3 contains a gate that verifies all five mandatory section headings exist in design.md before presenting it for human review
- [ ] [output] SKILL.md Phase 4.3 gate specifies that no recommended candidate can score 0 on any dimension (hard floor)
- [ ] [output] SKILL.md Phase 4.3 gate specifies that tied candidates are presented to the human for discussion and selection
- [ ] [output] SKILL.md Phase 4.3 distinguishes greenfield/arch-change missions (run scoring) from existing-codebase missions without arch change (document current architecture, skip scoring)
- [ ] [execution] `grep -c "design-template.md" skills/mission/SKILL.md` returns at least 1
- [ ] [execution] `grep -c "rules:" openspec/config.yaml && grep -c "design:" openspec/config.yaml` both return at least 1

## Task 2.1: Spec Traceability in Contract
<!-- Specs: spec-traceability/1 -->
- [ ] [output] `skills/mission/templates/contract.md` uses `<!-- Specs: capability/N -->` format instead of `<!-- PRD Requirements: #N -->`
- [ ] [output] SKILL.md Phase 5 instructions direct the orchestrator to trace each assertion back to a spec capability and scenario number
- [ ] [output] No remaining references to "PRD Requirements" in the contract template
- [ ] [execution] `grep -c "PRD Requirements" skills/mission/templates/contract.md` returns 0
- [ ] [execution] `grep -c "Specs:" skills/mission/templates/contract.md` returns at least 1

## Task 3.1: Deterministic Anchors
<!-- Specs: deterministic-anchors/1, deterministic-anchors/2, deterministic-anchors/3 -->
- [ ] [output] `skills/mission/templates/plan.md` contains a "Deterministic Anchors" section
- [ ] [output] The Deterministic Anchors section contains six fields: workspace layout, task decomposition strategy + target count, primary domain types, error handling strategy, test strategy, chosen architecture
- [ ] [output] Each anchor field is formatted as a one-liner pointer with a placeholder for the source reference
- [ ] [output] SKILL.md Phase 4.3 contains an instruction to populate 5 anchors (all except task strategy) from design.md after human approval
- [ ] [execution] `grep -c "Deterministic Anchors" skills/mission/templates/plan.md` returns at least 1

## Task 4.1: Task Strategy, Topology, and Quality Standard
<!-- Specs: task-strategy/1, task-strategy/2, task-topology/1, task-topology/2, task-topology/3, deterministic-anchors/2 (scenario 2) -->
- [ ] [output] SKILL.md contains a Phase 4.3a section between Phase 4.3 and Phase 4.4
- [ ] [output] Phase 4.3a requires the orchestrator to present: decomposition approach (layered/vertical/hybrid), target task count range, and dependency rationale
- [ ] [output] Phase 4.3a requires human approval before tasks.md is generated
- [ ] [output] Phase 4.3a includes an instruction to populate the task decomposition strategy anchor after approval
- [ ] [output] SKILL.md Phase 4.4 contains a Task Topology subsection requiring four fields per task: Depends on, Consumes, Produces, Layers
- [ ] [output] Phase 4.4 includes the fixed markdown format for topology metadata matching design D5
- [ ] [output] Phase 4.4 includes the >2 layers flag that applies only when the approved strategy is layered or hybrid (vertical strategy is exempt)
- [ ] [output] Phase 4.4 task quality standard says "each task must produce a concrete, testable output" rather than unconditionally requiring vertical slices
- [ ] [output] Phase 4.4 task quality standard references Phase 4.3a for the approved strategy and permits layered tasks when the strategy is layered or hybrid
- [ ] [output] `openspec/config.yaml` `rules.tasks` uses strategy-aware wording instead of unconditionally requiring vertical slices
- [ ] [output] SKILL.md Phase Index includes 4.3a
- [ ] [execution] `grep -c "4.3a" skills/mission/SKILL.md` returns at least 2 (Phase Index + section header)
- [ ] [execution] `grep -c "Depends on:" skills/mission/SKILL.md` returns at least 1

## Task 5.1: CLAUDE.md Closing Task and Template
<!-- Specs: project-conventions/1, project-conventions/2, project-conventions/3, project-conventions/4, project-conventions/5 -->
- [ ] [output] `skills/mission/templates/claude-md.md` exists
- [ ] [output] The template contains an include/exclude guidance table listing: include (commands Claude cannot guess, code style rules differing from defaults, testing instructions, architectural decisions, dev environment quirks, common gotchas) and exclude (anything inferable from code, standard conventions, detailed API docs, file-by-file descriptions, tutorials, self-evident practices)
- [ ] [output] The template contains eight sections: Purpose, Rules, Commands, Repo Map, Testing, Environment, @ARCH.md reference, @UBIQUITOUS_LANGUAGE.md reference
- [ ] [output] The template contains one example with an "adapt to language" note
- [ ] [output] The Rules section of the template includes "update ARCH.md when you change architecture"
- [ ] [output] SKILL.md Phase 4.4 mandatory closing tasks are ordered: smoke test, cleanup, module shape re-check, CLAUDE.md + ARCH.md generation, documentation, CI configuration (optional)
- [ ] [output] The CLAUDE.md closing task description specifies: create CLAUDE.md from template if absent, create ARCH.md from design.md if absent, do not modify existing files
- [ ] [execution] `grep -c "claude-md.md" skills/mission/SKILL.md` returns at least 1

## Task 6.1: Foundation Audit
<!-- Specs: foundation-audit/1, foundation-audit/2, foundation-audit/3 -->
- [ ] [output] SKILL.md Phase 8 Step 7 contains a foundation audit step that runs only after task 1 passes both validators
- [ ] [output] The foundation audit extracts module and type names from task 1's Produces topology field
- [ ] [output] The audit compares extracted module names against design.md's Module Inventory
- [ ] [output] The audit compares extracted type names against design.md's Type Catalog
- [ ] [output] When drift is detected, the audit generates a corrective task 1a: inserts it in tasks.md, adds assertions to contract.md, and presents both to the human for approval
- [ ] [output] When no drift is detected, the audit logs "Foundation audit PASSED" to progress.md
- [ ] [output] The audit explicitly does not run for any task other than task 1
- [ ] [execution] `grep -c "foundation audit" skills/mission/SKILL.md` returns at least 1 (case-insensitive match acceptable)
