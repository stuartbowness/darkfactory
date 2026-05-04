# Worker Brief: Task 1 - Foundation vocabulary files (DEEP-COMPONENTS.md, STACK.md)

## Your Task

Create the `skills/frontend/` directory and write two foundation reference files that define the deep-component rules and technology stack conventions.

**File 1: `skills/frontend/DEEP-COMPONENTS.md`**

Write a reference document defining the "deep components" thesis for React frontend architecture. Include:

### Four Rules (each with a diagnostic question)

1. **Small semantic interfaces** — A component's prop interface should represent domain concepts, not implementation knobs. Count concepts (props + context providers + URL params + cache dependencies), not raw prop count. A component with 6 props describing one concept is fine; a component with 1 prop bag containing 4 unrelated concerns is shallow.
   - Diagnostic: "How many independent domain concepts must a caller understand to use this component?"

2. **Derive don't sync** — Compute values during render (useMemo, inline computation) instead of syncing state with useEffect. Effects that transform data without I/O are a depth violation. Canonical source: React team's "You Might Not Need an Effect."
   - Diagnostic: "Can this value be computed from existing state/props during render instead of stored separately?"

3. **Composition over configuration** — Use compound component slots and children instead of config objects with many options. Callers compose behavior without understanding internals. A component accepting `items: [{label, icon, onClick, disabled, variant}]` is shallower than one accepting `<Menu><MenuItem>...</MenuItem></Menu>`.
   - Diagnostic: "Would this be clearer as compound components with children/slots instead of a config prop?"

4. **Collapse coordination** — A deep component owns one user-facing workflow end-to-end. It exposes semantic events (onSubmit, onCancel) not coordination knobs (onValidate, onStep1Complete, onReset). Parents choose what happens, not how the child reaches every intermediate state.
   - Diagnostic: "Does the parent coordinate state that belongs inside this component?"

### Allowed-Shallow Exceptions (each with rationale)

- **Design system leaves** (Button, Badge, Card sub-components) — Thin by design. CVA wrappers with data-slot attributes. Their job is visual consistency and accessibility hooks, not workflow ownership.
- **Route/layout glue** (App.tsx, page wrappers) — Read route params, hand data to a feature component. Acceptable if they stay thin and don't become workflow owners.
- **Adapter components** (auth-callback-page, OAuth redirect handlers) — Boundary concerns that adapt external flows. Not behavior ownership.
- **Headless behavior primitives** (Radix Dialog, Combobox, Popover wrappers) — Infrastructure that provides behavior without business rules. Should not know domain logic.

### Deletion Test

Include this diagnostic: "If you deleted this component, would complexity vanish (it was a pass-through) or reappear across every consumer (it was earning its keep)?"

**File 2: `skills/frontend/STACK.md`**

Write a reference document declaring the technology stack and conventions:

- **Stack:** React 19, Vite, TailwindCSS 4, Radix UI / ShadCN, Vitest, React Router
- **Import discipline:** Direct imports only — `import { Button } from '@/components/ui/button'`, NOT from barrel files like `'@/components/ui'`. Use `cn()` from `@/lib/utils` for conditional class composition.
- **File organization:** `components/ui/` for design system leaves, `components/<domain>/` for deep components (e.g., `components/chat/`, `components/toyo/`), `hooks/` for stateful logic extraction, `lib/` for pure utilities.
- **Semantic color tokens:** Use `bg-card`, `text-muted-foreground`, never raw Tailwind colors like `bg-blue-500`. No manual `dark:` overrides.
- **data-slot attributes:** Follow ShadCN convention — each component renders a `data-slot` attribute for styling hooks.
- **Testing:** Vitest + React Testing Library. Test components through rendered behavior, not implementation details.
- **React Router:** Use React Router for client-side routing.

## Project Context

Darkfactory is a mission orchestration system built as Claude Code skills. This mission adds frontend architecture guidance. These two files are the foundation that all other skill files reference.

## Architecture

These files live in `skills/frontend/` alongside SKILL.md (task 3), PATTERNS.md, and TESTING.md (task 4). SKILL.md will link to both files via markdown links. Gate 7 in the scrutiny validator (task 2) will reference DEEP-COMPONENTS.md criteria for its evaluation.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Deep component**: A React component with a small prop interface that hides significant state, effects, and rendering logic, owning one user-facing workflow end-to-end.
- **Allowed-shallow exception**: A component that is intentionally thin and exempt from depth rules.
- **Small semantic interface**: A component prop interface where each prop represents a domain concept, not an implementation knob.
- **Derive don't sync**: Computing values during render instead of syncing state with useEffect.
- **Composition over configuration**: Using compound component slots and children instead of config objects.
- **Collapse coordination**: A deep component owns one workflow end-to-end, exposing semantic events not coordination knobs.

## Domain Types

None for this task.

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

- Task 2 (gate 7) references DEEP-COMPONENTS.md criteria for React component evaluation
- Task 3 (SKILL.md) links to both files via markdown
- Task 4 (PATTERNS.md) illustrates the rules defined in DEEP-COMPONENTS.md

## Procedures

- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-1] Add foundation vocabulary files (DEEP-COMPONENTS.md, STACK.md)"
- Write your handoff report to openspec/changes/frontend-architecture-skill/mission/handoffs/task-1.md

## Handoff Report Format

- Task: Task 1 - Foundation vocabulary files
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
