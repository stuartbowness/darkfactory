# Worker Brief: Task 3 - Core /frontend SKILL.md

## Your Task

Write the main `skills/frontend/SKILL.md` file — the core builder guide for React/TypeScript component development. This is a prescriptive, worker-facing skill that defines the design-implement-test workflow.

The file MUST start with this YAML frontmatter:

```yaml
---
name: frontend
user-invokable: true
description: Prescriptive builder guide for React/TypeScript component development. Use when building frontend UI components, pages, or layouts. Defines the design-implement-test workflow with visual verification. Not for hooks, utilities, or backend code — those use /tdd.
---
```

## Workflow

Define a workflow with these phases, executed as a per-component loop:

### Phase 1: Scan
Instruct the worker to:
- Search existing components in the feature area (grep for domain nouns in component file names)
- Identify reusable ShadCN/Radix primitives already in use (`grep -r "@/components/ui"`)
- Check existing hooks to consume (not write — hook creation is a separate /tdd task)
- Open 3-5 nearby components and their test files to learn patterns
- Check for existing dev routes, stories, or fixtures for visual verification
- Write a **scan note** listing: similar components found, reusable primitives, existing hooks, test pattern, visual mount point, constraints

### Phase 2: Contract (per component, lightweight)
Instruct the worker to declare before building each component:
- **Component name**
- **Deep or allowed-shallow** classification with reason (if shallow)
- **Prop interface** with types — list each prop and what domain concept it represents
- **Key render states** — only those that apply: default, loading, error, empty, disabled

This is NOT a heavy artifact. Four items, written as a comment block or in the scan note.

### Phase 3: Build (per component, with visual verification)
Instruct the worker to:
- Implement the component, composing from existing ShadCN/Radix primitives
- Apply the four deep-component rules from [DEEP-COMPONENTS.md](DEEP-COMPONENTS.md)
- Follow stack conventions from [STACK.md](STACK.md)
- If `browser_tools: true` in mission config: visually verify via chrome-devtools-mcp (navigate to the component, check layout, check render states)
- If `browser_tools: false` (human override): skip visual verification and proceed to Test

### Phase 4: Test (per component)
Instruct the worker to:
- Write behavioral tests using Vitest + React Testing Library
- Follow testing guidance from [TESTING.md](TESTING.md)
- Test through rendered output: query by role, label, visible text; fire user events; assert visible outcomes
- Do NOT test implementation details (hook calls, internal state, prop plumbing, class names)

### Per-Component Loop (forcing function)
State clearly: "Complete Contract → Build → Test for each component before starting the next component. Do not build all components then test at the end."

### Deep-Component Audit (before commit)
Include a checklist the worker runs after all components are built and tested:

```
Before committing, review every .tsx file you created or modified:
[ ] Each prop represents a semantic domain concept, not an implementation knob
[ ] No useEffect syncing derived state (use useMemo or inline computation)
[ ] No config objects where compound components with slots would work
[ ] Parent doesn't coordinate state that belongs inside this component
[ ] Shallow components are classified as allowed-shallow with a reason
[ ] Behavioral tests cover key render states and user interactions
```

## References

Include these markdown links in the file:
- `[DEEP-COMPONENTS.md](DEEP-COMPONENTS.md)` — Four rules and allowed-shallow exceptions
- `[STACK.md](STACK.md)` — Technology stack and conventions
- `[PATTERNS.md](PATTERNS.md)` — Concrete code examples (created in task 4)
- `[TESTING.md](TESTING.md)` — Frontend behavioral testing guidance (created in task 4)

## Project Context

This skill is the frontend counterpart to `/tdd`. While `/tdd` uses red-green-refactor for code with testable interfaces, `/frontend` uses design-implement-test for React components where the output is visual and emergent.

## Architecture

The file goes in `skills/frontend/SKILL.md` alongside the foundation files created in task 1 (DEEP-COMPONENTS.md, STACK.md). It will be referenced by the worker brief template (task 7) when a task has `Skill: /frontend`.

## Key Terms

- **Methodology**: /tdd (red-green-refactor) or /frontend (design-implement-test). Mutually exclusive per task.
- **Design-implement-test**: The /frontend workflow. Design component interfaces, build with visual verification, then write behavioral tests.
- **Component contract**: Lightweight per-component declaration (name, deep/shallow, props, render states) written before building.
- **Visual verification**: Using chrome-devtools-mcp during Build to confirm correct rendering.
- **Deep component**: A React component with small prop interface hiding significant behavior, owning one workflow.
- **Allowed-shallow exception**: Intentionally thin component exempt from depth rules.

## Domain Types

None for this task.

## What Previous Tasks Built

### Task 1: Foundation vocabulary files
- Created: skills/frontend/DEEP-COMPONENTS.md, skills/frontend/STACK.md
- DEEP-COMPONENTS.md: four rules (Small Semantic Interfaces, Derive Don't Sync, Composition Over Configuration, Collapse Coordination), allowed-shallow exceptions (design system leaves, route/layout glue, adapters, headless primitives), deletion test
- STACK.md: React 19 + Vite + TailwindCSS 4 + Radix/ShadCN + Vitest + React Router, import discipline (no barrels, cn()), file organization, semantic tokens, data-slot

### Task 2: Scrutiny validator gate 7
- Modified: skills/mission/prompts/scrutiny-validator.md
- Inserted gate 7 (Frontend depth review) — self-skipping, checks four rules
- Renumbered gates 7-14 to 8-15 (now 15 gates total)
- Gate 1 now methodology-aware ("/frontend Test phase not followed")

## Downstream Consumers

- Task 4 (PATTERNS.md, TESTING.md) illustrates the workflow and rules defined here
- Task 7 (template modifications) references this skill in worker brief Procedures
- Task 8 (smoke test) verifies frontmatter and links

## Procedures

- Use `/tdd` to implement this task
- Commit all changes in a single commit with message: "[task-3] Add core /frontend SKILL.md"
- Write your handoff report to openspec/changes/frontend-architecture-skill/mission/handoffs/task-3.md

## Handoff Report Format

- Task: Task 3 - Core /frontend SKILL.md
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything uncertain>
