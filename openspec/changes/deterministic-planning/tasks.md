## 1. Design Template and Architecture Scoring

- [x] 1.1 Create `templates/design-template.md` with five mandatory sections (Architecture Decision with four-dimension scoring rubric and per-level criteria, Module Inventory table format, Type Catalog format, Error Strategy, Integration Points). Add `rules.design` entries to `openspec/config.yaml` for the five mandatory sections. Update SKILL.md Phase 4.3 to reference the design template, add the architecture scoring gate (verify five sections exist, enforce hard floor of 1 per dimension, escalate ties to human), and distinguish greenfield/arch-change missions from existing-codebase missions.
  Depends on: none
  Consumes: none
  Produces: templates/design-template.md, config.yaml rules.design, Phase 4.3 scoring gate
  Layers: templates, config, orchestrator

## 2. Spec Traceability in Contract

- [x] 2.1 Update `templates/contract.md` to use spec capability/scenario references (`<!-- Specs: capability/N -->`) instead of PRD row numbers (`<!-- PRD Requirements: #N -->`). Update SKILL.md Phase 5 instructions to direct the orchestrator to trace each assertion back to a spec capability and scenario number. Update all Phase 5 references to "PRD Requirements" to use "Specs" instead.
  Depends on: none
  Consumes: none
  Produces: contract.md spec reference format, Phase 5 spec traceability instructions
  Layers: templates, orchestrator

## 3. Deterministic Anchors

- [x] 3.1 Add Deterministic Anchors section to `templates/plan.md` after Engine Config with six one-liner pointer fields (workspace layout, task decomposition strategy + target count, primary domain types, error handling strategy, test strategy, chosen architecture). Add instruction to SKILL.md Phase 4.3: after human approves design, populate 5 anchors from design.md.
  Depends on: Task 1.1
  Consumes: Phase 4.3 scoring gate (Task 1.1)
  Produces: plan.md Deterministic Anchors section, Phase 4.3 anchor population instruction
  Layers: templates, orchestrator

## 4. Task Strategy, Topology, and Quality Standard

- [x] 4.1 Add Phase 4.3a (task strategy approval) to SKILL.md between Phase 4.3 and Phase 4.4: orchestrator presents decomposition approach (layered/vertical/hybrid), target task count range, and dependency rationale for human approval. After approval, populate the task decomposition strategy anchor in plan.md. Add task topology subsection to Phase 4.4: each task must include Depends on, Consumes, Produces, and Layers fields in the fixed markdown format from design D5. Add >2 layers flag with vertical strategy exemption. Update Phase 4.4 task quality standard from "each task must be a vertical slice" to the strategy-aware wording from design D6. Update `openspec/config.yaml` `rules.tasks` to match the strategy-aware wording. Update SKILL.md Phase Index to include 4.3a.
  Depends on: Task 3.1
  Consumes: Deterministic Anchors section (Task 3.1), Phase 4.3 anchor population (Task 3.1)
  Produces: Phase 4.3a, Phase 4.4 topology requirements, strategy-aware task quality standard, Phase Index update
  Layers: orchestrator, config

## 5. CLAUDE.md Closing Task and Template

- [x] 5.1 Create `templates/claude-md.md` with include/exclude guidance table (from Claude Code best practices), eight sections (Purpose, Rules, Commands, Repo Map, Testing, Environment, @ARCH.md reference, @UBIQUITOUS_LANGUAGE.md reference), one example with "adapt to language" note. Rules section must include "update ARCH.md when you change architecture." Update SKILL.md Phase 4.4 mandatory closing tasks: add CLAUDE.md + ARCH.md generation between module shape re-check and documentation. The closing task creates CLAUDE.md and ARCH.md only if they do not already exist; existing files are not modified. ARCH.md is extracted from design.md as a living document.
  Depends on: none
  Consumes: none
  Produces: templates/claude-md.md, updated closing task order in Phase 4.4
  Layers: templates, orchestrator

## 6. Foundation Audit

- [x] 6.1 Add foundation audit to SKILL.md Phase 8 Step 7: after task 1 passes both validators, extract module and type names from task 1's Produces topology field, compare against design.md Module Inventory and Type Catalog. If drift detected, generate corrective task 1a (insert in tasks.md, add assertions to contract.md, present to human for approval). If no drift, log "Foundation audit PASSED" to progress.md. Audit runs only after task 1, not after other tasks.
  Depends on: Task 1.1, Task 4.1
  Consumes: Type Catalog format (Task 1.1), topology Produces field format (Task 4.1)
  Produces: Phase 8 foundation audit step
  Layers: orchestrator

## 7. Closing: Smoke Test and Structural Verification

- [x] 7.1 Run structural verification: confirm Phase Index headings in SKILL.md match actual phase headers; confirm every `templates/*.md` link in SKILL.md resolves to an actual file; confirm contract template and Phase 5 use consistent spec reference format; confirm every phase gate has checkboxes for all listed outputs; confirm SKILL.md is under 950 lines; confirm config.yaml rules.design entries match design template section headings.

## 8. Closing: Cleanup

- [x] 8.1 Remove any unused files, leftover scaffolding, or temporary artifacts introduced during the build.

## 9. Closing: Module Shape Re-check

- [x] 9.1 Re-read design.md's file change map. Compare each listed file against what was actually modified. Document any drift in `mission/validations/module-shape-audit.md`: files that were planned but not modified, files modified but not planned, and any meaningful divergence in line counts.

## 10. Closing: Documentation

- [x] 10.1 Update `README.md` to reflect all changes: architecture candidate scoring in Stage 1, task strategy approval, spec traceability, CLAUDE.md/ARCH.md closing task, foundation audit in Stage 2, design-template.md and claude-md.md in the templates list. Update `UBIQUITOUS_LANGUAGE.md`: update Vertical Slice definition to reflect strategy-aware task quality standard, update anchor timing to reflect 5+1 split, verify all new terms from Phase 2 are still accurate after implementation.
  Depends on: Tasks 1.1-6.1
  Consumes: all prior task outputs
  Produces: updated README.md, updated UBIQUITOUS_LANGUAGE.md
  Layers: documentation
