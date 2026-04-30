## Context

The mission skill (`skills/mission/SKILL.md`, 837 lines) is the orchestrator's operating manual. It defines 10 phases (0-9) with gates between them. Templates in `skills/mission/templates/` define artifact formats. Prompts in `skills/mission/prompts/` are handed to validator subagents. The openspec config at `openspec/config.yaml` injects project-level context and rules into artifact generation.

This change adds deterministic constraints to the planning phases (4.3, 4.4, 5, 8) without restructuring the existing phase flow. All changes are additive -- no existing phases are removed or reordered.

## Goals / Non-Goals

**Goals:**
- Constrain the 5 highest-leverage decisions in spec generation: architecture choice, design specificity, task decomposition, convention establishment, and spec traceability
- Reduce variance between runs against identical PRDs by pinning decisions through rubrics, templates, gates, and anchors
- Keep changes compatible with the existing phase flow and OpenSpec schema

**Non-Goals:**
- Forking the OpenSpec schema (use config rules instead)
- Modifying validator prompts (scrutiny, user-test, adversarial)
- Modifying the worker brief template beyond what's needed for these features
- Guaranteeing identical outputs (reducing variance, not eliminating it)
- Enforcing ubiquitous language depth (tracked in issue #130)

## Decisions

### D1: Three-layer design enforcement (config rules + template + gate)

**Choice:** Inject mandatory section requirements via `openspec/config.yaml` rules.design entries. Define section content formats in `templates/design-template.md`. Verify sections exist via a SKILL.md gate in Phase 4.3.

**Alternative considered:** Fork the OpenSpec schema to modify the design.md template directly. Rejected because a fork creates sync burden with upstream OpenSpec changes. Config rules achieve the same injection without maintenance cost.

**Alternative considered:** Put all content in SKILL.md Phase 4.3 instructions. Rejected because the rubrics and table formats are too detailed for inline SKILL.md text. The template file keeps SKILL.md lean.

### D2: Architecture scoring lives in the design template, not SKILL.md

**Choice:** The four-dimension rubric with per-level criteria is defined in `templates/design-template.md`. SKILL.md Phase 4.3 references it and runs the gate. The scoring table appears in design.md as the Architecture Decision section. The hard floor (no dimension below 1) and tie escalation (present tied candidates to human) are enforced by the SKILL.md gate in Phase 4.3, not by the template.

**Why:** The rubric is ~40 lines of structured criteria tables. Inlining this in SKILL.md (already 837 lines) would bloat the orchestrator's main document. The template is read on demand when Phase 4.3 runs. The gate enforces the behavioral rules (floor, ties) because those are orchestrator decisions, not template content.

### D3: Type Catalog has no field-level detail

**Choice:** Types list name, purpose, and owning module. No fields.

**Why:** Field definitions are implementation decisions derived from spec WHEN/THEN blocks. The v3/v5 divergence was about type *names* (IronTodoError vs DomainError), not type *fields*. The Type Catalog prevents naming drift; fields are the worker's job.

### D4: Phase 4.3a inserted between design and tasks

**Choice:** New sub-phase for task strategy approval. The orchestrator presents decomposition approach, target task count, and dependency rationale. The human approves before tasks.md is generated.

**Why:** In the v3/v5 comparison, v3 used layered decomposition (7 groups) and v5 used flat vertical slices (15 tasks). Making this an explicit human decision prevents the model from silently choosing a strategy.

**Insertion point:** After Phase 4.3 (design review), before Phase 4.4 (task generation). The Phase Index is updated to include 4.3a. Note: 5 of the 6 Deterministic Anchors are populated immediately after design approval in Phase 4.3. The task decomposition strategy anchor is populated after Phase 4.3a approval, since the strategy doesn't exist until the human approves it.

### D5: Task topology as inline metadata, not a separate artifact

**Choice:** Each task in tasks.md includes topology fields directly in the task description, using a fixed markdown format:

```
- [ ] Task N: <name>
  Depends on: Task 1, Task 2
  Consumes: Todo type (domain), store interface (storage)
  Produces: search endpoint (transport), SearchResult type (domain)
  Layers: domain, transport
```

**Why fixed format:** The foundation audit (D9) reads the Produces field to extract module and type names for its check. Loose formatting would make the audit brittle. The four fields use consistent casing and comma-separated values.

**Layers are logical groupings** (domain, transport, storage, configuration, etc.) defined in the design narrative, not columns from the Module Inventory table. The Module Inventory lists modules; layers group modules by architectural concern.

**Alternative considered:** Separate `topology.md` file. Rejected because it splits task information across two files and creates a sync burden. Inline metadata keeps everything in one place.

### D6: Task quality standard updated to be strategy-aware

**Choice:** Phase 4.4's task quality standard changes from "each task must be a vertical slice" to "each task must produce a concrete, testable output." When the approved strategy (Phase 4.3a) is vertical, tasks cross all layers. When the strategy is layered or hybrid, tasks may be scoped to a single layer, provided each task still has a clear done state and a test surface.

**Why:** The current rule hardcodes vertical slices as the only valid approach. In the v3/v5 benchmark, v3 used layered decomposition (ignoring the rule) and scored 12 points higher. The rule was already being overridden by model judgment. Making it strategy-aware aligns the rule with Phase 4.3a and eliminates the contradiction.

**What stays:** The `/tdd` skill's vertical slice guidance is unchanged -- it describes how to implement within a task (test one path, make it pass, repeat), not how to scope tasks. A layered task still uses TDD vertical slices internally.

**Cascade updates required:** This wording change must also be applied to `openspec/config.yaml` rules.tasks and the `UBIQUITOUS_LANGUAGE.md` Vertical Slice definition to maintain consistency. The `docs/design-decisions.md` references are historical and stay as-is.

### D7: Spec references in contract replace row numbers (renumbered from D6)

**Choice:** Contract template changes from `<!-- PRD Requirements: #1, #3 -->` to `<!-- Specs: add-todo/1, search/3 -->`. Phase 5 instructions updated to match.

**Why:** Row numbers are arbitrary and break if the coverage audit is reordered. Spec capability/scenario identifiers are stable and meaningful. The coverage audit already maps PRD to specs, so there's no new mapping to create.

### D8: CLAUDE.md + ARCH.md as closing task, not pre-build phase

**Choice:** Generate CLAUDE.md and ARCH.md as a mandatory closing task (between module shape re-check and documentation). Create only if absent. Template follows Claude Code best practices.

**Alternative considered:** Generate CLAUDE.md before the build loop (Phase 4.5). Rejected because: (a) during the build, worker briefs already carry architecture excerpts, key terms, and codebase deltas; (b) for greenfield projects, task 1 creates the build system so commands don't exist yet; (c) generating from the actual built codebase produces a more accurate CLAUDE.md.

**ARCH.md:** Extracted from design.md as a living document. CLAUDE.md's Rules section includes "update ARCH.md when you change architecture" so it stays current.

### D9: Foundation audit scoped to task 1's declared produces

**Choice:** After task 1 passes both validators, check module structure and type names against design.md, but only for what task 1 said it would produce (from its "produces" topology field).

**Why:** Task 1 typically creates the project foundation but not all modules. Checking the full design would fail by definition since tasks 2-N haven't run. Scoping to declared produces avoids false positives while catching the naming and structure drift that cascaded in v5.

### D10: Deterministic anchors as one-liner pointers

**Choice:** Six fields in plan.md, each a one-line summary with a source reference. Not duplicated content from design.md.

**Why:** The plan must stay concise to survive compaction effectively. Full content duplication would bloat the plan and create a sync problem. Pointers tell the orchestrator what was decided and where to find details if needed.

## Risks / Trade-offs

**[Risk] Architecture scoring adds time to planning.** Evaluating 2-3 candidates with a rubric takes longer than generating one architecture directly.
→ Mitigation: For greenfield missions, the time invested prevents the cascade of problems seen in v5 (file size, complexity, naming drift). For existing codebases without architecture changes, scoring is skipped. For existing codebases with explicit architecture changes, the current architecture is included as one of the 2-3 candidates.

**[Risk] Config rules are advisory, not enforced by OpenSpec.** The openspec CLI injects rules into instructions, but the LLM could still produce a design without all sections.
→ Mitigation: The SKILL.md gate catches missing sections and regenerates. The config rules are guidance; the gate is enforcement.

**[Risk] Task topology metadata adds review burden.** Each task now has 4 extra fields the human must review.
→ Mitigation: The orchestrator generates the metadata -- the human reviews, not writes. The >2 layers flag focuses attention on problematic tasks. For small missions (2-3 tasks), the metadata is trivial.

**[Risk] Foundation audit may trigger unnecessary corrective tasks.** After task 1, minor naming differences could be flagged as drift.
→ Mitigation: Corrective tasks require human approval. The human can say "this is fine" and proceed. The audit is a signal, not an automatic fix.

**[Risk] CLAUDE.md template may produce generic content.** Even with the include/exclude guidance, the closing task worker might produce boilerplate.
→ Mitigation: The template has concrete sections (Commands, Repo Map, Testing, Environment) filled from the actual codebase. The include/exclude table explicitly bans generic content. The human reviews the closing task output through normal validation.

## File Change Map

| File | Change type | Lines added (est.) |
|------|------------|-------------------|
| `skills/mission/SKILL.md` | Modify phases 4.3, 4.4, 5, 8; add 4.3a; update Phase Index | ~75 |
| `skills/mission/templates/design-template.md` | New file | ~70 |
| `skills/mission/templates/claude-md.md` | New file | ~50 |
| `skills/mission/templates/plan.md` | Add Deterministic Anchors section | ~12 |
| `skills/mission/templates/contract.md` | Update spec reference format | ~2 |
| `openspec/config.yaml` | Add rules.design entries; update rules.tasks vertical-slice wording to be strategy-aware | ~12 |
| `README.md` | Reflect all changes | ~20 |
| `UBIQUITOUS_LANGUAGE.md` | Already updated in Phase 2 | 0 |

## Open Questions

None remaining -- all decisions resolved during grill session.
