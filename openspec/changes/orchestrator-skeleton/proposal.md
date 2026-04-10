## Why

There is no way to run the Darkfactory mission system today. The design decisions, ubiquitous language, and architecture are documented, but no executable skills or templates exist. This change creates the orchestrator -- the entry point that a user interacts with to start a mission. Without it, nothing else in the system can run.

## What Changes

- A new `/mission` skill that starts a mission from a goal description
- The orchestrator enters Claude Code plan mode and writes a durable operating manual
- The orchestrator grills the user for alignment using `/grill-me`
- The orchestrator establishes shared terminology using `/ubiquitous-language`
- The orchestrator creates OpenSpec artifacts via `/opsx:propose` (proposal, design, tasks)
- The orchestrator writes a `contract.md` with validation assertions before any workers start
- The orchestrator creates the `mission/` directory structure with `config.yaml` and `progress.md`
- The orchestrator presents the plan, contract, and task breakdown for human approval before proceeding

## Capabilities

### New Capabilities
- `mission-start`: The `/mission` skill entry point -- accepts a goal, runs the spec phase, produces all planning artifacts, and gets human approval
- `orchestrator-plan`: The Claude Code plan template that serves as the orchestrator's durable operating manual, referencing OpenSpec artifacts and mission state
- `validation-contract`: The `contract.md` artifact -- assertions grouped by task, written during planning, consumed only by validators
- `mission-directory`: The `mission/` directory structure with config.yaml and progress.md templates

### Modified Capabilities

(none -- this is a new project)

## Impact

- New files: `.claude/skills/mission/SKILL.md` (contract.md guidance and mission directory structure are embedded in the skill's instructions, not separate files)
- Dependencies: existing skills (`/grill-me`, `/ubiquitous-language`, `/opsx:propose`), Claude Code plan mode
- No code changes to other projects -- this is self-contained within the darkfactory repo
