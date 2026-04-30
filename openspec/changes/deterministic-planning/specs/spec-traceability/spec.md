## ADDED Requirements

### Requirement: Contract assertions reference spec identifiers
Contract assertions SHALL reference spec capability/scenario identifiers instead of arbitrary row numbers.

#### Scenario: Contract task section includes spec references
- **WHEN** the orchestrator writes a task section in contract.md
- **THEN** the comment line MUST use the format `<!-- Specs: capability/N, capability/M -->` where capability is the spec folder name and N is the scenario number within that spec

#### Scenario: Phase 5 instructions reference spec IDs
- **WHEN** the orchestrator follows Phase 5 instructions to write the contract
- **THEN** the instructions MUST direct the orchestrator to trace each assertion back to a specific spec capability and scenario number
