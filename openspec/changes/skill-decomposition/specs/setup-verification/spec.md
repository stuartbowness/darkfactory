## MODIFIED Requirements

### Requirement: /mission-init skill replaces /darkfactory-init with symlink verification
The `skills/mission-init/SKILL.md` skill SHALL check that `install.sh` has been run by verifying symlinks exist, instead of checking two separate source locations.

#### Scenario: Symlink check replaces dual-location check
- **WHEN** `/init` runs its skill checks
- **THEN** it verifies that `~/.claude/skills/<name>` is a symlink pointing to `~/.darkfactory/skills/<name>` for each of the 12 owned skills
- **AND** it does not check `.claude/skills/` inside the repo or `~/.darkfactory-skills/` as alternate install locations

#### Scenario: Broken symlink detected and reported
- **WHEN** a skill symlink exists in `~/.claude/skills/` but does not resolve (dangling symlink)
- **THEN** `/init` reports it as broken with the expected target path
- **AND** it suggests running `~/.darkfactory/install.sh` to fix it

#### Scenario: Missing symlink detected and reported
- **WHEN** a skill is not symlinked (symlink absent entirely)
- **THEN** `/init` reports it as missing
- **AND** it suggests running `~/.darkfactory/install.sh`

### Requirement: /mission-init reports all 12 expected skills
The skill SHALL check for all 12 skills by their current names.

#### Scenario: Skill list is correct
- **WHEN** `/mission-init` checks skills
- **THEN** the checklist includes: `mission`, `grill`, `language`, `prd`, `adversarial`, `dispatch`, `tdd`, `test-quality`, `security`, `mission-resume`, `mission-status`, `mission-init`
- **AND** the checklist does NOT include: `grill-me`, `write-a-prd`, `ubiquitous-language`, `darkfactory-init`, `test-quality-review`

### Requirement: /mission-init still verifies CLI tools
The skill SHALL still check for required CLI tools as before.

#### Scenario: CLI tools checked
- **WHEN** `/mission-init` runs
- **THEN** it checks for `codex`, `darkfactory-dispatch`, `gh`, `openspec`, and `jq`
- **AND** for each, it reports whether it is installed and authenticated (where applicable)

### Requirement: /mission-init reports install instructions when setup is incomplete
If any symlinks are missing, the skill SHALL print a single command to fix everything.

#### Scenario: Install command shown on failure
- **WHEN** one or more skill symlinks are missing or broken
- **THEN** `/mission-init` prints: `Run: ~/.darkfactory/install.sh`
- **AND** it exits after showing the report (does not attempt to create symlinks itself)

### Requirement: /mission-init verifies coverage tool per detected language
The skill SHALL detect the project language from build files and check the appropriate coverage tool is installed.

#### Scenario: Coverage tool checked for Rust project
- **WHEN** `/mission-init` runs in a directory with `Cargo.toml` at the root
- **THEN** it checks whether `cargo-tarpaulin` is installed
- **AND** reports `[x] cargo-tarpaulin` if installed, or `[ ] cargo-tarpaulin (install: cargo install cargo-tarpaulin)` if not

#### Scenario: Coverage tool checked for TypeScript/Node project
- **WHEN** `/mission-init` runs in a directory with `package.json` at the root
- **THEN** it checks `package.json` `dependencies` and `devDependencies` for `c8`, `@jest/coverage`, `istanbul`, or `jest`
- **AND** reports the found package (e.g. `[x] jest (built-in coverage)`) or `[ ] no coverage package found (install: npm install -D c8)` if none found

#### Scenario: Coverage tool checked for Python project
- **WHEN** `/mission-init` runs in a directory with `pyproject.toml` or `requirements.txt` at the root
- **THEN** it checks whether `pytest-cov` is installed
- **AND** reports `[x] pytest-cov` if installed, or `[ ] pytest-cov (install: pip install pytest-cov)` if not

#### Scenario: No recognized language detected
- **WHEN** `/mission-init` runs in a directory with no `Cargo.toml`, `package.json`, `pyproject.toml`, or `requirements.txt` at the root
- **THEN** it reports "No recognized language detected -- coverage tool check skipped"

### Requirement: /mission-init still initializes OpenSpec if not initialized
The skill SHALL still run `openspec init` if `openspec/config.yaml` does not exist in the current directory.

#### Scenario: OpenSpec init on first run
- **WHEN** `/mission-init` runs in a directory without `openspec/config.yaml`
- **THEN** it runs `openspec init --tools claude,codex --force`
- **AND** it reports "OpenSpec: initialized"
