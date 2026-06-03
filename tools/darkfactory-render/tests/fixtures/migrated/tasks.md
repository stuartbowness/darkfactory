## Feature Tasks

- [x] Task 1: Add Domain Types section to worker brief template
  Depends on: none
  Consumes: none
  Produces: Domain Types section in worker-brief.md
  Layers: templates

- [x] Task 2: Add domain type boundary and Type Catalog name verification
  Depends on: Task 1
  Consumes: none
  Produces: domain type boundary sentence in design-template.md, Type Catalog name verification instruction in SKILL.md
  Layers: templates, orchestrator

- [x] Task 3: Update README to reflect changes
  Depends on: Task 1, Task 2
  Consumes: Domain Types section (worker-brief.md), domain type boundary (design-template.md), Type Catalog name verification (SKILL.md)
  Produces: updated README.md
  Layers: documentation

## Closing Tasks

- [x] Task 4: Smoke test
  Depends on: Task 1, Task 2, Task 3
  Consumes: all prior task outputs
  Produces: validation that templates are syntactically valid markdown
  Layers: validation
  Closing: smoke-test

- [x] Task 5: Cleanup
  Depends on: Task 4
  Consumes: all prior task outputs
  Produces: clean codebase with no leftover scaffolding
  Layers: cleanup
  Closing: cleanup

- [x] Task 6: Module shape re-check
  Depends on: Task 5
  Consumes: design.md Module Inventory
  Produces: mission/validations/module-shape-audit.md
  Layers: validation
  Closing: module-shape

- [x] Task 7: CLAUDE.md + ARCH.md generation
  Depends on: Task 6
  Consumes: design.md, built codebase
  Produces: CLAUDE.md and/or ARCH.md if they don't already exist
  Layers: documentation
  Closing: claude-arch

- [x] Task 8: Documentation
  Depends on: Task 7
  Consumes: all changes from feature tasks
  Produces: updated documentation
  Layers: documentation
  Closing: documentation
