# User-Test Validation: Task 1

## Result: PASS

## Test Strategy

Task 1 is an internal model + parser change in the `darkfactory-render` Rust crate
(`tools/darkfactory-render/`). It extends `RequirementScenario` with `title` and
`behavior` fields, makes `spec_ref` an `Option<SpecReference>`, and changes
`parse_specs`/`parse_scenario` to retain acceptance-less scenarios instead of
dropping them. There is no new CLI, HTTP, or UI surface, so this is an
infrastructure-only / internal change: it cannot be exercised from outside via
HTTP or a browser.

I verified it the way the contract checks demand: build the crate and run its
test suite, then confirm the specific tests named in the contract exist, run, and
assert the required behavior. I ran the full suite via `cargo test`, then ran the
compiled test binaries directly (lib unit tests plus all integration binaries) to
read per-suite results and individual test names, since the contract checks call
out specific `render`/`shadow`/parser tests.

Build and test commands were run from `tools/darkfactory-render/` (no root
Cargo.toml).

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo test` (full suite, all 8 binaries) at commit `cafa105` | Manual (cargo) | PASS |
| 2 | Lib unit-test binary run directly: 60 passed, 0 failed (includes all `core::render::tests` and `core::shadow::tests`) | Manual (binary) | PASS |
| 3 | All `core::render::tests` (13) pass, incl. `repeat_render_is_byte_identical`, `statements_render_verbatim_without_rewording` | Manual (binary) | PASS |
| 4 | All `core::shadow::tests` (7) pass | Manual (binary) | PASS |
| 5 | Integration binaries: `closing_task_fragment` 2, `contract_render` 2, `migrated_contract` 3, `openspec_validate` 2, `cli` 10 — all pass | Manual (binary) | PASS |
| 6 | `acceptance_less_scenario_is_retained_with_title_and_behavior` asserts retention + title + verbatim behavior bullets | Manual (binary, by name) | PASS |
| 7 | `behavior_text_includes_and_excludes_comments_and_stops_at_acceptance` asserts AND captured, comments/blanks/acceptance excluded | Manual (binary, by name) | PASS |
| 8 | `acceptance_without_spec_ref_fails_loud` asserts `Err(MissingSpecRef)` for acceptance + no spec-ref | Manual (binary, by name) | PASS |

Full-suite result: `79 passed (8 suites, 0.99s)`. Direct binary tallies sum to the
same 79 (60 lib + 2 + 2 + 3 + 2 + 10). No failures anywhere.

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | [execution] `cargo test` passes, including every pre-existing `render` and `shadow` test | PASS | Full suite `79 passed`; lib binary shows all 13 `core::render::tests` and all 7 `core::shadow::tests` as `ok`. |
| 2 | [output] No existing `render`/`shadow` byte-output assertion or golden fixture weakened; mechanical accessor/helper updates for new `Option`/fields allowed | PASS | No fixtures in the diff. `render.rs` change is purely mechanical: a `let Some(spec_ref) = &scenario.spec_ref else { continue }` guard (the `None` case is acceptance-less and has no entries), `&scenario.spec_ref` → `spec_ref`, and a test-helper constructor adding `title`/`behavior`/`Some(...)`. `repeat_render_is_byte_identical` and `statements_render_verbatim_without_rewording` are unchanged and pass. |
| 3 | [execution] A test demonstrates `parse_specs` retains a scenario with no `**Acceptance:**` block and exposes its title and verbatim behavior bullets | PASS | `core::parse::tests::acceptance_less_scenario_is_retained_with_title_and_behavior` parses a scenario with no Acceptance block and asserts `scenarios.len() == 1`, `title == "Just a plain scenario"`, and `behavior == ["- **WHEN** something happens", "- **THEN** it is observed"]` verbatim. Ran by name: `ok`. |
| 4 | [output] `RequirementScenario.spec_ref` is `Option<SpecReference>`; parsing a scenario with acceptance but no Spec Reference still errors `MissingSpecRef` | PASS | `types.rs:122` declares `pub spec_ref: Option<SpecReference>`. `parse.rs:270` does `spec_ref_number.ok_or(RenderError::MissingSpecRef)?` only on the acceptance-carrying path. `core::parse::tests::acceptance_without_spec_ref_fails_loud` feeds acceptance + no spec-ref and asserts `Err(RenderError::MissingSpecRef)`. Ran by name: `ok`. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A — all four contract checks pass.
