# Scrutiny Validation: Task 2

## Result: PASS

Commit under validation: `eece9ea` ("[task-2] Remove async subsystem and engine status/control"), HEAD.
Diff range: `07833a0..eece9ea`.
Crate: `tools/darkfactory-dispatch/` (no root Cargo.toml). All cargo commands run there with
`TMPDIR=$PWD/target/tmp` and a repo-local `CARGO_HOME` (sandbox `/tmp` refuses to exec copied
mock scripts; the worker documented this and it does not reflect a product defect).

## Deterministic Pre-passes

### Named facts

- `Detected language: Rust` (Cargo.toml at `tools/darkfactory-dispatch/`, no other build files).
- `File sizes: src/cli.rs: 185, src/engine.rs: 1118, src/main.rs: 63, src/types.rs: 229` (touched
  production `.rs` files; `tests/cli.rs` and `tests/engine.rs` excluded as standalone test files).
- `Error handling smells: 0 matches in 0 changed production lines (0 per KLOC). Locations: none.`
  The production `.rs` diff is deletion-only (568 deletions; the "20 insertions" in `--numstat` are
  re-indented `use` lines and collapsed bindings, none matching a smell pattern).
- `Test runner: 74 passed, 0 failed, 0 ignored (100.0% pass rate)` (cli 13 + engine 36 +
  supervisor 5 + lib unit 20; verified by running each freshly built test binary directly with the
  correct TMPDIR).

## Gate Results

| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | PASS | Diff modifies `tests/cli.rs` (+/-) and `tests/engine.rs`; new sync-path tests added (async rejection, subcommand rejection, single-JSON, no-store-files, seeded-files-unchanged, missing-binary, retained-flags). |
| Gate 2: Build | PASS | `cargo build` succeeds. |
| Gate 3: Tests | PASS | `74 passed, 0 failed, 0 ignored (100.0% pass rate)`. Actual run rate 100%. Stale pre-change test binaries in `target/debug/deps` initially showed a 5-failure `cli-*` artifact; after `rm` of stale deps and rebuild, the current suite is all-green. A plain `cargo test` under default `/tmp` fails 4 cli tests because the copied mock `codex` script cannot exec there and the real codex on PATH shadows it -- an environment artifact, not a regression. |
| Gate 4: Lint | PASS | `cargo clippy --all-targets -- -D warnings`: no issues found. `cargo fmt --check` clean. |
| Gate 5: Code review | PASS | Independent architect review: no critical/warning/nit defects. Supervision intact; removed watchdog `select!` arm never touched the timeout state machine; spawn-error move correct; no dangling references to removed symbols; no new injection/secret surface (codex args are `Vec<OsString>`, no shell). |
| Gate 6: Module depth | PASS | `engine.rs` is 1118 lines (>900). `CLAUDE.md` states engine is intentionally one deep module ("Engine owns ALL dispatch logic... prefer fewer entry points... do not extract for testability"). Change narrows the interface (removes `store_dir`, `LiveStatus`, watchdog/control machinery) while preserving hidden behaviour -- deep-module-preserved safe harbour. No interface widening, no leaking, no premature split. |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files in diff. |
| Gate 8: Code standards | PASS | Error-handling smell density 0 per KLOC (<5 -> PASS). No backwards-compat shims, no `// removed` comments, no re-exports of removed types, no fallback lookups, no dead-code feature flags. `_parent_death_watch` in supervisor.rs is a live variable moved into a `pre_exec` closure and read on Linux -- and supervisor.rs is untouched by this diff. |
| Gate 9: Code simplify | PASS | Simplify subagent found only pre-existing quality nits (duplicated `status_name`, stale `#[expect(dead_code)]`, `StreamMode` now single-variant in untouched event.rs, `grace`/`heartbeat` double-normalization). None are defects and none are in scope for this removal task. |
| Gate 10: Security review | PASS | Diff touches process spawning and filesystem (removes predictable `/tmp/darkfactory-dispatch` status/control writes). Reviewed in Gate 5: removal shrinks the predictable-path write surface; codex args are non-shell `OsString`; output materialization still uses `create_new`/O_EXCL. No new trust-boundary risk. |
| Gate 11: Contract checks | PASS | All 12 checks PASS (table below). |
| Gate 12: Behavioral conformance | PASS | 9 behavioral checks executed against the built binary (tests A-H below); all match the contract. |
| Gate 13: Silent skip detection | PASS | No env-guarded early returns or `else { return; }` inside test fns; no `#[ignore]`. `env::var` calls are PATH save/restore in setup, not skip conditions. |
| Gate 14: Diff coverage | PASS | Tarpaulin: engine.rs 354/369 (95.9%), event.rs 72/77, supervisor.rs 94/103; cli.rs 0/54 and main.rs 0/28 are a subprocess-instrumentation artifact (CLI runs via `assert_cmd` in a child process tarpaulin cannot see). The changed CLI/main lines are exercised end-to-end by 13 named cli.rs integration tests plus behavioral tests A-H. Effective added-line coverage of changed lines is high; the 0% is a measurement limitation, not untested code. |
| Gate 15: Test quality review | WARN | No P0. 2 P1 (both pre-existing gaps, not introduced here): no CLI-level test for the exit-1 `failed`-dispatch path, and the non-`--json` human-readable branch is untested. 1-2 P1 = WARNING, not a fail. P2/P3: fragile shared-`/tmp` fixture, loose disjunction assertions. |

## Contract Checks

| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | `--async --prompt x` rejected: exit 2, "unexpected argument"/usage on stderr, no `async_started`, no JSON on stdout, no dispatch files | PASS | execution | Test A: exit 2, stdout empty, stderr `error: unexpected argument '--async' found` + Usage. |
| 2 | `status X`, `wait X`, `result X` each rejected as unrecognized subcommand: exit 2 (not runtime "dispatch not found"), no JSON | PASS | execution | Test B: each exits 2, stdout empty, stderr `unexpected argument '<sub>' found`; no "dispatch not found". |
| 3 | Normal `--json` dispatch (no user log/output) creates no files under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/` | PASS | execution | Test C: exit 0, one `completed` JSON; both paths absent afterward. |
| 4 | Files seeded under `/tmp/darkfactory-dispatch/` and `~/.darkfactory/dispatches/` are byte-identical after a dispatch | PASS | execution | Test D: cksums unchanged (tmp 3801065979/17, home 2729553272/18); only seed.txt present, no run-created files. |
| 5 | grep for removed symbols returns no matches | PASS | execution | `grep -rnE "control\.json|write_status_json_atomic|status_dir|store_dir|LiveStatus|enum Commands|async_mode" src/` exits 1 (no matches). |
| 6 | Spawn failure (engine binary not found) -> exactly one `failed` JSON on stdout, exit 2; any other dispatch failure -> `failed` with exit 1 | PASS | execution | Test F: 1 JSON `status:"failed"` `startup_failed`, exit 2. Tests G/H: non-spawn failures exit 1. main.rs maps only `BinaryNotFound` to 2. |
| 7 | Worker exits 0 without creating requested `--output` -> `failed` JSON | PASS | execution | Test G: exit 1, `status:"failed"`, `error.code:"missing_output"`, `output_exists:false`. |
| 8 | Worker exits non-zero -> `failed` JSON carrying the exit code, dispatcher exits 1 | PASS | execution | Test H: worker exit 7 -> `status:"failed"`, `exit_code:7`, `process_failed`, dispatcher exit 1. |
| 9 | `--output P` writes artifact to P; result reports `output` P with `output_exists` true; `--cwd`/`--log`/`--grace`/`--heartbeat` accepted without parse error | PASS | execution | Test E: exit 0, `output` = P, `output_exists:true`, out.md written; all four extra flags accepted. |
| 10 | `Cargo.toml` declares no `tracing`/`tracing-subscriber`; neither is a direct dep in `Cargo.lock` | PASS | inspection | Cargo.toml deps: tokio, serde, serde_json, clap, nix, ulid, chrono, thiserror only. `grep tracing Cargo.lock` = 0 matches. |
| 11 | From the crate dir, `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all pass | PASS | execution | Build ok; tests 74/0/0; clippy no issues; fmt clean (with TMPDIR set so mock scripts exec). |
| 12 | Supervision preserved (two-phase timeout, heartbeat via `emit_heartbeat_with_interval`, process-group `setpgid`, parent-death now always-on) | PASS | inspection | `emit_heartbeat_with_interval` at event.rs:166 called engine.rs:154; `setpgid` supervisor.rs:127; `parent_death_watch(true)` engine.rs:43; two-phase soft-SIGTERM/hard-SIGKILL select arms intact (engine.rs:83-160). supervisor.rs/event.rs untouched by this diff. |

## Notes on non-blocking findings

- **Gate 15 (test quality) WARN, 2 P1:** both are pre-existing coverage gaps, not caused by this
  change. (1) No CLI-level test asserts the exit-1 `failed`-dispatch path end-to-end (the engine
  suite covers `failed` in-process; behavioral Tests G/H here confirm it via the binary). (2) The
  non-`--json` human-readable output branch (`print_result` else-arm) has no test. Under the rubric
  1-2 P1 findings are warnings and do not fail the task. Recommend the fix loop consider closing
  both if cheap.

- **Gate 14 (diff coverage) instrumentation artifact:** tarpaulin reports 0% for cli.rs and main.rs
  because their logic executes in `assert_cmd` subprocesses outside tarpaulin's instrumentation. The
  changed lines live almost entirely in those two files and are proven exercised by 13 named cli.rs
  integration tests and behavioral Tests A-H. Not a real coverage gap.

- **Simplify (Gate 9) nits, out of scope:** duplicated `status_name` in cli.rs and engine.rs; a
  stale module-level `#[expect(dead_code)]` in types.rs; `StreamMode` reduced to a single variant in
  the untouched event.rs; `grace`/`heartbeat` normalized in both cli.rs and engine.rs. None are
  correctness issues; all are cleanup candidates unrelated to the removal contract.

## Overall

Every gate passes or is a non-blocking WARN/SKIP, and all 12 contract checks pass by execution or
inspection. The change is a surgical removal of the async subsystem and engine status/control
machinery; supervision is fully preserved and the unified sync failure path is correct.

**PASS**
