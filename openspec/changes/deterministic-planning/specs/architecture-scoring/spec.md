## ADDED Requirements

### Requirement: Architecture candidate evaluation
The orchestrator SHALL evaluate 2-3 architecture candidates during Phase 4.3 for greenfield missions or missions with explicit architecture changes.

#### Scenario: Greenfield mission reaches design phase
- **WHEN** the mission is greenfield (no existing codebase) and Phase 4.3 begins
- **THEN** the orchestrator MUST present 2-3 architecture candidates with a scoring table before writing design.md

#### Scenario: Existing codebase without architecture change
- **WHEN** the mission modifies an existing codebase and the PRD does not request an architecture change
- **THEN** the orchestrator MUST document the current architecture and explain how planned changes fit within it, without scoring candidates

#### Scenario: Explicit architecture change requested
- **WHEN** the PRD explicitly requests an architecture change (e.g., "migrate from monolith to workspace")
- **THEN** the orchestrator MUST evaluate 2-3 candidates including the current architecture as one option

### Requirement: Four-dimension scoring rubric
Each architecture candidate SHALL be scored on four dimensions using a 0-3 scale with concrete per-level criteria.

#### Scenario: Separation of concerns scoring
- **WHEN** scoring a candidate on separation of concerns
- **THEN** the orchestrator MUST assign: 3 if each responsibility maps to exactly one unit with no mixing of domain/transport/storage/config; 2 if mostly isolated but 1-2 units mix concerns pragmatically; 1 if several units mix concerns; 0 if no clear separation

#### Scenario: Testability scoring
- **WHEN** scoring a candidate on testability
- **THEN** the orchestrator MUST assign: 3 if each module testable in isolation with swappable dependencies; 2 if most modules testable but 1-2 require real external dependencies; 1 if tests require significant setup or multiple services; 0 if no practical way to test in isolation

#### Scenario: Task independence scoring
- **WHEN** scoring a candidate on task independence
- **THEN** the orchestrator MUST assign: 3 if separately buildable or deployable units where a task can modify one without touching others; 2 if explicit interface boundaries within a single build unit with enforced contracts; 1 if directory/namespace separation only with no enforcement; 0 if flat structure with no meaningful boundaries

#### Scenario: Module depth scoring
- **WHEN** scoring a candidate on module depth
- **THEN** the orchestrator MUST assign: 3 if modules expose small public API relative to implementation; 2 if reasonable ratio but 1-2 modules expose more surface than they hide; 1 if several modules are thin wrappers; 0 if no meaningful abstraction

### Requirement: Hard floor enforcement
No recommended architecture candidate SHALL score 0 on any dimension.

#### Scenario: Candidate scores zero on a dimension
- **WHEN** a candidate scores 0 on any of the four dimensions
- **THEN** the orchestrator MUST flag the candidate and recommend against it, explaining which dimension failed and why

### Requirement: Tie escalation to human
When architecture candidates tie on total score, the orchestrator SHALL present both to the human for discussion and selection.

#### Scenario: Two candidates tie on total score
- **WHEN** two or more candidates have the same total score (sum of all four dimensions)
- **THEN** the orchestrator MUST present the tied candidates with their tradeoffs and let the human choose, rather than applying an arbitrary tie-breaker
