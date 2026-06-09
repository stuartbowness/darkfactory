# Spec Coverage Audit

PRD: GitHub issue #204. Every PRD requirement maps to at least one task. No gaps, no deviations.

## Coverage Table
| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | Computed gap report runs alongside the hand-authored coverage table in shadow mode (US1; replacement/cutover explicitly out of scope) | Coverage reports a gap for every uncovered scenario | Task 2, Task 4 | COVERED |
| 2 | Report lists each uncovered scenario with capability, scenario number/identifier, behavior text (US2) | Gap report content and determinism | Task 2, Task 3 | COVERED |
| 3 | Each gap tagged with its Gap kind (US3) | Each gap declares its gap kinds | Task 2, Task 3 | COVERED |
| 4 | One-line summary (total/covered/gaps) (US4) | Summary line; Canonical Gap report format | Task 2, Task 3 | COVERED |
| 5 | Tolerate incomplete input, report gaps instead of erroring (US5) | Tolerant of unresolved links, strict on malformed input | Task 2 | COVERED |
| 6 | Strict contract Renderer fail-loud behavior unchanged (US6) | Existing render and shadow behavior is unchanged | Task 1 | COVERED |
| 7 | Distinct exit code for "gaps found" vs "tool broke" (US7) | Coverage subcommand and exit codes | Task 4 | COVERED |
| 8 | `coverage` subcommand runnable by hand against any change (US8) | Coverage subcommand and exit codes | Task 4 | COVERED |
| 9 | Deterministic, byte-identical output (US9) | Gap report content and determinism; Canonical Gap report format | Task 3 | COVERED |
| 10 | Fixture-based equivalence tests per Gap kind + archived Stage 1 change (US10) | (test strategy) | Task 2, Task 4, Task 5 | COVERED |
| 11 | Reuse the existing spec parser so the two modes cannot disagree (US11) | (design: single extended parsed model) | Task 1 | COVERED |
| 12 | Land additively on main for a future cutover (US12) | (scope: additive, no cutover) | Tasks 1-8 | COVERED |
| 13 | Extend `parse_specs` to retain all scenarios + behavior text (impl decision) | Coverage reports a gap (considers every scenario); Gap report content (behavior text) | Task 1 | COVERED |
| 14 | New coverage domain types `CoverageGap`/`GapKind`/`CoverageReport` (impl decision) | (design Type Catalog) | Task 2 | COVERED |
| 15 | Canonical output format pinned for determinism (open item resolved in spec) | Canonical Gap report format | Task 3 | COVERED |
| 16 | Tolerance boundary: which conditions are gaps vs exit-2 errors (open item resolved) | Tolerant of unresolved links, strict on malformed input | Task 2, Task 4 | COVERED |
| 17 | Duplicate task number -> exit 2 (open item resolved) | Tolerant of unresolved links, strict on malformed input | Task 2, Task 4 | COVERED |
| 18 | Acceptance-carrying scenario without Spec Reference -> exit 2 (design review fix) | Tolerant of unresolved links, strict on malformed input | Task 1, Task 4 | COVERED |
| 19 | Acceptance-less scenario without Spec Reference still reported (by title) | Gap report content and determinism | Task 2, Task 3 | COVERED |
| 20 | Documentation: tool README + repo README + ARCH.md (closing) | (closing task) | Task 8 | COVERED |
| 21 | Mandatory closing tasks: smoke, cleanup, module-shape re-check (closing) | (closing tasks) | Task 5, Task 6, Task 7 | COVERED |
| 22 | Unlinked acceptance line: `task_link` stays mandatory (resolved — not optional); a malformed/absent link exits 2 (PRD open item) | Tolerant of unresolved links, strict on malformed input | Task 1, Task 4 | COVERED |
| 23 | Partially-linked scenario is NOT a gap; unresolved sibling links surface at render, not in coverage (PRD open item) | Coverage reports a gap (covered scenario with an unresolved sibling link) | Task 2 | COVERED |
| 24 | `coverage` subcommand I/O contract: `--specs <dir>` + `--tasks <md>`, Gap report markdown to stdout | Coverage subcommand and exit codes; Canonical Gap report format | Task 4 | COVERED |

## Gaps
None. Every PRD requirement maps to at least one task.

## Deviations Requiring Approval
None. No requirement is implemented differently from its spec.
