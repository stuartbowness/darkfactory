---
name: frontend
user-invokable: true
description: Prescriptive builder guide for React/TypeScript component development. Use when building frontend UI components, pages, or layouts. Defines the design-implement-test workflow with visual verification. Not for hooks, utilities, or backend code — those use /tdd.
---

# Frontend Component Development

Use this skill for React/TypeScript components, pages, and layouts. This is the frontend counterpart to `/tdd`: `/tdd` uses red-green-refactor for code with testable interfaces, while `/frontend` uses design-implement-test for visual React work where the final output emerges through component behavior, layout, and interaction.

Do not use this skill for hooks, pure utilities, backend code, or non-rendering logic. Those belong in `/tdd`.

## Method

Run the workflow as a per-component loop:

1. Scan the local frontend context.
2. Declare a lightweight component contract.
3. Build the component and visually verify it when browser tools are enabled.
4. Test rendered behavior.

Complete Contract → Build → Test for each component before starting the next component. Do not build all components then test at the end.

## Phase 1: Scan

Before writing component code, learn the local frontend shape.

- Search existing components in the feature area. Grep for domain nouns in component file names.
- Identify reusable ShadCN/Radix primitives already in use with `grep -r "@/components/ui"`.
- Check existing hooks that should be consumed. Do not create hooks in a `/frontend` task; hook creation is a separate `/tdd` task.
- Open 3-5 nearby components and their test files to learn naming, layout, state handling, event patterns, and assertion style.
- Check for existing dev routes, stories, fixtures, or test harnesses that can mount the component for visual verification.

Write a scan note before building. Keep it short, but include:

- Similar components found
- Reusable primitives
- Existing hooks to consume
- Test pattern
- Visual mount point
- Constraints

## Phase 2: Contract

Before building each component, declare its contract. This is not a heavy artifact. Write the four items as a comment block near the work, in the scan note, or in the task log.

- Component name
- Deep or allowed-shallow classification, with the reason if shallow
- Prop interface with types, listing each prop and the domain concept it represents
- Key render states that apply: default, loading, error, empty, disabled

Example:

```tsx
/*
Component: InvoiceReviewPanel
Depth: deep; owns the invoice review workflow from summary through submit/cancel.
Props:
- invoice: InvoiceReviewDraft - the invoice being reviewed
- reviewer: ReviewerIdentity - the active reviewer shown in review context
- onSubmit(decision: InvoiceDecision): void - final review outcome
- onCancel(): void - user exits review without a decision
Render states: default, loading, error, disabled
*/
```

## Phase 3: Build

Implement one component at a time.

- Compose from existing ShadCN/Radix primitives before creating new UI leaves.
- Follow the four deep-component rules in [DEEP-COMPONENTS.md](DEEP-COMPONENTS.md): Small Semantic Interfaces, Derive Don't Sync, Composition Over Configuration, and Collapse Coordination.
- Follow stack conventions in [STACK.md](STACK.md), including direct imports, `cn()`, semantic tokens, `data-slot`, and local file organization.
- Use [PATTERNS.md](PATTERNS.md) for concrete examples.
- Keep route and layout glue thin. Push meaningful workflow ownership into the deep component.
- If new state is derived from props or other state, compute it inline or with `useMemo`. Do not sync derived state with `useEffect`.
- If a component needs stateful logic that should live outside the component, stop and split that hook into a `/tdd` task instead of hiding hook creation inside `/frontend`.

### Visual Verification

If `browser_tools: true` in mission config, visually verify during the Build phase with `chrome-devtools-mcp`.

- Navigate to the dev route, story, fixture, or harness that mounts the component.
- Check the default layout at realistic viewport sizes.
- Check each relevant render state from the contract: loading, error, empty, and disabled.
- Exercise the main interaction path enough to see the resulting UI state.
- Fix visible layout, overflow, spacing, focus, and state issues before moving to Test.

If `browser_tools: false` in mission config, or the human has explicitly overridden browser use, skip visual verification and proceed to Test. Note the skip in the handoff.

## Phase 4: Test

Write behavioral tests for the component using Vitest + React Testing Library. Follow [TESTING.md](TESTING.md) for detailed frontend testing guidance.

Test through rendered output:

- Query by role, label, and visible text.
- Fire user events that match real interaction.
- Assert visible outcomes and semantic events.
- Cover the key render states from the component contract.

Do not test implementation details:

- No assertions on hook calls
- No assertions on internal state names
- No prop plumbing tests
- No class name or Tailwind implementation assertions
- No tests that only prove a child component was passed a value

Tests should describe what a user can see or do. They should survive internal refactors that preserve behavior.

## Deep-Component Audit

Before committing, review every `.tsx` file you created or modified:

```text
[ ] Each prop represents a semantic domain concept, not an implementation knob
[ ] No useEffect syncing derived state (use useMemo or inline computation)
[ ] No config objects where compound components with slots would work
[ ] Parent doesn't coordinate state that belongs inside this component
[ ] Shallow components are classified as allowed-shallow with a reason
[ ] Behavioral tests cover key render states and user interactions
```

If a component fails the audit, fix it before committing. If it is intentionally shallow, record the allowed-shallow reason from [DEEP-COMPONENTS.md](DEEP-COMPONENTS.md).

## References

- [DEEP-COMPONENTS.md](DEEP-COMPONENTS.md) - Four rules and allowed-shallow exceptions
- [STACK.md](STACK.md) - Technology stack and conventions
- [PATTERNS.md](PATTERNS.md) - Concrete code examples
- [TESTING.md](TESTING.md) - Frontend behavioral testing guidance
