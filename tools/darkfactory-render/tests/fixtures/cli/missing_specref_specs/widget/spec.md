# widget Specification (deliberately broken: acceptance with no spec reference)

## Requirements

### Requirement: Widget greets a caller

#### Scenario: Widget greets a caller
- **WHEN** a caller asks the widget to greet
- **THEN** the widget returns a greeting

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0
