# Review: PRD (#200, Darkfactory 2 Stage 1 — contract renderer)

Phase: 3
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 6 (1 CRITICAL, 3 HIGH, 2 MEDIUM) — all addressed in issue #200

Codex (gpt-5.5) reviewed the Stage 1 PRD against UBIQUITOUS_LANGUAGE.md and EPIC #199. All findings were addressed by editing #200 before human approval:

- [CRITICAL] Not every contract check traces to a spec scenario — the mandatory closing-task checks (smoke test, cleanup, docs, module-shape, CLAUDE/ARCH) have no spec source, so a one-check-per-scenario renderer could not reproduce a real contract. Fixed with a two-source model: spec-derived checks rendered from acceptance, operational checks sourced from a standing closing-task fragment.
- [HIGH] "Missing acceptance is an error" conflicts with v1 writing only the minimum check set → a scenario without acceptance now renders nothing (not an error); fail-loud narrowed to dangling task link, duplicate id, or a feature task with zero checks.
- [HIGH] Spec Reference comment grammar undefined → specified: collect a task's references, dedupe, order by capability then N, single comment, `(DEVIATED)` marking.
- [HIGH] Stable Spec Reference deferred but required → invariants pinned in the PRD (stable across insertion, duplicate = error, archived refs migratable); concrete mechanism remains a Design decision (human-confirmed split).
- [MEDIUM] Source model drifted from canonical "requirement record" → made explicit that Stage 1 realizes the record inside the scenario (transitional).
- [MEDIUM] Inspection cap named but unassigned → renderer warns (non-fatal) when a task's [inspection] checks exceed 20%.

Human approved PRD #200 after fixes.
