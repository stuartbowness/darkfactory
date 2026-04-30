## Feature Tasks

- [x] Task 1: Add Domain Types section to worker brief template
  Depends on: none
  Consumes: none
  Produces: Domain Types section in worker-brief.md
  Layers: templates

  Add a "Domain Types" section to `skills/mission/templates/worker-brief.md` after the existing "Key Terms" section. The section lists Type Catalog entries relevant to the task (from both Produces and Consumes fields) with a one-line instruction to use exact names. When no domain types are relevant, the section says "None for this task."

- [x] Task 2: Add domain type boundary and Type Catalog name verification
  Depends on: Task 1
  Consumes: none
  Produces: domain type boundary sentence in design-template.md, Type Catalog name verification instruction in SKILL.md
  Layers: templates, orchestrator

  Two single-sentence additions:
  1. In `skills/mission/templates/design-template.md`, add a sentence to the Type Catalog section clarifying that domain types model the problem space and technical helpers are excluded unless they are public compatibility surfaces.
  2. In `skills/mission/SKILL.md`, extend the module shape re-check closing task instructions to include verifying that every domain type in the Type Catalog exists in the committed code with a matching name.

- [x] Task 3: Update README to reflect changes
  Depends on: Task 1, Task 2
  Consumes: Domain Types section (worker-brief.md), domain type boundary (design-template.md), Type Catalog name verification (SKILL.md)
  Produces: updated README.md
  Layers: documentation

  Update README.md to reflect the new Domain Types section in worker briefs, the domain type boundary clarification in the design template, and the Type Catalog name verification in the module shape re-check. Note that UBIQUITOUS_LANGUAGE.md was already updated before the build loop and should not be re-edited.

## Closing Tasks

- [x] Task 4: Smoke test
  Depends on: Task 1, Task 2, Task 3
  Consumes: all prior task outputs
  Produces: validation that templates are syntactically valid markdown
  Layers: validation

  Verify all modified markdown files are syntactically valid. Run a markdown lint or parse check on worker-brief.md, design-template.md, SKILL.md, UBIQUITOUS_LANGUAGE.md, and README.md. No project-internal mocks. Since this mission produces no runnable user flow, note that in the handoff.

- [x] Task 5: Cleanup
  Depends on: Task 4
  Consumes: all prior task outputs
  Produces: clean codebase with no leftover scaffolding
  Layers: cleanup

  Remove unused dependencies, dead code, leftover scaffolding, temporary artifacts, and any backwards-compatibility shims. Check for any debris from the build process.

- [x] Task 6: Module shape re-check
  Depends on: Task 5
  Consumes: design.md Module Inventory
  Produces: mission/validations/module-shape-audit.md
  Layers: validation

  Re-read design.md's module descriptions. For each module, compare the description to what the files actually contain. Write findings to mission/validations/module-shape-audit.md. Also verify that every domain type in the Type Catalog exists in the committed code with a matching name (which is trivially satisfied since this mission introduces no domain types).

- [x] Task 7: CLAUDE.md + ARCH.md generation
  Depends on: Task 6
  Consumes: design.md, built codebase
  Produces: CLAUDE.md and/or ARCH.md if they don't already exist
  Layers: documentation

  If CLAUDE.md does not exist in the project root, create one. If it already exists, do not modify it. Similarly for ARCH.md.

- [x] Task 8: Documentation
  Depends on: Task 7
  Consumes: all changes from feature tasks
  Produces: updated documentation
  Layers: documentation

  Write or update documentation for what was built. Use terms from UBIQUITOUS_LANGUAGE.md consistently. Any commands shown in docs must be executed against the built artifacts and the actual output pasted.
