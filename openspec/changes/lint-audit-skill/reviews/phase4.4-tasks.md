# Artifact Review: tasks.md

## Summary

4 findings across coverage gaps, ambiguity, terminology consistency, and contradictions.

## Findings

### [HIGH] Apply capability has no explicit implementation task
- **Dimension:** 5. Coverage Gaps
- **Quote:** "Task 3: Write SKILL.md workflow and RULES.md documentation"
- **Problem:** Prior specs require apply-all, skip-all, per-item review, oxlint config updates, tsconfig updates, and lint script updates. The task list only names audit detection/comparison and workflow docs. A worker could reasonably write instructions that present findings but never implement the config-apply behavior.
- **Suggestion:** Expand Task 3 or add a separate task that explicitly implements config-apply behavior for oxlint rules, tsconfig strictness settings, and `--type-aware` lint script changes.

### [MEDIUM] Ruleset version pin is missing from the data task
- **Dimension:** 5. Coverage Gaps
- **Quote:** "Produces: `skills/lint-audit/rules.json` -- array of rule objects with canonical names, categories, severities, default status, rationale"
- **Problem:** The specs require oxlint default status to be pinned to a specific oxlint version. `audit.sh` consumes `rules.json`, but Task 1 does not require a version field or provenance. That leaves the script with an unversioned source of truth.
- **Suggestion:** Require `rules.json` and `RULES.md` to include the pinned oxlint version, and require Task 3 to verify both files agree.

### [MEDIUM] Module shape re-check is not the closing task
- **Dimension:** 3. Contradictions
- **Quote:** "Task 6: Module shape re-check" and "Task 7: Update install.sh, README.md, ARCH.md"
- **Problem:** The glossary defines the Module shape re-check as a mandatory closing task that catches cumulative drift across all tasks. Here it runs before Task 7, which still changes integration code via `install.sh` and durable docs. Human review could see a module shape audit that does not represent the final change.
- **Suggestion:** Move Task 7 before Task 6, and make Task 6 depend on Task 7.

### [MEDIUM] Smoke test artifacts have no durable location
- **Dimension:** 1. Ambiguity
- **Quote:** "Produces: test harness script, edge case log with pass/fail results for 200 unique scenarios"
- **Problem:** The artifact does not say whether the harness and log live in `/tmp`, the mission directory, or the repo. If committed to the repo, Task 5 may delete them as temp artifacts. If written to `/tmp`, validators and human reviewers may not have durable evidence.
- **Suggestion:** Specify exact locations, for example a temp harness under `/tmp` and a durable summary under the mission directory.