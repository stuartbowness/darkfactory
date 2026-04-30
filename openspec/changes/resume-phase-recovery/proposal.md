## Why

When a mission session ends during planning phases (1-7), `mission-resume` cannot recover to the correct phase. It always enters the Phase 8 build loop, skipping unfinished planning steps like contract writing and launch confirmation. The plan template lacks the granularity to distinguish planning phases, mission-resume ignores the existing plan file, and orphaned commit detection returns false positives from other branches.

## What Changes

- Add an `## Orchestrator Phase` field to the plan template that records the current major phase (0-9, 4a) with a timestamp. The mission orchestrator updates this field at every major phase gate.
- Add a `plan_file` field to mission/config.yaml so mission-resume can find the plan file directly.
- Add Phase 3c to mission-resume that detects the current orchestrator phase using the plan file as primary source, with progress.md marker scanning as fallback.
- Modify mission-resume Phase 4 to read and reconcile the existing plan file instead of re-deriving from scratch.
- Scope orphaned commit detection (Phase 3b) to the mission branch using config.yaml's `base_branch` and `git merge-base`.
- When mission-resume detects a planning phase, report the phase and ask the user what to do instead of entering the build loop.

## Capabilities

### New Capabilities
- `phase-tracking`: Recording and updating the orchestrator's current major phase in the plan file at every phase gate, and storing the plan file path in config.yaml for reliable lookup.
- `resume-recovery`: Detecting the current orchestrator phase during mission-resume (plan file primary, progress.md fallback), restoring and reconciling the plan file, and handling planning-phase resume by reporting to the user.

### Modified Capabilities

## Impact

- `skills/mission/templates/plan.md` -- new field added to template
- `skills/mission/SKILL.md` -- phase-update instructions added at ~10 phase gates, plan_file added to config.yaml template
- `skills/mission-resume/SKILL.md` -- new Phase 3c, modified Phase 4, modified Phase 3b
- No runtime code, dependencies, or API changes. All changes are to markdown instruction files.
