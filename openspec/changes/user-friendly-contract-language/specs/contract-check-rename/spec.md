## ADDED Requirements

### Requirement: Contract items use "check" terminology
All living source files SHALL use "check" (not "assertion") when referring to a single verifiable statement in a contract. The term "assertion" SHALL be reserved exclusively for code test assertions (e.g., tautological assertion, `assert_eq!`, `expect().toBe()`).

#### Scenario: Contract definition in UBIQUITOUS_LANGUAGE.md
- **WHEN** a reader looks up the Contract definition in UBIQUITOUS_LANGUAGE.md
- **THEN** the definition says "a set of checks" (not "a set of assertions")

#### Scenario: Check definition in UBIQUITOUS_LANGUAGE.md
- **WHEN** a reader looks up contract item terminology in UBIQUITOUS_LANGUAGE.md
- **THEN** the entry is named "Check" with "Assertion (in contract context)" listed as an alias to avoid

#### Scenario: Mission skill contract rules
- **WHEN** the orchestrator reads the contract rules in skills/mission/SKILL.md
- **THEN** the rules use "check" for contract items (e.g., "Group checks by task", "Write the minimum set of checks")

#### Scenario: Contract template format
- **WHEN** the orchestrator reads the contract template in templates/contract.md
- **THEN** the template items use "check" (e.g., `[execution] <check capturing specific spec scenario detail>`)

#### Scenario: Validator report section headings
- **WHEN** a validator produces a report using the scrutiny or user-test validator prompt
- **THEN** the report uses "Contract Checks" and "Failed Checks" as section headings, and "Check" as the column header in results tables

#### Scenario: PR body template
- **WHEN** the orchestrator generates a PR body from templates/pr-body.md
- **THEN** the template says "Total checks: <M>" (not "Total assertions")

#### Scenario: Progress log language
- **WHEN** the orchestrator writes to progress.md during launch confirmation or task completion
- **THEN** the entries use "checks" (e.g., "<N> tasks, <M> checks in contract")

#### Scenario: Worker brief and fix brief guidance
- **WHEN** a worker reads a brief or fix brief
- **THEN** the guidance says "Do NOT copy check text from the contract" (not "assertion text")

#### Scenario: Documentation consistency
- **WHEN** a developer reads docs/provenance.md, docs/design-decisions.md, or docs/reference_factory_talk.md
- **THEN** references to contract items use "checks" (not "assertions")

#### Scenario: README consistency
- **WHEN** a developer reads README.md
- **THEN** references to contract items use "checks" (not "assertions")

### Requirement: Code test assertion terminology preserved
The term "assertion" SHALL be preserved in all contexts that refer to code test assertions, including the test-quality skill, tdd skill, and test presence gate.

#### Scenario: Tautological assertion unchanged
- **WHEN** a reader looks up "Tautological assertion" in UBIQUITOUS_LANGUAGE.md or the test-quality skill
- **THEN** the term remains "Tautological assertion" (not "Tautological check")

#### Scenario: Test quality skill unchanged
- **WHEN** the scrutiny validator invokes the test-quality skill
- **THEN** the skill uses "assertion quality", "tautological assertion", and "test assertion" unchanged

#### Scenario: TDD skill unchanged
- **WHEN** a developer reads the tdd skill guidance
- **THEN** it says "one logical assertion per test" (not "one logical check per test")

#### Scenario: Test presence gate code test reference
- **WHEN** the scrutiny validator runs the test presence gate
- **THEN** the prompt says "zero new test assertions" referring to code-level assertions in test files

### Requirement: Archived and in-flight files untouched
Living source files SHALL be renamed, but archived mission files and in-flight change directories SHALL NOT be modified.

#### Scenario: Archived missions preserved
- **WHEN** a grep for "assertion" is run against files under openspec/changes/archive/
- **THEN** the results are unchanged from before this mission

#### Scenario: In-flight changes preserved
- **WHEN** a grep for "assertion" is run against in-flight change directories (e.g., skill-decomposition, deterministic-planning)
- **THEN** the results are unchanged from before this mission

### Requirement: Resolved ambiguity updated
The resolved ambiguity about "check" and "assertion" in UBIQUITOUS_LANGUAGE.md SHALL document the three-way split.

#### Scenario: Three-way split documented
- **WHEN** a reader looks at the resolved ambiguities section
- **THEN** there is an entry explaining: Gate = scrutiny validator step, Check = contract item, Assertion = code test assertion only

### Requirement: Relationships section updated
The Relationships section of UBIQUITOUS_LANGUAGE.md SHALL use "Checks" for contract items.

#### Scenario: Contract contains Checks
- **WHEN** a reader looks at the Relationships section
- **THEN** it says "A Contract contains Checks grouped by Task" (not "Assertions")
