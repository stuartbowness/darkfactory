# Spec Coverage Audit

Proposal requirements extracted in document order: the 8 Approach bullets (1-8) then the 3 Capabilities (9-11).

## Coverage Table
| # | Proposal Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | Remove the async-dispatch subsystem (`--async`, `status`/`wait`/`result`, the `~/.darkfactory/dispatches/<ulid>/` store + 7-day pruning; pre-existing dirs left untouched) | dispatch-cli: "Async flag and lifecycle subcommands are removed" (rejected, non-zero, no JSON, no files) | Task 2 | COVERED |
| 2 | Remove the control-file abort channel (`control.json` watchdog) | dispatch-cli: "A planted control file cannot abort a dispatch" | Task 2 | COVERED |
| 3 | Remove the now-dead automatic `/tmp` status.json writes + the watchdog interval | dispatch-cli: "No automatic filesystem side effects" (no files under `/tmp/darkfactory-dispatch/`) | Task 2 | COVERED |
| 4 | Remove dead test-only code + delete their tests | no spec (internal dead code) | Task 1 | COVERED |
| 4b | Live `emit_heartbeat_with_interval` path NOT touched (heartbeats preserved) | synchronous-dispatch: "Heartbeat emission" | Task 5 (retained engine heartbeat tests pass) | COVERED |
| 5 | Drop `tracing` + `tracing-subscriber` deps (Cargo.toml + Cargo.lock) | no spec (build dependency) | Task 2 | COVERED |
| 6 | Convert `/adversarial` to synchronous `--json` + `run_in_background` + `BashOutput` | adversarial-sync-review: both requirements (sync pattern; correct REVIEW_COMPLETE/FAILED) | Task 3 | COVERED |
| 7 | Preserve all load-bearing supervision (process group, two-phase timeout, heartbeat, parent-death always-on, structured result incl. spawn-error JSON, `--log`, `--grace`/`--heartbeat`) | synchronous-dispatch: all 6 requirements (sync result, timeout, heartbeat, process-group, parent-death, log capture) | Task 2 (preserves while cutting; spawn-error JSON; retained-flag + pre-existing-file tests), Task 5 (retained timeout/process-group/heartbeat tests pass) | COVERED |
| 8 | Update `README.md` + tool `CLAUDE.md` (drop async/lifecycle docs, keep `tail -f` monitoring) | dispatch-cli: "Help text omits the removed surface" (for `--help`); README/CLAUDE.md prose has no spec | Task 4 | COVERED |
| 9 | Capability: `synchronous-dispatch` | synchronous-dispatch spec (6 requirements, 9 scenarios) | Task 2, Task 5 | COVERED |
| 10 | Capability: `dispatch-cli` | dispatch-cli spec (4 requirements, 7 scenarios) | Task 2 (behavior + retained-flag acceptance + no-files/pre-existing-file tests), Task 4 (help text) | COVERED |
| 11 | Capability: `adversarial-sync-review` | adversarial-sync-review spec (2 requirements, 3 scenarios) | Task 3 | COVERED |

## Gaps
None. Every proposal requirement maps to at least one task.

## Deviations Requiring Approval
None. This is a purely subtractive change with no substitutions or removals of promised behavior. (The one behavior-preservation detail — moving `failed_start_result` into the unified sync path — upholds requirement 7 and the synchronous-dispatch spec; it is not a deviation.)
