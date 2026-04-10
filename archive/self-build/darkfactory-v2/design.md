## Context

The v1 Darkfactory skills were written by hand during a grill-me session. This change rebuilds them using the Darkfactory mission system itself -- workers receive specs and design decisions, produce skills independently, and validators check them against a contract.

This is a dogfooding exercise. The v2 output lives alongside v1 for comparison.

## Goals / Non-Goals

**Goals:**
- Generate all three skills from specs and design decisions alone
- Workers do NOT see v1 skill files (independent generation)
- Validate that specs are sufficient to produce working skills
- Compare v1 and v2 output

**Non-Goals:**
- Replacing v1 (v2 is a parallel output for comparison)
- Runtime testing (these are markdown skills, not executable code)
- Modifying specs or design decisions based on v2 output

## Decisions

### Workers use Claude subagents, not Codex
This is a skill-writing mission. The workers need to read specs and produce structured markdown. Claude is better suited than Codex for this. Config: `worker: claude`.

### Workers receive specs as primary source material
Each worker brief includes: the relevant OpenSpec specs (WHEN/THEN scenarios), excerpts from docs/design-decisions.md (architecture, flow, templates), and UBIQUITOUS_LANGUAGE.md terms. Workers do NOT receive v1 SKILL.md files.

### Validation is structural, not runtime
Since the output is markdown skill files, validation means:
- Scrutiny: valid frontmatter, phase coverage, correct file paths, ubiquitous language usage, self-consistency
- User-test: completeness audit against design-decisions.md (every decision and flow step represented)
- No demo video (no frontend)

### Output goes to darkfactoryV2/ directory
All v2 skills are written to `darkfactoryV2/.claude/skills/` to keep them separate from v1 for comparison.

## Risks / Trade-offs

- **[Workers may produce different structure than v1]** -> That's the point. Different is fine if it covers the same requirements. The contract checks coverage, not exact structure.
- **[Specs may be insufficient]** -> If a worker can't produce a complete skill from specs alone, that tells us the specs need improvement. Valuable signal either way.
