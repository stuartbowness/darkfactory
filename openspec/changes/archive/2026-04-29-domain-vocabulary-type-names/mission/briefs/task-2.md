# Worker Brief: Task 2 - Add domain type boundary and Type Catalog name verification

## Your Task
Two single-sentence additions to two files:

1. In `skills/mission/templates/design-template.md`, add a sentence to the Type Catalog section (after the existing "List domain types only..." line) clarifying that domain types model the problem space and technical helpers (CLI parsers, test fixtures, generic containers) are excluded unless they are public compatibility surfaces.

2. In `skills/mission/SKILL.md`, extend the module shape re-check closing task instructions to include verifying that every domain type in the Type Catalog exists in the committed code with a matching name. The instruction should specify that drift is resolved by either updating design.md to match the code (deliberate change) or refactoring the code to match design.md (unintended drift).

The module shape re-check is in SKILL.md under Phase 8, Step 7, in the "Mandatory Closing Tasks" section, item 3 ("Module shape re-check"). Add the Type Catalog name verification as additional text within that existing item.

## Project Context
Darkfactory is a multi-agent mission system. The design template guides how design.md is generated during planning. SKILL.md is the orchestrator's operating manual. The Type Catalog in design.md lists domain types, but the boundary between domain types and helper types is currently implicit. The module shape re-check compares design.md to code but currently only checks module surface area, not type names.

## Architecture
- `skills/mission/templates/design-template.md` is a markdown template read during Phase 4.3 when generating design.md. The Type Catalog section starts at the line "## Type Catalog".
- `skills/mission/SKILL.md` is the main mission skill file. The module shape re-check is item 3 under "Mandatory Closing Tasks" in Phase 8.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
- **Type Catalog**: A design.md section listing domain types with name, purpose, and owning module. Type names are drawn from the glossary when a matching term exists; types not in the glossary are named by the worker.
- **Domain type**: A type that models the problem space. Excludes technical helpers unless they are public compatibility surfaces.
- **Module shape re-check**: A mandatory closing task that compares design.md module descriptions to actual code surface area.

## Domain Types
None for this task. This mission modifies markdown templates only and introduces no domain types.

## What Previous Tasks Built
### Task 1: Add Domain Types section to worker brief template
- Modified: skills/mission/templates/worker-brief.md (+10 lines)
- Added section: "## Domain Types" at line 15, after "## Key Terms", before "## What Previous Tasks Built"
- Section includes: exact-name instruction, Type Catalog placeholder (Produces/Consumes), "None for this task" fallback

## Downstream Consumers
Task 3 (README) references these changes but does not consume them programmatically.

## Procedures
- Edit `skills/mission/templates/design-template.md` and `skills/mission/SKILL.md`
- Commit all changes in a single commit with message: "[task-2] Add domain type boundary and Type Catalog name verification"
- Write your handoff report to openspec/changes/domain-vocabulary-type-names/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2 - Add domain type boundary and Type Catalog name verification
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
