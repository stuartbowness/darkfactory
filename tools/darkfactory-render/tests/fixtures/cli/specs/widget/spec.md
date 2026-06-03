# widget Specification

## Requirements

### Requirement: Widget greets a caller

The widget SHALL greet a caller when asked.

#### Scenario: Widget greets a caller
<!-- spec-ref: 1 -->
- **WHEN** a caller asks the widget to greet
- **THEN** the widget returns a greeting

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0
- [task:3] [output] the greeting output contains the caller's name
