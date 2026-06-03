# Review: design.md

Phase: 4.3
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 5 (3 HIGH, 2 MEDIUM) — all applied at source

Design gate: six mandatory headings present; Architecture Decision has 3 scored candidates (A recommended 12/12, no zero dimensions, clear winner); `openspec validate` PASS.

Codex (gpt-5.5) reviewed design.md against the glossary, proposal, all three specs, and PRD #200. All findings applied:

- [HIGH] Task identity under-specified (acceptance linked by `N`, fragment matched by name) → D2 pins a single canonical task key (number `N`); D5 has tasks.md tag closing tasks with a fragment-known `Closing: <kind>`, joined by the renderer; added `UnknownClosingKind`.
- [HIGH] Primary fixture could not be reproduced — its Task 3 (README) is a feature task whose check has no spec source → D6: no third source; migration authors a scenario for such checks (or models them as a documentation closing task); fixture parity is equivalence (same checks/outcomes), not byte-identity.
- [HIGH] Scenario `status` had no authoring syntax → D4 pins `<!-- status: <value> -->` under the scenario heading beside `<!-- spec-ref: N -->`.
- [MEDIUM] Tied Spec References (multiple acceptance entries per scenario) had undefined order → D7 adds source-order tie-break (scenario file order, then entry order).
- [MEDIUM] `RenderedContract` was in the Type Catalog but `render()` returned `String` → render now returns `RenderedContract` with `to_markdown()`; Module Inventory and Integration Points updated.

Human approval pending.
