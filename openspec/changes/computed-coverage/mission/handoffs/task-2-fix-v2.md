# Worker Handoff: Task 2 - Coverage analysis (fix v2)

- Task: Task 2 - Coverage analysis (fix v2)
- Commit SHA: 45e0efd55c71d223cfe0ac7d0068556bb656b45a
- make check: PASSED (exit 0; fmt + clippy -D warnings + 90 tests all green)

## Approach

An acceptance-less scenario with no `<!-- spec-ref: N -->` comment parses to
`spec_ref: None`. Coverage derived a gap's capability from `spec_ref` only, so
those gaps reported the empty capability `""`. The fix carries the capability
on the scenario itself.

Followed `/tdd` red-green-refactor.

- RED: added a coverage test
  (`acceptance_less_scenario_without_spec_ref_gaps_by_its_capability`) that
  parses an acceptance-less, no-spec-ref scenario from capability `"widget"` via
  `parse_specs`, runs `analyze`, and asserts the resulting gap is a
  `NoAcceptance` gap identified by title `"A plain jotted scenario"` with
  capability `"widget"` (not `""`) and `spec_ref` still `None`. Confirmed it
  failed on the capability assertion (got `""`).

- GREEN: four code changes.
  - `src/core/types.rs`: added a `capability: String` field to
    `RequirementScenario`, documented as always-present even when `spec_ref` is
    `None`.
  - `src/core/parse.rs`: populate `capability: capability.to_string()` in BOTH
    `RequirementScenario` constructors in `parse_scenario` (the acceptance-less
    branch and the acceptance-carrying branch), using the `capability` argument
    `parse_specs` already receives. `spec_ref` left exactly as-is.
  - `src/core/coverage.rs`: `analyze` now sets the gap's capability from
    `scenario.capability.clone()` instead of `capability_of(scenario)`. Removed
    the now-dead `capability_of` helper that derived capability from `spec_ref`.
  - Updated the remaining `RequirementScenario` struct literals for the new
    field: the `scenario(..)` test helper and the spec-ref gap test in
    `coverage.rs`, and the `scenario(..)` test helper in `render.rs`.

- REFACTOR: none beyond deleting the obsolete `capability_of` helper.

## Render/shadow parity

Render is unaffected. It uses `spec_ref` for the `<!-- Specs: capability/N -->`
comments on acceptance-carrying scenarios (which always carry `Some(spec_ref)`).
The new `capability` field is not read by render. The
`tests/migrated_contract.rs` equivalence test and all render tests stay green,
so output is byte-identical.

## Commands run

- `cargo test acceptance_less_scenario_without_spec_ref_gaps_by_its_capability`
  -> RED (failed on capability assertion), then GREEN after the fix.
- `cargo test` -> 89 -> 90 passing (added one test).
- `make check` (from `tools/darkfactory-render/`) -> exit 0 (fmt clean, clippy
  clean with `-D warnings`, all tests pass).

## Staging

Staged only the four files under `tools/darkfactory-render/src/core/`
(coverage.rs, parse.rs, render.rs, types.rs). Did not stage the pre-existing
unrelated `UBIQUITOUS_LANGUAGE.md` modification, and did not stage anything under
`openspec/changes/computed-coverage/`. Committed once with the exact required
message.

## Concerns

- A pre-existing unstaged modification to `UBIQUITOUS_LANGUAGE.md` was present in
  the working tree on arrival (HEAD = e644153). It is unrelated to this fix and
  was left untouched.
