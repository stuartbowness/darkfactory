# Adversarial Review B

Reviewer: Claude (adversarial-b)
Base: 690f314ebb8602acc6736fa258ee7dba9d1b7930
Branch: factory-frontend (11 commits)

---

## Task 1: ea9c876 — Foundation vocabulary files (DEEP-COMPONENTS.md, STACK.md)

### MEDIUM: No numeric threshold for "small semantic interface"

DEEP-COMPONENTS.md defines the "small semantic interface" rule qualitatively ("count concepts, not props") but provides no numeric threshold. Gate 7 in the scrutiny validator (task 2) introduces a hard threshold of "more than 7 independent semantic concepts." The skill that workers read says nothing about 7. A worker building a component with 8 semantic concepts would pass their own pre-commit checklist but fail gate 7. The threshold should appear in DEEP-COMPONENTS.md so workers can self-check, or gate 7 should explain why it chose 7 and link back.

**File:** `skills/frontend/DEEP-COMPONENTS.md` (rule 1 section)
**File:** `skills/mission/prompts/scrutiny-validator.md` (gate 7, line 165)

### LOW: STACK.md says "React Router" without version

STACK.md lists "React Router" but not the version. React Router v7 (the current stable since early 2025, rebranded from Remix) has a substantially different API from v6 (loaders, actions, file-based routing). Workers will not know which API surface to target. This is less critical because the skill says "use React Router for client-side routing" generically, but it could lead to confusion on projects using v7's framework mode vs SPA mode.

**File:** `skills/frontend/STACK.md` (line 13)

### LOW: Test file committed and later deleted

`openspec/changes/frontend-architecture-skill/mission/tests/test-task-1.sh` was committed in task 1 and deleted in task 9. Not harmful, but indicates the worker brief for task 1 should have specified where test artifacts belong or the test should have been in a non-committed location.

---

## Task 2: 43d6810 — Frontend depth gate 7 and renumber gates

### MEDIUM: Gate 7 threshold of 7 not justified or cross-referenced

Gate 7 introduces a hard threshold: "Flag components where the caller must understand more than 7 independent semantic concepts." This number does not appear in DEEP-COMPONENTS.md or UBIQUITOUS_LANGUAGE.md. It seems to be a magic number. There is no rationale for why 7 and not 5 or 10. The threshold should either be documented in DEEP-COMPONENTS.md (so it appears in one canonical place) or the gate should reference where the threshold comes from.

**File:** `skills/mission/prompts/scrutiny-validator.md` (line 165)

### MEDIUM: Gate 7 "more than 5 configuration entries" threshold also undocumented

The composition-over-configuration sub-rule in gate 7 says "option objects with more than 5 configuration entries." This threshold also does not appear in DEEP-COMPONENTS.md. Same issue as above -- workers have no way to self-check against this number.

**File:** `skills/mission/prompts/scrutiny-validator.md` (line 167)

### LOW: Gate 7 does not mention `.ts` files containing JSX

Some projects configure TypeScript to allow JSX in `.ts` files (not `.tsx`). Gate 7 only checks for `.tsx` and `.jsx`. This is an edge case but worth noting. The self-skip logic could miss React components in `.ts` files.

### LOW: Gate 1 update assumes `Skill:` field exists

Gate 1 now says "If the task has `Skill: /tdd`" and "If the task has `Skill: /frontend`." For non-TypeScript projects, the task topology does not include a `Skill:` field. Gate 1 should handle the case where no `Skill:` field exists (fall back to the original "/tdd not followed" rationale). The current wording is ambiguous about this fallback.

**File:** `skills/mission/prompts/scrutiny-validator.md` (line 113)

---

## Task 3: 794e939 — Core /frontend SKILL.md

### HIGH: "feature component" used as synonym for "deep component"

SKILL.md line 75 says "Push meaningful workflow ownership into the deep feature component." UBIQUITOUS_LANGUAGE.md explicitly lists "feature component" as an alias to avoid for "deep component." This appears in three files:

- `skills/frontend/SKILL.md` line 75: "deep feature component"
- `skills/frontend/DEEP-COMPONENTS.md` line 48: "hand that data to a feature component"
- `skills/frontend/STACK.md` line 67: "pass domain facts into deep feature components"

The term should be "deep component" in all three locations. "Feature component" is a legacy React community term (smart/dumb, container/presentational, feature/ui) that the ubiquitous language intentionally replaces.

**Files:** `skills/frontend/SKILL.md`, `skills/frontend/DEEP-COMPONENTS.md`, `skills/frontend/STACK.md`

### MEDIUM: SKILL.md Phase 2 Contract and Phase 1 Scan overlap with ubiquitous language term "component contract"

UBIQUITOUS_LANGUAGE.md defines "component contract" as "a lightweight per-component declaration written before implementation, specifying name, deep/shallow classification, prop interface, and key render states." SKILL.md's Phase 2 matches this definition exactly, which is good. However, Phase 1 (Scan) asks the worker to "Write a scan note" with items like "Similar components found, Reusable primitives, Existing hooks to consume, Test pattern, Visual mount point, Constraints." The relationship between the scan note and the component contract is unclear -- are they separate artifacts or does the scan note feed into the component contract? This could cause workers to produce redundant artifacts.

### LOW: SKILL.md audit checklist is rendered twice

Lines 114-124 contain a markdown code block with the audit checklist, but line 117 starts with "Before committing, review every .tsx file you created or modified:" which duplicates the text on line 114 that precedes the code block. This is a minor formatting issue -- the instruction appears both as prose and as the first line inside the code block.

**File:** `skills/frontend/SKILL.md` (lines 112-124)

### LOW: SKILL.md references PATTERNS.md conditionally

Line 74 says "Use [PATTERNS.md](PATTERNS.md) for concrete examples when it exists." The "when it exists" qualifier is correct for task ordering (PATTERNS.md is created in task 4, after SKILL.md in task 3), but after the mission completes, PATTERNS.md always exists. The conditional phrasing will confuse workers reading the skill outside a mission context.

---

## Task 4: cb16105 — PATTERNS.md and TESTING.md

### LOW: PATTERNS.md examples come from a single project

All examples in PATTERNS.md reference `~/Code/toyo`. This is fine for internal use but makes the patterns feel project-specific rather than universal. If this skill is used across multiple projects, workers may wonder whether the patterns apply to their codebase. The source annotations could be more generic (e.g., "Example: file management drawer" rather than "Source: `~/Code/toyo` file management drawer").

### LOW: PATTERNS.md has only one anti-pattern example

The contract check says "at least 1 anti-pattern with a before/after or 'don't do this'/'do this' structure." PATTERNS.md has exactly one (the useEffect syncing derived state anti-pattern at line 130). For a comprehensive reference, anti-patterns for the other three rules (small semantic interface, composition over configuration, collapse coordination) would strengthen the guidance.

### LOW: TESTING.md does not mention async testing patterns

TESTING.md covers query-by-role, userEvent, and what not to test, but does not mention `waitFor`, `findBy*`, or async rendering patterns. Components that fetch data or use transitions will need async assertions. Workers unfamiliar with React Testing Library may write synchronous assertions that pass on fast machines but flake in CI.

---

## Task 5: 4a2212c — /ui-system skill

### HIGH: /react-perf SKILL.md is 4003 lines and heavily Next.js-specific

(Filed under task 5 for the constraint skill pair, but applies to task 6.)

The react-perf skill's description says "React and Next.js performance optimization." STACK.md declares the default stack as React 19 + Vite -- not Next.js. The react-perf SKILL.md contains extensive Next.js-specific guidance:

- Section 3 "Server-Side Performance" (10 rules) is almost entirely Next.js-specific: RSC props, Server Actions, `next/server`, `next/headers`, `next/og`, `React.cache()` for request deduplication, `after()` from `next/server`.
- Section 6.7 references `<Activity>` which is React 19 experimental.
- Section 8.4 references `useEffectEvent` which is still experimental.
- `next/dynamic` for lazy loading (section 2.4) is Next.js-specific; Vite projects use `React.lazy`.

A Vite-based worker reading this skill will encounter large sections that do not apply and may attempt to use Next.js APIs in a Vite project. The skill needs either:
1. Clear section-level markers indicating which rules are Next.js-only vs universal React, or
2. A preamble explaining that sections 3.x and some 6.x rules only apply to Next.js projects.

This is the single largest risk in the mission. A confused worker could import `next/dynamic` or `next/server` in a Vite project.

**File:** `skills/react-perf/SKILL.md`

### MEDIUM: /ui-system references components that may not exist in every project

The ui-system skill references specific components like `FieldGroup`, `Field`, `FieldLabel`, `InputGroup`, `InputGroupInput`, `InputGroupAddon`, `FieldSet`, `FieldLegend`, `Empty`, `Spinner`. These are not part of the core shadcn/ui library -- they may be project-specific additions or from specific registries. A worker on a fresh shadcn/ui installation will not have these components and will be confused by the prescriptive guidance to use them.

The skill says "Use `npx shadcn@latest search` to check registries" but does not clarify which components are core vs community vs project-specific.

**File:** `skills/ui-system/SKILL.md` (lines 104-108, 141-144)

### LOW: /ui-system "never use --overwrite without user approval" is vague for autonomous workers

Line 172 says "Never use `--overwrite` without user approval." In a mission context, the worker is autonomous and cannot ask the user mid-task. This should specify that `--overwrite` requires a handoff concern or a circuit-breaker stop, not just "user approval."

---

## Task 6: ae379f3 — /react-perf skill

### HIGH: 4003-line skill file exceeds practical context limits

At 4003 lines (~113KB), the react-perf SKILL.md is by far the largest file in the repository. When a worker is instructed to "Also follow `/react-perf`", the entire file enters the context window. This will consume a significant fraction of the worker's available context, potentially crowding out the actual task brief, codebase context, and other skills. For comparison, the mission SKILL.md is 58KB and is the orchestrator's operating manual, not a worker constraint.

Consider splitting into a core rules file (the top 2 categories: waterfalls + bundle size, marked CRITICAL) and an extended reference for the remaining 6 categories. Workers would read the core by default and the extended only when specifically relevant.

**File:** `skills/react-perf/SKILL.md`

### MEDIUM: `<Activity>` component is experimental

Section 6.7 recommends `import { Activity } from 'react'` for show/hide optimization. As of React 19 stable (December 2024), `<Activity>` is an experimental API (previously called `<Offscreen>`). It is not available in production React builds. Workers following this guidance will get import errors. The section should note the experimental status and provide a fallback pattern.

**File:** `skills/react-perf/SKILL.md` (lines 2590-2610)

### MEDIUM: `useEffectEvent` is experimental

Section 8.4 imports `useEffectEvent` from `'react'`. This hook is still experimental as of React 19 and is only available behind the `experimental` channel. The section title "Advanced Patterns" and impact "LOW" somewhat mitigate this, but a worker will still try to use it and hit an import error.

**File:** `skills/react-perf/SKILL.md` (lines 3641-3695)

---

## Task 7: 7b888ed — Wire frontend skills into mission infrastructure

### MEDIUM: Worker brief template includes meta-instructions

The worker brief template (lines 53-58) now contains template meta-instructions like:
```
- Use `<Skill from task topology>` to implement this task
  - If Skill: /frontend -- "Use `/frontend` to implement this task"
  - If Skill: /tdd -- "Use `/tdd` to implement this task"
- <For each Constraint in task topology, append:>
```

This is clearly a template for the orchestrator to fill in, not content that should appear verbatim in an assembled brief. But the template format is inconsistent with the rest of the file -- other sections use `<angle bracket placeholders>` while this section mixes angle brackets with prose instructions. If the orchestrator reads this too literally, workers could receive briefs containing "If Skill: /frontend --" as literal text.

**File:** `skills/mission/templates/worker-brief.md` (lines 53-58)

### MEDIUM: Constraint validation rule says /ui-system and /react-perf "only valid with Skill: /frontend" but the skills themselves say they are independently invokable

Both /ui-system and /react-perf have `user-invokable: true` in their frontmatter and their descriptions say they can be used independently. But the mission SKILL.md validation rule (line 273) says they are "only valid with `Skill: /frontend`." This creates a contradiction: the skill says it can be used standalone, but the mission infrastructure rejects it as a constraint on non-/frontend tasks.

This is probably intentional (standalone invocation by a human vs mission-assigned constraint), but the distinction is not documented. A developer might expect to assign `/react-perf` as a constraint to a `/tdd` task for a performance-sensitive hook, which would be rejected.

**File:** `skills/mission/SKILL.md` (lines 270-274)
**File:** `skills/ui-system/SKILL.md` (frontmatter)
**File:** `skills/react-perf/SKILL.md` (frontmatter)

### LOW: mission-init browser tools check has a placeholder implementation

The mission-init SKILL.md step 4 contains:
```bash
# Check if chrome-devtools MCP tools are available.
# The exact check depends on the MCP registration mechanism.
```

This is a comment, not an actual check. The step describes what to do if the check fails but does not specify how to perform the check. This means mission-init will not actually validate chrome-devtools-mcp availability -- it will rely on the implementer to figure out the check mechanism.

**File:** `skills/mission-init/SKILL.md` (lines 127-130)

### LOW: design-template.md frontend guidance references skill file directly

The design template (line 71) says "Reference `/frontend` skill's DEEP-COMPONENTS.md for the four deep-component rules." Design.md is written during the planning phase before skills are necessarily available or relevant. The design template should reference the rules by name (from UBIQUITOUS_LANGUAGE.md) rather than pointing to a specific skill file path.

---

## Task 8: cb32394 — Smoke test + mission artifacts

### MEDIUM: Unrelated `user-friendly-contract-language` change tree committed

The smoke test commit includes the entire `openspec/changes/user-friendly-contract-language/` directory tree (briefs, handoffs, logs, validations, etc.) from a previous mission. This is 60+ unrelated files committed in the same changeset. These should either be committed separately or excluded from this branch's PR.

This was already flagged by adversarial-a.

### LOW: .openspec.yaml created date says 2026-05-03

The `.openspec.yaml` file has `created: 2026-05-03` but the commits are dated 2026-05-03/04. Minor timestamp issue, not functional.

---

## Task 9: dd6dd48 — Cleanup + module shape audit

No issues found. The cleanup correctly removes the leftover test file and the module shape audit is thorough.

---

## Task 10: 9d2e32a — README, UBIQUITOUS_LANGUAGE, ARCH updates

### LOW: UBIQUITOUS_LANGUAGE.md gate examples only partially updated

The UBIQUITOUS_LANGUAGE.md gate number examples were updated from "gate 10: contract checks" to "gate 11: contract checks" and "gate 11" to "gate 12" for behavioral conformance. However, the Gate definition still says "(e.g., gate 3: tests, gate 11: contract checks)" -- should verify gate 3 is still correct after renumbering (it is: test presence is gate 1, build is 2, test is 3 -- gate 3 for tests is correct).

No issue found after verification. The updates are correct.

---

## Cross-cutting findings

### HIGH: Next.js/Vite stack mismatch in /react-perf (see task 5/6)

The most significant risk across the entire mission. STACK.md says the default stack is Vite. /react-perf contains ~30% Next.js-specific content with no markers or filtering. This will confuse Vite-based workers.

### MEDIUM: "feature component" terminology violation (see task 3)

Three files use "feature component" which UBIQUITOUS_LANGUAGE.md marks as an alias to avoid. Should use "deep component" consistently.

### MEDIUM: Magic numbers in gate 7 not documented in worker-facing material (see tasks 1, 2)

Workers cannot self-check against the thresholds gate 7 will use (7 semantic concepts, 5 config entries) because these numbers only appear in the validator, not in DEEP-COMPONENTS.md.

---

## Summary by severity

| Severity | Count | Key items |
|----------|-------|-----------|
| CRITICAL | 0 | -- |
| HIGH | 3 | Next.js/Vite mismatch in react-perf; 4003-line file size; "feature component" terminology violation |
| MEDIUM | 7 | Magic thresholds in gate 7; Activity/useEffectEvent experimental; constraint validation vs standalone invocation; unrelated files in smoke test commit; ui-system non-core components; worker brief template meta-instructions |
| LOW | 9 | React Router version; test file churn; .ts JSX edge case; gate 1 fallback; audit checklist duplication; PATTERNS.md single-project examples; async testing gap; mission-init placeholder; design template skill reference |

### Recommendation

Ship with fixes for the three HIGH items:
1. Replace "feature component" with "deep component" in SKILL.md, DEEP-COMPONENTS.md, and STACK.md (3 edits).
2. Add a preamble to react-perf SKILL.md noting which sections are Next.js-only vs universal React, or split the file.
3. Accept the 4003-line file size as a known limitation and document it, or split into core/extended.

The MEDIUM items are worth fixing but do not block ship.
