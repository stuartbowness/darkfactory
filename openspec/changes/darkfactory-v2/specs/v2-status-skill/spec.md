## ADDED Requirements

### Requirement: Skill has valid SKILL.md frontmatter
The skill file SHALL have YAML frontmatter with name and description fields.

#### Scenario: Valid frontmatter
- **WHEN** the file is parsed
- **THEN** it has a name field and a description explaining mission status checking

### Requirement: Skill is read-only
The skill SHALL NOT modify any mission files.

#### Scenario: No side effects
- **WHEN** the skill runs
- **THEN** no files are created, modified, or deleted

### Requirement: Skill finds active missions
The skill SHALL search for OpenSpec changes with mission/ directories.

#### Scenario: Mission discovery
- **WHEN** the skill runs
- **THEN** it searches for mission directories and handles zero, one, or multiple results

### Requirement: Skill displays formatted progress
The skill SHALL show: mission name, tasks completed vs total, current task status, validation results per task, and recent activity from progress.md.

#### Scenario: Progress displayed
- **WHEN** an active mission is found
- **THEN** the skill shows a formatted summary with completion count, per-task validation results, and recent log entries
