# Scrutiny Validation: Task 7

**Commit:** 7b888ed  
**Task:** Template modifications, config, and mission-init  
**Files reviewed:** skills/mission/templates/worker-brief.md, skills/mission/templates/design-template.md, skills/mission/SKILL.md, skills/mission-init/SKILL.md

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | worker-brief.md contains "/frontend" (conditional methodology reference) | PASS | Line in Procedures: `If Skill: /frontend -- "Use /frontend to implement this task"` |
| 2 | worker-brief.md contains "Skill:" (reads task topology field) | PASS | Two occurrences: `If Skill: /frontend` and `If Skill: /tdd` in Procedures section |
| 3 | worker-brief.md contains "Constraints:" or "constraint" (appends constraint references) | PASS | `<For each Constraint in task topology, append:>` in Procedures section |
| 4 | design-template.md contains "prop interface" or "semantic concepts" (frontend module guidance) | PASS | `**Prop interface:** how many semantic concepts the caller must understand` in Frontend Module Guidance section (line 68) |
| 5 | design-template.md contains "deep" and "shallow" (component classification guidance) | PASS | Both present on line 69: `deep component (owns a workflow end-to-end) or allowed-shallow exception` |
| 6 | SKILL.md contains "browser_tools" (config flag) | PASS | Multiple occurrences: `browser_tools: true` (line 227), `browser_tools: false` (line 485, 561) |
| 7 | SKILL.md contains "frontend/component" (layer vocabulary) | PASS | Line 216 in task decomposition table: `frontend/component \| /frontend \| Visual React component work` |
| 8 | SKILL.md contains "frontend/hooks" and "/tdd" in proximity (hooks to /tdd mapping) | PASS | Same table row, line 219: `frontend/hooks \| /tdd \| Hooks have testable interfaces` |
| 9 | SKILL.md contains "/ui-system" and "/react-perf" and "Skill: /frontend" in proximity (constraint validation) | PASS | All three within lines 267-273 in the "Skill and Constraint Validation" section: `/ui-system` and `/react-perf` on line 267/273, `Skill: /frontend` on line 271/273 |
| 10 | SKILL.md contains "chrome-devtools" and "dispatch" or "pre-flight" (dispatch pre-flight check) | PASS | Section header line 567: `Browser Tools Pre-flight (Skill: /frontend tasks only)`. Lines 569-571: `chrome-devtools-mcp is available` and `Before dispatching a /frontend task` |
| 11 | SKILL.md contains "browser_tools: false" and "override" or "skip" (human override handling) | PASS | Line 561: `If browser_tools: false in config.yaml (human override), omit visual verification...skips the chrome-devtools-mcp visual verify step` |
| 12 | mission-init/SKILL.md contains "chrome-devtools" or "browser_tools" (init availability check) | PASS | Line 125: `browser_tools: true` and `chrome-devtools-mcp is registered`; line 133: `chrome-devtools-mcp - not registered` |
| 13 | mission-init/SKILL.md contains "install" or "missing" (fail-fast with instructions) | PASS | Line 135: `Fix: Install chrome-devtools-mcp from https://github.com/nichochar/chrome-devtools-mcp` |

## Overall Result

**PASS** -- all 13 contract checks satisfied.
