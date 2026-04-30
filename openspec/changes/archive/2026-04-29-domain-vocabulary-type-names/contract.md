# Contract

## Task 1: Add Domain Types section to worker brief template
<!-- Specs: brief-domain-types/1, brief-domain-types/2, brief-domain-types/3, brief-domain-types/4 -->
- [ ] [output] `skills/mission/templates/worker-brief.md` contains a section headed "## Domain Types" that appears after the "## Key Terms" section
- [ ] [output] The Domain Types section includes a one-line instruction telling the worker to use exact names for domain types
- [ ] [output] The Domain Types section includes a placeholder for listing Type Catalog entries relevant to the task from both Produces and Consumes fields
- [ ] [output] The Domain Types section includes a "None for this task" fallback for tasks with no relevant domain types
- [ ] [inspection] The Domain Types section is separate from the Key Terms section (not merged into it)

## Task 2: Add domain type boundary and Type Catalog name verification
<!-- Specs: domain-type-boundary/1, type-name-audit/1, type-name-audit/2 -->
- [ ] [output] `skills/mission/templates/design-template.md` Type Catalog section contains a sentence stating that domain types model the problem space and that technical helpers are excluded unless they are public compatibility surfaces
- [ ] [output] `skills/mission/SKILL.md` module shape re-check closing task instructions mention verifying that every domain type in the Type Catalog exists in the committed code with a matching name
- [ ] [output] The SKILL.md instruction specifies that drift is resolved by either updating design.md or refactoring the code, matching the existing module surface drift resolution pattern

## Task 3: Update README to reflect changes
- [ ] [output] `README.md` mentions the Domain Types section in worker briefs
- [ ] [output] `README.md` mentions the domain type boundary clarification in the design template
- [ ] [output] `README.md` mentions the Type Catalog name verification in the module shape re-check

## Task 4: Smoke test
- [ ] [execution] All modified markdown files parse without syntax errors: `worker-brief.md`, `design-template.md`, `SKILL.md`, `UBIQUITOUS_LANGUAGE.md`, `README.md`

## Task 5: Cleanup
- [ ] [inspection] No unused scaffolding, temporary files, or backwards-compatibility shims remain from the build process

## Task 6: Module shape re-check
- [ ] [output] `mission/validations/module-shape-audit.md` exists and lists each module from design.md with a drift assessment

## Task 7: CLAUDE.md + ARCH.md generation
- [ ] [output] If `CLAUDE.md` did not exist before the mission, it now exists in the project root. If it already existed, it is unchanged.
- [ ] [output] If `ARCH.md` did not exist before the mission, it now exists in the project root. If it already existed, it is unchanged.

## Task 8: Documentation
- [ ] [output] Documentation reflects what was built, using terms from `UBIQUITOUS_LANGUAGE.md` consistently
