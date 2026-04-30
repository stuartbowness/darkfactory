# Issue: Task 1a - Update mission SKILL.md skill references (Attempt 1)

## Failed Checks

- Contract assertion 6: `grep -c "/mission-init" skills/mission/SKILL.md` returns 0, required ≥1

## Observed Symptoms

The SKILL.md never contained `/darkfactory-init`, so the 4th substitution was a no-op. No `/mission-init` reference exists in the file.

## Validator Recommendations

Add a `/mission-init` call at an appropriate location in `skills/mission/SKILL.md`.

## Human Decision

Add a `/mission-init` call to Phase 0, after the Darkfactory SHA check, to verify all required tools and symlinks are in place before starting the mission.
