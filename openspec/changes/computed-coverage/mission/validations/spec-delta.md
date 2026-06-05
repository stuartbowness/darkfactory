# Spec Delta Check

## Result: CLEAN

Built code on branch `mission/computed-coverage` (HEAD `9f0bf79`) matches the
coverage audit. All 24 COVERED requirements are present and plausible in
`tools/darkfactory-render/`. No GAPs and no APPROVED-DEVIATIONs were in the
audit, and none were discovered in the code.

## Coverage Verification
| # | PRD Requirement | Audit Status | Verified | Notes |
|---|----------------|-------------|----------|-------|
| 1 | Computed gap report runs in shadow mode, gap for every uncovered scenario (US1) | COVERED | YES | `coverage` subcommand added alongside `render`/`shadow`; `analyze` (coverage.rs:28) reports a gap for every uncovered scenario, additive only (no cutover) |
| 2 | Lists each uncovered scenario with capability, identifier, behavior text (US2) | COVERED | YES | `CoverageGap` carries capability, spec_ref, title, behavior (types.rs:253); `render_report` emits identifier + behavior bullets (coverage.rs:110-140) |
| 3 | Each gap tagged with its Gap kind (US3) | COVERED | YES | `GapKind` enum NoAcceptance/DanglingTaskLink/ClosingTaskLink (types.rs:240); `classify` assigns distinct kinds (coverage.rs:67-94); `- Gap kind:` line emitted |
| 4 | One-line summary total/covered/gaps (US4) | COVERED | YES | `## Summary` line `<total> scenarios, <covered> covered, <gaps> gaps` (coverage.rs:113-119) |
| 5 | Tolerate incomplete input, report gaps instead of erroring (US5) | COVERED | YES | `analyze` is infallible; unresolved/closing links become gap kinds, not errors (coverage.rs:8, 75-84) |
| 6 | Strict Renderer fail-loud behavior unchanged (US6) | COVERED | YES | render.rs diff is additive only (skip acceptance-less scenarios with no entries); render/shadow still fail loud on malformed input; smoketest Tests 1-3 confirm unchanged render/shadow output |
| 7 | Distinct exit code for gaps vs tool broke (US7) | COVERED | YES | `EXIT_GAPS=3` (cli.rs:43) distinct from `EXIT_USAGE`/`EXIT_RENDER_ERROR`=2 and `EXIT_DIFFERENT`=1; `run_coverage` returns 3 on gaps, 0 clean (cli.rs:151-175) |
| 8 | `coverage` subcommand runnable by hand against any change (US8) | COVERED | YES | `coverage` dispatched in `run` (cli.rs:71); takes `--specs`/`--tasks`, prints report to stdout |
| 9 | Deterministic, byte-identical output (US9) | COVERED | YES | sorted capability dir traversal (cli.rs:241), stable gap sort by capability+position (coverage.rs:50), pure `render_report`; integration test `coverage_is_byte_identical_across_two_runs` |
| 10 | Fixture-based equivalence tests per Gap kind + archived Stage 1 (US10) | COVERED | YES | coverage.rs tests cover NoAcceptance/Dangling/Closing/mixed; integration tests in tests/coverage.rs incl. migrated fixture; Stage 1 archived at openspec/changes/archive/2026-06-03-contract-renderer |
| 11 | Reuse existing spec parser so modes cannot disagree (US11) | COVERED | YES | both `render_pipeline` and `coverage_pipeline` call the same `parse_specs`/`read_scenarios` (cli.rs:189, 207, 247) |
| 12 | Land additively on main for future cutover (US12) | COVERED | YES | branch diff is additive (new coverage.rs, new subcommand, extended types); no removals of render/shadow behavior |
| 13 | Extend `parse_specs` to retain all scenarios + behavior text | COVERED | YES | `parse_specs` retains every scenario incl. acceptance-less (parse.rs:56-69); `behavior` Vec collected from WHEN/THEN/AND bullets (parse.rs:231-244, 288) |
| 14 | New coverage domain types CoverageGap/GapKind/CoverageReport | COVERED | YES | all three defined in types.rs:240-277 |
| 15 | Canonical output format pinned for determinism | COVERED | YES | `render_report` layout documented and pinned (coverage.rs:96-140); canonical-layout tests assert byte-exact output |
| 16 | Tolerance boundary: gaps vs exit-2 errors | COVERED | YES | dangling/closing/no-acceptance are gaps (analyze); duplicate task number and malformed/missing input are exit-2 errors (coverage_pipeline cli.rs:185-194) |
| 17 | Duplicate task number -> exit 2 | COVERED | YES | `check_duplicate_tasks` (coverage.rs:177) called in `coverage_pipeline`; fails loud -> mapped to `EXIT_RENDER_ERROR`=2; integration test `a_duplicate_task_number_is_a_parse_error` |
| 18 | Acceptance-carrying scenario without Spec Reference -> exit 2 | COVERED | YES | `parse_scenario` returns `MissingSpecRef` when acceptance present but no spec-ref (parse.rs:271); surfaces as exit-2; integration test `an_acceptance_carrying_scenario_with_no_spec_ref_is_a_parse_error` |
| 19 | Acceptance-less scenario without Spec Reference still reported (by title) | COVERED | YES | acceptance-less scenario retained with `spec_ref: None` (parse.rs:249-263); gap heading falls back to bare capability/title (coverage.rs:144-149); test `render_report_uses_bare_capability_when_a_gap_has_no_spec_ref` |
| 20 | Documentation: tool README + repo README + ARCH.md | COVERED | YES | tool README coverage section (README.md:159-217); repo README.md:141 + audit gate lines 84/120; ARCH.md:88-89, 97 |
| 21 | Mandatory closing tasks: smoke, cleanup, module-shape | COVERED | YES | smoketest.sh present (5 e2e checks incl. coverage gap/no-gap paths) with Makefile `smoke` target; branch is clean; all referenced fixtures exist |
| 22 | Unlinked acceptance line: `task_link` mandatory; malformed/absent link exits 2 | COVERED | YES | `task_link: u32` is non-optional (types.rs:113); a malformed `[task:N]` fails loud (parse.rs:368-369); acceptance-carrying scenario w/o spec-ref exits 2 |
| 23 | Partially-linked scenario is NOT a gap; unresolved sibling surfaces at render | COVERED | YES | `classify` returns covered (None) as soon as one entry resolves to a feature task (coverage.rs:82); test `one_valid_feature_link_covers_the_scenario_despite_an_unresolved_sibling` |
| 24 | `coverage` I/O contract: `--specs <dir>` + `--tasks <md>`, Gap report to stdout | COVERED | YES | `CoverageArgs` requires exactly `--specs` and `--tasks` (cli.rs:314-329); report written to `out`/stdout (cli.rs:169); USAGE line documents it (cli.rs:49) |

## Discrepancies (if any)
None.

## APPROVED-DEVIATION Summary
None. The audit declared no deviations, and the built code introduces none.
