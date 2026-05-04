# Contract

## Task 1: Foundation vocabulary files (DEEP-COMPONENTS.md, STACK.md)
<!-- Specs: frontend/1, frontend/8, frontend/9, frontend/10, frontend/11, frontend/12, frontend/13 -->
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` exists and contains the heading or phrase "Small semantic interface" (rule 1)
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains the heading or phrase "Derive don't sync" (rule 2)
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains the heading or phrase "Composition over configuration" (rule 3)
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains the heading or phrase "Collapse coordination" (rule 4)
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains "design system" in the allowed-shallow exceptions section
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains "route" or "layout" in the allowed-shallow exceptions section
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains "adapter" in the allowed-shallow exceptions section
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains "headless" in the allowed-shallow exceptions section
- [ ] [output] `skills/frontend/DEEP-COMPONENTS.md` contains "deletion test"
- [ ] [output] `skills/frontend/STACK.md` exists and contains "React 19"
- [ ] [output] `skills/frontend/STACK.md` contains "Vite"
- [ ] [output] `skills/frontend/STACK.md` contains "TailwindCSS" or "Tailwind"
- [ ] [output] `skills/frontend/STACK.md` contains "Radix" or "ShadCN"
- [ ] [output] `skills/frontend/STACK.md` contains "Vitest"
- [ ] [output] `skills/frontend/STACK.md` contains "barrel" (in context of prohibiting barrel file imports)
- [ ] [output] `skills/frontend/STACK.md` contains "cn()" (utility reference)
- [ ] [output] `skills/frontend/STACK.md` contains "components/ui/" (file organization)
- [ ] [output] `skills/frontend/STACK.md` contains "React Router"
- [ ] [output] `skills/frontend/STACK.md` contains "semantic" in context of color tokens
- [ ] [output] `skills/frontend/STACK.md` contains "data-slot"

## Task 2: Scrutiny validator gate 7 + gate renumbering + gate 1 update
<!-- Specs: frontend-validation/1, frontend-validation/2, frontend-validation/3, frontend-validation/4, frontend-validation/5, frontend-validation/6, frontend-validation/7, frontend-validation/8 -->
- [ ] [output] `skills/mission/prompts/scrutiny-validator.md` contains a gate named "Frontend depth review" or "frontend depth"
- [ ] [execution] The scrutiny-validator.md contains exactly 15 gate headings in its gate sequence, numbered sequentially from 1 to 15
- [ ] [output] The gate sequence in scrutiny-validator.md has "Frontend depth" after "Module depth" and before "Code standards"
- [ ] [output] scrutiny-validator.md contains "no React component files in diff" (self-skip text)
- [ ] [output] scrutiny-validator.md gate 7 section contains ".tsx" or ".jsx" (file extension trigger)
- [ ] [output] scrutiny-validator.md gate 7 section references "semantic concepts" or "prop" (rule 1 check)
- [ ] [output] scrutiny-validator.md gate 7 section references "useEffect" and "setState" (rule 2 check)
- [ ] [output] scrutiny-validator.md gate 7 section references "config" or "configuration" (rule 3 check)
- [ ] [output] scrutiny-validator.md gate 7 section references "coordination" or "semantic events" (rule 4 check)
- [ ] [output] scrutiny-validator.md gate 7 section references "*/ui/*" or "ui/" (allowed-shallow directory pattern)
- [ ] [output] scrutiny-validator.md gate 7 section references "@radix-ui" (Radix wrapper exception)
- [ ] [output] scrutiny-validator.md gate 1 section contains "/frontend Test phase not followed" (updated rationale)
- [ ] [output] scrutiny-validator.md gate 1 section contains "Skill:" or "/frontend" (methodology-aware test presence)

## Task 3: Core /frontend SKILL.md
<!-- Specs: frontend/2, frontend/3, frontend/4, frontend/5, frontend/6, frontend/7 -->
- [ ] [output] `skills/frontend/SKILL.md` exists and starts with YAML frontmatter containing `name: frontend`
- [ ] [output] SKILL.md frontmatter contains `user-invokable: true`
- [ ] [output] SKILL.md frontmatter description contains "frontend" or "React" or "component"
- [ ] [output] SKILL.md contains "Scan" as a workflow phase heading or label
- [ ] [output] SKILL.md contains "Contract" as a workflow phase heading or label
- [ ] [output] SKILL.md contains "Build" as a workflow phase heading or label
- [ ] [output] SKILL.md contains "Test" as a workflow phase heading or label
- [ ] [output] SKILL.md contains "scan note" (Scan phase output)
- [ ] [output] SKILL.md contains "deep" and "shallow" in the Contract phase section (classification)
- [ ] [output] SKILL.md contains "chrome-devtools" or "visual" in the Build phase section
- [ ] [output] SKILL.md contains "browser_tools" (conditional visual verification)
- [ ] [output] SKILL.md contains "per-component" or "one component at a time" (forcing function)
- [ ] [output] SKILL.md contains a checklist section with "audit" or "before commit"
- [ ] [output] SKILL.md contains markdown links to `DEEP-COMPONENTS.md`, `STACK.md`, `PATTERNS.md`, `TESTING.md`
- [ ] [execution] Run `test -f skills/frontend/DEEP-COMPONENTS.md && test -f skills/frontend/STACK.md` — both files exist (SKILL.md link targets)

## Task 4: PATTERNS.md and TESTING.md
<!-- Specs: frontend/14, frontend/15, frontend/16, frontend/17, frontend/18, frontend/19, frontend/20 -->
- [ ] [output] `skills/frontend/PATTERNS.md` exists and contains at least 2 code blocks showing deep component examples
- [ ] [output] PATTERNS.md contains at least 2 code blocks showing allowed-shallow examples (design system leaves or compound components)
- [ ] [output] PATTERNS.md contains at least 1 code block showing a hook with derive-don't-sync pattern
- [ ] [output] PATTERNS.md contains at least 1 anti-pattern with a before/after or "don't do this"/"do this" structure
- [ ] [output] `skills/frontend/TESTING.md` exists and contains "role" or "label" (query-by-role testing)
- [ ] [output] TESTING.md contains "userEvent" or "user event" or "fire events" (user interaction testing)
- [ ] [output] TESTING.md contains "prop plumbing" or "internal state" or "implementation" (what not to test)
- [ ] [output] TESTING.md contains "snapshot" (prohibiting snapshot tests for layout)
- [ ] [output] TESTING.md contains "/tdd" (hook logic boundary — tested in separate /tdd tasks)

## Task 5: /ui-system skill
<!-- Specs: ui-system/1, ui-system/2, ui-system/3, ui-system/4, ui-system/5 -->
- [ ] [output] `skills/ui-system/SKILL.md` exists and starts with YAML frontmatter containing `name: ui-system`
- [ ] [output] SKILL.md frontmatter contains `user-invokable: true`
- [ ] [output] SKILL.md frontmatter description contains "ShadCN" or "Radix" or "design system"
- [ ] [output] SKILL.md contains "search existing" or "existing components" (component selection)
- [ ] [output] SKILL.md contains "semantic" and "token" (semantic token discipline)
- [ ] [output] SKILL.md contains "raw" or "bg-blue" (prohibiting raw color values)
- [ ] [output] SKILL.md contains "cn()" (conditional classes utility)
- [ ] [output] SKILL.md contains "gap" (gap-* over space-x/y-*)
- [ ] [output] SKILL.md contains "Title" and "Dialog" or "Sheet" or "Drawer" (accessibility defaults)
- [ ] [output] SKILL.md contains "AvatarFallback" or "fallback" (Avatar accessibility)
- [ ] [output] SKILL.md contains "form" or "Form" or "FieldGroup" (form patterns)
- [ ] [output] SKILL.md contains "icon" (icon handling guidance)
- [ ] [output] SKILL.md contains "CLI" or "npx" or "shadcn" (CLI usage guidance)

## Task 6: /react-perf skill
<!-- Specs: react-perf/1, react-perf/2, react-perf/3, react-perf/4, react-perf/5 -->
- [ ] [output] `skills/react-perf/SKILL.md` exists and starts with YAML frontmatter containing `name: react-perf`
- [ ] [output] SKILL.md frontmatter contains `user-invokable: true`
- [ ] [output] SKILL.md frontmatter description contains "performance" or "perf"
- [ ] [output] SKILL.md contains "Promise.all" or "waterfall" (waterfall elimination)
- [ ] [output] SKILL.md contains "barrel" or "dynamic import" (bundle size)
- [ ] [output] SKILL.md contains "startTransition" or "useTransition" (re-render discipline)
- [ ] [output] SKILL.md contains "Suspense" (React 19 patterns)
- [ ] [output] SKILL.md contains "derive" or "derived" or "render" in context of state (derive during render)
- [ ] [output] SKILL.md contains "server" or "Server" (server-side patterns)
- [ ] [output] SKILL.md contains "content-visibility" or "visibility" (rendering patterns)
- [ ] [output] SKILL.md contains "Map" or "Set" or "flatMap" (JS performance patterns)

## Task 7: Template modifications, config, and mission-init
<!-- Specs: skill-assignment/1, skill-assignment/2, skill-assignment/3, skill-assignment/4, skill-assignment/5, skill-assignment/6, skill-assignment/7, skill-assignment/8, skill-assignment/9, skill-assignment/10 -->
- [ ] [output] `skills/mission/templates/worker-brief.md` contains "/frontend" (conditional methodology reference)
- [ ] [output] worker-brief.md contains "Skill:" (reads task topology field)
- [ ] [output] worker-brief.md contains "Constraints:" or "constraint" (appends constraint references)
- [ ] [output] `skills/mission/templates/design-template.md` contains "prop interface" or "semantic concepts" (frontend module guidance)
- [ ] [output] design-template.md contains "deep" and "shallow" (component classification guidance)
- [ ] [output] `skills/mission/SKILL.md` contains "browser_tools" (config flag)
- [ ] [output] skills/mission/SKILL.md contains "frontend/component" (layer vocabulary)
- [ ] [output] skills/mission/SKILL.md contains "frontend/hooks" and "/tdd" in proximity (hooks → /tdd mapping)
- [ ] [output] skills/mission/SKILL.md contains "/ui-system" and "/react-perf" and "Skill: /frontend" in proximity (constraint validation)
- [ ] [output] skills/mission/SKILL.md contains "chrome-devtools" and "dispatch" or "pre-flight" (dispatch pre-flight check)
- [ ] [output] skills/mission/SKILL.md contains "browser_tools: false" and "override" or "skip" (human override handling)
- [ ] [output] `skills/mission-init/SKILL.md` contains "chrome-devtools" or "browser_tools" (init availability check)
- [ ] [output] skills/mission-init/SKILL.md contains "install" or "missing" (fail-fast with instructions)

## Task 8: Smoke test
<!-- Specs: frontend/1, ui-system/1, react-perf/1, frontend-validation/2, skill-assignment/1 -->
- [ ] [execution] Run `./install.sh` — output contains "frontend" in the linked skills
- [ ] [execution] Run `./install.sh` — output contains "ui-system" in the linked skills
- [ ] [execution] Run `./install.sh` — output contains "react-perf" in the linked skills
- [ ] [execution] Run `ls -la ~/.claude/skills/frontend` — symlink exists and points to ~/.darkfactory/skills/frontend
- [ ] [execution] Run `ls -la ~/.claude/skills/ui-system` — symlink exists and points to ~/.darkfactory/skills/ui-system
- [ ] [execution] Run `ls -la ~/.claude/skills/react-perf` — symlink exists and points to ~/.darkfactory/skills/react-perf
- [ ] [execution] All markdown links in skills/frontend/*.md resolve to existing files
- [ ] [execution] Scrutiny validator gates are numbered 1-15 with no gaps or duplicates
- [ ] [execution] Run `grep -r "/shadcn" skills/ UBIQUITOUS_LANGUAGE.md README.md` — returns zero matches
- [ ] [execution] worker-brief.md does not contain hardcoded "Use `/tdd` to implement this task" as the only methodology option
- [ ] [execution] scrutiny-validator.md gate 1 contains "/frontend Test phase not followed"
- [ ] [execution] skills/mission/SKILL.md contains layer vocabulary mapping rules and constraint validation rules

## Task 9: Cleanup + module shape re-check
<!-- Specs: (closing task — structural integrity) -->
- [ ] [execution] No unused or orphaned files in skills/frontend/, skills/ui-system/, skills/react-perf/
- [ ] [output] `mission/validations/module-shape-audit.md` exists with findings for each design.md module

## Task 10: Documentation
<!-- Specs: frontend-validation/8 -->
- [ ] [output] README.md contains "frontend" skill with description
- [ ] [output] README.md contains "ui-system" skill with description
- [ ] [output] README.md contains "react-perf" skill with description
- [ ] [execution] UBIQUITOUS_LANGUAGE.md gate number references match the 1-15 sequence (no stale references to old numbering)
