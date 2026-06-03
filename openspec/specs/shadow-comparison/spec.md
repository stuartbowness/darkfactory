# shadow-comparison Specification

## Purpose
TBD - created by archiving change contract-renderer. Update Purpose after archive.
## Requirements
### Requirement: Shadow mode renders without replacing

The renderer SHALL provide a shadow mode that renders a contract from the acceptance source and compares it against an existing hand-authored `contract.md`, without modifying that file or any pipeline state.

#### Scenario: Existing contract is not modified
- **WHEN** the renderer runs in shadow mode against an existing `contract.md`
- **THEN** it produces the rendered contract and a comparison
- **AND** the existing `contract.md` is left unchanged

### Requirement: Report the comparison as a diagnostic diff

Shadow mode SHALL report differences between the rendered contract and the hand-authored contract as a text diff for human inspection. The text diff is diagnostic only; it is not the cutover criterion.

#### Scenario: Differences are reported
- **WHEN** the rendered contract differs from the hand-authored contract
- **THEN** shadow mode reports the differences as a diff

#### Scenario: No differences
- **WHEN** the rendered contract and the hand-authored contract are identical
- **THEN** shadow mode reports no differences

### Requirement: Never auto-trigger cutover

The renderer SHALL NOT switch the pipeline from hand-authored to rendered contracts. Cutover is a separate human action gated on validator-outcome parity across diverse mission shapes, acceptance and control contract authored in separate sessions, a seeded failure plus at least one real missed-or-weak requirement caught, and a rollback path.

#### Scenario: Shadow run leaves the pipeline unchanged
- **WHEN** the renderer runs in shadow mode
- **THEN** it does not modify the mission skill or disable hand-authoring
- **AND** any switch to rendered contracts remains a separate, gated action

### Requirement: Document the cutover gate and rollback

The shadow-comparison documentation SHALL define the cutover gate criteria and a rollback procedure, so an operator can switch to rendered contracts deliberately and reverse it. Stage 1 documents these; it does not enable cutover.

#### Scenario: Rollback procedure is documented
- **WHEN** an operator consults the shadow-comparison documentation
- **THEN** it describes the cutover gate criteria
- **AND** it describes a rollback procedure: regenerate the hand-authored contract and restore the prior validator flow

