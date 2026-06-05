# Review: spec computed-coverage

Phase: 4.2
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 5 (2 HIGH, 2 MEDIUM, 1 LOW)

Codex (gpt-5.5, read-only) reviewed the spec against the proposal, PRD #204, the glossary, and the existing acceptance-field spec. It confirmed the "no acceptance-field delta" call holds (links stay mandatory, omitted acceptance already valid, render-time resolution unchanged), conditional only on the Spec Reference point below.

All 5 addressed by spec edits:
- HIGH (Spec Reference for acceptance-less scenarios): resolved by making coverage TOLERANT of a missing Spec Reference — identify such a scenario by capability + title and still report it as a gap, rather than erroring or requiring an acceptance-field delta. A missing Spec Reference is tolerated; a DUPLICATE within a capability is exit 2. Added a scenario for it.
- HIGH (canonical output not pinned for byte-identical tests): added a "Canonical Gap report format" requirement with the exact markdown layout, fixed gap-kind labels (`no acceptance`, `dangling task link`, `closing-task link`), the no-gaps `None.` form, and a full worked-example scenario (input -> exact output).
- MEDIUM (behavior text under-defined): defined as the verbatim ordered behavior bullets (`- **WHEN**`/`- **THEN**`/`- **AND**`) before `**Acceptance:**`, excluding comments and blank lines.
- MEDIUM (malformed task-link boundary vague): defined the grammar — exactly `[task:N]`, N a positive integer, no leading zeros, no surrounding whitespace; anything else exits 2. Added a malformed-link scenario.
- LOW (no-delta claim conditional): resolved once the Spec Reference point above was settled; remains a new computed-coverage capability with no acceptance-field MODIFIED delta.

Spec now has 8 requirements; `openspec validate` passes.
