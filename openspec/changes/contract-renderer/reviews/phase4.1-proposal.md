# Review: proposal.md

Phase: 4.1
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 3 (2 HIGH, 1 MEDIUM) — all applied at source

Codex (gpt-5.5) reviewed proposal.md against UBIQUITOUS_LANGUAGE.md, PRD #200, and EPIC #199. All three findings applied before human review:

- [HIGH] `acceptance` read as singular → reworded to "a list of one or more entries" so multi-check scenarios and Spec Reference collapse work. Also reflected in the `acceptance-field` capability.
- [HIGH] Stable Spec Reference invariants incomplete (only insertion) → added the full set: insertion does not renumber siblings, duplicate Spec References fail loudly, archived position-numbered references have a migration path.
- [MEDIUM] Scenario `status` had no home despite `(DEVIATED)` rendering → added `status` to the `acceptance-field` capability; Stage 1 reads `deviated` for the marker, full lifecycle out of scope.

Human approval pending.
