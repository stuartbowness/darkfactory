# Spec Coverage Audit

## Coverage Table
| # | PRD Requirement (User Story) | Spec Requirement | Task(s) | Status |
|---|------------------------------|-----------------|---------|--------|
| 1 | US1: Assign /frontend or /tdd per task based on layer | skill-assignment: Layer vocabulary determines Skill assignment (7 scenarios) | Task 7 | COVERED |
| 2 | US2: Prescriptive workflow (Scan → Contract → Build → Test) | frontend: SKILL.md defines the design-implement-test workflow (7 scenarios) | Task 3 | COVERED |
| 3 | US3: Visual verify via chrome-devtools-mcp during Build | frontend: Build phase with visual verification; skill-assignment: browser_tools config flag | Tasks 3, 7 | COVERED |
| 4 | US4: Per-component contract template | frontend: Contract phase scenario | Task 3 | COVERED |
| 5 | US5: Deep-component audit checklist before commit | frontend: Deep-component audit before commit scenario | Task 3 | COVERED |
| 6 | US6: Self-skipping gate 7 (frontend depth) | frontend-validation: Gate 7 exists, self-skips, checks four rules, recognizes exceptions, produces verdict (13 scenarios) | Task 2 | COVERED |
| 7 | US7: /ui-system guidance on component selection and tokens | ui-system: skill exists, includes verbatim ShadCN content (4 scenarios); valid/invalid constraint pairing (2 scenarios) | Tasks 5, 7 | COVERED |
| 8 | US8: /react-perf guidance on waterfalls, bundle, re-renders | react-perf: skill exists, includes verbatim Vercel content (4 scenarios); valid/invalid constraint pairing (2 scenarios) | Tasks 6, 7 | COVERED |
| 9 | US9: browser_tools config flag auto-sets | skill-assignment: browser_tools config flag (5 scenarios) | Task 7 | COVERED |
| 10 | US10: Specific layer vocabulary for skill assignment | skill-assignment: Layer vocabulary determines Skill assignment (7 scenarios) | Task 7 | COVERED |
| 11 | US11: Task topology Skill: and Constraints: fields | skill-assignment: Task topology includes fields, exactly one Methodology, constraints validated (10 scenarios) | Task 7 | COVERED |
| 12 | US12: Developer can invoke /frontend directly | frontend: skill exists with user-invokable: true; ui-system: independent invocation; react-perf: independent invocation | Tasks 3, 5, 6 | COVERED |
| 13 | US13: Split complex frontend features (hook + component) | skill-assignment: Task decomposition splits complex features (2 scenarios) | Task 7 | COVERED |
| 14 | US14: Soft pre-flight check for chrome-devtools-mcp at dispatch | skill-assignment: Soft pre-flight at dispatch scenario | Task 7 | COVERED |
| 15 | US15: TESTING.md for behavioral component testing | frontend: TESTING.md defines frontend behavioral testing (3 scenarios) | Task 4 | COVERED |
| 16 | US16: Non-TypeScript missions unchanged | skill-assignment: Non-TypeScript projects scenario; frontend-validation: No React files self-skip | Tasks 2, 7 | COVERED |
| 17 | ID: Gate renumbering (7→8 through 14→15) | frontend-validation: Gate numbering sequence scenario | Task 2 | COVERED |
| 18 | ID: Gate 1 rationale update for /frontend tasks | frontend-validation: Gate 1 rationale updated (2 scenarios) | Task 2 | COVERED |
| 19 | ID: DEEP-COMPONENTS.md four rules + exceptions | frontend: DEEP-COMPONENTS.md defines four rules and exceptions (3 scenarios) | Task 1 | COVERED |
| 20 | ID: STACK.md technology reference | frontend: STACK.md defines technology reference (3 scenarios) | Task 1 | COVERED |
| 21 | ID: PATTERNS.md concrete code examples | frontend: PATTERNS.md provides concrete code examples (4 scenarios) | Task 4 | COVERED |
| 22 | ID: Worker brief template becomes conditional | skill-assignment: Worker brief references correct methodology (3 scenarios) | Task 7 | COVERED |
| 23 | ID: Design template frontend module guidance | skill-assignment: Design template includes frontend module guidance (1 scenario) | Task 7 | COVERED |
| 24 | ID: Supplementary skills verbatim upstream content | ui-system: verbatim ShadCN; react-perf: verbatim Vercel | Tasks 5, 6 | COVERED |
| 25 | ID: install.sh unchanged (auto-discovers all 3 skills) | frontend: Skill registration; ui-system: Skill registration; react-perf: Skill registration | Tasks 5, 6, 8 (smoke test) | COVERED |
| 26 | ID: UBIQUITOUS_LANGUAGE.md gate refs updated | frontend-validation: Gate references updated scenario | Task 10 | COVERED |

## Gaps
(none)

## Deviations Requiring Approval
(none)
