## ADDED Requirements

### Requirement: Offer per-item and batch apply modes
The skill SHALL offer three apply modes after presenting the report.

#### Scenario: Apply all
- **WHEN** the user chooses "apply all"
- **THEN** the skill applies every missing recommendation (oxlint rules, tsconfig settings, and type-aware linting enablement) without further prompts

#### Scenario: Skip all
- **WHEN** the user chooses "skip all"
- **THEN** the skill exits without making any changes

#### Scenario: Per-item review
- **WHEN** the user chooses "review per-item"
- **THEN** the skill presents each missing recommendation individually with accept/deny, applying only the accepted ones

### Requirement: Apply oxlint rule changes to JSON config
The skill SHALL create or update `.oxlintrc.json` when applying oxlint rule recommendations.

#### Scenario: Create new config
- **WHEN** no oxlint config exists and the user accepts oxlint rule recommendations
- **THEN** the skill creates `.oxlintrc.json` at the project root with the accepted rules

#### Scenario: Update existing JSON config
- **WHEN** `.oxlintrc.json` exists and the user accepts additional rules
- **THEN** the skill adds the accepted rules to the existing config's `rules` object without removing existing rules

#### Scenario: JSONC config format
- **WHEN** `.oxlintrc.jsonc` exists and the user accepts additional rules
- **THEN** the skill prints the rule configuration as JSON that the user can manually add, and does NOT modify the `.jsonc` file (JSONC comment preservation is fragile)

#### Scenario: TypeScript config format
- **WHEN** `oxlint.config.ts` is the active config and the user accepts rules
- **THEN** the skill prints the rule configuration as JSON that the user can manually add, and does NOT modify the `.ts` file

### Requirement: Apply tsconfig strictness changes
The skill SHALL update tsconfig.json files when applying strictness recommendations.

#### Scenario: Enable strict mode
- **WHEN** the user accepts the `strict` recommendation for a specific tsconfig
- **THEN** the skill adds `"strict": true` to `compilerOptions` in that tsconfig

#### Scenario: Enable noUncheckedIndexedAccess
- **WHEN** the user accepts the `noUncheckedIndexedAccess` recommendation
- **THEN** the skill adds `"noUncheckedIndexedAccess": true` to `compilerOptions`

#### Scenario: Enable erasableSyntaxOnly
- **WHEN** the user accepts the `erasableSyntaxOnly` recommendation
- **THEN** the skill adds `"erasableSyntaxOnly": true` to `compilerOptions`

#### Scenario: Show migration scope before applying strictness settings
- **WHEN** the user is reviewing `strict`, `noUncheckedIndexedAccess`, or `erasableSyntaxOnly` in per-item mode
- **THEN** the skill shows the error count for that specific setting before asking for accept/deny

### Requirement: Recommend type-aware linting enablement
The skill SHALL recommend enabling type-aware linting when it is not active.

#### Scenario: Recommend --type-aware flag
- **WHEN** oxlint is installed but type-aware linting is not enabled
- **THEN** the skill recommends adding `--type-aware` to the project's lint script and shows the exact command modification needed
