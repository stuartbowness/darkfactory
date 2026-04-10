## ADDED Requirements

### Requirement: Orchestrator creates draft PR when all tasks complete
The orchestrator SHALL create a draft PR via the /pr skill after the build phase finishes with all tasks validated.

#### Scenario: Draft PR created
- **WHEN** all tasks in tasks.md are marked complete
- **THEN** a draft PR exists on GitHub with a mission summary in the body

### Requirement: PR body contains mission summary
The draft PR body SHALL include: list of tasks with completion status, validation results per task, links to demo videos (if any), issues encountered and fixes applied, and total attempt count.

#### Scenario: Mission summary is comprehensive
- **WHEN** the draft PR is created
- **THEN** the body contains enough information for the human to review without reading mission files

### Requirement: PR is always a draft
The orchestrator SHALL never create a ready-for-review PR. The human promotes it.

#### Scenario: PR created as draft
- **WHEN** the orchestrator creates the PR
- **THEN** it is marked as a draft on GitHub
