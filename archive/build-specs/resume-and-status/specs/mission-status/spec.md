## ADDED Requirements

### Requirement: User can check mission progress
The system SHALL provide a `/mission-status` skill that shows a formatted progress summary.

#### Scenario: Status with active mission
- **WHEN** user runs `/mission-status` and an active mission exists
- **THEN** the skill displays: mission name, tasks completed vs total, current task (if in progress), validation results per completed task, any active issues, and time since mission started

#### Scenario: Status with no active mission
- **WHEN** user runs `/mission-status` and no active missions exist
- **THEN** the skill reports "No active missions found"

### Requirement: Status is read-only
The `/mission-status` skill SHALL NOT modify any mission files. It only reads and reports.

#### Scenario: No side effects
- **WHEN** `/mission-status` runs
- **THEN** no files in the mission directory are created, modified, or deleted

### Requirement: Status detects active change
The skill SHALL find OpenSpec changes with a mission/ directory. If multiple exist, it asks the user which one.

#### Scenario: Multiple missions
- **WHEN** multiple changes have mission/ directories
- **THEN** the skill lists them and asks the user to pick one
