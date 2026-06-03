# Scrutiny Validation: Task 3

## Result: PASS

## Validation method note
Deterministic gates (1,2,3,4,8,13) run directly by the orchestrator (reproducible from commit bf06a9d0). Judgment gates (5,6,9,15) and contract-check coverage (11,12,14) performed by an independent Claude validator subagent (had not seen proposal.md; design.md consulted only for the core::parse boundary). Findings consolidated here.

## Deterministic Pre-passes (named facts)
- Detected language: Rust
- File sizes: src/core/parse.rs grew by ~275 lines (now holds parse_specs + parse_tasks + parse_fragment + validate_closing_kinds + helpers + tests)
- Error handling smells: 0 in production code; 0 per KLOC
- Test runner: 35 passed, 0 failed, 0 ignored (100.0% pass rate)

## Gate Results
| Gate | Result | Notes |
|------|--------|-------|
| Gate 1: Test presence | PASS | 6 new unit tests + an integration test file (closing_task_fragment.rs) added |
| Gate 2: Build | PASS | `make check` exit 0 |
| Gate 3: Tests | PASS | 35 passed, 0 failed, 0 ignored (100%) |
| Gate 4: Lint | PASS | clippy -D warnings clean; fmt clean |
| Gate 5: Code review | PASS | Independent subagent probed malformed input (unclosed bracket, overflow task number, empty method bracket, orphan checks, duplicate Closing) — no panics, all slice indexing guarded. Existing parse_specs untouched (only a 2-line import changed). Documented edges (content before first heading silently dropped; last Closing wins) are intentional. |
| Gate 6: Module depth | PASS (note) | `parse_tasks`/`parse_fragment` match the design exactly. `validate_closing_kinds` is a 4th public fn on `core::parse`, which design.md pins at 3; per design D5 the UnknownClosingKind detection arguably belongs in Task 4's renderer. One justified, narrow, tested function is not systematic widening, so PASS — but the PLACEMENT is a deviation. CARRY-FORWARD: Task 4 decides whether render owns the detection (move it / make it private), and Task 9 (module-shape re-check) reconciles design.md vs the code. |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files |
| Gate 8: Code standards | PASS | 0 production smells; `bracketed` reused (no duplicate bracket logic); no anti-patterns |
| Gate 9: Code simplify | PASS | Small single-purpose helpers; idiomatic; no dead code |
| Gate 10: Security review | N/A | Pure in-memory string parsing |
| Gate 11: Contract checks | PASS | All 4 checks covered with exact-value/exact-variant assertions (table below) |
| Gate 12: Behavioral conformance | PASS | Verified by running `cargo test`; integration test pins the on-disk template and a real 11-task tasks.md |
| Gate 13: Silent skip detection | PASS | No silent-skip patterns in the new tests (the openspec_available hits are Task 2's already-fixed fail-loud `assert!`) |
| Gate 14: Diff coverage | PASS | Main paths covered; a few documented edges untested (see Gate 15) — not a fail |
| Gate 15: Test quality | PASS (P2) | Assertions check real values, no tautologies, low coupling. P2 gaps (non-blocking): no test for multiple `Closing:` lines (last-wins), `Closing:` before first heading dropped, empty-name heading, or a phantom-task negative (e.g. `Depends on: Task 1` not creating a task). Behaviors verified by subagent probe; just untested. |

## Contract Checks
| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | Parsing tasks.md extracts each task's number, name, and `Closing: <kind>` where present | PASS | execution | `parse_tasks_extracts_number_and_name_from_a_feature_task`, `parse_tasks_handles_all_checkbox_prefixes_and_closing_kind`, and integration `real_tasks_validate_against_the_committed_fragment` (real 11-task file) |
| 2 | Parsing closing-task-checks.md yields kinds mapped to their checks | PASS | execution | `parse_fragment_yields_kinds_mapped_to_verbatim_checks` (exact verbatim check vectors), `committed_fragment_declares_the_five_mandatory_kinds` |
| 3 | templates/closing-task-checks.md exists and declares smoke-test, cleanup, module-shape, claude-arch, documentation | PASS | output | File exists; `committed_fragment_declares_the_five_mandatory_kinds` asserts parsed names equal exactly those five (via include_str! of the on-disk file) |
| 4 | A task whose `Closing: <kind>` is absent from the fragment returns UnknownClosingKind | PASS | execution | `validate_closing_kinds_rejects_a_kind_absent_from_the_fragment` asserts `Err(UnknownClosingKind("bogus"))`; positive control `..._accepts_a_fragment_known_kind` |

## Failed Checks
None.

## Recommendations
None blocking. Carry-forwards:
- **Task 4 / Task 9 (placement):** decide whether `validate_closing_kinds` (UnknownClosingKind detection) should live in the renderer (per design D5) rather than `core::parse`; reconcile design.md's `core::parse` 3-function boundary in the Task 9 module-shape re-check.
- **Test gaps (Stage 2 / opportunistic):** add tests for multiple `Closing:` lines, a `Closing:` before the first heading, an empty-name heading, and a phantom-task negative.
