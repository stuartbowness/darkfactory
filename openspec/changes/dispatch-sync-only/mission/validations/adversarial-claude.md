# Adversarial Review (Reviewer B / Claude): dispatch-sync-only

Scope: whole-change review of the mission that made `darkfactory-dispatch` synchronous-only and rewired `/adversarial` to synchronous collection. Reviewed the full mission diff (`98a1d35..HEAD`), the current state of `tools/darkfactory-dispatch/src/`, `tools/darkfactory-dispatch/tests/`, and `skills/adversarial/SKILL.md`.

## Verification performed

- `cargo clippy --all-targets -- -D warnings`: clean, "No issues found". This is load-bearing: a stale `#[expect(dead_code)]` or any orphaned import/unreachable branch would fail this gate. It passed, so there is no dead code or stale suppression left by the removals.
- `cargo test`: 79 tests pass (20 unit + 16 cli + 36 engine + 7 supervisor), 1 helper intentionally `#[ignore]`d. All green.
- Note on environment: a raw `cargo test` in this sandbox reports 13 failures, but every one is an artifact of `/tmp` being mounted `noexec` (`rw,nosuid,nodev,noexec`). The tests copy the mock `codex` into `std::env::temp_dir()` (= `/tmp`) and exec it; the OS returns exit 126 (permission denied), so the dispatch legitimately reports `failed`. Re-running with an exec-capable `TMPDIR` under `$HOME` yields 79/79 green. This is not a code defect and not attributable to the change.
- Manually reproduced the missing-binary and failed-child exit-code paths; both match the contract.
- Confirmed `supervisor.rs` was functionally untouched by this mission (only dead test-only helpers removed in `07833a0`); the parent-death machinery predates the change and is unchanged.

## CRITICAL

None.

## HIGH

None.

## MEDIUM

None.

## LOW

- **Redundant `error:` message on the engine-error path** (`src/main.rs:44-46`). On a failed dispatch start, `run()` calls `eprintln!("error: {error}")` and then returns `Ok(code)`, so the outer `main` does not print again — good, no duplication. But `cli::print_result(&result, true)` at line 44 hardcodes `json=true`, so the `failed` JSON always goes to stdout even when the user did not pass `--json`. For the missing-binary case this is intentional and matches the `/adversarial` contract (the skill parses `.status` unconditionally). It is a deliberate behavior change from the base (base returned only a stderr error string with no stdout JSON in non-`--json` mode), and the `missing_engine_binary_emits_failed_json_and_cli_error_exit` test locks it in. Flagging only so it is a conscious choice: a caller running without `--json` now receives a JSON object on stdout for spawn failures but a plain-text block for normal completion. Not a bug; mildly inconsistent stdout shape across the two exit paths.

- **`--heartbeat` value is validated twice with divergent behavior** (`src/cli.rs:59-60` vs `src/cli.rs:89-91` vs `src/engine.rs:789-795`). Clap sets `--heartbeat` default to `DEFAULT_HEARTBEAT_INTERVAL_SECS`, `dispatch_spec()` rejects `0` with a CLI error, and `effective_heartbeat_sec()` in the engine *also* coerces `0` back to the default. The engine coercion is now unreachable for CLI callers (the CLI already rejected `0`), but it is still exercised by engine unit tests that build a `DispatchSpec` directly. Harmless belt-and-suspenders, not dead code (tests reach it). Noted only because the two layers disagree on how `0` is handled (reject vs. default), which could confuse a future maintainer.

## What the change got right (spot-checked against the brief)

- **Timeout state machine intact** (`src/engine.rs:56-161`). Removing the watchdog `select!` arm did not disturb the soft-sleep / hard-sleep escalation. `soft_armed` still gates the SIGTERM branch, `hard_sleep` is still armed at soft timeout, and `timed_out` is derived correctly (`!soft_armed` on clean-exit-after-SIGTERM at line 107, `true` on the hard-kill branch at line 149). `two_phase_timeout_escalates_from_sigterm_to_sigkill` and `soft_timeout_reports_timed_out_after_sigterm_exit` cover both edges.

- **Heartbeat preserved** (`src/engine.rs:152-159`, `src/event.rs:158-170`). The heartbeat `select!` arm survives; `emit_heartbeat_with_interval` is the single retained entry point. Removing the unused `emit_heartbeat`, `heartbeat_ticker`, `CancellationToken`, and `StreamMode` did not touch the live path. `custom_heartbeat_interval_controls_heartbeat_rate` and `heartbeat_events_are_emitted_during_running_dispatch` pass.

- **Process-group teardown intact** (`src/supervisor.rs:126-142`, `233-274`). `setpgid(0,0)` in pre_exec and `kill(-pgid, ...)` in `signal_group` are untouched. `kill_terminates_the_whole_process_group` passes.

- **Parent-death is genuinely always-on.** `engine.rs:43` hardcodes `.parent_death_watch(true)`, `SupervisorBuilder::new` defaults the field to `true` (`supervisor.rs:71`), and the old `spec.store_dir.is_none()` gate is gone. On Linux both the child (pre_exec `prctl` at `supervisor.rs:133-134`) and the dispatcher itself (parent-side `prctl` at `supervisor.rs:325`) arm PDEATHSIG; on macOS the kqueue watcher thread is spawned. `parent_death_terminates_supervised_child` passes. There is no longer any async path that could disable it.

- **Exit codes correct.** `exit_code_for_result` maps completed→0, failed/timed_out→1 (`cli.rs:153-158`). `exit_code_for_engine_error` maps `BinaryNotFound`→2, all other engine errors→1 (`main.rs:58-63`). The spawn-error `failed` JSON is emitted on stdout in the unified sync path (`main.rs:41-47`) with the right exit code. `json_failed_dispatch_exits_1_and_reports_worker_exit_code` and `missing_engine_binary_emits_failed_json_and_cli_error_exit` lock both in.

- **No orphaned references.** Grep for every removed symbol (`store_dir`, `StreamMode`, `LiveStatus`, `watchdog`, `control.json`, `status.json`, `graceful_stop`, `heartbeat_ticker`, `CancellationToken`, `status_name`, `tracing`, `prune`, lifecycle subcommands) returns only legitimate `async fn` hits and the deliberate negative tests that assert `--async` and `status`/`wait`/`result` are now rejected by the parser. The `#[expect(dead_code)]` on `main.rs`'s `supervisor` mod and the `event`/`types` inner-attribute suppressions were removed; the one surviving `#[expect(dead_code)]` in `tests/engine.rs:20-23` has an accurate reason and is validated by the clean clippy run.

- **`/adversarial` synchronous collection is correct** (`skills/adversarial/SKILL.md:60-124`). The original cross-Bash-call bug is gone: the dispatch, `.status` parse, prompt-file cleanup, and the `REVIEW_COMPLETE`/`REVIEW_FAILED` decision all happen inside one backgrounded Bash script, so nothing relies on a shell variable surviving between separate tool calls. Step 4 reads the Codex report only after seeing a `REVIEW_COMPLETE` line (success), and treats `REVIEW_FAILED` or a `.status` of `failed`/`timed_out` as failure without claiming a report. The `--json` result and `--log`-captured stderr are correctly separated (`--log "$LOG_PATH" ... --json 2>>"$LOG_PATH"`), and the mutually-exclusive `--async`/`--json` guidance was removed to match the collapsed CLI.

- **Retained flags and result schema unchanged.** `--grace`/`--heartbeat`/`--output`/`--cwd`/`--log` all still parse and flow into `DispatchSpec` identically (`cli.rs:53-135`); `DispatchResult` fields are untouched in `types.rs`. `retained_flags_are_accepted_and_output_is_reported` and `log_file_combines_worker_output_and_redirected_dispatcher_events` confirm the log capture and JSON shape are preserved.

## Overall assessment

The change is sound: supervision is fully preserved, exit codes and the spawn-error JSON path are correct, no dead code or stale suppressions remain (clippy `-D warnings` clean, 79/79 tests green), and the `/adversarial` rewrite eliminates the cross-call shell-variable bug while reading the Codex report only on success — I found no CRITICAL/HIGH/MEDIUM defects, only two LOW consistency notes.
