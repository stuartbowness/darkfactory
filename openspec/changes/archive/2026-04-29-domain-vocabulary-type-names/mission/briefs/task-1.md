# Worker Brief: Task 1 - Add Domain Types section to worker brief template

## Your Task
Add a "Domain Types" section to `skills/mission/templates/worker-brief.md` after the existing "Key Terms" section. The section:
- Lists Type Catalog entries relevant to the task (from both Produces and Consumes topology fields) with a one-line instruction to use exact names
- Includes a placeholder showing the orchestrator what to fill in
- Includes a "None for this task" fallback for tasks with no relevant domain types
- Must be a separate section from Key Terms (not merged into it)

## Project Context
Darkfactory is a multi-agent mission system. Workers receive briefs telling them what to build. Currently, briefs carry "Key Terms" from the ubiquitous language glossary, but don't tell workers which exact code type names to use. This causes naming drift where workers implement domain concepts using reasonable aliases instead of the planned names.

## Architecture
The worker brief template is at `skills/mission/templates/worker-brief.md`. It's a markdown template with placeholder sections that the orchestrator fills in when assembling a brief for each task. The new section goes after the existing "## Key Terms" section and before "## What Previous Tasks Built".

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
- **Type Catalog**: A design.md section listing domain types with name, purpose, and owning module. Type names are drawn from the glossary when a matching term exists; types not in the glossary are named by the worker.
- **Domain type**: A type that models the problem space. Excludes technical helpers (CLI parsers, test fixtures, generic containers) unless they are public compatibility surfaces.
- **Brief**: A self-contained markdown file the orchestrator writes for a worker, containing the task description, project context, and procedures.

## Domain Types
None for this task. This mission modifies markdown templates only and introduces no domain types.

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
Task 2 and Task 3 reference the Domain Types section but do not consume it programmatically. No integration test needed.

## Procedures
- Edit the file `skills/mission/templates/worker-brief.md` to add the new section
- Commit all changes in a single commit with message: "[task-1] Add Domain Types section to worker brief template"
- Write your handoff report to openspec/changes/domain-vocabulary-type-names/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Add Domain Types section to worker brief template
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
