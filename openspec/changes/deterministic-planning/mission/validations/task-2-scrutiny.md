# Scrutiny Validation: Task 2

**Commit:** c152c840bc16907e24c0439e58c23ec9e667c554
**Date:** 2026-04-28
**Result:** PASS

## Assertion Results

### 1. [output] contract.md uses `<!-- Specs: capability/N -->` format instead of `<!-- PRD Requirements: #N -->`
**PASS**

Evidence: `contract.md` at the commit contains:
```
<!-- Specs: capability/1, capability/3 -->
<!-- Specs: capability/2, capability/4 (DEVIATED) -->
```
No `<!-- PRD Requirements: ... -->` comments remain.

### 2. [output] SKILL.md Phase 5 instructions direct the orchestrator to trace each assertion back to a spec capability and scenario number
**PASS**

Evidence: Phase 5 contains the explicit instruction:
> "Each assertion must trace back to a specific spec capability and scenario number."

This appears in the "Cross-Referencing Specs" subsection (line 321 of SKILL.md). The process steps also direct the orchestrator to "Look up the spec capability/scenario identifiers mapped to this task in the coverage table."

### 3. [output] No remaining references to "PRD Requirements" in the contract template
**PASS**

Evidence: `grep -c "PRD Requirements" contract.md` returns `0`.

### 4. [execution] `grep -c "PRD Requirements" skills/mission/templates/contract.md` returns 0
**PASS**

Evidence: Command output is `0`.

### 5. [execution] `grep -c "Specs:" skills/mission/templates/contract.md` returns at least 1
**PASS**

Evidence: Command output is `2`.

## Code Review Notes

The commit modifies two files:
- `skills/mission/SKILL.md` (18 insertions, 13 deletions)
- `skills/mission/templates/contract.md` (5 insertions, 5 deletions)

Changes are consistent across both files. The rename from "PRD Requirements" to spec-based references is applied uniformly. The DEVIATED marking convention is preserved and updated to reference "Specs comment" instead of "PRD Requirements comment." No "PRD Requirements" references remain in Phase 5 of SKILL.md (verified via grep, returns 0).

The template format `<!-- Specs: capability/N, capability/M -->` is documented in Phase 5 with an explanation of what `capability` and `N` represent (spec folder name and scenario number respectively).

## Overall Result

**PASS** -- All 5 assertions verified. The commit cleanly replaces PRD-centric references with spec-centric references in both the contract template and Phase 5 instructions.
