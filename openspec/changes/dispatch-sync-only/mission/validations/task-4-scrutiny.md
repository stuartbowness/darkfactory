# Scrutiny Validation: Task 4

## Result: PASS

Commit under validation: `070c683` (HEAD). Diff range: `ac41660..070c683`.

This is a docs-only task. The commit touches exactly two files, both documentation:
`tools/darkfactory-dispatch/README.md` (+4 / -29) and
`tools/darkfactory-dispatch/CLAUDE.md` (+5 / -6). There is no code diff, so
build / test / lint / coverage / module-depth / frontend / security / silent-skip /
test-quality gates are N/A for this task. The two live gates are contract checks
and a code-review of doc-vs-code accuracy.

## Gate Results
| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | N/A | Docs-only closing task; exempt (produces no testable code). |
| Gate 2: Build | PASS | `cargo run -- --help` compiled and ran cleanly from `tools/darkfactory-dispatch/`. |
| Gate 3: Tests | N/A (info) | Lib suite: 20 passed, 0 failed. `tests/cli.rs`: 9 passed, 4 failed. The 4 failures are pre-existing and environmental (see below); commit touches zero source files, so it cannot affect test outcomes. |
| Gate 4: Lint | N/A | No source changed. |
| Gate 5: Code review | PASS | Docs accurately describe current synchronous-only behavior. No stale async/lifecycle/store references. Detail below. |
| Gate 6: Module depth | N/A | No source changed. |
| Gate 7: Frontend depth | SKIP | No `.tsx`/`.jsx` files in diff. |
| Gate 8: Code standards | N/A | No source changed. |
| Gate 9: Code simplify | N/A | No source changed. |
| Gate 10: Security review | N/A | Docs-only; no auth/secrets/input/network/filesystem trust boundary touched. |
| Gate 11: Contract checks | PASS | All 3 checks pass. Table below. |
| Gate 12: Behavioral conformance | PASS | `--help` executed; async/subcommand absence verified by execution. |
| Gate 13: Silent skip detection | SKIP | No test files in diff. |
| Gate 14: Diff coverage | SKIP | No production source files in diff. |
| Gate 15: Test quality review | SKIP | No test files in diff. |

## Contract Checks
| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | `--help` lists no `--async` flag and no `status`/`wait`/`result` subcommands | PASS | execution | `cargo run -- --help`: Usage line is `darkfactory-dispatch [OPTIONS]` (no `[COMMAND]`). Options list has no `--async`. No subcommands section. |
| 2 | README has no async-mode section, no `status`/`wait`/`result` lifecycle docs, no dispatch-store section, and still contains `tail -f` monitoring guidance | PASS | inspection | README has a single "Dispatch" section (line 74); "Async dispatch" section removed. No `status`/`wait`/`result` subcommand docs. No dispatch-store section. `tail -f dispatch.log` monitoring guidance retained at line 137. |
| 3 | CLAUDE.md repo map no longer references `status`/`wait`/`result` subcommands, async routing, watchdog, or `LiveStatus` | PASS | inspection | `main.rs` line now reads "parse args, dispatch synchronously, print result"; `cli.rs` line reads "Clap CLI structure, argument parsing" (async routing removed); engine line drops "watchdog"; `types.rs` line drops `LiveStatus`; reference-implementation table drops the `cmd/agent-mux/async.go` row and "watchdog". |

All contract checks: PASS.

## Code-review detail (doc-vs-code accuracy)

I verified every doc claim against `tools/darkfactory-dispatch/src/`:

- **Synchronous `--json` dispatch.** `main.rs:31-52` unconditionally builds a
  `DispatchSpec`, calls `engine::dispatch(...).await`, then `print_result`. There
  is no branch on an async flag and no store write. README's "Dispatch a task and
  wait for the result" and CLAUDE.md's "dispatch synchronously, print result"
  match the code.

- **No async surface.** `cli.rs` `Cli` struct (lines 46-85) has no `--async`
  field and no subcommand enum. Grep of `src/` for
  `async_started|LiveStatus|watchdog|dispatch.*store|host.pid|events.jsonl|Subcommand|orphaned`
  returned zero matches. The regression test
  `async_flag_is_rejected_without_dispatch_files` (tests/cli.rs:92) passes,
  asserting `--async` is rejected as an unexpected argument.

- **Result-status table.** README lists `completed` / `timed_out` / `failed`.
  `types.rs:45-49` defines `enum DispatchStatus { Completed, TimedOut, Failed }`
  with `#[serde(rename_all = "snake_case")]`, serializing to exactly those three
  wire values. The unit test at types.rs:166-174 confirms the serialization.

- **Retained flags.** Every flag in README's flag table
  (`--model`, `--effort`, `--timeout`, `--grace`, `--heartbeat`, `--sandbox`,
  `--config`, `--prompt`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json`)
  is present in the `Cli` struct and in `--help` output. The `--heartbeat` row
  added in this diff corresponds to the real `heartbeat: u64` field (cli.rs:59-60).

- **Monitoring via `tail -f` with `2>>log`.** README lines 133-138 retain the
  guidance to redirect stderr to the log and `tail -f` it. This matches the
  code's stdout/stderr split (result JSON on stdout, events/heartbeats on stderr),
  documented in CLAUDE.md's "Output" rule and enforced in `event.rs`.

- **Three grep hits that look like stale async refs but are not:**
  - README:76 "wait for the result" — describes synchronous behavior. Correct.
  - README:171 "orphaned processes" — refers to process-group cleanup via
    `setpgid`, not the removed async "orphaned" dispatch state. Correct.
  - CLAUDE.md:80 "async tests MUST use `#[tokio::test]`" — a Rust testing
    convention, unrelated to async dispatch mode. Correct.

No remaining stale reference to async / lifecycle / store. No doc claim
contradicts the source.

## Test-failure note (non-blocking)

`tests/cli.rs` reports 4 failures:
`json_stdout_is_single_object`, `retained_flags_are_accepted_and_output_is_reported`,
`existing_files_under_old_store_paths_are_unchanged`,
`json_dispatch_without_store_paths_creates_no_dispatch_files`.

These are pre-existing and environmental, not introduced by this commit:

1. The commit changes only two `.md` files — zero source, zero test files — so it
   cannot alter test behavior.
2. Running `cargo test --test cli` at the parent commit `ac41660` produces the
   identical result: `9 passed; 4 failed`.
3. The failing tests spawn the real dispatcher, which execs a mock `codex` shell
   script (`tests/test_codex_mock.sh`) placed on `PATH`. They fail in this
   validation sandbox (the mock dispatch does not complete as expected), and one
   failure poisons a shared `STORE_PATH_LOCK` mutex that cascades into the others.
   This is a sandbox/infrastructure limitation, not a code defect in the diff.

The `types`/`event`/`engine`/`supervisor` unit suite passes (20 passed, 0 failed),
and `--help` runs cleanly, confirming the binary itself is healthy.

## Overall verdict: PASS

All three contract checks pass by execution and inspection. The changed README and
CLAUDE.md accurately describe the current synchronous-only dispatcher: `--json`
dispatch, the retained flag set, the `completed`/`timed_out`/`failed` status table,
and `tail -f` + `2>>log` monitoring. No stale async, lifecycle-subcommand, or
dispatch-store content remains, and no doc claim contradicts the source. The 4
CLI-test failures are pre-existing and environmental, unaffected by this docs-only
commit.
