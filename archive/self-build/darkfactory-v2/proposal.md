## Why

Darkfactory was built by hand through an iterative grill-me session. We need to validate that the system can build itself -- that the specs, design decisions, and ubiquitous language are sufficient for workers to produce working skills independently. This also gives us a v1 vs v2 comparison to see where the autonomous process adds value or falls short.

## What Changes

- Create `darkfactoryV2/.claude/skills/mission/SKILL.md` -- the main orchestrator skill covering all 8 phases (plan mode, grill, ubiquitous language, propose, contract, mission directory, build loop, completion)
- Create `darkfactoryV2/.claude/skills/mission-resume/SKILL.md` -- resume interrupted missions from file state
- Create `darkfactoryV2/.claude/skills/mission-status/SKILL.md` -- read-only progress check
- All skills generated from specs and design decisions only, not from v1 skill files

## Capabilities

### New Capabilities
- `v2-mission-skill`: The rebuilt /mission skill generated independently from specs
- `v2-resume-skill`: The rebuilt /mission-resume skill generated independently from specs
- `v2-status-skill`: The rebuilt /mission-status skill generated independently from specs

### Modified Capabilities

(none -- v2 is a parallel output, v1 is not modified)

## Impact

- New files: `darkfactoryV2/.claude/skills/mission/SKILL.md`, `darkfactoryV2/.claude/skills/mission-resume/SKILL.md`, `darkfactoryV2/.claude/skills/mission-status/SKILL.md`
- v1 skills are NOT modified -- v2 lives in a separate directory for comparison
- Dependencies: existing OpenSpec specs (19 spec files across 6 changes), docs/design-decisions.md, UBIQUITOUS_LANGUAGE.md
