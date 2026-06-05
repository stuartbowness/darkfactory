# User-Test Validation: Task 2

## Result: PASS

## Test Strategy

Validated against the fixed commit `45e0efd5` (HEAD of `mission/computed-coverage`). Task 2 adds the internal `core::coverage` module (`analyze`, `check_duplicate_tasks`), the `GapKind` / `CoverageGap` / `CoverageReport` types, the `RenderError::DuplicateTaskNumber` variant, and a new `RequirementScenario.capability` field. There is no CLI, HTTP, or UI surface yet, so this is an infrastructure-only change. HTTP and browser testing do not apply.

Per the repo note, every contract check is demonstrable from the crate's unit tests, so I verified by building the crate and running its tests rather than marking anything SKIP. I ran cargo from `tools/darkfactory-render/` (no root Cargo.toml). I built it, ran the full suite (89 tests across 8 suites, 0 failures), ran the 10-test `coverage` subset, and ran the contract-relevant tests by name including the just-fixed one. The just-fixed check (#2) is exercised end-to-end through the real `parse_specs` parser (not a hand-built struct), so it proves the production path. I also read both `parse_specs` construction paths to confirm `capability` is set unconditionally, and read the `parse_tasks` body to confirm it has no de-duplication (the second half of check #7).

No checks were SKIPped; all seven are demonstrable.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo build` at `tools/darkfactory-render/` | Manual (cargo) | PASS |
| 2 | `cargo test` full suite (89 tests, 8 suites) | Manual (cargo) | PASS |
| 3 | `cargo test --lib coverage` (10 coverage tests) | Manual (cargo) | PASS |
| 4 | `acceptance_less_scenario_without_spec_ref_gaps_by_its_capability` (the fix) by name | Manual (cargo) | PASS |
| 5 | `check_duplicate_tasks` accept + reject (2 tests) by name | Manual (cargo) | PASS |
| 6 | `gaps_are_ordered_by_capability_then_document_position` by name | Manual (cargo) | PASS |
| 7 | `distinct_kinds_are_listed_once_each_in_fixed_order` by name | Manual (cargo) | PASS |
| 8 | `one_valid_feature_link_covers_the_scenario_despite_an_unresolved_sibling` by name | Manual (cargo) | PASS |
| 9 | `parse_tasks` tests still pass; `parse_tasks` body read, no de-dup | Manual + read | PASS |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | `analyze` reports a no-acceptance scenario as a gap with only kind "no acceptance" | PASS | `classify` returns `Some(vec![GapKind::NoAcceptance])` when `scenario.acceptance.is_empty()` (coverage.rs:68-70). Test `scenario_with_no_acceptance_entries_is_a_no_acceptance_gap` asserts `gaps[0].kinds == [NoAcceptance]`, `len == 1`, `covered == 0`. Passed. |
| 2 | `analyze` reports an acceptance-less scenario with no Spec Reference as a gap identified by its REAL capability (non-empty) and scenario title | PASS | The fix sets `capability: scenario.capability.clone()` in `analyze` (coverage.rs:37) and deleted the old `capability_of` helper that derived capability from spec-ref and defaulted to the empty string. `parse_specs` sets `capability` on the no-acceptance, no-spec-ref path (parse.rs:252-262). Test `acceptance_less_scenario_without_spec_ref_gaps_by_its_capability` drives real `parse_specs("widget", ...)` then `analyze` and asserts `gap.capability == "widget"` (non-empty), `gap.title == "A plain jotted scenario"`, `gap.kinds == [NoAcceptance]`, `gap.spec_ref.is_none()`. Passed. Capability is verified non-empty. |
| 3 | `analyze` reports a dangling-link-only scenario as a dangling gap and a closing-task-link-only scenario as a closing-task gap | PASS | `classify` sets `dangling` on an unmatched task number and `closing` on a task with `closing_kind.is_some()` (coverage.rs:80-81). Tests `scenario_linking_only_to_a_missing_task_is_a_dangling_gap` (kinds == [DanglingTaskLink]) and `scenario_linking_only_to_a_closing_task_is_a_closing_gap` (kinds == [ClosingTaskLink]). Both passed. |
| 4 | `analyze` does NOT report a scenario with at least one valid feature-task link, even with another unresolved entry | PASS | `classify` returns `None` (covered) on the first entry resolving to a feature task (coverage.rs:82). Test `one_valid_feature_link_covers_the_scenario_despite_an_unresolved_sibling` uses `[entry(99), entry(1)]` with only feature 1 present, asserts `gaps.is_empty()`, `covered == 1`. Passed. |
| 5 | A multi-reason gap lists each distinct kind once in fixed order | PASS | `classify` accumulates `dangling`/`closing` booleans then pushes `DanglingTaskLink` before `ClosingTaskLink`, each once (coverage.rs:86-92); `NoAcceptance` short-circuits earlier so the three never co-occur. Test `distinct_kinds_are_listed_once_each_in_fixed_order` (two dangling + one closing) asserts `kinds == [DanglingTaskLink, ClosingTaskLink]`. Passed. |
| 6 | Gaps ordered by capability ascending then document position | PASS | `analyze` stable-sorts `by capability.cmp().then(global_index.cmp())` (coverage.rs:50). Test `gaps_are_ordered_by_capability_then_document_position` inputs document order beta/1, alpha/2, alpha/1 and asserts output alpha/2, alpha/1, beta/1. Passed. |
| 7 | `check_duplicate_tasks` rejects duplicate task numbers while `parse_tasks` still accepts them | PASS | `check_duplicate_tasks` returns `Err(DuplicateTaskNumber(num, name))` on a repeated number, `Ok(())` otherwise (coverage.rs:105-117); tests `check_duplicate_tasks_rejects_a_shared_number` (asserts `DuplicateTaskNumber(1, "feature 1")`) and `check_duplicate_tasks_accepts_distinct_numbers` (asserts `Ok(())`) both passed. `parse_tasks` (parse.rs:85-101) pushes every task heading with no de-dup and always returns `Ok`, so it still accepts duplicates; both committed `parse_tasks` tests still pass. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

Not applicable; all seven contract checks PASS. Full suite: 89 passed, 0 failed.
