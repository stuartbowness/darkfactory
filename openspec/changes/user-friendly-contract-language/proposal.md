## Why

Non-developers interacting with Darkfactory mission output (progress logs, PR bodies, validator reports, contract files) are confused by "assertions" appearing everywhere. The word carries developer-specific meaning from testing contexts. Meanwhile, the most natural replacement -- "check" -- was blocked because the ubiquitous language used "check" for scrutiny validator steps, creating an ambiguity that was resolved by banning "check" for contract items. A three-way terminology split resolves both problems.

## What Changes

- Rename contract items from "assertion" to "check" across all living source files (skills/, templates/, prompts/, docs/, UBIQUITOUS_LANGUAGE.md, README.md)
- Rename scrutiny validator steps from "check" to "gate" to free up "check" for contract items
- Reserve "assertion" exclusively for code test assertions (tautological assertion, test assertions in code)
- Update UBIQUITOUS_LANGUAGE.md with the new three-way split (already completed in Phase 2)
- Update all validator prompts, contract templates, worker brief templates, PR body templates, and documentation to use consistent new terminology

## Capabilities

### New Capabilities
- `contract-check-rename`: Rename all contract-item uses of "assertion" to "check" across skills, templates, prompts, and docs
- `validator-gate-rename`: Rename all scrutiny validator step uses of "check" to "gate" in validator prompts and related references

### Modified Capabilities

## Impact

- ~160 occurrences across ~15 living source files
- Bulk of changes (~50+) in skills/mission/SKILL.md (the main mission orchestrator skill)
- Validator prompts (scrutiny, user-test, adversarial) produce reports with renamed section headings
- Contract template items use new terminology
- PR body template and progress log language changes
- No runtime code changes -- darkfactory has no application code
- No breaking changes to external interfaces -- all changes are internal to skill definitions and templates
- Future missions will use new terminology; past missions retain historical terminology
