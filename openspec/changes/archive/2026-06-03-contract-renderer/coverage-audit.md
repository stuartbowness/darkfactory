# Spec Coverage Audit

Change: contract-renderer (Stage 1 of EPIC #199). PRD: https://github.com/layercodedev/darkfactory/issues/200.

## Coverage Table

| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | Author each spec-derived check once as scenario `acceptance` | acceptance-field: Scenario carries acceptance entries | Task 2 | COVERED |
| 2 | Acceptance entry carries method tag + task link + (inherited) Spec Reference + verbatim statement | acceptance-field: entries; verification method; links to a task; stable Spec Reference | Task 1 (enums), Task 2 (parse) | COVERED |
| 3 | A scenario without acceptance renders nothing (not an error) | acceptance-field: Scenario omits acceptance | Task 2, Task 4 | COVERED |
| 4 | Fail loud on schema inconsistencies (empty statement, empty list, missing/duplicate spec-ref, unknown method/status) | acceptance-field: fail-loud requirements | Task 1, Task 2 | COVERED |
| 5 | Stable Spec Reference: insertion does not renumber; duplicates fail | acceptance-field: Scenario has a stable Spec Reference | Task 2 | COVERED |
| 6 | Legacy position-numbered references are migratable | acceptance-field: Legacy position-numbered references are migratable | Task 6 | COVERED |
| 7 | Scenario `status`; `deviated` read for the marker; unknown fails loud | acceptance-field: Scenario carries a status | Task 1, Task 2 | COVERED |
| 8 | Deterministic renderer; byte-identical output; task-list + Spec Reference + source ordering | contract-render: Produce deterministic output; Render checks grouped by task | Task 4 | COVERED |
| 9 | Emit contract.md in the existing format | contract-render: Match the existing contract format | Task 4 | COVERED |
| 10 | Carry the check statement verbatim | contract-render: Render checks grouped by task (verbatim) | Task 4 | COVERED |
| 11 | One deduped/ordered Spec Reference comment per task; `(DEVIATED)` marking | contract-render: Render one Spec Reference comment per task | Task 4 | COVERED |
| 12 | Operational/closing-task checks sourced from the standing fragment; no Spec Reference | contract-render: Include operational checks from the closing-task fragment | Task 3, Task 4 | COVERED |
| 13 | Render fail-loud (dangling task link, acceptance→closing task, feature task zero checks, unknown closing kind) | contract-render: Fail loudly on inconsistency | Task 3 (unknown closing kind), Task 4 (render-time) | COVERED |
| 14 | Warn when a task's `[inspection]` checks exceed 20% | contract-render: Warn when inspection checks exceed the cap | Task 4 (compute), Task 5 (emit) | COVERED |
| 15 | Shadow mode renders + diffs an existing contract.md without modifying it | shadow-comparison: Shadow mode renders without replacing; Report the comparison as a diagnostic diff | Task 5 | COVERED |
| 16 | Never auto-trigger cutover | shadow-comparison: Never auto-trigger cutover | Task 5 | COVERED |
| 17 | Documented cutover gate + rollback procedure | shadow-comparison: Document the cutover gate and rollback | Task 11 | COVERED |
| 18 | Parity judged by validator outcomes (equivalence), not text | shadow-comparison (cutover gate) + design Migration | Task 6 (fixture equivalence test), Task 11 (documented gate) | COVERED (interpretation — see notes) |
| 19 | `openspec validate` still passes on a spec carrying acceptance | (design Test Strategy / acceptance-field syntax) | Task 2 | COVERED |
| 20 | Tool is a Rust crate under `tools/`, built/installed/tested like darkfactory-dispatch | (design Architecture Decision) | Task 1 | COVERED |
| 21 | Smoke test of the primary flow against the real binary | (mandatory closing task) | Task 7 | COVERED |
| 22 | Documentation: tool README + repo README update | (mandatory closing task; CLAUDE.md rule) | Task 11 | COVERED |
| 23 | ARCH.md updated for the new tool | (CLAUDE.md project rule) | Task 10 | COVERED |
| 24 | Worker never sees `acceptance` or the rendered contract | (preserved structurally) | -- | COVERED |
| 25 | v1 keeps shipping unchanged during shadow (Phase 5 untouched) | (scope guard / non-goal) | -- | COVERED |
| 26 | Fixes land on `acceptance` at source; never patch a generated file | (model property; enabled by the renderer + docs) | Task 5, Task 11 | COVERED |

## Gaps

None. Every PRD requirement maps to a task or is satisfied structurally.

| # | PRD Requirement | AI Recommendation | Human Decision |
|---|----------------|-------------------|----------------|
| -- | -- | -- | -- |

## Deviations Requiring Approval

None. The tasks implement the specs faithfully; no substitutions or removals.

| # | PRD Requirement | Classification | What spec says | What we're doing instead | Prerequisites met? | Impact | Human Decision |
|---|----------------|---------------|----------------|-------------------------|-------------------|--------|----------------|
| -- | -- | -- | -- | -- | -- | -- | -- |

## Notes on structural / no-task coverage

- **#24 (worker never sees acceptance/contract):** preserved by scope — Stage 1 does not modify the worker brief or wire the renderer into Phase 5, so nothing feeds acceptance or the contract to workers.
- **#25 (v1 keeps shipping):** the scope guard (no Phase 5 change) guarantees the live pipeline is untouched until the later, gated cutover.
- **#26 (fixes to source):** the renderer makes the contract a projection of `acceptance`, so the source-of-truth-then-re-render workflow is enabled by Tasks 4–5 and documented in Task 11; enforcing "never patch the generated file" is a usage discipline, not a build artifact.
- **#18 (validator-outcome parity):** Stage 1 delivers the single-mission equivalence test (Task 6) and the documented gate (Task 11). The full cross-mission parity measurement is operational and runs at cutover (a later change), per design Migration Plan — an interpretation, not a gap.
