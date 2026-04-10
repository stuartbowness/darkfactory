## ADDED Requirements

### Requirement: Skill has valid SKILL.md frontmatter
The skill file SHALL have YAML frontmatter with name and description fields.

#### Scenario: Valid frontmatter
- **WHEN** the file is parsed
- **THEN** it has a name field and a description explaining mission resume

### Requirement: Skill finds active missions from file state
The skill SHALL search for OpenSpec changes with mission/ directories and unchecked tasks.

#### Scenario: Mission discovery
- **WHEN** the skill runs
- **THEN** it searches openspec/changes/*/mission/config.yaml and checks tasks.md for unchecked items

### Requirement: Skill handles multiple or no active missions
The skill SHALL handle zero, one, or multiple active missions appropriately.

#### Scenario: No missions
- **WHEN** no active missions exist
- **THEN** the skill reports "No active missions found"

#### Scenario: Multiple missions
- **WHEN** multiple active missions exist
- **THEN** the skill lists them and asks the user to pick one

### Requirement: Skill reconstructs the orchestrator plan
The skill SHALL enter plan mode and write a complete orchestrator operating manual from file state.

#### Scenario: Plan reconstructed
- **WHEN** resuming a mission
- **THEN** the skill reads tasks.md, progress.md, config.yaml, contract.md, proposal.md, and design.md to write the plan

### Requirement: Skill detects orphaned commits
The skill SHALL check git log for commits matching the task pattern when no handoff file exists.

#### Scenario: Orphaned commit found
- **WHEN** a commit matching [task-N] exists but no handoff file
- **THEN** the skill asks the human whether to count it as done or re-run

### Requirement: Skill continues the build loop
After reconstructing state, the skill SHALL continue from the next unchecked task using the same build phase logic as /mission.

#### Scenario: Build loop resumes
- **WHEN** the plan is reconstructed
- **THEN** the skill follows the same worker dispatch, validation, and progress tracking flow as /mission Phase 7
