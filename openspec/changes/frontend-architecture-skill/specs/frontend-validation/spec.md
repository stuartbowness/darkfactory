## ADDED Requirements

### Requirement: Gate 7 (frontend depth) exists in the scrutiny validator
The scrutiny validator SHALL include a new gate 7 named "Frontend depth review" inserted after gate 6 (module depth review). Existing gates 7-14 SHALL be renumbered to 8-15.

#### Scenario: Gate present in validator
- **WHEN** the scrutiny validator runs against a task diff
- **THEN** gate 7 SHALL appear in the output table as "Frontend depth review" between "Module depth review" (gate 6) and "Code standards" (gate 8)

#### Scenario: Gate numbering sequence
- **WHEN** the scrutiny validator runs all gates
- **THEN** gates SHALL be numbered 1-15 sequentially with no gaps or duplicates

### Requirement: Gate 7 self-skips when no React files in diff
Gate 7 SHALL be a self-skipping gate that only evaluates when the diff contains `.tsx` or `.jsx` files.

#### Scenario: No React files in diff
- **WHEN** the diff contains zero `.tsx` and zero `.jsx` files
- **THEN** gate 7 SHALL report `SKIP` with note "no React component files in diff"

#### Scenario: React files present in diff
- **WHEN** the diff contains one or more `.tsx` or `.jsx` files
- **THEN** gate 7 SHALL evaluate those files against the four deep-component rules

### Requirement: Gate 7 checks the four deep-component rules
When React files are present, gate 7 SHALL evaluate each `.tsx`/`.jsx` file against the four rules defined in `/frontend`'s DEEP-COMPONENTS.md.

#### Scenario: Small semantic interface check
- **WHEN** a component has a prop interface with more than 7 semantic concepts (props + context + URL params + cache dependencies) AND is not in the allowed-shallow exception list
- **THEN** gate 7 SHALL flag it as a potential depth violation

#### Scenario: Derive don't sync check
- **WHEN** a component contains `useEffect` paired with `setState` where the effect body is pure data transformation (no I/O, no subscriptions)
- **THEN** gate 7 SHALL flag the pattern and suggest useMemo or inline computation

#### Scenario: Composition over configuration check
- **WHEN** a component accepts configuration-style props (items arrays, option objects with more than 5 config entries) that could be compound components with children
- **THEN** gate 7 SHALL flag the pattern

#### Scenario: Collapse coordination check
- **WHEN** a component exposes coordination knobs (e.g., onStep1Complete, onValidate, onReset) rather than semantic events (e.g., onSubmit, onCancel)
- **THEN** gate 7 SHALL flag the pattern

### Requirement: Gate 7 recognizes allowed-shallow exceptions
Gate 7 SHALL NOT flag components that fall into recognized allowed-shallow categories.

#### Scenario: Design system leaf components
- **WHEN** a component is in a `*/ui/*` directory AND has only `className` + `children` + `...rest` props
- **THEN** gate 7 SHALL treat it as an allowed-shallow exception and not flag depth violations

#### Scenario: Radix wrapper components
- **WHEN** a component imports from `@radix-ui/*` and wraps a Radix primitive with styling
- **THEN** gate 7 SHALL treat it as an allowed-shallow exception

### Requirement: Gate 7 produces a clear verdict
Gate 7 SHALL report PASS, FAIL, or WARN based on the severity and quantity of findings.

#### Scenario: No violations found
- **WHEN** no React anti-patterns are found OR all flagged items are in allowed-shallow categories
- **THEN** gate 7 SHALL report `PASS`

#### Scenario: Systematic violations
- **WHEN** depth violations are found across multiple components in the diff
- **THEN** gate 7 SHALL report `FAIL`

#### Scenario: Isolated finding
- **WHEN** a single depth violation is found in one component
- **THEN** gate 7 SHALL report `WARN`

### Requirement: Gate 1 rationale updated for /frontend tasks
Gate 1 (test presence) SHALL adjust its failure rationale based on the task's methodology.

#### Scenario: /tdd task with no tests
- **WHEN** a task has `Skill: /tdd` and the diff contains zero test files and zero new test assertions
- **THEN** gate 1 SHALL report `FAIL` with rationale "TDD not followed"

#### Scenario: /frontend task with no tests
- **WHEN** a task has `Skill: /frontend` and the diff contains zero test files and zero new test assertions
- **THEN** gate 1 SHALL report `FAIL` with rationale "/frontend Test phase not followed"

### Requirement: UBIQUITOUS_LANGUAGE.md gate references updated
All gate number references in UBIQUITOUS_LANGUAGE.md SHALL reflect the new 1-15 numbering.

#### Scenario: Gate number consistency
- **WHEN** UBIQUITOUS_LANGUAGE.md references a gate by number
- **THEN** the number SHALL match the new sequence (e.g., contract checks = gate 11, not gate 10)
