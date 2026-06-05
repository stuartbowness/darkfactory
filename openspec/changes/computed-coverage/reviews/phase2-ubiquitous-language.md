# Review: UBIQUITOUS_LANGUAGE.md

Phase: 2
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 8 (2 HIGH, 4 MEDIUM, 2 LOW)

Codex (gpt-5.5, read-only) reviewed the Stage 2 glossary additions.

Addressed by glossary edits (7 of 8):
- Defined three terms used-but-undefined: Feature task, Closing task, Scenario.
- Harmonized near-synonyms: "coverage gap report" -> Gap report; "acceptance records" -> Requirement records.
- Clarified Coverage audit (v1) vs Computed coverage (v2); both coexist in shadow mode (removed the wrong "avoid: coverage audit").
- Tightened the Gap report exit-code wording (gaps-found is a tolerant, non-failing result; exact codes in the spec).

Carried to design (Phase 4.3), NOT a glossary fix (1 of 8, HIGH):
- Partially-linked scenarios: a covered scenario whose other acceptance entries carry missing/dangling/closing-task links is not a gap under the scenario-level rule, so the design must define how those are surfaced. Ties to the optional-task-link decision.
