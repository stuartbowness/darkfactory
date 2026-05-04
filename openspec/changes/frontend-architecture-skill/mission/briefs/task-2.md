# Worker Brief: Task 2 - Scrutiny validator gate 7 + gate renumbering + gate 1 update

## Your Task

Modify `skills/mission/prompts/scrutiny-validator.md` to:
1. Insert a new gate 7 (Frontend depth review) after gate 6 (Module depth review)
2. Renumber existing gates 7-14 to 8-15
3. Update gate 1 (test presence) to be methodology-aware

The file is 301 lines. The gate sequence starts at line 113 with `1. **Test presence gate:**`. The current gate numbering is 1-14.

## Exact Changes

### Change 1: Insert gate 7 after gate 6

After the current gate 6 (Module depth review, which ends around line 159), insert the following new gate:

```markdown
7. **Frontend depth review:** If the diff contains zero `.tsx` or `.jsx` files, report `frontend-depth | SKIP | no React component files in diff` and move to the next gate.

   When `.tsx`/`.jsx` files are present, evaluate each against the four deep-component rules defined in `skills/frontend/DEEP-COMPONENTS.md`:

   - **Small semantic interface:** Flag components where the caller must understand more than 7 independent semantic concepts (count props + context providers consumed + URL params read + cache keys depended on). Do not count raw prop count alone — a component with 6 props describing one domain concept is deep. Exempt components in `*/ui/*` directories, components with only `className` + `children` + `...rest` props (design system leaves), and components importing from `@radix-ui/*` that wrap a Radix primitive with styling (headless behavior primitives).
   - **Derive don't sync:** Flag `useEffect` paired with `setState` where the effect body is pure data transformation (no I/O, no subscriptions, no browser APIs). Suggest `useMemo` or inline computation as remediation. Do not flag effects that perform I/O, subscribe to external sources, or call imperative browser APIs.
   - **Composition over configuration:** Flag components accepting config-style props — arrays of item objects or option objects with more than 5 configuration entries — where compound components with children/slots would be clearer. Do not flag headless primitives or design system components that use config props by convention.
   - **Collapse coordination:** Flag components that expose coordination knobs such as `onStep1Complete`, `onValidate`, `onReset`, `onLoadingChange` rather than semantic events such as `onSubmit`, `onCancel`, `onSelect`, `onChange`. Coordination knobs indicate the parent is driving the child's internal workflow rather than the child owning it.

   Allowed-shallow exceptions (do not flag these for depth violations):
   - Components in `*/ui/*` directories
   - Components with only `className` + `children` + `...rest` props
   - Components importing from `@radix-ui/*` that wrap a Radix primitive
   - Route/layout glue components (thin wrappers that read route params and pass data)

   Verdict:
   - PASS: No violations found, or all flagged items are in allowed-shallow exception categories.
   - WARN: Isolated finding in a single component.
   - FAIL: Systematic violations found across multiple components in the diff.

   Report: Write one bullet per finding instance; state which component, which rule, and what specifically violates it. If nothing flagged, write "No frontend depth issues."
```

### Change 2: Renumber gates 7-14 to 8-15

After inserting the new gate 7, renumber the existing gates:
- Current gate 7 (Code standards) → gate 8
- Current gate 8 (Code simplify) → gate 9
- Current gate 9 (Security review) → gate 10
- Current gate 10 (Contract checks) → gate 11
- Current gate 11 (Behavioral conformance) → gate 12
- Current gate 12 (Silent skip detection) → gate 13
- Current gate 13 (Diff coverage) → gate 14
- Current gate 14 (Test quality review) → gate 15

### Change 3: Update gate 1

Current gate 1 text (line 113):
```
1. **Test presence gate:** ... Workers are required to use /tdd -- no new tests means TDD was not followed. ...
```

Change to be methodology-aware:
```
1. **Test presence gate:** Run `git show <commit-sha> --stat` and check if any test files were added or modified. If the diff contains zero test files and zero new test assertions, this is an automatic FAIL. If the task has `Skill: /tdd`, the rationale is "TDD not followed." If the task has `Skill: /frontend`, the rationale is "/frontend Test phase not followed." Exception: closing tasks that produce no testable code (documentation, cleanup, module shape re-check) are exempt from this gate.
```

### Change 4: Update Gate Results output format

The output format table (around line 270) lists gate rows. Add a row for gate 7 and renumber:
```
| Gate 7: Frontend depth review | PASS/FAIL/WARN/SKIP | ... |
```

## Project Context

The scrutiny validator is the spine of Darkfactory's validation system. Every worker's commit passes through these gates. This change adds React-specific quality checks while preserving all existing functionality.

## Architecture

The gate sequence is a flat numbered list in a single markdown file. Gates are independent — each reads the diff and produces a verdict. The new gate follows the self-skipping pattern established by gate 9 (Security review): check a condition, skip if not applicable.

## Key Terms

- **Gate**: A numbered step in the scrutiny validator's sequence. Reports PASS, FAIL, WARN, SKIP, or N/A.
- **Self-skipping gate**: A gate that naturally skips itself when the diff contains no relevant files.
- **Deep component**: A React component with a small prop interface hiding significant behavior.
- **Allowed-shallow exception**: A component exempt from depth rules (design system leaf, route glue, adapter, headless primitive).
- **Methodology**: /tdd (red-green-refactor) or /frontend (design-implement-test), assigned per task.

## Domain Types

None for this task.

## What Previous Tasks Built

### Task 1: Foundation vocabulary files
- Created: skills/frontend/DEEP-COMPONENTS.md, skills/frontend/STACK.md
- DEEP-COMPONENTS.md defines: four deep-component rules (Small Semantic Interfaces, Derive Don't Sync, Composition Over Configuration, Collapse Coordination), allowed-shallow exceptions (design system leaves, route/layout glue, adapter components, headless behavior primitives), and the deletion test
- STACK.md defines: React 19 + Vite + TailwindCSS 4 + Radix/ShadCN + Vitest + React Router stack, import discipline, file organization, semantic tokens, data-slot conventions

## Downstream Consumers

- Gate 7 is consumed by every future mission that touches .tsx/.jsx files
- The gate renumbering affects any UBIQUITOUS_LANGUAGE.md references to gate numbers (updated in task 10)

## Procedures

- Use `/tdd` to implement this task
- Commit all changes in a single commit with message: "[task-2] Add frontend depth gate 7 and renumber gates"
- Write your handoff report to openspec/changes/frontend-architecture-skill/mission/handoffs/task-2.md

## Handoff Report Format

- Task: Task 2 - Scrutiny validator gate 7 + renumbering + gate 1 update
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
