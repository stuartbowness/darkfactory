## ADDED Requirements
### Requirement: Design template clarifies domain type boundary
The design template's Type Catalog section SHALL include a definition of what qualifies as a domain type versus a helper type.

#### Scenario: Domain type boundary in design template
<!-- spec-ref: 1 -->
- **WHEN** the orchestrator reads the design template to generate design.md
- **THEN** the Type Catalog section contains guidance that domain types model the problem space and that technical helpers (CLI parsers, test fixtures, generic containers) are excluded unless they are public compatibility surfaces

**Acceptance:**
- [task:2] [output] `skills/mission/templates/design-template.md` Type Catalog section contains a sentence stating that domain types model the problem space and that technical helpers are excluded unless they are public compatibility surfaces

### Requirement: UBIQUITOUS_LANGUAGE.md states Type Catalog relationship
The UBIQUITOUS_LANGUAGE.md Type Catalog definition SHALL state its relationship to the glossary: type names are drawn from the glossary when a matching term exists, and types not in the glossary are named by the worker.

#### Scenario: Type Catalog definition includes glossary relationship
- **WHEN** a reader looks up "Type Catalog" in UBIQUITOUS_LANGUAGE.md
- **THEN** the definition states that type names are drawn from the glossary when a matching term exists, and that types not in the glossary are named by the worker

### Requirement: UBIQUITOUS_LANGUAGE.md documents the type naming chain
The UBIQUITOUS_LANGUAGE.md Relationships section SHALL document the full type naming chain from glossary through to code.

#### Scenario: Type naming chain in Relationships
- **WHEN** a reader looks at the Relationships section of UBIQUITOUS_LANGUAGE.md
- **THEN** there is an entry describing the chain: Type Catalog draws names from the glossary, worker briefs list Type Catalog entries, foundation audit checks task 1, module shape re-check verifies all names at the end

### Requirement: UBIQUITOUS_LANGUAGE.md defines Domain type
UBIQUITOUS_LANGUAGE.md SHALL include a "Domain type" term definition.

#### Scenario: Domain type term present
- **WHEN** a reader looks up "Domain type" in UBIQUITOUS_LANGUAGE.md
- **THEN** the definition distinguishes domain types from technical helper types
