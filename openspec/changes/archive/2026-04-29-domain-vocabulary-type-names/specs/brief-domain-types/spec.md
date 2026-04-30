## ADDED Requirements

### Requirement: Worker brief carries exact Type Catalog names
The worker brief template SHALL include a "Domain Types" section listing Type Catalog entries relevant to the task. The section MUST instruct the worker to use these exact names for domain types.

#### Scenario: Brief for a task that produces domain types
- **WHEN** the orchestrator assembles a brief for a task whose Produces field references domain types listed in the Type Catalog
- **THEN** the brief includes a "Domain Types" section with the exact type names from the Type Catalog and a one-line instruction to use them

#### Scenario: Brief for a task that consumes but does not produce domain types
- **WHEN** the orchestrator assembles a brief for a task whose Consumes field references domain types from the Type Catalog but whose Produces field does not
- **THEN** the brief includes a "Domain Types" section listing the consumed type names from the Type Catalog with the same instruction to use exact names

#### Scenario: Brief for a task with no domain types
- **WHEN** the orchestrator assembles a brief for a task whose Produces and Consumes fields reference no domain types from the Type Catalog
- **THEN** the brief includes the "Domain Types" section with the text "None for this task"

#### Scenario: Domain Types section is separate from Key Terms
- **WHEN** the worker brief is assembled
- **THEN** the "Domain Types" section is a separate section from the existing "Key Terms" section, because they serve different purposes: Key Terms defines vocabulary, Domain Types specifies code names
