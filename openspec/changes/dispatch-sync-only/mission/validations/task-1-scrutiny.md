# Scrutiny Validation: Task 1

## Result: PASS

## Deterministic Pre-passes

- **Detected language:** Rust (`tools/darkfactory-dispatch/Cargo.toml`; no root `Cargo.toml`).
- **File sizes** (touched production `.rs` files, `#[cfg(test)]` blocks included): `src/event.rs: 348`, `src/main.rs: 110`, `src/supervisor.rs: 329`. None exceed 900 lines.
- **Error handling smells:** 0 matches in 3 changed production lines (0 per KLOC). The 3 added `src/` lines all live inside the `#[cfg(test)]` module of `event.rs`; the only `.expect(...)` uses carry a non-empty message ("emit heartbeat"), a safe pattern. Locations: none.
- **Test runner:** 76 passed, 0 failed, 0 ignored (100.0% pass rate). The project's `cargo test` wrapper summarizes as `cargo test: 76 passed (4 suites, 22.26s)`; exit code 0.

## Gate Results
| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | PASS | Diff modifies 2 test files (`tests/engine.rs`, `tests/supervisor.rs`) plus the `#[cfg(test)]` module in `src/event.rs`. Pure dead-code removal: it deletes the tests of deleted APIs and rewires surviving tests off removed helpers. Test files present, gate satisfied. |
| Gate 2: Build | PASS | `cargo build` from `tools/darkfactory-dispatch/` succeeded (68 crates compiled, exit 0). |
| Gate 3: Tests | PASS | `76 passed, 0 failed, 0 ignored (100.0% pass rate)`. Actual run rate 100%, no WARNING. |
| Gate 4: Lint | PASS | `cargo clippy -- -D warnings` -> "No issues found" (exit 0). `cargo fmt --check` clean (exit 0). |
| Gate 5: Code review | PASS | No bugs, security, or logic issues. See findings below. |
| Gate 6: Module depth | PASS | Interface narrows (7 public/internal APIs deleted, 0 added). No file exceeds 900 lines. Deep-module safe-harbour case. |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files in diff. |
| Gate 8: Code standards | PASS | 0 smells (0 per KLOC, under 5 threshold). Deletion removes dead `#[expect(dead_code)]` shims; no back-compat code, fallbacks, or defensive cruft introduced. |
| Gate 9: Code simplify | PASS | Diff is itself a simplification (net -192 lines). No remaining duplication or dead branches introduced. |
| Gate 10: Security review | N/A | Diff touches no auth, secrets, user input, network, or filesystem trust boundaries. It deletes a SIGTERM/SIGKILL helper but the surviving signal paths (`kill_soft`, `kill`) are unchanged. |
| Gate 11: Contract checks | PASS | All 3 contract checks pass (table below). |
| Gate 12: Behavioral conformance | PASS | Contract checks verified by execution (grep, build, test, clippy). |
| Gate 13: Silent skip detection | PASS | No `is_err()`/`env::var`/`else { return }` conditional-return patterns in added test lines (grep exit 1). |
| Gate 14: Diff coverage | PASS | 100% of trackable added production lines covered. |
| Gate 15: Test quality review | PASS | Surviving test rewires preserve assertion strength; deleted tests targeted only deleted APIs. See notes. |

## Contract Checks
| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | `grep -rnE "fn graceful_stop\|fn heartbeat_ticker\|struct CancellationToken\|fn emit_heartbeat\b\|StreamMode::(Silent\|Verbose)\|StdinMode::Piped\|fn pid\b" src/` returns no matches | PASS | execution | grep over `tools/darkfactory-dispatch/src/` returned no lines (exit 1). All 7 named dead APIs are gone. |
| 2 | `cargo build` and `cargo test` both succeed | PASS | execution | `cargo build` exit 0 (68 crates); `cargo test` exit 0, 76 passed / 0 failed. |
| 3 | `cargo clippy -- -D warnings` passes | PASS | execution | clippy exit 0, "No issues found". |

## Critical preservation check (live heartbeat path)
**PASS.** The live production heartbeat was NOT removed:
- `emit_heartbeat_with_interval` still defined at `src/event.rs:166` and called from `src/engine.rs:205`.
- The inline `heartbeat_interval` select arm is intact at `src/engine.rs:203` (`_ = heartbeat_interval.tick() => { emitter.emit_heartbeat_with_interval(...) }`).

Only the convenience wrapper `emit_heartbeat` (which hard-coded the default interval) and the unused `heartbeat_ticker` spawn helper were deleted. The constant `DEFAULT_HEARTBEAT_INTERVAL_SECS` removed from `event.rs` is a dead duplicate; the live default of the same name lives in `src/types.rs` and remains in active use across `cli.rs`/`engine.rs`.

## Code review findings
- **No dangling references.** Grepped src/ and tests/ for all 7 removed symbols. The only residual matches are unrelated: the surviving `DEFAULT_HEARTBEAT_INTERVAL_SECS` in `types.rs`/`cli.rs`/`engine.rs` (a separate live constant), and `cmd.arg(...)` in `tests/cli.rs` (clap/`assert_cmd` API, not the deleted `SupervisorBuilder::arg`).
- **Test rewiring is sound.** `tests/supervisor.rs::wait_reports_sigterm_and_sigkill` swaps the deleted `graceful_stop(...)` for `kill_soft()` + `wait()`, still asserting `signal == SIGTERM`. The `.arg("300")` -> `.args(["300"])` swaps are forced by the deletion of `SupervisorBuilder::arg` and preserve behavior.
- **`should_emit` collapse is correct.** With `StreamMode` reduced to a single `Normal` variant, `should_emit` now returns `true` unconditionally and its parameter is correctly renamed `_event_type`. No caller depends on the removed filtering.

## Diff coverage detail
Added production (non-test) source lines, after exclusions: `src/event.rs:195` (`fn should_emit(&self, _event_type: &str)`) and `src/event.rs:197` (`StreamMode::Normal => true`). Tarpaulin 0.35.4: line 195 = 3 hits (covered); line 197 is a literal-return match arm not tracked as an executable line. 1/1 trackable added line covered = 100% (>= 80% threshold).

## Sandbox note
This environment's `/tmp` refuses to exec copied scripts, so all cargo commands ran with `TMPDIR=tools/darkfactory-dispatch/target/tmp` and `CARGO_HOME=tools/darkfactory-dispatch/target/cargo-home`, matching how the worker ran. No gate was failed on the sandbox limitation; all gates executed cleanly under the writable paths.

## Failed Checks
None.
