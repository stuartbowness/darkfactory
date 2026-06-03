# widget Specification (deliberately broken)

## Requirements

### Requirement: Dangling task link

#### Scenario: Acceptance points at a task that does not exist
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** it fails loud

**Acceptance:**
- [task:99] [execution] this check links to a task that is not in tasks.md
