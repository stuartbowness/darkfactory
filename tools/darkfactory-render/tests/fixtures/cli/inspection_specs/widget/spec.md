# widget Specification (inspection-heavy)

## Requirements

### Requirement: Inspection-heavy task

#### Scenario: A task whose inspection checks exceed the cap
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** it renders but warns about the inspection ratio

**Acceptance:**
- [task:2] [execution] an execution check that runs
- [task:2] [inspection] an inspection check that reads code
