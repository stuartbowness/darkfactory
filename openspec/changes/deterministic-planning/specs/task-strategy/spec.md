## ADDED Requirements

### Requirement: Task strategy approval before task generation
The orchestrator SHALL present a task decomposition strategy for human approval before generating tasks.md.

#### Scenario: Strategy presentation after design approval
- **WHEN** the human approves design.md in Phase 4.3
- **THEN** the orchestrator MUST present a task strategy in Phase 4.3a containing: decomposition approach (layered, vertical, or hybrid), target task count range with justification, and dependency ordering rationale

#### Scenario: Human approves strategy
- **WHEN** the human approves the task strategy
- **THEN** the orchestrator MUST record the approved strategy in the plan's Deterministic Anchors and proceed to generate tasks.md following that strategy

#### Scenario: Human rejects strategy
- **WHEN** the human requests changes to the task strategy
- **THEN** the orchestrator MUST revise the strategy and re-present it before generating tasks.md

### Requirement: Task quality standard is strategy-aware
Phase 4.4's task quality standard SHALL follow the approved strategy from Phase 4.3a rather than unconditionally requiring vertical slices.

#### Scenario: Vertical strategy approved
- **WHEN** the approved strategy is vertical
- **THEN** each task MUST cross all layers and produce a concrete, testable output

#### Scenario: Layered strategy approved
- **WHEN** the approved strategy is layered
- **THEN** tasks MAY be scoped to a single layer, provided each task still has a clear done state and a test surface

#### Scenario: Hybrid strategy approved
- **WHEN** the approved strategy is hybrid
- **THEN** foundation tasks MAY be scoped to a single layer, and subsequent tasks MAY cross all layers, provided each task still has a clear done state and a test surface
