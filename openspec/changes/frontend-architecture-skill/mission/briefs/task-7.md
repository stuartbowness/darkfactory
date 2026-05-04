# Worker Brief: Task 7 - Template modifications, config, and mission-init

## Your Task

Modify 4 existing files to integrate the new `/frontend`, `/ui-system`, and `/react-perf` skills into the mission infrastructure.

## Change 1: Worker Brief Template

**File:** `skills/mission/templates/worker-brief.md`
**Line 53** currently reads:
```
- Use `/tdd` to implement this task
```

Replace with conditional methodology + constraints:
```
- Use `<Skill from task topology>` to implement this task
  - If Skill: /frontend — "Use `/frontend` to implement this task"
  - If Skill: /tdd — "Use `/tdd` to implement this task"
- <For each Constraint in task topology, append:>
  - "Also follow `/ui-system` for component selection and token discipline"
  - "Also follow `/react-perf` for performance optimization patterns"
```

The orchestrator reads the task's `Skill:` and `Constraints:` fields from topology metadata and populates this section. The template should show the conditional pattern so the orchestrator knows what to insert.

## Change 2: Design Template

**File:** `skills/mission/templates/design-template.md`
**Location:** Module Inventory section (around line 56-62)

After the existing Module Inventory table format, add a paragraph for frontend modules:

```markdown
### Frontend Module Guidance

For React frontend modules, describe:
- **Prop interface:** how many semantic concepts the caller must understand (count props + context + URL params + cache dependencies, not raw prop count)
- **State/effects encapsulated:** what state management and effects the component hides behind its interface
- **Classification:** deep component (owns a workflow end-to-end) or allowed-shallow exception (design system leaf, route/layout glue, adapter, headless behavior primitive) with the reason for the classification

Reference `/frontend` skill's DEEP-COMPONENTS.md for the four deep-component rules and allowed-shallow categories.
```

## Change 3: Mission Config Template

**File:** `skills/mission/SKILL.md`
**Location:** Phase 6 config.yaml template (search for the `config.yaml` section that defines the engines)

Add `browser_tools: false` to the config.yaml template, and add a comment explaining it:

```yaml
engines:
  worker: codex
  scrutiny_validator: claude
  usertest_validator: claude
  adversarial_reviewer_a: codex
  adversarial_reviewer_b: claude
  demo_video: claude

browser_tools: false
```

Note: The orchestrator auto-sets this to `true` during Phase 4.3a when any task has `Skill: /frontend`.

## Change 4: Orchestrator Behaviors in Mission SKILL.md

**File:** `skills/mission/SKILL.md`

Add the following orchestrator behaviors. Find the appropriate locations in the existing phases:

### In Phase 4.3a (Task Strategy Approval section):

Add after the existing decomposition approach guidance:

```markdown
#### Frontend Task Decomposition (TypeScript React projects only)

For TypeScript React projects, use the specific task layer vocabulary:

| Layer | Skill | Rationale |
|---|---|---|
| `frontend/component` | `/frontend` | Visual React component work |
| `frontend/page` | `/frontend` | Page composition |
| `frontend/layout` | `/frontend` | Visual structure |
| `frontend/hooks` | `/tdd` | Hooks have testable interfaces |
| `backend` | `/tdd` | APIs, server logic |
| `shared/types` | `/tdd` | Type contracts |
| `shared/schema` | `/tdd` | Zod validation, pure logic |
| `shared/utils` | `/tdd` | Pure utility functions |

**Mechanical rule:** `/frontend` only when the layer starts with `frontend/` AND is not `frontend/hooks`. Everything else gets `/tdd`.

Auto-set `browser_tools: true` in config.yaml when any task has `Skill: /frontend`.

When a frontend feature contains complex logic (validation, state machines, data transformation), split it: one `/tdd` task for the logic as a hook/utility, one `/frontend` task for the component that consumes it. The `/tdd` task comes first in the task list.
```

### In Phase 4.4 (Generate and Review Tasks section):

Add after the existing Task Topology format:

```markdown
#### Skill and Constraint Validation (TypeScript React projects only)

Task topology for TypeScript React projects includes two additional fields:

```
Skill: </tdd or /frontend>
Constraints: </ui-system, /react-perf, or "none">
```

Validation rules:
- Exactly one `Skill:` value per task — either `/tdd` or `/frontend`
- Zero or more `Constraints:` values
- `/ui-system` and `/react-perf` are only valid with `Skill: /frontend`
- Flag invalid combinations during Phase 4.4 human review
```

### In Step 2 (Dispatch Worker section):

Add a pre-flight check:

```markdown
#### Browser Tools Pre-flight (Skill: /frontend tasks only)

Before dispatching a `/frontend` task, check that chrome-devtools-mcp is available:
- If available: proceed with dispatch, include visual verification in the brief
- If unavailable: block dispatch. Log to progress.md: `<timestamp> -- Task N: chrome-devtools-mcp unavailable at dispatch. Pausing.` Surface to human with options: retry (check again), skip visual verify for this task, or abort mission
```

### In Step 1 (Assemble Brief section):

Add:

```markdown
#### Browser Tools Override

If `browser_tools: false` in config.yaml (human override), omit visual verification procedures from the worker brief even for `/frontend` tasks. The worker follows the `/frontend` workflow but skips the chrome-devtools-mcp visual verify step in the Build phase.
```

## Change 5: Mission-Init Update

**File:** `skills/mission-init/SKILL.md`

Add a new check in Step 3 (Verify CLI Dependencies) or as a new step:

```markdown
### Browser Tools Check (conditional)

If `browser_tools: true` in the mission's config.yaml (or if any task in the current mission has `Skill: /frontend`), check that chrome-devtools-mcp is registered:

```bash
# Check if chrome-devtools MCP tools are available
# The exact check depends on the MCP registration mechanism
```

If chrome-devtools-mcp is not registered:
- Report: `[!] chrome-devtools-mcp - not registered (required when browser_tools: true)`
- Fix: Install chrome-devtools-mcp from https://github.com/nichochar/chrome-devtools-mcp and add it to MCP configuration
- This blocks mission launch for missions with `/frontend` tasks

If `browser_tools: false` or no `/frontend` tasks exist, skip this check.
```

## Project Context

These changes wire the new frontend skills into the mission orchestrator. After this task, the orchestrator knows how to assign `/frontend` vs `/tdd`, validate constraints, check browser tools, and assemble the right worker briefs.

## Architecture

All changes are to existing markdown files in `skills/mission/` and `skills/mission-init/`. No new files.

## Key Terms

- **Methodology**: /tdd or /frontend, mutually exclusive per task
- **Constraint**: /ui-system or /react-perf, additive, only with /frontend
- **Task layer**: Vocabulary value in topology that determines methodology
- **Browser tools**: Config flag for chrome-devtools-mcp availability
- **Visual verification**: chrome-devtools-mcp usage during /frontend Build phase

## What Previous Tasks Built

### Tasks 1-4: /frontend skill (complete)
- skills/frontend/: SKILL.md, DEEP-COMPONENTS.md, STACK.md, PATTERNS.md, TESTING.md

### Task 2: Scrutiny validator
- Gate 7 (Frontend depth review) inserted, gates renumbered 1-15, gate 1 methodology-aware

### Tasks 5-6: Supplementary skills
- skills/ui-system/SKILL.md: ShadCN/Radix component guidelines
- skills/react-perf/SKILL.md: Vercel React performance best practices

## Downstream Consumers

- Task 8 (smoke test) verifies all these changes
- Every future mission uses the updated templates and orchestrator behaviors

## Procedures

- Use `/tdd` to implement this task
- Commit all changes in a single commit with message: "[task-7] Wire frontend skills into mission infrastructure"
- Write your handoff report to openspec/changes/frontend-architecture-skill/mission/handoffs/task-7.md

## Handoff Report Format

- Task: Task 7 - Template modifications, config, and mission-init
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything uncertain>
