## ADDED Requirements

### Requirement: CLAUDE.md as mandatory closing task
The mission SHALL include a CLAUDE.md generation step as a mandatory closing task, ordered after module shape re-check and before documentation.

#### Scenario: Closing task order
- **WHEN** the orchestrator generates tasks.md with mandatory closing tasks
- **THEN** the closing tasks MUST be ordered: smoke test, cleanup, module shape re-check, CLAUDE.md + ARCH.md generation, documentation, CI configuration (optional)

### Requirement: CLAUDE.md created only if absent
The closing task SHALL create CLAUDE.md only if the project does not already have one.

#### Scenario: No CLAUDE.md exists
- **WHEN** the CLAUDE.md closing task runs and no CLAUDE.md exists in the project root
- **THEN** the worker MUST create CLAUDE.md from the template with 8 sections: Purpose, Rules, Commands, Repo Map, Testing, Environment, @ARCH.md reference, @UBIQUITOUS_LANGUAGE.md reference

#### Scenario: CLAUDE.md already exists
- **WHEN** the CLAUDE.md closing task runs and a CLAUDE.md already exists in the project root
- **THEN** the worker MUST NOT modify the existing CLAUDE.md

### Requirement: ARCH.md created only if absent
The closing task SHALL create ARCH.md from design.md only if the project does not already have one.

#### Scenario: No ARCH.md exists
- **WHEN** the CLAUDE.md closing task runs and no ARCH.md exists in the project root
- **THEN** the worker MUST create ARCH.md from the architecture decisions in design.md as a living document

#### Scenario: ARCH.md already exists
- **WHEN** the CLAUDE.md closing task runs and an ARCH.md already exists in the project root
- **THEN** the worker MUST NOT modify the existing ARCH.md

### Requirement: CLAUDE.md template follows best practices
The CLAUDE.md template SHALL follow Claude Code best practices for content inclusion and exclusion.

#### Scenario: Template includes guidance table
- **WHEN** the worker reads the CLAUDE.md template
- **THEN** the template MUST include an include/exclude guidance table: include commands Claude cannot guess, code style rules differing from defaults, testing instructions, architectural decisions, dev environment quirks, and common gotchas; exclude anything inferable from code, standard conventions, detailed API docs, file-by-file descriptions, tutorials, and self-evident practices

#### Scenario: Template is language-agnostic
- **WHEN** the worker reads the CLAUDE.md template
- **THEN** the template MUST contain one example with an "adapt to language" note rather than language-specific example blocks

### Requirement: CLAUDE.md rules include ARCH.md maintenance
The generated CLAUDE.md SHALL include a rule directing future sessions to update ARCH.md when architecture changes.

#### Scenario: Rules section content
- **WHEN** CLAUDE.md is created from the template
- **THEN** the Rules section MUST include "update ARCH.md when you change architecture"
