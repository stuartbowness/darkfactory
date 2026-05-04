## ADDED Requirements

### Requirement: Task topology includes Skill and Constraints fields
Task topology metadata in tasks.md SHALL include `Skill:` and `Constraints:` fields for TypeScript React projects.

#### Scenario: Topology template format
- **WHEN** a task is defined in tasks.md for a TypeScript React project
- **THEN** the topology SHALL follow this format:
  ```
  - [ ] Task N: <name>
    Depends on: <prior task numbers, or "none">
    Skill: </tdd or /frontend>
    Constraints: </ui-system, /react-perf, or "none">
    Consumes: <surfaces from prior tasks>
    Produces: <surfaces this task creates>
    Layers: <specific layer value>
  ```

#### Scenario: Field placement
- **WHEN** topology metadata is written
- **THEN** `Skill:` and `Constraints:` SHALL appear after `Depends on:` and before `Consumes:`

#### Scenario: Non-TypeScript projects
- **WHEN** a project is Rust, Python, or any non-TypeScript language
- **THEN** tasks SHALL NOT include `Skill:` or `Constraints:` fields — topology remains unchanged

### Requirement: Exactly one Methodology per task
Each task SHALL have exactly one `Skill:` value — either `/tdd` or `/frontend`. The `Skill:` field carries the task's Methodology as defined in UBIQUITOUS_LANGUAGE.md.

#### Scenario: Valid methodology
- **WHEN** a task has `Skill: /frontend` or `Skill: /tdd`
- **THEN** the topology is valid and the `Skill:` value is the task's Methodology

#### Scenario: Missing methodology
- **WHEN** a task in a TypeScript React project has no `Skill:` field
- **THEN** the orchestrator SHALL flag this as invalid during Phase 4.4 review

#### Scenario: Multiple methodologies
- **WHEN** a task has both `/tdd` and `/frontend` in the `Skill:` field
- **THEN** the orchestrator SHALL flag this as invalid — methodologies are mutually exclusive

### Requirement: Constraints are additive and validated
Tasks SHALL have zero or more `Constraints:` values. Constraint validity depends on the `Skill:` value.

#### Scenario: Valid constraint with /frontend
- **WHEN** a task has `Skill: /frontend` and `Constraints: /ui-system, /react-perf`
- **THEN** the topology is valid

#### Scenario: No constraints
- **WHEN** a task has `Constraints: none`
- **THEN** the topology is valid regardless of `Skill:` value

#### Scenario: Invalid constraint with /tdd
- **WHEN** a task has `Skill: /tdd` and `Constraints: /ui-system`
- **THEN** the orchestrator SHALL flag this as invalid — `/ui-system` is only valid with `Skill: /frontend`

### Requirement: Layer vocabulary determines Skill assignment
The orchestrator SHALL assign `Skill:` values mechanically based on the `Layers:` field during task decomposition.

#### Scenario: frontend/component layer
- **WHEN** a task has `Layers: frontend/component`
- **THEN** the orchestrator SHALL assign `Skill: /frontend`

#### Scenario: frontend/page layer
- **WHEN** a task has `Layers: frontend/page`
- **THEN** the orchestrator SHALL assign `Skill: /frontend`

#### Scenario: frontend/layout layer
- **WHEN** a task has `Layers: frontend/layout`
- **THEN** the orchestrator SHALL assign `Skill: /frontend`

#### Scenario: frontend/hooks layer
- **WHEN** a task has `Layers: frontend/hooks`
- **THEN** the orchestrator SHALL assign `Skill: /tdd`

#### Scenario: backend layer
- **WHEN** a task has `Layers: backend`
- **THEN** the orchestrator SHALL assign `Skill: /tdd`

#### Scenario: shared layers
- **WHEN** a task has `Layers: shared/types`, `shared/schema`, or `shared/utils`
- **THEN** the orchestrator SHALL assign `Skill: /tdd`

#### Scenario: Mechanical rule
- **WHEN** the orchestrator assigns a `Skill:` value
- **THEN** the rule SHALL be: `/frontend` only when layer starts with `frontend/` AND is not `frontend/hooks`; everything else gets `/tdd`

### Requirement: Task decomposition splits complex frontend features
The orchestrator SHALL split frontend features with complex logic into separate tasks with different methodologies.

#### Scenario: Feature with validation logic
- **WHEN** a frontend feature requires validation, state machines, or data transformation logic
- **THEN** the orchestrator SHALL create one `/tdd` task for the logic as a hook/utility and one `/frontend` task for the component that consumes it

#### Scenario: Task ordering
- **WHEN** a `/frontend` task consumes a hook from a `/tdd` task
- **THEN** the `/tdd` task SHALL appear first in tasks.md with the `/frontend` task listing it in `Depends on:`

### Requirement: Worker brief references the correct methodology
The worker brief template SHALL conditionally reference the task's assigned `Skill:` value in the Procedures section.

#### Scenario: /frontend task brief
- **WHEN** a task has `Skill: /frontend`
- **THEN** the worker brief Procedures section SHALL say "Use `/frontend` to implement this task"

#### Scenario: /tdd task brief
- **WHEN** a task has `Skill: /tdd`
- **THEN** the worker brief Procedures section SHALL say "Use `/tdd` to implement this task"

#### Scenario: Constraints in brief
- **WHEN** a task has `Constraints: /ui-system, /react-perf`
- **THEN** the worker brief SHALL append constraint references after the methodology reference

### Requirement: browser_tools config flag
Mission config SHALL include a `browser_tools` flag that controls chrome-devtools-mcp availability.

#### Scenario: Auto-set to true
- **WHEN** any task in the mission has `Skill: /frontend`
- **THEN** the orchestrator SHALL set `browser_tools: true` in config.yaml during Phase 4.3a

#### Scenario: Auto-set to false
- **WHEN** no task in the mission has `Skill: /frontend`
- **THEN** the orchestrator SHALL set `browser_tools: false` in config.yaml

#### Scenario: Hard check at mission-init
- **WHEN** `browser_tools: true` and chrome-devtools-mcp is not available
- **THEN** `/mission-init` SHALL fail fast with a clear error message

#### Scenario: Soft pre-flight at dispatch
- **WHEN** a `/frontend` task is about to be dispatched and chrome-devtools-mcp is unavailable
- **THEN** the orchestrator SHALL block dispatch, log to progress.md, and surface options to the human: retry, skip visual verify for this task, or abort mission

#### Scenario: Human override
- **WHEN** the human manually sets `browser_tools: false` in config.yaml for a mission with `/frontend` tasks
- **THEN** the orchestrator SHALL respect the override and skip visual verification procedures in worker briefs

### Requirement: Design template includes frontend module guidance
The design template's Module Inventory section SHALL include guidance for describing frontend modules.

#### Scenario: Frontend module description
- **WHEN** a design.md Module Inventory entry describes a React component module
- **THEN** the template SHALL guide the author to describe: prop interface (semantic concepts), state/effects encapsulated, and whether it is deep or allowed-shallow with classification reason
