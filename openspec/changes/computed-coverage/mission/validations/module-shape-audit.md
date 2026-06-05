# Module Shape Re-check (Task 7)

Date: 2026-06-05
Auditor: orchestrator (audit task; no code commit, per the closing-task pattern)
Code as of: commit 13c6ec2 (Task 6).

Compared design.md's Module Inventory and Type Catalog to what the committed code exposes.

## Type Catalog — all present with matching names
- `GapKind` (enum: NoAcceptance / DanglingTaskLink / ClosingTaskLink) — `core::types`. PRESENT.
- `CoverageGap` — `core::types`. PRESENT (fields: capability, spec_ref, title, behavior, kinds).
- `CoverageReport` — `core::types`. PRESENT (gaps, total, covered).
- `RequirementScenario` (extended) — `core::types`. PRESENT. **Drift reconciled below.**
- `RenderError::DuplicateTaskNumber` — `core::types`. PRESENT.

## Module Inventory — all present
- `core::coverage` (new): `analyze`, `render_report`, `check_duplicate_tasks` — PRESENT; wired via `pub mod coverage` in `lib.rs`.
- `core::parse`: `parse_specs` extended (retains all scenarios, sets capability); `parse_tasks` left permissive — PRESENT.
- `core::types`: the new types above — PRESENT.
- `cli`: `coverage` subcommand (`CoverageArgs`, `run_coverage`, `coverage_pipeline`, `EXIT_GAPS=3`) — PRESENT.
- `core::render`, `core::shadow`: behaviorally unchanged (render/shadow tests + smoke byte-identical) — confirmed.

## Drift found and resolved
**RequirementScenario.capability** — the code carries a `capability: String` field (added in Task 2's fix so acceptance-less, no-spec-ref scenarios report under their real capability). The design.md Type Catalog entry for `RequirementScenario` did not list it. This is a **deliberate** change (required to meet spec scenario /11), so design.md was UPDATED to document the field (Type Catalog entry now lists `capability`). No code refactor needed.

## Other notes (non-drift)
- `capability` is intentionally also nested in `SpecReference.capability` (used by `render`); doc-noted in Task 6 as always equal to the scenario's `capability`. Not drift — a documented, invariant-linked duplication kept for render compatibility.

## Conclusion
No unintended drift. One deliberate addition (RequirementScenario.capability) reconciled by updating design.md. Module structure and type names match the design.
