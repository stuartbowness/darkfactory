# User-Test Validation: Task 2

## Result: PASS

## Test Strategy

This task delivers a Rust parser library (`core::parse::parse_specs`) with one
integration test that shells out to `openspec`. There is no HTTP API and no
browser UI, so I verified behavior by running the code:

- Ran the crate's own test binaries directly (bypassing a cargo output wrapper
  that summarizes results) to see every test name and result.
- Ran `openspec validate` (1.3.1) manually on the committed acceptance-carrying
  fixture to confirm exit 0 (check 6).
- Re-ran the `openspec` integration test with `openspec` removed from `PATH` to
  confirm the fix makes it fail loud instead of silently reporting `ok`.
- Wrote a throwaway external integration test against the public `parse_specs`
  API to independently exercise checks 1-5 with my own fixtures and assertions,
  then deleted it.

Environment confirmed: HEAD is `8c103c641eda8ca6a3242928c17dfcffd1d22d95`,
`openspec --version` reports `1.3.1`. The full suite is 27 tests across 4
binaries; all pass.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | Full crate suite (`cargo test`) | Manual (cargo) | PASS (27 passed) |
| 2 | Library unit tests run directly (25 parse/types tests) | Manual (test binary) | PASS |
| 3 | `openspec` integration suite run directly (2 tests) | Manual (test binary) | PASS |
| 4 | `openspec validate widget --type spec` on committed fixture | Manual (openspec CLI) | PASS (exit 0) |
| 5 | `openspec` integration test with `openspec` off PATH | Manual (test binary) | PASS (panics, exit 101 — fails loud) |
| 6 | Throwaway external test: checks 1-5 via `parse_specs` | Manual (test binary) | PASS (7 passed) |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | One acceptance entry exposes statement, method tag, task link; Spec Reference inherited from scenario | PASS | `parses_a_single_acceptance_entry` passes; my throwaway `check1` parsed `- [task:5] [inspection] ...` under `<!-- spec-ref: 9 -->` and asserted `task_link=5`, `method=Inspection`, verbatim statement, and inherited `gadget/9` (entry carries no own ref; scenario holds the single `SpecReference`). |
| 2 | Multiple acceptance entries yield independent entries that all share the scenario's Spec Reference | PASS | `parses_multiple_entries_with_independent_links_and_methods` passes; my throwaway `check2` parsed three entries (links 1/8/4, methods execution/output/inspection) all under one `spec-ref: 2`, asserting per-entry independence and the shared scenario ref. |
| 3 | A scenario that omits `acceptance` parses successfully and contributes no check | PASS | `acceptance_less_scenario_parses_and_contributes_no_entries` and `acceptance_less_scenario_does_not_require_a_spec_ref` pass; my throwaway `check3` confirmed an acceptance-less scenario yields an empty result. |
| 4 | Present-but-empty `acceptance` list, and empty/whitespace-only statement, each return a `RenderError` naming the offending scenario/entry | PASS | `present_but_empty_acceptance_list_fails_loud` -> `EmptyAcceptanceList("widget/7")`; `empty_statement_fails_loud_naming_the_offender` -> `EmptyStatement("widget/1")`; `whitespace_only_statement_fails_loud_naming_the_offender` -> `EmptyStatement("widget/5")` and asserts the message contains `widget/5`. My throwaway `check4` reproduced all three naming `gadget/3`, `gadget/6`, `gadget/7`. The fix changed `EmptyStatement` to carry the scenario identifier (`types.rs:177`, `parse.rs:219`). |
| 5 | Acceptance without `spec-ref` returns `MissingSpecRef`; two scenarios sharing a Spec Reference return `DuplicateSpecReference` | PASS | `acceptance_without_spec_ref_fails_loud` -> `MissingSpecRef`; `duplicate_spec_ref_within_capability_fails_loud` -> `DuplicateSpecReference("widget/3")`. My throwaway `check5` reproduced `MissingSpecRef` and `DuplicateSpecReference("gadget/4")`. |
| 6 | `openspec validate` on a fixture spec carrying an acceptance block exits 0 | PASS | Ran `openspec validate widget --type spec` on `tests/fixtures/acceptance_spec.md` in a minimal project: "Specification 'widget' is valid", exit 0. The integration test `acceptance_carrying_spec_passes_openspec_validate` passes; the fix replaced the silent early-return skip with an `assert!`, and running the test with `openspec` off PATH panicked (exit 101), proving it now fails loud rather than reporting `ok`. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A — all six contract checks pass.
