# Scrutiny Validation: Task 6

## Result: PASS

Final cleanup task for the synchronous-only dispatcher. Adds two CLI tests, removes
single-variant `StreamMode`, dedupes status-string handling into
`DispatchStatus::as_str()`, removes a stale `#[expect(dead_code)]` and three
unconstructed `DispatchError` variants, and corrects a stale note in `CLAUDE.md`.
Commit `41b43e7`, diff range `05a1c67..41b43e7`.

## Deterministic Pre-passes

### Named facts

- `Detected language: Rust` (Cargo.toml at tools/darkfactory-dispatch/)
- `File sizes: src/cli.rs: 177, src/engine.rs: 1110, src/event.rs: 329, src/main.rs: 63, src/types.rs: 205` (standalone tests/ excluded)
- `Error handling smells: 0 matches in ~10 changed production lines (0 per KLOC). Locations: none`
- `Test runner: 79 passed, 0 failed, 1 ignored (98.8% pass rate)` (lib 20 + cli 16 + engine 36 + supervisor 7 passed, 1 supervisor ignored)

Note: the added production lines are minimal (the `as_str()` method plus one-line
call-site swaps in cli.rs and engine.rs and an import/signature change). No added
production line matches any flagged Rust smell pattern. The 6 `.expect("...")`
occurrences in the new test code use descriptive messages (safe form) and are in a
standalone test file, so they are excluded.

## Gate Results
| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | PASS | tests/cli.rs adds two new `#[test]` functions with real assertions; tests/engine.rs updated. |
| Gate 2: Build | PASS | `cargo build` via `make check` compiled clean with the TMPDIR/CARGO_HOME sandbox workaround. |
| Gate 3: Tests | PASS | `79 passed, 0 failed, 1 ignored (98.8% pass rate)`. The 1 ignored is `parent_death_helper_spawns_supervised_child` (pre-existing `#[ignore]`, unrelated). Actual run rate 98.8% (over 90%), no WARNING. |
| Gate 4: Lint | PASS | `cargo fmt --check` exit 0; `cargo clippy --all-targets -- -D warnings` exit 0. No dead/orphaned references after removals. |
| Gate 5: Code review | PASS | Subagent review: clean. Removed `DispatchError` variants truly unconstructed; `StreamMode` removal changes no output/exit; `as_str()` returns identical strings; both call sites are mechanical swaps. |
| Gate 6: Module depth | PASS | Change narrows interface (removes an enum, an emitter param, a dead gate; consolidates two free fns into one method). engine.rs is 1110 lines but this task touched only 10 of them (removing a duplicate helper); pre-existing, not introduced or worsened here. Interface narrows, no leaks, no premature split. Safe-harbour: interface narrowed while hidden behaviour consolidated. |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files in diff. |
| Gate 8: Code standards | PASS | Error-handling smell density 0 per KLOC (under 5). No backwards-compat shims, no fallback lookups, no belt-and-suspenders, no feature flags for dead code. This task deletes dead code rather than gating it, matching project rules. |
| Gate 9: Code simplify | PASS | Subagent /simplify: consolidation is complete and idiomatic. No residual duplication or dead code. `emit_dispatch_end` staying stringly-typed is defensible (uniform event-sink boundary); `"timed_out"` at engine.rs:661 is a distinct `DispatchErrorInfo.code`, not the status-name duplication. |
| Gate 10: Security review | N/A | Diff touches no auth, secrets, user input, network, or filesystem trust boundary. Pure internal refactor plus tests and a doc comment. |
| Gate 11: Contract checks | PASS | See Contract Checks table. |
| Gate 12: Behavioral conformance | PASS | Ran the real binary against the mock: both new tests pass by execution (`json_failed_dispatch_... ok`, `plain_dispatch_output_... ok`). |
| Gate 13: Silent skip detection | PASS | No `else { return }`, `is_err() { return }`, or env-var conditional returns in added test lines. |
| Gate 14: Diff coverage | PASS | Meaningful added production lines are covered: `as_str()` (types.rs 5/5 in tarpaulin; all three arms exercised across the suite), engine.rs:188 call site (engine.rs 348/364), cli.rs:147 plain-output call site exercised by the new `plain_dispatch_output` test. tarpaulin's 0/49 on cli.rs is an `assert_cmd` instrumentation artifact (separate binary), not a real gap. Effective diff coverage ~100%. |
| Gate 15: Test quality review | PASS | Subagent /test-quality: no findings. Both new tests parse output from the real compiled binary, assert on dispatcher-derived values (not inputs), and would fail under plausible regressions in exit-code mapping, JSON serialization, or the plain formatter. |

## Contract Checks
| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | From `tools/darkfactory-dispatch/`, `make check` (fmt `--check` + clippy `-D warnings` + test) is green | PASS | execution | `make check` = fmt + lint + test, all green; fmt exit 0, clippy `-D warnings` exit 0, 79 passed / 0 failed / 1 ignored. No orphaned imports or leftover scaffolding. |

## Additional Verifications Requested

- **Exit-1 test genuinely asserts behavior:** PASS. `json_failed_dispatch_exits_1_and_reports_worker_exit_code` runs the dispatcher with `CODEX_MOCK_MODE=failed` (mock writes output then `exit 1`). Asserts dispatcher exit `Some(1)` and parses the emitted JSON for `status:"failed"`, `exit_code:1`, `signal:null`, `error.code:"process_failed"`. These are values the dispatcher derives from the child's real non-zero exit, not test inputs. Non-tautological.

- **Plain-output test genuinely asserts behavior:** PASS. `plain_dispatch_output_reports_status_duration_and_output` omits `--json` (the only test hitting the plain formatter), asserts exactly 3 stdout lines (`status: completed`, numeric `duration_ms` parsed as `u64`, `output: <path>`), exit 0, and output file content `mock output\n`. Parses duration rather than matching a literal. Non-tautological.

- **`StreamMode` removal is behavior-preserving:** PASS. It was single-variant (`Normal`) gating `should_emit()`, which always returned `true`. Removing it means `emit()` always writes, identical to before. No emitted output or exit behavior changes. `main.rs` and both test constructors updated to drop the argument; clippy `-D warnings` clean confirms nothing dangles.

- **Removed `DispatchError` variants genuinely unconstructed:** PASS. `grep` finds no `DispatchError::TimedOut|ProcessFailed|ParseError` anywhere in src/ or tests/. `map_supervisor_error` folds all non-`BinaryNotFound` supervisor errors into `DispatchError::Other`; every production construction site uses only `BinaryNotFound`, `SpawnFailed`, or `Other`. Timeout/process-failure/parse outcomes surface through the separate `DispatchStatus` enum and `DispatchErrorInfo` JSON, both untouched. The lone `TimedOut` reference (engine.rs:660) is `DispatchStatus::TimedOut`, a different enum, still live.

- **`as_str()` dedupe produces same status strings:** PASS. The old `status_name()` in cli.rs and engine.rs was byte-for-byte identical; `DispatchStatus::as_str()` reproduces the same three arms (`completed`/`timed_out`/`failed`), consistent with the enum's serde `rename_all = "snake_case"`. Both call sites (cli.rs:147 plain output, engine.rs:188 `emit_dispatch_end`) are mechanical swaps. Smoke test and result JSON see the same strings.

- **No regression:** PASS. Full suite (lib, cli, engine, supervisor) passes with the workaround: 79 passed, 0 failed, 1 pre-existing ignored.

## Failed Checks

None.

## Notes

- The critical sandbox workaround was required and applied: `make check` and the new
  CLI tests fail environmentally without `TMPDIR="$PWD/target/tmp"` and
  `CARGO_HOME="$PWD/target/cargo-home"` set from `tools/darkfactory-dispatch/`. With
  the workaround, everything is green, including the two new tests run by name.
- `CLAUDE.md` correctly drops "stream mode filtering" from the event.rs description,
  matching the removal.
