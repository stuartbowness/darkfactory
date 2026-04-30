## ADDED Requirements

### Requirement: Mandatory design sections
Design.md SHALL include five mandatory sections defined in the design template reference document.

#### Scenario: Design generation follows template
- **WHEN** the orchestrator generates design.md during Phase 4.3
- **THEN** the design MUST include sections for: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, and Integration Points

#### Scenario: Architecture Decision section content
- **WHEN** the Architecture Decision section is written (greenfield or arch-change mission)
- **THEN** it MUST contain a scoring table with 2-3 candidates, their scores on all four dimensions, and the rationale for the chosen architecture

#### Scenario: Module Inventory section content
- **WHEN** the Module Inventory section is written
- **THEN** it MUST contain a table with columns for module name, interface width, what it hides, and why this boundary exists

#### Scenario: Type Catalog section content
- **WHEN** the Type Catalog section is written
- **THEN** it MUST list each domain type with name (from UBIQUITOUS_LANGUAGE.md), purpose (one line), and owning module -- without field-level detail

#### Scenario: Error Strategy section content
- **WHEN** the Error Strategy section is written
- **THEN** it MUST specify the error type hierarchy, propagation rules, and which layer throws vs catches

#### Scenario: Integration Points section content
- **WHEN** the Integration Points section is written
- **THEN** it MUST describe how modules call each other with caller, callee, method/interface, and direction

### Requirement: OpenSpec config rules injection
The mandatory section requirements SHALL be injected into design generation instructions via openspec config.yaml rules.

#### Scenario: Config rules present
- **WHEN** openspec config.yaml contains rules.design entries listing the five mandatory sections
- **THEN** the openspec CLI MUST include those rules in the instructions returned for design generation

### Requirement: Design template reference document
A reference document SHALL exist at `templates/design-template.md` defining the content format for each mandatory section, including the architecture scoring rubric with per-level criteria.

#### Scenario: Template file exists
- **WHEN** the mission skill is loaded
- **THEN** `templates/design-template.md` MUST exist and define content formats for all five mandatory sections, including the four-dimension scoring rubric with concrete per-level criteria for each dimension

### Requirement: Design section gate
The orchestrator SHALL verify all five mandatory sections exist before presenting design.md for human review. The gate also enforces the architecture scoring hard floor and tie escalation.

#### Scenario: All sections present
- **WHEN** design.md contains headings for all five mandatory sections
- **THEN** the orchestrator proceeds to present the design for human review

#### Scenario: Missing section detected
- **WHEN** design.md is missing one or more of the five mandatory section headings
- **THEN** the orchestrator MUST regenerate the design before presenting it to the human

#### Scenario: Architecture candidate scores zero
- **WHEN** the recommended architecture candidate scores 0 on any dimension
- **THEN** the gate MUST reject the design and require the orchestrator to recommend a different candidate or regenerate

#### Scenario: Architecture candidates tie
- **WHEN** two or more architecture candidates have the same total score
- **THEN** the gate MUST present both tied candidates to the human for discussion and selection rather than applying an arbitrary tie-breaker
