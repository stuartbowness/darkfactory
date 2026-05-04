## Why

Darkfactory's skills and validation pipeline assume backend code. Workers building React/TypeScript frontend tasks get `/tdd` (red-green-refactor), which doesn't fit component development — the output is visual and emergent, not driven by test-first interfaces. There is no prescriptive guidance on component architecture, no design system discipline, no performance patterns, and no visual verification during implementation.

## What Changes

- New `/frontend` skill: a prescriptive, worker-facing builder guide for React component development. Introduces a "deep components" thesis (four architectural rules), a design-implement-test workflow, and visual verification via chrome-devtools-mcp during the Build phase.
- New `/ui-system` skill: ShadCN/Radix component selection, token discipline, and composition patterns. Verbatim upstream content.
- New `/react-perf` skill: React performance patterns from Vercel best practices. Verbatim upstream content.
- New scrutiny validator gate 7 (frontend depth): self-skipping gate after module depth (gate 6) that checks React component quality against the four deep-component rules. Existing gates 7-14 renumber to 8-15.
- New task topology fields: `Skill:` (methodology, mutually exclusive) and `Constraints:` (additive design guidance) per task. Assigned during task decomposition based on layer vocabulary.
- New mission config flag: `browser_tools: true/false`, auto-set when any task has `Skill: /frontend`.
- Modified worker brief template: Procedures section becomes conditional based on `Skill:` field.
- Modified design template: frontend module patterns added to Module Inventory guidance.
- Modified gate 1 (test presence): rationale updated for `/frontend` tasks.

## Capabilities

### New Capabilities
- `frontend`: Core `/frontend` builder guide — workflow, deep-component rules, stack reference, component patterns, testing guidance, self-check audit
- `ui-system`: `/ui-system` supplementary skill — verbatim upstream ShadCN/Radix guidelines for component selection, token discipline, composition
- `react-perf`: `/react-perf` supplementary skill — verbatim upstream Vercel React best practices
- `frontend-validation`: Scrutiny validator gate 7 (frontend depth) — self-skipping gate for React component quality
- `skill-assignment`: Two-axis skill model — task topology `Skill:`/`Constraints:` fields, layer vocabulary, orchestrator detection rules, worker brief conditioning

### Modified Capabilities
(none — no existing spec-level behavior changes)

## Impact

- **Skills directory:** 3 new directories (`frontend/`, `ui-system/`, `react-perf/`) auto-discovered by install.sh
- **Scrutiny validator:** Gate renumbering (7→8 through 14→15), new gate 7 inserted. All UBIQUITOUS_LANGUAGE.md gate number references must update.
- **Worker brief template:** Procedures section conditional on `Skill:` field
- **Design template:** Module Inventory section extended with frontend guidance
- **Mission config template:** New `browser_tools` flag
- **Task topology format:** Two new fields (`Skill:`, `Constraints:`)
- **Gate 1:** Rationale text changes for `/frontend` tasks
- **Tool dependency:** chrome-devtools-mcp required when `browser_tools: true`
- **Non-TypeScript missions:** Zero impact — no new fields, no new gates fire, unchanged behavior
