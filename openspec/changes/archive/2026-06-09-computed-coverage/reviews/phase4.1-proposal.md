# Review: proposal.md

Phase: 4.1
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 3 (0 HIGH, 2 MEDIUM, 1 LOW)

Codex (gpt-5.5, read-only) reviewed proposal.md against the glossary and PRD #204. It explicitly confirmed keeping `computed-coverage` as the single new capability (with parser retention folded in, no Modified Capability) is correct.

All 3 addressed by proposal edits:
- MEDIUM: stated the exact CLI shape (`darkfactory-render coverage --specs <dir> --tasks <md>`, no `--fragment`).
- MEDIUM: added an Impact bullet requiring new fixture-based equivalence tests (one per Gap kind, no-gap case, exit codes 0/2/3, real Stage 1 archived change).
- LOW: replaced "acceptance records" with "Scenarios and their Acceptance task links" (glossary-canonical wording).
