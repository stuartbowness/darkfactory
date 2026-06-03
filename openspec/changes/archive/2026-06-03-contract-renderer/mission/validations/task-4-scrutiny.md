# Scrutiny Validation: Task 4

## Result: PASS

## Validation method note
Deterministic gates (1,2,3,4,8,13) run by the orchestrator (reproducible from commit a12d37f1). Judgment gates (5,6,9,15) and contract-check coverage (11,12) performed by an independent Claude validator subagent that read the actual asserted output (not just test names), traced determinism and the inspection-cap boundary, and ruled on the silent-fallback question. Findings consolidated here.

## Deterministic Pre-passes (named facts)
- Detected language: Rust
- File sizes: src/core/render.rs: 824 (incl. ~480 lines of #[cfg(test)] tests; ~290 lines of logic) — under the 900-line module-depth threshold
- Error handling smells: production code has one soft spot (render.rs:313 `unwrap_or(Execution)` — see Gate 8); no `.unwrap()`/`panic!`/`todo!` on external data
- Test runner: 51 passed, 0 failed, 0 ignored (100.0% pass rate)

## Gate Results
| Gate | Result | Notes |
|------|--------|-------|
| Gate 1: Test presence | PASS | 14 new core::render unit tests + 2 pipeline integration tests |
| Gate 2: Build | PASS | `make check` exit 0 |
| Gate 3: Tests | PASS | 51 passed, 0 failed, 0 ignored (100%) |
| Gate 4: Lint | PASS | clippy -D warnings clean; fmt clean |
| Gate 5: Code review | PASS | Subagent verified the subtle logic: sort key `(capability, N, scenario_index, entry_index)` correct + stable; Specs-comment dedup over the sorted run is correct (no needed ref dropped); `(DEVIATED)` marker lands on the right ref (per-scenario flag carried on each placed entry); closing-kind→fragment join correct; all four fail-loud errors raised before any output (no partial-contract bug); no panic risk on malformed input |
| Gate 6: Module depth | PASS | `core::render` deep behind a narrow interface (build_model + render + ContractModel opaque + to_markdown/warnings); all grammar/sort/dedup/join/cap logic in private helpers. build_model as a 2nd public entry (vs design's single `render`) is a defensible build-vs-project split, narrated in the module doc |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files |
| Gate 8: Code standards | PASS (with required follow-up) | One flagged item: render.rs:313 `VerificationMethod::parse(token).unwrap_or(VerificationMethod::Execution)` is a SILENT fallback for an impossible state (fragment methods are pre-validated by parse_fragment), and the chosen default (Execution) would *deflate* the inspection-cap count if ever reached. Mirrored dead branch at render.rs:257 `unwrap_or_default()`. Both are provably unreachable and correctness-neutral today (reviewer traced reachability char-by-char), so not a blocker — but they soften the fail-loud principle in the deep core. **MANDATORY CARRY-FORWARD to Task 8 (cleanup): replace both with fail-loud handling (`.expect("parse_fragment validated this")`, or the deeper fix of storing the parsed method on the fragment model).** No other anti-patterns. |
| Gate 9: Code simplify | PASS | Tight ~290-line logic; minor: the render.rs:257 dead defensive branch (folded into the Gate 8 follow-up); `placed_entries` re-flattened per task is O(tasks x entries), fine at these sizes |
| Gate 10: Security review | N/A | Pure in-memory model→string projection |
| Gate 11: Contract checks | PASS | All 12 checks covered with byte-exact expected-output assertions (table below) |
| Gate 12: Behavioral conformance | PASS | Verified by running `cargo test`; the format/ordering/DEVIATED/cap behaviors assert full expected strings |
| Gate 13: Silent skip detection | PASS | No silent-skip patterns in the new tests |
| Gate 14: Diff coverage | PASS | Every one of the 12 behaviors has an asserting test; the only untested branch is the unreachable render.rs:257 fallback (cannot be exercised without bypassing the upstream guard) |
| Gate 15: Test quality | PASS | Tests assert real rendered output and exact RenderError variants/payloads; no tautologies; low coupling (through the public API) |

## Contract Checks
| # | Check | Result | Method | Evidence (test) |
|---|-------|--------|--------|-----------------|
| 1 | Spec check under named `## Task N`, `- [ ] [method] stmt` | PASS | execution | `a_feature_task_check_renders_under_its_task_with_a_specs_comment` (exact body) |
| 2 | Check text verbatim (no rewording) | PASS | execution | `statements_render_verbatim_without_rewording` (backticks/brackets/% survive) |
| 3 | Ordered by Spec Reference, ties by source order | PASS | execution | `feature_checks_order_by_spec_reference_then_source_order` (a/3 before b/5; two a/3 keep source order) |
| 4 | One deduped+ordered `<!-- Specs -->` per task | PASS | execution | `a_deviated_scenarios_ref_is_marked_and_deduped_once` + ordering test |
| 5 | `status: deviated` → `(DEVIATED)` in Specs comment | PASS | execution | `a_deviated_scenarios_ref_is_marked_and_deduped_once` (`acceptance-field/4 (DEVIATED)`) |
| 6 | Closing checks under closing task, NO Specs comment | PASS | execution | `a_closing_task_renders_fragment_checks_with_no_specs_comment` + integration |
| 7 | Acceptance → closing task = AcceptanceLinksClosingTask | PASS | execution | `an_acceptance_entry_linking_to_a_closing_task_fails_loud` |
| 8 | Byte-identical on repeat render | PASS | execution | `repeat_render_is_byte_identical` (compares bytes + model equality) |
| 9 | Task sections in task-list order | PASS | execution | `tasks_render_in_task_list_order_not_link_order` (link order 3,1 → render 1,3) |
| 10 | Dangling link / zero-check feature / dup ref → error, no partial | PASS | execution | `a_dangling_task_link_fails_loud`, `a_feature_task_with_no_checks_fails_loud`, integration `duplicate_spec_reference_surfaces_through_the_full_pipeline` |
| 11 | Existing-format fidelity | PASS | output | Integration `parse_then_build_then_render_produces_the_expected_contract_shape` + format unit tests |
| 12 | `[inspection]` >20% warns; ≤20% does not | PASS | execution | `a_task_over_the_inspection_cap_warns` (50%), `a_task_at_the_inspection_cap_does_not_warn` (exactly 20%), `a_closing_tasks_inspection_cap_reads_the_fragment_method_tag` (100%); boundary is strict `>` with integer math |

## Failed Checks
None.

## Recommendations
- **MANDATORY (Task 8 cleanup):** replace the silent `unwrap_or(Execution)` at render.rs:313 and the dead `unwrap_or_default()` at render.rs:257 with fail-loud handling. This is recorded as a binding carry-forward to keep the deep core fail-loud.
- (Task 9 module-shape) reconcile design.md: `core::render` now exposes `build_model` in addition to `render`; and `validate_closing_kinds` placement (core::parse vs render) — decide and document.
