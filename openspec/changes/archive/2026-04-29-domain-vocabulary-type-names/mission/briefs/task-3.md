# Worker Brief: Task 3 - Update README to reflect changes

## Your Task
Update `README.md` to reflect three changes made in this mission:

1. **Domain Types in worker briefs** -- The worker brief template now includes a "Domain Types" section that lists exact Type Catalog names for each task, so workers use the planned domain type names instead of inventing aliases. Mention this in the context of how workers receive their briefs (around line 90-92 and/or line 133 where the brief contents are described).

2. **Domain type boundary in the design template** -- The design template's Type Catalog section now clarifies that domain types model the problem space and technical helpers (CLI parsers, test fixtures, generic containers) are excluded. Mention this in the context of the design phase (around line 77 where the design and its mandatory gate are described).

3. **Type Catalog name verification in the module shape re-check** -- The module shape re-check closing task now also verifies that every domain type in the Type Catalog exists in the committed code with matching names. Mention this in the closing tasks description (around line 80).

Keep the additions concise -- one clause or short sentence each, woven into the existing text. Do not restructure or rewrite surrounding paragraphs.

IMPORTANT: UBIQUITOUS_LANGUAGE.md was already updated before the build loop. Do NOT edit UBIQUITOUS_LANGUAGE.md.

## Project Context
Darkfactory is a multi-agent mission system. This mission makes the relationship between ubiquitous language, Type Catalog, and code type names explicit and consistently applied. The README is the project's primary documentation.

## Architecture
`README.md` is in the project root. It describes the mission workflow, skills, and architecture. Changes should be woven into the existing narrative, not added as a new section.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
- **Type Catalog**: A design.md section listing domain types with name, purpose, and owning module.
- **Domain type**: A type that models the problem space. Excludes technical helpers unless they are public compatibility surfaces.

## Domain Types
None for this task.

## What Previous Tasks Built
### Task 1: Add Domain Types section to worker brief template
- Modified: skills/mission/templates/worker-brief.md (+10 lines)
- Added section: "## Domain Types" at line 15, after "## Key Terms", before "## What Previous Tasks Built"
- Section includes: exact-name instruction, Type Catalog placeholder (Produces/Consumes), "None for this task" fallback

### Task 2: Add domain type boundary and Type Catalog name verification
- Modified: skills/mission/templates/design-template.md (+1 line)
- Modified: skills/mission/SKILL.md (+1 line, -1 line -- extended module shape re-check paragraph)
- design-template.md: added domain type boundary sentence after "List domain types only..." in Type Catalog section
- SKILL.md: added Type Catalog name verification + drift resolution to module shape re-check closing task

## Downstream Consumers
None.

## Procedures
- Edit `README.md` only
- Commit all changes in a single commit with message: "[task-3] Update README to reflect domain vocabulary changes"
- Write your handoff report to openspec/changes/domain-vocabulary-type-names/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Update README to reflect changes
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
