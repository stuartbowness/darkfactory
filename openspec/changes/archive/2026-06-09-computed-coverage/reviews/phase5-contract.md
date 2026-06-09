# Review: contract.md

Phase: 5
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 5 (1 HIGH, 2 MEDIUM, 2 LOW)

Codex (gpt-5.5, read-only) reviewed contract.md against the spec (scenarios 1..25) and tasks. All addressed:

- HIGH (uncovered scenario): scenario /11 (acceptance-less scenario with no Spec Reference -> reported as a gap by title) had no check. FIXED: added a Task 2 execution check; moved /11 to Task 2's Specs.
- MEDIUM (determinism too narrow): /10 says "produced twice from the same specs and task list" but the check re-rendered an in-memory object. FIXED: replaced with a Task 4 check running the full parse->analyze->render path twice on the same on-disk inputs and comparing stdout byte-for-byte; moved /10 to Task 4.
- MEDIUM (render-unchanged mixed execution/inspection): "without being modified / byte-identical" wasn't directly provable by cargo test. FIXED: split into [execution] (pre-existing render/shadow tests pass) + [output] (the task diff modifies no existing render/shadow test or golden fixture).
- LOW (archived fixture not spec-backed): kept as legitimate integration coverage (PRD US10); the task-level Specs comment does not claim 1:1 per check.
- LOW (module-shape inspection-in-disguise): FIXED: Task 7 split into [output] (audit file exists/records) + [execution] (crate compiles + `rg` confirms the type/subcommand names exist).

Final contract: 28 checks across 8 tasks. Testability pre-flight: 17 execution, 11 output, 0 inspection (0% <= 20% cap) — PASS.
