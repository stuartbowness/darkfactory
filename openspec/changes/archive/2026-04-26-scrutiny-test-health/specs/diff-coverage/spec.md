## ADDED Requirements

### Requirement: Measure coverage on added lines per task
The scrutiny validator SHALL run the language-appropriate coverage tool and measure line coverage on added lines (green lines in the diff) only. The worker is responsible for covering code they wrote, not backfilling pre-existing coverage debt.

#### Scenario: Rust project coverage measurement
- **WHEN** a task's diff contains `.rs` files and `Cargo.toml` exists at the repo root
- **THEN** the scrutiny validator runs `cargo tarpaulin --out json` (or `cargo llvm-cov --json` as fallback), filters the coverage report to files in `git diff --name-only`, and reports coverage percentage on added lines

#### Scenario: TypeScript project coverage measurement
- **WHEN** a task's diff contains `.ts`/`.tsx`/`.js`/`.jsx` files and `package.json` exists at the repo root
- **THEN** the scrutiny validator runs `npx jest --coverage --json` or `npx c8 --reporter json`, filters the coverage report to files in `git diff --name-only`, and reports coverage percentage on added lines

#### Scenario: Python project coverage measurement
- **WHEN** a task's diff contains `.py` files and `pyproject.toml` or `requirements.txt` exists at the repo root
- **THEN** the scrutiny validator runs `pytest --cov --cov-report json`, filters the coverage report to files in `git diff --name-only`, and reports coverage percentage on added lines

### Requirement: Coverage thresholds with graduated response
The scrutiny validator SHALL apply graduated thresholds to diff coverage results. 80%+ is PASS. 70-79% is WARNING (included in report, does not fail the task). Below 70% is FAIL (triggers the fix loop).

#### Scenario: High coverage passes
- **WHEN** diff coverage on added lines is 85%
- **THEN** the scrutiny validator reports `diff-coverage | PASS | 85% coverage on changed lines`

#### Scenario: Medium coverage warns
- **WHEN** diff coverage on added lines is 74%
- **THEN** the scrutiny validator reports `diff-coverage | WARN | 74% coverage on changed lines (target: 80%)` with details listing under-covered files and line ranges

#### Scenario: Low coverage fails
- **WHEN** diff coverage on added lines is 55%
- **THEN** the scrutiny validator reports `diff-coverage | FAIL | 55% coverage on changed lines (minimum: 70%)` with details listing under-covered files and line ranges

### Requirement: Language detection from build files
The scrutiny validator SHALL detect the project's primary language from build files at the repo root: `Cargo.toml` for Rust, `package.json` for TypeScript/Node, `pyproject.toml` or `requirements.txt` for Python. Coverage runs for the primary language only.

#### Scenario: Rust project detected
- **WHEN** `Cargo.toml` exists at the repo root
- **THEN** the scrutiny validator uses Rust coverage tooling (`cargo tarpaulin` or `cargo llvm-cov`)

#### Scenario: Multi-language project
- **WHEN** both `Cargo.toml` and `package.json` exist at the repo root
- **THEN** the scrutiny validator runs coverage for the primary language only (determined by which language has the most files in the diff)

### Requirement: Missing coverage tool handling
The scrutiny validator SHALL skip the coverage check with a WARNING if the coverage tool is not installed, then attempt auto-install as a fallback. The primary mitigation is `darkfactory-init` verifying tools upfront.

#### Scenario: Coverage tool missing
- **WHEN** `cargo tarpaulin` is not installed and the project is Rust
- **THEN** the scrutiny validator reports `diff-coverage | SKIP | cargo-tarpaulin not installed` and attempts `cargo install cargo-tarpaulin`

### Requirement: Diff coverage check is self-gating
The check SHALL skip itself when the diff contains no production source files (only documentation, config, or test files).

#### Scenario: Documentation-only task
- **WHEN** a task's diff contains only `.md` files and no production source files
- **THEN** the scrutiny validator reports `diff-coverage | SKIP | no production source files in diff`
