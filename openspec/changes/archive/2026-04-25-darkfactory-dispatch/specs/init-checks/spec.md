## MODIFIED Requirements

### Requirement: darkfactory-dispatch is a required dependency
The darkfactory-init skill SHALL check for `darkfactory-dispatch` on PATH and report it as a required CLI tool.

#### Scenario: darkfactory-dispatch installed
- **WHEN** `darkfactory-dispatch --version` succeeds
- **THEN** the init report shows `[x] darkfactory-dispatch` under CLI tools

#### Scenario: darkfactory-dispatch missing
- **WHEN** `darkfactory-dispatch` is not on PATH
- **THEN** the init report shows `[ ] darkfactory-dispatch (not installed)` and provides install instructions: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
