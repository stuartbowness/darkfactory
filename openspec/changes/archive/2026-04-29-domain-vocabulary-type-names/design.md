## Architecture Decision

This mission modifies existing markdown templates and skill instructions. No architecture change is introduced. The current architecture -- Claude Code skills as markdown files, with templates and prompts in subdirectories -- remains unchanged.

The changes fit the existing architecture by tightening three existing touch points in the mission workflow:
1. The worker brief template (already carries Key Terms from UBIQUITOUS_LANGUAGE.md)
2. The module shape re-check closing task (already reads design.md and compares to code)
3. The design template (already instructs "domain types only" for the Type Catalog)

No scored architecture candidates are needed because no architecture decision is being made.

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `skills/mission/templates/worker-brief.md` | Narrow: one new section added ("Domain Types") | How the orchestrator populates the section from the Type Catalog and task topology | The brief is the worker's only view of the mission. Changes here affect every worker without touching dispatch logic |
| `skills/mission/templates/design-template.md` | Narrow: one sentence added to Type Catalog section | The distinction between domain types and helper types | The design template is read once during Phase 4.3. Changes here affect every future design.md without touching the orchestrator |
| `skills/mission/SKILL.md` | Narrow: one sentence added to module shape re-check | How the orchestrator runs the Type Catalog name verification during the closing task | SKILL.md is the orchestrator's operating manual. The module shape re-check is already a defined closing task |
| `UBIQUITOUS_LANGUAGE.md` | Moderate: one definition updated, one term added, one relationship added, one dialogue updated | The full type naming chain from glossary through to code | The glossary is the shared language document. It's read in Phase 0 and Phase 2, and referenced by the design template and worker briefs |
| `README.md` | Narrow: reflects changes | Project documentation | Required by CLAUDE.md rule: update README after any skill/config change |

## Type Catalog

No domain types are introduced by this mission. All changes are to markdown templates and instructions that guide how domain types are named in future missions.

## Error Strategy

Not applicable. This mission modifies markdown documentation files. There is no runtime code, no error types, and no error propagation.

## Integration Points

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| Orchestrator (Phase 8, Step 1) | `worker-brief.md` template | Reads template to assemble brief | Internal |
| Orchestrator (Phase 8, Step 1) | `design.md` Type Catalog | Reads Type Catalog to populate Domain Types section in brief | Internal |
| Orchestrator (Phase 8, Step 1) | `tasks.md` topology | Reads Produces/Consumes fields to identify relevant domain types | Internal |
| Orchestrator (Phase 4.3) | `design-template.md` | Reads template to generate design.md | Internal |
| Orchestrator (Phase 8, closing task) | `SKILL.md` module shape re-check | Reads instructions for Type Catalog name verification | Internal |
| Orchestrator (Phase 0, Phase 2) | `UBIQUITOUS_LANGUAGE.md` | Reads glossary for shared terminology | Internal |

## Test Strategy

This mission produces no runtime code. All changes are to markdown files. Verification is by inspection:

- Each modified file is valid markdown
- The worker brief template contains the new Domain Types section with the specified format
- The design template's Type Catalog section contains the domain type boundary clarification
- SKILL.md's module shape re-check contains the Type Catalog name verification instruction
- UBIQUITOUS_LANGUAGE.md contains the updated definition, new term, new relationship, and updated dialogue
- README.md reflects the changes

No test framework, runner, or coverage tooling applies.

## Decisions

### D1: Domain Types section is separate from Key Terms in the brief

**Choice:** Add a new "Domain Types" section after the existing "Key Terms" section, rather than merging type names into Key Terms.

**Why:** Key Terms defines vocabulary ("a Mission is an autonomous run of the full plan-build-validate cycle"). Domain Types specifies code names ("use the exact name `Mission` for this type"). They serve different purposes and the worker should read them differently. Merging them would conflate definitions with instructions.

### D2: Module shape re-check handles Type Catalog names, not a separate closing task

**Choice:** Extend the existing module shape re-check with one sentence about Type Catalog name verification, rather than creating a dedicated "Type Catalog audit" closing task.

**Why:** The module shape re-check already reads design.md and compares module surface area to code. Type Catalog names are part of that surface. Adding a sentence to existing instructions is lighter than adding a new mandatory closing task that reads the same design.md for a closely related check.

### D3: No changes to contracts, tasks.md Produces field, or foundation audit

**Choice:** Leave contracts purely behavioral. Leave the Produces field informal and human-readable. Leave the foundation audit unchanged.

**Why:** Contracts define "done" through observable behavior, not implementation details. Type names are implementation concerns enforced through briefs (input) and audits (output). The Produces field is a planning artifact for human review, not a machine-readable input -- the foundation audit can map Produces entries to Type Catalog entries itself since it has access to both documents.

## Risks / Trade-offs

**[Risk] Workers may ignore the Domain Types section.** Adding a section to the brief doesn't guarantee the worker uses the exact names.
→ Mitigation: The foundation audit (task 1) and module shape re-check (closing task) catch drift. The brief is guidance; the audits are enforcement.

**[Risk] Module shape re-check becomes overloaded.** It now checks module surface area AND type names.
→ Mitigation: The type name check is one additional comparison per Type Catalog entry. For typical missions with 5-10 domain types, this is trivial added work.

**[Risk] Domain type boundary is still a judgment call.** "Models the problem space" is guidance, not a bright line.
→ Mitigation: This is intentional. The alternative -- enumerating every type category that qualifies -- would be brittle and incomplete. The design template provides examples (CLI parsers, test fixtures, generic containers) of what to exclude.

## File Change Map

| File | Change type | Lines added (est.) |
|------|------------|-------------------|
| `skills/mission/templates/worker-brief.md` | Modify: add Domain Types section | ~5 |
| `skills/mission/templates/design-template.md` | Modify: add domain type boundary sentence | ~1 |
| `skills/mission/SKILL.md` | Modify: extend module shape re-check | ~1 |
| `UBIQUITOUS_LANGUAGE.md` | Already modified in Phase 2 | 0 |
| `README.md` | Modify: reflect changes | ~5 |

## Open Questions

None remaining -- all decisions resolved during grill session.
