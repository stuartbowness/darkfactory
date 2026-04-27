## MODIFIED Requirements

### Requirement: Verify coverage tool availability per project language
`darkfactory-init` SHALL detect the project's language from build files and verify that the appropriate coverage tool is available. If missing, it SHALL suggest the install command. It SHALL NOT auto-install.

#### Scenario: Rust project with cargo-tarpaulin missing
- **WHEN** `Cargo.toml` exists at the project root and `cargo tarpaulin --version` fails
- **THEN** darkfactory-init reports `cargo-tarpaulin` as missing and suggests `cargo install cargo-tarpaulin`

#### Scenario: Rust project with cargo-tarpaulin installed
- **WHEN** `Cargo.toml` exists at the project root and `cargo tarpaulin --version` succeeds
- **THEN** darkfactory-init reports coverage tooling as available

#### Scenario: TypeScript project coverage check
- **WHEN** `package.json` exists at the project root
- **THEN** darkfactory-init checks whether jest or vitest has coverage support available (looks for `c8`, `@jest/coverage`, or `istanbul` in deps/devDeps). If missing, suggests `npm install -D c8`

#### Scenario: Python project with pytest-cov missing
- **WHEN** `pyproject.toml` or `requirements.txt` exists at the project root and `pip show pytest-cov` fails
- **THEN** darkfactory-init reports `pytest-cov` as missing and suggests `pip install pytest-cov`

#### Scenario: No recognized language
- **WHEN** no `Cargo.toml`, `package.json`, `pyproject.toml`, or `requirements.txt` exists at the project root
- **THEN** darkfactory-init skips the coverage tool check with a note: "No recognized language detected -- coverage tool check skipped"

### Requirement: Coverage check is part of the existing dependency checklist
The coverage tool check SHALL be integrated into the existing Step 4 (CLI dependencies) output, following the same checklist format used for codex, darkfactory-dispatch, gh, openspec, and jq.

#### Scenario: Coverage tool shown in status output
- **WHEN** darkfactory-init runs its full check sequence
- **THEN** the coverage tool status appears in the Step 4 output alongside other CLI dependencies, using the same pass/fail/missing format
