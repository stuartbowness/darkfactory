# Review: PRD (#204)

Phase: 3
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 6 (1 HIGH, 4 MEDIUM, 1 LOW)

Codex (gpt-5.5, read-only) reviewed PRD issue #204 against the glossary.

Addressed on the issue (HIGH):
- The current parser drops scenarios with no acceptance block and does not retain WHEN/THEN behavior text, so "no acceptance authored" gaps and the report's behavior column could not be produced. PRD now requires extending parse_specs to retain every scenario + behavior text, strict renderer unchanged; parse-level well-formedness stays fatal, only link resolution is tolerant.

Folded into the PRD's "Open design decisions" list (4 MEDIUM + 1 LOW), to resolve in design/spec:
- Scenario number derivation for no-acceptance scenarios.
- Gap kind precedence (or multiple kinds) when one scenario has several invalid links.
- Tolerance boundary: enumerate gap conditions vs. exit-2 parse errors; duplicate Task N -> exit 2.
- Canonical Gap report output format for byte-identical determinism tests.
