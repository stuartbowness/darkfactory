## ADDED Requirements

### Requirement: Per-task topology metadata
Each task in tasks.md SHALL include topology metadata declaring its dependencies and surfaces, using the fixed markdown format defined in the design (Depends on, Consumes, Produces, Layers).

#### Scenario: Task includes topology fields
- **WHEN** a task is written in tasks.md
- **THEN** it MUST include four fields: Depends on (prior task numbers), Consumes (surfaces from prior tasks it uses -- types, interfaces, endpoints), Produces (surfaces it creates for downstream tasks -- types, modules, interfaces, endpoints), and Layers (logical architectural layers this task touches, e.g. domain, transport, storage)

#### Scenario: Layers are logical groupings
- **WHEN** the orchestrator assigns layers to a task
- **THEN** layers MUST be logical groupings of modules from the design narrative (e.g., domain, transport, storage, configuration), not columns from the Module Inventory table

### Requirement: Architectural layer flagging
Tasks touching more than 2 architectural layers SHALL be flagged for human review when the approved strategy is layered or hybrid.

#### Scenario: Layered or hybrid strategy and task touches 3 or more layers
- **WHEN** the approved task strategy (Phase 4.3a) is layered or hybrid AND a task's Layers field lists more than 2 architectural layers
- **THEN** the orchestrator MUST flag the task and present it to the human for review with a recommendation to split or approve as-is

#### Scenario: Vertical strategy
- **WHEN** the approved task strategy (Phase 4.3a) is vertical
- **THEN** the layer count flag MUST NOT apply, since vertical tasks are expected to cross all layers

#### Scenario: Layered or hybrid strategy and task touches 2 or fewer layers
- **WHEN** the approved task strategy is layered or hybrid AND a task's Layers field lists 2 or fewer architectural layers
- **THEN** the task proceeds without flagging
