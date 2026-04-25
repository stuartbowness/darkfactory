## ADDED Requirements

### Requirement: Accurate skill source attribution in README
The README skill table SHALL accurately describe the source of each tool and command. `/opsx:archive` SHALL be attributed to Darkfactory, not to `openspec init`.

#### Scenario: Reader checks where /opsx:archive comes from
- **WHEN** a developer reads the README skill table row for `/opsx:archive`
- **THEN** the source column says "Bundled with Darkfactory" (not "Installed by `openspec init`")
