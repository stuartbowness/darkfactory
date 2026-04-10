## ADDED Requirements

### Requirement: Contract contains assertions grouped by task
The contract.md SHALL contain plain language assertions organized under headings that match the tasks in tasks.md.

#### Scenario: Assertions map to tasks
- **WHEN** the contract is written
- **THEN** each task in tasks.md has a corresponding section in contract.md with one or more assertions

### Requirement: Assertions are verifiable from outside the code
Each assertion SHALL describe observable behavior that a validator can check without knowing the implementation.

#### Scenario: Validator can check assertion
- **WHEN** a validator reads an assertion
- **THEN** it can determine pass or fail by running tests, hitting APIs, or driving a browser -- without reading the proposal or design

### Requirement: Contract is written before any workers start
The contract SHALL be created during the spec phase, after OpenSpec artifacts exist but before any implementation begins.

#### Scenario: Contract precedes implementation
- **WHEN** the orchestrator writes the contract
- **THEN** no worker has been spawned and no implementation commits exist

### Requirement: Workers never see the contract
The contract SHALL only be consumed by validators. Worker briefs SHALL NOT include contract content.

#### Scenario: Adversarial separation maintained
- **WHEN** a worker brief is assembled
- **THEN** it does not contain any content from contract.md
