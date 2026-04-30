# Worker Brief: Task 5.1 - CLAUDE.md Closing Task and Template

## Your Task

Create the CLAUDE.md template and update the mandatory closing tasks in SKILL.md Phase 4.4 to include CLAUDE.md + ARCH.md generation.

This task produces two outputs:
1. A new file `skills/mission/templates/claude-md.md`
2. Updated mandatory closing tasks in SKILL.md Phase 4.4

### Detailed requirements:

**templates/claude-md.md** -- Create a template for generating project CLAUDE.md files. The template must contain:

1. **Include/exclude guidance table** at the top, following Claude Code best practices:

| Include | Exclude |
|---------|---------|
| Commands Claude cannot guess | Anything Claude can infer from code |
| Code style rules that differ from defaults | Standard language conventions |
| Testing instructions and preferred runners | Detailed API docs (link instead) |
| Architectural decisions specific to project | File-by-file codebase descriptions |
| Dev environment quirks (required env vars) | Long explanations or tutorials |
| Common gotchas or non-obvious behaviors | Self-evident practices |

After the table, add: "For each line, ask: would removing this cause Claude to make mistakes? If not, cut it."

2. **Eight sections** to fill in:

```
# <project-name>

## Purpose
<One paragraph: what this project does and who it's for>

## Rules
<Specific, actionable coding rules. Include "update ARCH.md when you change architecture." Only rules that differ from language defaults.>

## Commands
<build, test, lint, format commands. Only commands Claude cannot guess from build files.>

## Repo Map
<High-level: what lives where. One line per major directory or module. Not file-by-file.>

## Testing
<Framework, where tests go by change type, any special setup like serial execution or DB requirements.>

## Environment
<Required services, env vars, config files, dev setup quirks.>

## Architecture
See @ARCH.md for architecture decisions.

## Key Terms
See @UBIQUITOUS_LANGUAGE.md for domain terminology.
```

3. **One example** showing what a filled-in CLAUDE.md looks like, with a note: "Adapt section content to the project's language and framework. The example below is illustrative; match the density and specificity to your project."

The example should be concise (~20 lines) and language-neutral.

**SKILL.md Phase 4.4 Mandatory Closing Tasks** -- Find the section listing mandatory closing tasks (smoke test, cleanup, module shape re-check, documentation, CI configuration). Insert a new closing task between "module shape re-check" and "documentation":

```
4. **CLAUDE.md + ARCH.md generation (mandatory).** If the project root does not have a CLAUDE.md, create one from [templates/claude-md.md](templates/claude-md.md) using the actual built codebase (real commands, real repo structure, real conventions). If a CLAUDE.md already exists, do not modify it. Similarly, if no ARCH.md exists, create one from design.md's architecture decisions as a living document. If ARCH.md already exists, do not modify it.
```

After inserting, renumber the subsequent closing tasks (documentation becomes 5, CI configuration becomes 6).

## Project Context

Darkfactory missions should produce a CLAUDE.md for the project so future Claude sessions have persistent conventions. The template follows Claude Code's own best practices for what to include and exclude. ARCH.md is a living architecture document extracted from design.md.

## Architecture

SKILL.md Phase 4.4 has a "Mandatory Closing Tasks" section listing numbered items. The current order is: 1. Smoke test, 2. Cleanup, 3. Module shape re-check, 4. Documentation, 5. CI configuration (optional). The new item slots in at position 4, pushing documentation to 5 and CI to 6.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Design Template**: A reference document defining mandatory design.md sections (different from the CLAUDE.md template)

## What Previous Tasks Built

### Task 1.1: Design Template and Architecture Scoring
- Created: skills/mission/templates/design-template.md
- Modified: openspec/config.yaml, SKILL.md Phase 4.3

### Task 2.1: Spec Traceability in Contract
- Modified: skills/mission/templates/contract.md, SKILL.md Phase 5

### Task 3.1: Deterministic Anchors
- Modified: skills/mission/templates/plan.md, SKILL.md Phase 4.3

### Task 4.1: Task Strategy, Topology, Quality Standard
- Added: SKILL.md Phase 4.3a, Phase 4.4 topology subsection
- Modified: SKILL.md Phase 4.4 task quality standard, openspec/config.yaml rules.tasks, Phase Index

## Downstream Consumers

- Future missions use the CLAUDE.md template during the closing task
- The generated CLAUDE.md references @ARCH.md and @UBIQUITOUS_LANGUAGE.md

## Procedures
- Commit all changes in a single commit with message: "[task-5.1] Add CLAUDE.md template and closing task"
- Write your handoff report to openspec/changes/deterministic-planning/mission/handoffs/task-5.md

## Handoff Report Format
- Task: Task 5.1 - CLAUDE.md Closing Task and Template
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
