# Review: specs/acceptance-field/spec.md

Phase: 4.2
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 4 (2 HIGH, 2 MEDIUM) — all applied at source

Codex (gpt-5.5) reviewed the three Stage 1 specs together (cross-spec). Findings scoped to acceptance-field, all applied:

- [HIGH] Acceptance entries could carry a Spec Reference different from their scenario, breaking migration/duplicate-detection/(DEVIATED) marking → entries now inherit the scenario's Spec Reference and MUST NOT set their own; only the `task` link, statement, and method may differ per entry.
- [HIGH] Empty/whitespace check statements were not rejected → added a fail-loud requirement and scenario for empty statements and for a present-but-empty `acceptance` list.
- [MEDIUM] "zero or more entries" collapsed the absent-field and empty-list cases (proposal says "one or more") → omitted `acceptance` is valid (no check); a present `acceptance` must have >=1 entry.
- [MEDIUM] Status grammar / missing-status behavior unspecified → status is one of proposed/approved/built/verified/deviated; omitted = non-deviated; unrecognized value fails loudly; Stage 1 acts only on `deviated`.

Mechanical `openspec validate`: PASS. Human approval pending.
