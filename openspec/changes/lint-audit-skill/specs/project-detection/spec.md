## ADDED Requirements

### Requirement: Detect project linter from package.json
The skill SHALL detect which linter the project uses by reading `devDependencies` and `dependencies` in `package.json`.

#### Scenario: Oxlint project
- **WHEN** `package.json` contains `oxlint` in `devDependencies` or `dependencies`
- **THEN** the skill proceeds with the oxlint audit

#### Scenario: ESLint-only project
- **WHEN** `package.json` has a dependency with the exact package name `eslint` but not `oxlint`
- **THEN** the skill reports "ESLint-only project, skipping" and exits without auditing. Packages like `eslint-plugin-react` or `@eslint/js` alone do not trigger the skip.

#### Scenario: Both oxlint and ESLint present
- **WHEN** `package.json` contains both `oxlint` and `eslint`
- **THEN** the skill audits oxlint config and notes "ESLint also present, auditing oxlint only"

#### Scenario: No linter detected
- **WHEN** `package.json` contains neither `oxlint` nor `eslint`
- **THEN** the skill reports "No linter detected" and shows the install command (`npm install -D oxlint`) without running it

#### Scenario: No package.json
- **WHEN** no `package.json` exists at the project root
- **THEN** the skill reports "Not a Node/TypeScript project" and exits

### Requirement: Detect oxlint config format
The skill SHALL detect the oxlint configuration file in any supported format.

#### Scenario: Dotfile JSON config
- **WHEN** `.oxlintrc.json` exists at the project root
- **THEN** the skill reads and parses it as the active oxlint config

#### Scenario: JSONC config
- **WHEN** `.oxlintrc.jsonc` exists at the project root
- **THEN** the skill reads it as the active oxlint config (strip comments before parsing)

#### Scenario: TypeScript config
- **WHEN** `oxlint.config.ts` exists at the project root
- **THEN** the skill reports "TypeScript oxlint config detected" and notes that auto-apply is limited to manual instructions for this format

#### Scenario: Multiple config formats
- **WHEN** more than one oxlint config format exists in the same directory
- **THEN** the skill warns "Multiple oxlint config formats found" and asks the user which to use

#### Scenario: No config file
- **WHEN** no oxlint config file exists
- **THEN** the skill reports "No oxlint config found (using defaults)" and proceeds with audit

### Requirement: Discover tsconfig files in monorepo
The skill SHALL find all TypeScript configuration files across the project.

#### Scenario: Single tsconfig at root
- **WHEN** only `tsconfig.json` exists at the project root
- **THEN** the skill audits that single file

#### Scenario: Monorepo with subproject tsconfigs
- **WHEN** multiple `tsconfig*.json` files exist in subproject directories
- **THEN** the skill discovers files up to depth 3 and reports each one separately. If deeper files are found, it warns "Deeper tsconfig files may exist beyond depth 3"

#### Scenario: Extended tsconfig
- **WHEN** a `tsconfig.json` uses `extends` to inherit from another config
- **THEN** the skill resolves the effective config via `tsc --showConfig -p <tsconfig>` for strictness evaluation, so inherited settings are not falsely reported as missing

#### Scenario: Named tsconfig variants
- **WHEN** `tsconfig.app.json`, `tsconfig.node.json`, or other named variants exist
- **THEN** the skill includes them in discovery alongside `tsconfig.json`

### Requirement: Verify type-aware linting is enabled
The skill SHALL check that oxlint runs with type-aware mode.

#### Scenario: Type-aware flag present
- **WHEN** the project's lint script contains `--type-aware` or the oxlint config enables type-aware mode
- **THEN** the skill reports type-aware linting as enabled

#### Scenario: Type-aware flag missing
- **WHEN** the project's lint script does not contain `--type-aware` and the config does not enable it
- **THEN** the skill reports type-aware linting as missing and recommends adding `--type-aware` to the lint command
