# widget Specification (deliberately broken: duplicate spec reference)

## Requirements

### Requirement: Widget greets a caller

#### Scenario: Widget greets a caller
<!-- spec-ref: 1 -->
- **WHEN** a caller asks the widget to greet
- **THEN** the widget returns a greeting

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0

#### Scenario: Widget greets a caller again
<!-- spec-ref: 1 -->
- **WHEN** a caller asks the widget to greet once more
- **THEN** the widget returns a greeting

**Acceptance:**
- [task:2] [execution] `widget greet` still prints a greeting and exits 0
