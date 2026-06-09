# Scrutiny Validation: Task 2 (final, after fix v2)

## Result: PASS

(Validated at commit 45e0efd — Task 2 + fix v2. Persisted by the orchestrator from the validator transcript; the scrutiny subagent reported PASS but its file write did not land.)

## History
- Initial commit e644153: both validators returned PASS, but the orchestrator overrode — contract check #2 was only superficially met (acceptance-less no-spec-ref scenarios reported an empty capability, violating spec /11). Fix v2 dispatched.
- Fix commit 45e0efd: added `RequirementScenario.capability` (set by `parse_specs` for every scenario); `analyze` uses it. Both validators re-run.

## Gate Results (at 45e0efd)
| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | PASS | coverage tests incl. the new real-capability test. |
| Gate 2: Build | PASS | clean. |
| Gate 3: Tests | PASS | 89 tests, 0 failed. |
| Gate 4: Lint | PASS | clippy -D warnings clean. |
| Gate 5: Code review | PASS | clean; 3 nits only (below). |
| Gate 6: Module depth | PASS | core::coverage narrow interface. |
| Gate 7: Frontend depth | PASS | self-skip (no .tsx). |
| Gate 8: Code standards | PASS | no smells/debris. |
| Gate 9: Code simplify | PASS | none. |
| Gate 10: Security | PASS | N/A. |
| Gate 11: Contract checks | PASS | all 7 PASS; check #2 now reports the real capability ("widget"), verified end-to-end via parse_specs. |
| Gate 12: Behavioral conformance | PASS | verified by executing tests. |
| Gate 13: Silent skip | PASS | none. |
| Gate 14: Diff coverage | PASS | changed lines covered. |
| Gate 15: Test quality | PASS | no P0/P1. |

## Nits (advisory, non-blocking)
- `check_duplicate_tasks` is O(n^2) via `Vec::contains` — fine for task-list sizes; a `HashSet` would be O(n).
- `capability` is now stored on both `RequirementScenario.capability` and `SpecReference.capability` with no invariant linking them (always set from the same source in `parse.rs`, so they cannot diverge in the production path). Consider, in the Task 7 module-shape re-check / Task 6 cleanup, either a doc note on `SpecReference.capability` or dropping it from `SpecReference` since the scenario now owns the capability.
- `classify` relies on the `closing_kind.is_some()` match arm preceding the catch-all `Some(_)` arm — load-bearing ordering, currently correct.
