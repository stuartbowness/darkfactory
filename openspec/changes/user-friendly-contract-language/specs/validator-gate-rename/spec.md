## ADDED Requirements

### Requirement: Scrutiny validator steps use "gate" terminology
All references to numbered scrutiny validator steps SHALL use "gate" (not "check"). This frees "check" for contract items.

#### Scenario: Gate definition in UBIQUITOUS_LANGUAGE.md
- **WHEN** a reader looks up scrutiny validator step terminology in UBIQUITOUS_LANGUAGE.md
- **THEN** there is a "Gate" entry defining it as a numbered step in the scrutiny validator's sequence, with outcomes PASS, FAIL, WARN, SKIP, or N/A

#### Scenario: Scrutiny validator prompt uses gates
- **WHEN** the orchestrator fills in the scrutiny validator prompt from prompts/scrutiny-validator.md
- **THEN** numbered steps are called "gate N" (not "check N") throughout the prompt

#### Scenario: Behavioral conformance references gate
- **WHEN** a reader looks up "Behavioral conformance" in UBIQUITOUS_LANGUAGE.md
- **THEN** it references "gate 11" (not "check 11")

#### Scenario: Self-skipping gate renamed
- **WHEN** a reader looks up the concept of a validator step that auto-skips
- **THEN** the term is "Self-skipping gate" (not "Self-gating check")

#### Scenario: Mission skill references gates
- **WHEN** the orchestrator reads skills/mission/SKILL.md for validator step references
- **THEN** the text uses "gate" for validator steps (e.g., "gate 10: contract checks" not "check 10: contract assertions")

#### Scenario: Scrutiny validator report gate numbering
- **WHEN** a scrutiny validator produces a report following the prompt template
- **THEN** each validator step result is labeled as a gate (e.g., "Gate 1: Test presence", "Gate 10: Contract checks")
