# Review: contract.md

Phase: 5
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 2 (2 MEDIUM) — all applied at source

Codex (gpt-5.5) reviewed contract.md against coverage-audit.md, tasks.md, and all three specs.

- [MEDIUM] Task 3 cited `contract-render/6` (a render scenario) but Task 3 only parses → moved `contract-render/6` to Task 4 (where the closing-task render check verifies it); Task 3's Specs comment now describes it as foundational parsing producing the fragment source.
- [MEDIUM] Task 8's "cargo build reports no unused-dependency warnings" is not reliably observable (cargo build doesn't report unused deps; the repo has no udeps tool) → replaced with a grep check for leftover `todo!()`/`unimplemented!()`/`#[allow(dead_code)]`, alongside the existing `clippy -D warnings` gate.

## Testability pre-flight
- execution: ~36, output: ~8, inspection: 0 (0%). Under the 20% inspection cap.
- No checks were flagged as too vague to script.
- (Breakdown to be echoed into progress.md when the mission directory is created in Phase 6.)

Human pre-approved the contract (post-fixes).
