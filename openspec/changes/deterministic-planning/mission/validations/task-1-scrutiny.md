# Scrutiny Validation: Task 1.1

## Contract Assertions

| # | Assertion | Method | Result | Evidence |
|---|-----------|--------|--------|----------|
| 1 | `skills/mission/templates/design-template.md` exists and contains headings for all five mandatory sections | [output] | PASS | File exists. Contains `## Architecture Decision`, `## Module Inventory`, `## Type Catalog`, `## Error Strategy`, `## Integration Points` -- all as H2 headings. |
| 2 | Architecture Decision section contains four-dimension scoring rubric with per-level criteria (0-3) | [output] | PASS | Rubric under `### Scoring Rubric` with four H4 subsections: Separation of Concerns, Testability, Task Independence, Module Depth. Each has a table with scores 0, 1, 2, 3. |
| 3 | Each scoring dimension has concrete criteria for all four levels referencing observable structural properties | [output] | PASS | All criteria reference observable properties (e.g., "Each responsibility maps to exactly one unit", "Separately buildable or deployable units", "Tests require significant setup or multiple running services"). No subjective-only language. |
| 4 | Module Inventory section defines table with columns: module name, interface width, what it hides, why this boundary | [output] | PASS | Table header: `Module name | Interface width | What it hides | Why this boundary`. |
| 5 | Type Catalog specifies name (from UBIQUITOUS_LANGUAGE.md), purpose (one line), owning module -- no field-level detail | [output] | PASS | Table columns: `Type name | Purpose | Owning module`. Text says "Type names must come from `UBIQUITOUS_LANGUAGE.md` when the term exists there. Do not include field-level detail." Purpose column placeholder reads "One-line purpose". |
| 6 | `openspec/config.yaml` contains `rules.design` entries listing five mandatory design sections | [output] | PASS | `rules.design` block lists five entries: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points. |
| 7 | SKILL.md Phase 4.3 references `templates/design-template.md` by path | [output] | PASS | Line 171: "Follow the required structure in `templates/design-template.md`." |
| 8 | SKILL.md Phase 4.3 contains gate verifying all five mandatory section headings exist before human review | [output] | PASS | Gate bullet: "Verify `design.md` contains all five required section headings from `templates/design-template.md`: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, and Integration Points. If any are missing, regenerate `design.md` before human review." |
| 9 | SKILL.md Phase 4.3 gate specifies no recommended candidate can score 0 on any dimension (hard floor) | [output] | PASS | Gate bullet: "Enforce the hard floor: no recommended candidate can score 0 on any scoring dimension. If a candidate scores 0 on any dimension, flag it and recommend against it." |
| 10 | SKILL.md Phase 4.3 gate specifies tied candidates presented to human for discussion and selection | [output] | PASS | Gate bullet: "If candidates tie on total score, present the tied candidates to the human for discussion and selection. Do not apply an arbitrary tie-breaker." |
| 11 | SKILL.md Phase 4.3 distinguishes greenfield/arch-change (run scoring) from existing-codebase without arch change (document current, skip scoring) | [output] | PASS | Two gate bullets: greenfield/arch-change requires "2-3 scored architecture candidates"; existing codebase without arch changes says "scoring is not required. Verify the Architecture Decision section documents the current architecture and explains how the planned changes fit." |
| 12 | `grep -c "design-template.md" skills/mission/SKILL.md` returns at least 1 | [execution] | PASS | Returns 2. |
| 13 | `grep -c "rules:" openspec/config.yaml` and `grep -c "design:" openspec/config.yaml` both return at least 1 | [execution] | PASS | Both return 1. |

## Code Review Findings

### Ordering nit (non-blocking)

Lines 188-189 of SKILL.md read:

> Present it to the human with a recommendation.
>
> Before presenting it, run the Phase 4.3 design gate:

The instruction to "present" appears before the instruction to run the gate that must precede presentation. While an LLM reader will likely interpret "Before presenting it" correctly as a temporal override, the sequencing is mildly confusing. A clearer order would place the gate block first, then the "present" instruction. This is a readability nit, not a functional failure -- the "Before presenting it" clause makes the intended order unambiguous.

### Structural integrity: preserved

- `#### Module Depth` subsection (line 173) is present and unchanged from prior content.
- `#### Existing Codebases` subsection (line 184) is present and unchanged.
- Phase flow 4.1 -> 4.2 -> 4.3 -> 4.4 is intact. No phases skipped or misordered.
- No unrelated content was modified.

### Template completeness

All five mandatory sections in `design-template.md` have substantive guidance -- not just headings. Each section includes a table format or structured instructions. The scoring rubric is thorough with 4 dimensions x 4 levels = 16 cells of criteria.

### Config consistency

The five section names in `openspec/config.yaml` match the five H2 headings in `design-template.md` and the five names listed in the SKILL.md gate. No mismatches.

## Result: PASS
