## ADDED Requirements

### Requirement: Compare oxlint rules against agent-friendly ruleset
The skill SHALL compare the project's oxlint configuration against the recommended agent-friendly ruleset defined in RULES.md.

#### Scenario: Rule explicitly enabled
- **WHEN** an agent-friendly rule (e.g., `typescript/no-explicit-any`) is set to `"error"` or `"warn"` in the oxlint config
- **THEN** the skill reports it as `[x]` (enabled)

#### Scenario: Rule missing from config
- **WHEN** an agent-friendly rule is not present in the oxlint config
- **THEN** the skill checks whether the rule is enabled by default in oxlint, and reports accordingly: `[x]` if default-enabled, `[ ]` if not

#### Scenario: Rule explicitly disabled
- **WHEN** an agent-friendly rule is set to `"off"` or `"allow"` in the oxlint config
- **THEN** the skill reports it as `[!]` (explicitly disabled -- user chose to turn it off) with a note explaining why it matters for agents

#### Scenario: No oxlint config (defaults only)
- **WHEN** no oxlint config file exists
- **THEN** the skill checks each agent-friendly rule against oxlint's default configuration and reports which are covered by defaults and which require explicit config

### Requirement: Define RULES.md schema for agent-friendly ruleset
The RULES.md reference file SHALL use a structured format for each recommended rule.

#### Scenario: Rule entry format
- **WHEN** a rule is defined in RULES.md
- **THEN** it includes: rule name (oxlint canonical), category, recommended severity (`error` or `warn`), whether it is enabled by default in oxlint, and a one-line rationale explaining why it matters for agents

#### Scenario: Oxlint defaults pinned
- **WHEN** RULES.md lists whether a rule is default-enabled
- **THEN** the default status is pinned to a specific oxlint version noted at the top of RULES.md, so the audit script has a deterministic source of truth without shelling out

### Requirement: Audit tsconfig strictness settings
The skill SHALL check each discovered tsconfig.json for recommended strictness settings.

#### Scenario: strict mode enabled
- **WHEN** `compilerOptions.strict` is `true` in the tsconfig
- **THEN** the skill reports `strict` as `[x]` for that tsconfig

#### Scenario: strict mode missing
- **WHEN** `compilerOptions.strict` is not enabled in the effective tsconfig (resolved via `tsc --showConfig`)
- **THEN** the skill reports `strict` as `[ ]` and runs `tsc -p <tsconfig> --strict --noEmit --pretty false 2>&1 | grep -c 'error TS'` to show migration scope

#### Scenario: noUncheckedIndexedAccess enabled
- **WHEN** `compilerOptions.noUncheckedIndexedAccess` is `true`
- **THEN** the skill reports it as `[x]`

#### Scenario: noUncheckedIndexedAccess missing
- **WHEN** `compilerOptions.noUncheckedIndexedAccess` is not enabled in the effective tsconfig
- **THEN** the skill reports it as `[ ]`, runs `tsc -p <tsconfig> --noUncheckedIndexedAccess --noEmit --pretty false 2>&1 | grep -c 'error TS'` for migration scope, and shows the recommendation "Enables T | undefined for indexed access"

#### Scenario: erasableSyntaxOnly enabled
- **WHEN** `compilerOptions.erasableSyntaxOnly` is `true`
- **THEN** the skill reports it as `[x]`

#### Scenario: erasableSyntaxOnly missing
- **WHEN** `compilerOptions.erasableSyntaxOnly` is absent or `false`
- **THEN** the skill reports it as `[ ]` with the recommendation "Bans enum, namespace, parameter properties at compiler level"

### Requirement: Produce structured findings report
The skill SHALL produce a single combined report covering both oxlint rules and tsconfig settings.

#### Scenario: Report format
- **WHEN** the audit completes
- **THEN** the report uses status markers (`[x]` enabled, `[ ]` missing, `[!]` partially/explicitly disabled) grouped under "Oxlint Rules" and "TypeScript Compiler" sections

#### Scenario: Report includes migration scope
- **WHEN** a tsconfig is missing `strict`
- **THEN** the report shows the error count next to the recommendation: `[ ] strict (N errors to fix)`

#### Scenario: Report includes rule rationale
- **WHEN** a rule is reported as missing
- **THEN** the report includes a one-line rationale from RULES.md explaining why the rule matters for agents
