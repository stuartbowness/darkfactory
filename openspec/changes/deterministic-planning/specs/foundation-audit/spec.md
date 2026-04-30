## ADDED Requirements

### Requirement: Foundation audit after task 1
The orchestrator SHALL run a foundation audit after task 1 passes both validators.

#### Scenario: Task 1 passes validation
- **WHEN** task 1 passes both scrutiny and user-test validation
- **THEN** the orchestrator MUST run a foundation audit before proceeding to task 2

#### Scenario: Audit does not run for other tasks
- **WHEN** any task other than task 1 passes validation
- **THEN** the foundation audit MUST NOT run

### Requirement: Audit scoped to task 1's declared produces
The foundation audit SHALL check only what task 1 declared it would produce, not the full design. The audit reads module names and type names from task 1's Produces topology field, then checks those against the design.

#### Scenario: Module structure check
- **WHEN** the foundation audit runs
- **THEN** it MUST extract module names from task 1's Produces field, look up those modules in design.md's Module Inventory, and verify that corresponding directories or files exist in the committed code with matching names

#### Scenario: Type name check
- **WHEN** the foundation audit runs
- **THEN** it MUST extract type names from task 1's Produces field, look up those types in design.md's Type Catalog, and verify that corresponding type definitions exist in the committed code with matching names

### Requirement: Drift triggers corrective task
When the foundation audit detects drift, the orchestrator SHALL generate a corrective task before proceeding.

#### Scenario: Drift detected
- **WHEN** the foundation audit finds that module structure or type names do not match the design
- **THEN** the orchestrator MUST generate a corrective task 1a, insert it in tasks.md between task 1 and task 2, add assertions for it to contract.md, and present both to the human for approval before continuing

#### Scenario: No drift detected
- **WHEN** the foundation audit finds no discrepancies
- **THEN** the orchestrator MUST log "Foundation audit PASSED" to progress.md and proceed to task 2
