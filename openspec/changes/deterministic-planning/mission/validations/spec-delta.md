# Spec Delta Check

## Coverage Audit Verification

| # | PRD Requirement | Status | Evidence |
|---|----------------|--------|----------|
| 1 | Evaluate 2-3 architecture candidates against rubric | VERIFIED | SKILL.md Phase 4.3 gate (line 193): "verify the Architecture Decision section contains 2-3 scored architecture candidates using the template rubric." design-template.md has the scoring table with Candidate A/B rows. |
| 2 | Concrete per-level scoring criteria for each dimension | VERIFIED | design-template.md contains four scoring rubrics (Separation of Concerns, Testability, Task Independence, Module Depth) each with 0-3 level criteria in table format. |
| 3 | Hard floor of 1 on every scoring dimension | VERIFIED | SKILL.md line 195: "Enforce the hard floor: no recommended candidate can score 0 on any scoring dimension. If a candidate scores 0 on any dimension, flag it and recommend against it." |
| 4 | Tied candidates presented to human | VERIFIED | SKILL.md line 196: "If candidates tie on total score, present the tied candidates to the human for discussion and selection. Do not apply an arbitrary tie-breaker." design-template.md also states: "If candidates tie on total score, present the tied candidates and their tradeoffs for human selection." |
| 5 | Scoring only for greenfield or explicit arch changes | VERIFIED | SKILL.md lines 193-194: greenfield/explicit arch changes require scored candidates; existing codebase missions without architecture changes document the current architecture instead. |
| 6 | Design includes Module Inventory table | VERIFIED | design-template.md has "## Module Inventory" section with table format (Module name, Interface width, What it hides, Why this boundary). config.yaml rule: "design.md must include a section heading for Module Inventory". SKILL.md Phase 4.3 gate verifies it. |
| 7 | Design includes Type Catalog | VERIFIED | design-template.md has "## Type Catalog" section with table format (Type name, Purpose, Owning module). config.yaml rule: "design.md must include a section heading for Type Catalog". SKILL.md Phase 4.3 gate verifies it. |
| 8 | Design includes Error Strategy | VERIFIED | design-template.md has "## Error Strategy" section defining error type hierarchy, propagation rules, and layer responsibilities. config.yaml rule: "design.md must include a section heading for Error Strategy". SKILL.md Phase 4.3 gate verifies it. |
| 9 | Design includes Integration Points | VERIFIED | design-template.md has "## Integration Points" section with table format (Caller, Callee, Method/interface, Direction). config.yaml rule: "design.md must include a section heading for Integration Points". SKILL.md Phase 4.3 gate verifies it. |
| 10 | Config rules inject design section requirements | VERIFIED | openspec/config.yaml contains five explicit rules under `rules.design`: one rule per mandatory section heading (Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points). |
| 11 | SKILL.md gate verifies five sections exist | VERIFIED | SKILL.md line 192: "Verify design.md contains all five required section headings from templates/design-template.md: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, and Integration Points. If any are missing, regenerate design.md before human review." |
| 12 | Task strategy approval before tasks.md | VERIFIED | SKILL.md Phase 4.3a (lines 202-214): "Before generating tasks.md, present a task decomposition strategy for human approval" with three required elements (decomposition approach, target task count range, dependency ordering rationale). Gate requires approval before proceeding to Phase 4.4. |
| 13 | Approved strategy recorded in Deterministic Anchors | VERIFIED | SKILL.md line 212: "After approval, populate the task decomposition strategy anchor in the plan's Deterministic Anchors section." Plan template line 34 has the anchor: "Task decomposition strategy: <e.g., 'layered, 7 feature tasks + 5 closing' (see Phase 4.3a approval)>". |
| 14 | Each task includes topology metadata | VERIFIED | SKILL.md lines 232-237 define the fixed topology format: Depends on, Consumes, Produces, Layers fields per task. |
| 15 | Tasks >2 layers flagged for review | VERIFIED | SKILL.md line 242: "When the approved strategy (Phase 4.3a) is layered or hybrid and a task's Layers field lists more than 2 architectural layers, flag the task for human review with a recommendation to split or approve as-is. When the approved strategy is vertical, the layer count flag does not apply since vertical tasks are expected to cross all layers." |
| 16 | Contract assertions reference spec IDs | VERIFIED | contract.md template has `<!-- Specs: capability/1, capability/3 -->` and `<!-- Specs: capability/2, capability/4 (DEVIATED) -->` comments. SKILL.md Phase 5 (lines 355-376) details the cross-referencing process: look up spec capability/scenario identifiers from the coverage table, write assertions that trace to specific scenarios. |
| 17 | Deterministic Anchors in plan.md with six fields | VERIFIED | plan.md lines 30-38 contain the Deterministic Anchors section with six fields: Workspace layout, Task decomposition strategy, Primary domain types, Error handling strategy, Test strategy, Chosen architecture. Each has an example value and source reference. |
| 18 | Anchors populated after design approval | VERIFIED | SKILL.md line 198: "After the human approves design.md, populate 5 of the 6 Deterministic Anchors in the plan: workspace layout, primary domain types, error handling strategy, test strategy, and chosen architecture. Source values from design.md's Architecture Decision, Type Catalog, and Error Strategy sections." |
| 19 | Foundation audit after task 1 | VERIFIED | SKILL.md lines 698-713: "Foundation audit (after task 1 only). If this is task 1 and it passed both validators, run a foundation check before proceeding to task 2." Line 713: "This audit runs only after task 1. It does not run after any other task." |
| 20 | Audit checks against Module Inventory and Type Catalog | VERIFIED | SKILL.md lines 701-702: "Extract module names from the Produces field. Look up those modules in design.md's Module Inventory. Verify that corresponding directories or files exist in the committed code with matching names." Same for types from Type Catalog. |
| 21 | Drift triggers corrective task 1a | VERIFIED | SKILL.md lines 704-709: "If drift is detected... Generate a corrective task 1a. Insert it in tasks.md between task 1 and task 2. Add assertions for it to contract.md. Present both to the human for approval. The corrective task goes through the normal build loop before task 2 starts." |
| 22 | Mandatory CLAUDE.md closing task | VERIFIED | SKILL.md line 258: "CLAUDE.md + ARCH.md generation (mandatory)." Listed as closing task #4 in the mandatory closing tasks section. |
| 23 | CLAUDE.md template follows best practices | VERIFIED | templates/claude-md.md opens with an Include/Exclude guidance table. Line 10: "For each line, ask: would removing this cause Claude to make mistakes? If not, cut it." Template structure follows best practices with focused, actionable sections. |
| 24 | ARCH.md created from design.md if absent | VERIFIED | SKILL.md line 258: "if no ARCH.md exists, create one from design.md's architecture decisions as a living document." |
| 25 | Existing CLAUDE.md and ARCH.md left untouched | VERIFIED | SKILL.md line 258: "If a CLAUDE.md already exists, do not modify it." and "If ARCH.md already exists, do not modify it." |
| 26 | CLAUDE.md template has 8 sections with example | VERIFIED | templates/claude-md.md Template section contains 8 headings: Purpose, Rules, Commands, Repo Map, Testing, Environment, Architecture, Key Terms. Example section (line 42) provides a complete filled-in example (Invoice Sync). |
| 27 | Closing task order: smoke, cleanup, shape, CLAUDE.md, docs, CI | VERIFIED | SKILL.md lines 248-262 list closing tasks in order: 1. Smoke test, 2. Cleanup task, 3. Module shape re-check, 4. CLAUDE.md + ARCH.md generation, 5. Documentation task, 6. CI configuration task (optional). |

## UBIQUITOUS_LANGUAGE.md Coverage

All planning determinism terms are present in UBIQUITOUS_LANGUAGE.md under the "Planning determinism" section (lines 87-98): Deterministic Anchors, Architecture Candidate Scoring, Task Independence, Task Topology, Foundation Audit, Type Catalog, Module Inventory, Design Template, Spec Reference.

Relationships section (lines 117-121) cross-references Deterministic Anchors, Foundation Audit, Spec References, Task Topology, and Design Template to their usage locations.

Example dialogue section (lines 141-153) illustrates the planning determinism features.

## README.md Coverage

README.md documents: architecture candidate scoring rubric and mandatory design gate (line 77), task strategy approval and topology metadata (line 78), mandatory closing tasks including CLAUDE.md + ARCH.md (lines 80-81), coverage audit with substitution prerequisite checking (lines 82-83), spec traceability in contract assertions (line 84), foundation audit after task 1 (lines 97-98), and planning determinism example dialogue (lines 141-153).

## Summary

- Items verified: 27/27
- Items missing: 0/27
- Approved deviations: 0

## Result

CLEAN
