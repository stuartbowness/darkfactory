# acceptance-field Specification

## Purpose
TBD - created by archiving change contract-renderer. Update Purpose after archive.
## Requirements
### Requirement: Scenario carries acceptance entries

A spec scenario MAY carry an `acceptance` field. When present, `acceptance` SHALL be a list of one or more entries; an omitted `acceptance` field is valid and contributes no contract check. Each entry SHALL hold a non-empty check statement (carried verbatim into the contract), a verification-method tag, and a `task` link. Each entry inherits the scenario's Spec Reference; an entry MUST NOT carry a Spec Reference different from its scenario. An empty or whitespace-only check statement, or a present-but-empty `acceptance` list, MUST fail loudly.

#### Scenario: Scenario declares one acceptance entry
- **WHEN** a spec scenario declares a single `acceptance` entry
- **THEN** the entry exposes its check statement, verification-method tag, and `task` link
- **AND** the entry's Spec Reference is the scenario's Spec Reference

#### Scenario: Scenario declares multiple acceptance entries
- **WHEN** a spec scenario declares more than one `acceptance` entry
- **THEN** each entry is independent in its statement, verification-method tag, and `task` link
- **AND** every entry shares the scenario's Spec Reference

#### Scenario: Scenario omits acceptance
- **WHEN** a spec scenario declares no `acceptance` field
- **THEN** the scenario is valid
- **AND** it contributes no check to the rendered contract

#### Scenario: Present acceptance list is empty
- **WHEN** a scenario declares an `acceptance` field with no entries
- **THEN** it fails loudly

#### Scenario: Check statement is empty
- **WHEN** an acceptance entry has an empty or whitespace-only check statement
- **THEN** it fails loudly, naming the entry

### Requirement: Acceptance entry declares a verification method

Each acceptance entry SHALL declare its verification method as exactly one of `[execution]`, `[output]`, or `[inspection]`. Any other value MUST fail loudly.

#### Scenario: Entry declares a recognised method
- **WHEN** an acceptance entry declares its method as `[execution]`, `[output]`, or `[inspection]`
- **THEN** the entry is accepted

#### Scenario: Entry declares an unrecognised method
- **WHEN** an acceptance entry declares a method outside that set
- **THEN** parsing fails loudly, naming the entry and the offending value

### Requirement: Acceptance entry links to a task

Each acceptance entry SHALL name the task that makes its check first verifiable. The link is a plain task reference; resolution against the task list is enforced at render time.

#### Scenario: Entry names a task
- **WHEN** an acceptance entry is declared
- **THEN** it names exactly one task as its `task` link

### Requirement: Scenario has a stable Spec Reference

Each scenario SHALL have a Spec Reference of the form `capability/N` whose identity is stable. Inserting a new scenario into a capability MUST NOT renumber the Spec References of existing scenarios. Two scenarios in one capability MUST NOT share a Spec Reference.

#### Scenario: Insertion does not renumber siblings
- **WHEN** a new scenario is inserted into a capability that already has scenarios
- **THEN** the Spec References of the existing scenarios are unchanged

#### Scenario: Duplicate Spec Reference is rejected
- **WHEN** two scenarios within one capability carry the same Spec Reference
- **THEN** it is reported as a loud error naming the duplicated reference

### Requirement: Legacy position-numbered references are migratable

Existing contracts that reference scenarios by file position SHALL have a migration path to stable Spec References, so archived changes remain traceable.

#### Scenario: Migrate a position-numbered capability
- **WHEN** a capability whose scenarios were referenced by file position is migrated
- **THEN** each scenario receives a stable Spec Reference
- **AND** references in the associated contract resolve to the same scenarios as before

### Requirement: Scenario carries a status

A scenario MAY carry a `status` that is one of `proposed`, `approved`, `built`, `verified`, or `deviated`. An omitted status is treated as non-deviated. An unrecognized status value MUST fail loudly. Stage 1 acts only on `deviated`, used to mark the scenario's Spec Reference in the rendered contract; the full deviation lifecycle is out of scope.

#### Scenario: Deviated status is read for the marker
- **WHEN** a scenario's `status` is `deviated`
- **THEN** the status is exposed to the renderer for marking its Spec Reference

#### Scenario: Status omitted
- **WHEN** a scenario declares no `status`
- **THEN** it is treated as non-deviated and its Spec Reference is not marked

#### Scenario: Unrecognized status
- **WHEN** a scenario declares a `status` outside the known set
- **THEN** it fails loudly, naming the scenario and the offending value

