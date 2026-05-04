## 1. Foundation

- [x] Task 1: Foundation vocabulary files (DEEP-COMPONENTS.md, STACK.md)
  Depends on: none
  Skill: /tdd
  Constraints: none
  Consumes: none
  Produces: skills/frontend/DEEP-COMPONENTS.md, skills/frontend/STACK.md
  Layers: shared/types

  Create `skills/frontend/` directory. Write DEEP-COMPONENTS.md defining: four deep-component rules (small semantic interfaces, derive don't sync, composition over configuration, collapse coordination) each with a diagnostic question; allowed-shallow exception list (design system leaves, route/layout glue, adapter components, headless behavior primitives) each with rationale; deletion test. Write STACK.md declaring: React 19, Vite, TailwindCSS 4, Radix UI / ShadCN, Vitest, React Router; import discipline (direct imports, no barrel files, cn() utility); file organization (components/ui/, components/<domain>/, hooks/, lib/); semantic color tokens; data-slot conventions.

## 2. Validation

- [x] Task 2: Scrutiny validator gate 7 + gate renumbering + gate 1 update
  Depends on: 1
  Skill: /tdd
  Constraints: none
  Consumes: DEEP-COMPONENTS.md (criteria for gate 7 evaluation)
  Produces: updated scrutiny-validator.md with 15 gates
  Layers: backend

  Insert gate 7 (Frontend depth review) after gate 6 (Module depth review) in skills/mission/prompts/scrutiny-validator.md. Gate 7 is a self-skipping gate: if the diff contains zero .tsx/.jsx files, report SKIP with note "no React component files in diff". When React files are present, evaluate each .tsx/.jsx file against the four rules from DEEP-COMPONENTS.md with these specific checks: (1) Small semantic interface — flag components with more than 7 semantic concepts (props + context + URL params + cache dependencies) that are not in the allowed-shallow list; (2) Derive don't sync — flag useEffect paired with setState where the effect body is pure data transformation (no I/O, no subscriptions), suggest useMemo or inline computation; (3) Composition over configuration — flag components accepting config-style props (items arrays, option objects with more than 5 config entries) that could be compound components; (4) Collapse coordination — flag components exposing coordination knobs (onStep1Complete, onValidate, onReset) rather than semantic events (onSubmit, onCancel). Recognize allowed-shallow exceptions: components in */ui/* directories, components with only className + children + ...rest props, Radix wrapper components (check for @radix-ui/* imports). Verdict: PASS (no violations or all allowed-shallow), WARN (isolated finding in one component), FAIL (systematic violations across multiple components). Renumber existing gates 7-14 to 8-15. Update gate 1 (test presence) to use rationale "/frontend Test phase not followed" when task has Skill: /frontend. Update the output format table to include the new gate row.

## 3. Core Skill

- [x] Task 3: Core /frontend SKILL.md
  Depends on: 1
  Skill: /tdd
  Constraints: none
  Consumes: DEEP-COMPONENTS.md, STACK.md
  Produces: skills/frontend/SKILL.md
  Layers: shared/types

  Write SKILL.md with YAML frontmatter (name: frontend, user-invokable: true, description with trigger words for frontend, React, component, UI, TypeScript). Define the design-implement-test workflow with four phases: (1) Scan — search existing components, identify reusable primitives, check existing hooks, identify test patterns, write scan note; (2) Contract — per component: name, deep/shallow classification with reason, prop interface with types, key render states; (3) Build — implement component, compose from existing, visually verify via chrome-devtools-mcp (conditional on browser_tools config); (4) Test — behavioral tests via Vitest + React Testing Library. Define the per-component loop: complete Contract → Build → Test for each component before starting the next. Include deep-component audit checklist before commit. Reference DEEP-COMPONENTS.md, STACK.md, PATTERNS.md, TESTING.md via markdown links.

## 4. Examples and Testing Guide

- [x] Task 4: PATTERNS.md and TESTING.md
  Depends on: 3
  Skill: /tdd
  Constraints: none
  Consumes: SKILL.md, DEEP-COMPONENTS.md
  Produces: skills/frontend/PATTERNS.md, skills/frontend/TESTING.md
  Layers: shared/types

  Write PATTERNS.md with concrete code snippets (5-10 lines each) illustrating: at least two deep component examples (small prop interface owning a workflow), at least two allowed-shallow examples (design system leaves with compound composition like Card.Header/Card.Content), at least one hook pattern demonstrating derive-don't-sync, and at least one anti-pattern with before/after fix. Snippets will be pre-extracted from ~/Code/toyo and included in the worker brief. Write TESTING.md covering: behavioral testing principles (test visible behavior, interact as user would, query by role/label/text, fire events, assert visible outcomes); what not to test (prop plumbing, hook calls, internal state, class names, snapshot tests for layout); hook logic boundary (tested in separate /tdd tasks).

## 5. Supplementary Skills

- [x] Task 5: /ui-system skill
  Depends on: none
  Skill: /tdd
  Constraints: none
  Consumes: none
  Produces: skills/ui-system/SKILL.md
  Layers: shared/types

  Create `skills/ui-system/` directory. Write SKILL.md with YAML frontmatter (name: ui-system, user-invokable: true, description with trigger words for ShadCN, Radix, component library, design system, Tailwind tokens). Include verbatim upstream ShadCN skill content from skills.sh covering: component selection (search existing before writing custom), semantic token discipline (no raw values, no manual dark: overrides, className for layout only), composition patterns (cn() for conditional classes, gap-* over space-x/y-*, data-slot conventions), accessibility defaults (Dialog/Sheet/Drawer need Title, Avatar needs AvatarFallback), form patterns, icon handling, CLI usage. The upstream content will be fetched by the orchestrator and included verbatim in the worker brief.

- [x] Task 6: /react-perf skill
  Depends on: none
  Skill: /tdd
  Constraints: none
  Consumes: none
  Produces: skills/react-perf/SKILL.md
  Layers: shared/types

  Create `skills/react-perf/` directory. Write SKILL.md with YAML frontmatter (name: react-perf, user-invokable: true, description with trigger words for React performance, waterfalls, bundle size, re-renders, optimization). Include verbatim upstream Vercel React best practices content from skills.sh covering: waterfall elimination (sync checks before awaiting, Promise.all, start early await late, Suspense streaming), bundle size (direct imports, dynamic imports, post-hydration analytics, preload on hover), server-side patterns (authenticate server actions, React.cache, minimize client data, after() for non-blocking), re-render discipline (don't subscribe unused state, derive during render, functional setState, no components inside components, startTransition), rendering patterns (content-visibility, ternary not &&, useTransition), JS performance (Map for lookups, Set for O(1), flatMap, idle time deferral). The upstream content will be fetched by the orchestrator and included verbatim in the worker brief.

## 6. Infrastructure

- [x] Task 7: Template modifications, config, and mission-init
  Depends on: 1, 3, 5, 6
  Skill: /tdd
  Constraints: none
  Consumes: skills/frontend/SKILL.md, skills/ui-system/SKILL.md, skills/react-perf/SKILL.md
  Produces: updated worker-brief.md, design-template.md, config.yaml template in skills/mission/SKILL.md, updated mission-init SKILL.md
  Layers: backend

  Modify skills/mission/templates/worker-brief.md (line 53): change Procedures section from hardcoded "Use `/tdd` to implement this task" to conditional — read task's Skill: field, reference /frontend or /tdd accordingly, append Constraints as additional references (e.g., "Also follow `/ui-system` for component selection and token discipline"). Modify skills/mission/templates/design-template.md: add a paragraph to the Module Inventory section with frontend module guidance (describe prop interface as semantic concepts, state/effects encapsulated, deep vs allowed-shallow classification with reason). Update the Phase 6 config.yaml template in skills/mission/SKILL.md to include `browser_tools: true` field under engines. Update skills/mission-init/SKILL.md to check chrome-devtools-mcp availability when config.yaml has `browser_tools: true` (check that the MCP server is registered in the session, fail fast with error message and install instructions if missing). Add the following orchestrator behaviors to skills/mission/SKILL.md: (a) Phase 4.3a — for TypeScript React projects, use specific layer vocabulary (frontend/component, frontend/page, frontend/layout → Skill: /frontend; frontend/hooks, backend, shared/* → Skill: /tdd); auto-set browser_tools: true when any task has Skill: /frontend; (b) Phase 4.4 — validate that Skill: has exactly one value, that Constraints: /ui-system and /react-perf are only used with Skill: /frontend, flag invalid combinations; split complex frontend features into /tdd hook task + /frontend component task; (c) Step 2 (Dispatch) — soft pre-flight check for chrome-devtools-mcp before /frontend task dispatch, block and surface to human if unavailable; (d) Step 1 (Brief) — respect browser_tools: false override by omitting visual verification from worker brief.

## 7. Closing

- [ ] Task 8: Smoke test
  Depends on: 1, 2, 3, 4, 5, 6, 7
  Skill: /tdd
  Constraints: none
  Consumes: all prior task outputs
  Produces: verified structural integrity
  Layers: test

  Run ./install.sh and verify the 3 new symlinks exist (frontend, ui-system, react-perf). Verify all markdown links in new skill files resolve to existing files. Verify scrutiny validator gates are numbered 1-15 sequentially with no gaps or duplicates. Verify SKILL.md frontmatter for all 3 new skills has name, user-invokable: true, and description fields. Run a grep to confirm no remaining references to /shadcn in any file. Verify worker-brief.md Procedures section references /frontend or /tdd conditionally (not hardcoded). Verify worker-brief.md appends Constraints after methodology reference. Verify config.yaml template contains browser_tools field. Verify gate 1 in scrutiny-validator.md contains "/frontend Test phase not followed" rationale text for /frontend tasks. Verify skills/mission/SKILL.md contains: layer vocabulary mapping rules, Skill:/Constraints: validation rules, browser_tools auto-set logic, dispatch pre-flight check, and browser_tools: false override handling.

- [ ] Task 9: Cleanup + module shape re-check
  Depends on: 8
  Skill: /tdd
  Constraints: none
  Consumes: all prior task outputs, design.md Module Inventory
  Produces: clean codebase, mission/validations/module-shape-audit.md
  Layers: test

  Remove any unused files, dead references, or leftover scaffolding from the build process. Compare design.md Module Inventory descriptions to actual file structure. Verify each module listed in the inventory exists with the described interface. Write findings to mission/validations/module-shape-audit.md.

- [ ] Task 10: Documentation
  Depends on: 9
  Skill: /tdd
  Constraints: none
  Consumes: all prior task outputs
  Produces: updated README.md, UBIQUITOUS_LANGUAGE.md gate refs, CLAUDE.md/ARCH.md if needed
  Layers: shared/types

  Update README.md to document the 3 new skills (frontend, ui-system, react-perf) with descriptions matching their SKILL.md frontmatter. Update any UBIQUITOUS_LANGUAGE.md gate number references that still reference the old 1-14 numbering to the new 1-15 numbering. If CLAUDE.md or ARCH.md exist and reference skill counts or gate counts, update them. Do not create CLAUDE.md or ARCH.md if they don't exist. After all updates, verify UBIQUITOUS_LANGUAGE.md gate number references match the new 1-15 sequence (this check was deferred from task 8 because task 10 may update the file).
