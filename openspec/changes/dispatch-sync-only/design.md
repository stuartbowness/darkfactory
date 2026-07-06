# Design: dispatch-sync-only

## Architecture Decision

This is an existing-codebase change with **no architecture change**, so scored candidates are not required (per the design template). The dispatcher's shape is preserved; the change is purely subtractive.

**Current architecture (preserved):** `darkfactory-dispatch` is one Rust binary with a single supervised path:

```
main.rs  ──parse──▶ cli.rs ──DispatchSpec──▶ engine.rs::dispatch()
                                               │ spawns + supervises one Codex subprocess
                                               ├─ supervisor.rs  (setpgid, signals, parent-death)
                                               └─ event.rs       (heartbeats + NDJSON → stderr)
                                               ▼
                                         DispatchResult (JSON on stdout)
```

The load-bearing supervision — process-group isolation, two-phase SIGTERM/SIGKILL timeout while draining output, 15s heartbeat emission, parent-death detection, NDJSON parse to a structured result — is sound and stays untouched.

**How the change fits:** the engine currently carries a second, optional path (an async lifecycle backed by a `store_dir` on the spec) plus a watchdog select-arm that reads a `control.json` and writes `status.json`. The change removes that second path and the watchdog arm, collapsing the engine to the single synchronous path that production already uses. Removing `store_dir` makes parent-death always-on (it was disabled only in async mode). No new modules or types are introduced.

Rationale for subtraction over restructure: the async surface has at most one (broken) consumer, duplicates Claude Code's own background primitive, and exposes a world-readable `/tmp` channel. Narrowing the existing modules is lower-risk than any re-layering.

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `main.rs` | narrow — process entry (`run()`) | sequencing of cli → engine, exit-code mapping | process lifecycle separate from dispatch logic |
| `cli.rs` | moderate — clap parser + `DispatchSpec` builder (retained flags only) | arg parsing/validation | CLI surface separate from the engine |
| `engine.rs` | narrow — `dispatch(spec) -> DispatchResult` | the async select loop, two-phase timeout sequencing, NDJSON draining | dispatch orchestration separate from OS-level control |
| `supervisor.rs` | narrow — `SupervisorBuilder`/`RunHandle` | `setpgid`, group signaling, parent-death (`PR_SET_PDEATHSIG`/kqueue) | OS/process-group details isolated in one place |
| `event.rs` | narrow — `Emitter` | event serialization and stderr writing | event/IO separate from dispatch logic |
| `types.rs` | data types | nothing (shared data vocabulary) | one home for the result/spec/status vocabulary |

What narrows in this change (no module added or removed): `cli.rs` loses `--async`, the `Commands` enum/subcommands, and the store helpers; `engine.rs` loses the watchdog arm (the control-file read and the in-loop status write) AND the separate post-dispatch status write — both `write_status_json_atomic` call sites — plus the `store_dir` branching, so `write_status_json_atomic`/`status_dir` are removed entirely and no status file is written at any point; `supervisor.rs` loses `graceful_stop`/`RunHandle::pid` (and the now-dead `RunState.pid` field)/`StdinMode::Piped`/singular `arg`; `event.rs` loses `CancellationToken`/`heartbeat_ticker`/`StreamMode::Silent|Verbose`/`emit_heartbeat` and its local heartbeat constant; and `types.rs` loses the `LiveStatus` status-store type (with its `cli.rs`/`engine.rs` imports and uses).

## Type Catalog

| Type name | Purpose | Owning module |
|-----------|---------|---------------|
| `DispatchSpec` | Inputs to one dispatch (command, timeout, grace, heartbeat, sandbox, log, output, cwd). The `store_dir` field is removed. | `types.rs` |
| `DispatchResult` | The structured result emitted on stdout (status, exit code, signal, duration, tokens, activity, output). | `types.rs` |
| `DispatchStatus` | Outcome discriminant: `completed` \| `failed` \| `timed_out`. | `types.rs` |
| `DispatchError` / `DispatchErrorInfo` | Error type for dispatch failures (thiserror). | `types.rs` |
| `TokenUsage` | Token accounting carried in the result. | `types.rs` |
| `Activity` | Files-changed / commands-run summary in the result. | `types.rs` |
| `HarnessEvent` / `EventKind` | Events emitted to stderr (heartbeat, tool activity, dispatch end). | `event.rs` |

Technical helpers (`Emitter`, `RunHandle`, `SupervisorBuilder`) are excluded as non-domain. Type changes: `DispatchSpec` loses `store_dir` and the `LiveStatus` status-store type is deleted; no other type changes shape. (`orphaned` was never a `DispatchStatus` variant — it was an async-lifecycle state string that disappears with the subcommands.) The pre-existing unused `DispatchError` variants and their `#[expect(dead_code)]` are left as-is (out of scope).

## Error Strategy

- **Hierarchy:** a single `thiserror`-based `DispatchError` enum in `types.rs` (preserved unchanged).
- **Propagation:** `engine.rs::dispatch()` returns `Result<DispatchResult, DispatchError>` — a started worker's outcome becomes a `DispatchResult`; a pre-start failure (e.g., missing engine binary) returns `Err`. `supervisor.rs` surfaces the spawn/signal errors that feed this.
- **Spawn-error result (preserved into the sync path):** today `main.rs` turns a dispatch `Err` into a `failed` JSON result only on the async `store_dir` branch (`cli::failed_start_result`); the sync branch returns a bare `Err` with no JSON. Collapsing to one path moves that conversion into the single synchronous path, so a dispatch `Err` prints exactly one `failed` `DispatchResult` on stdout — upholding the `synchronous-dispatch` requirement ("exactly one JSON result") and the README's `failed`-on-spawn-error semantics. This is behavior preservation during the async→sync collapse, not new behavior.
- **Exit mapping (per the crate convention):** `completed` → exit 0; `failed`/`timed_out` → exit 1; CLI/usage errors (bad args, the removed `--async`/subcommands, missing engine binary) → exit 2.
- **Out of scope (do not fix here):** the unused `DispatchError` variants (`TimedOut`/`ProcessFailed`/`ParseError`) and their dead-code annotation; the error-path worker-orphan gap (no `kill_on_drop`) and the unbounded `stderr` await. These are real but live in the kept core and belong to a separate change.

## Integration Points

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| `main.rs` | `cli.rs` | arg parse → `DispatchSpec` | internal |
| `main.rs` | `engine.rs` | `dispatch(spec)` | internal |
| `engine.rs` | `supervisor.rs` | spawn + signal (`SupervisorBuilder`/`RunHandle`) | internal |
| `engine.rs` | `event.rs` | `Emitter` event emission | internal |
| mission Phase 8 (orchestrator) | `darkfactory-dispatch` | synchronous `--json` CLI | inbound — unchanged |
| `/adversarial` skill | `darkfactory-dispatch` | synchronous `--json` via `run_in_background` + `BashOutput` | inbound — **CHANGED** (was `--async` + `wait`) |
| `/dispatch`, `/artifact-review`, `/mission`, `/mission-init` | `darkfactory-dispatch` | synchronous `--json` | inbound — unchanged, must keep working |
| `darkfactory-dispatch` | Codex CLI | `codex exec` subprocess | outbound — unchanged |

`/adversarial` conversion note: because the dispatcher exits non-zero on `failed`/`timed_out`, the converted skill must capture the dispatcher's stdout (the JSON result) separately from the process exit, then parse `.status` from that captured stdout to decide `REVIEW_COMPLETE` vs `REVIEW_FAILED` — it must not treat a non-zero background-shell exit as an unparseable error. This mirrors the mission Phase 9 reviewer script: `DISPATCH_RESULT=$(... --json 2>>log)` then parse `.status`.

Deployment note: the installed binary at `~/.local/bin/darkfactory-dispatch` is built via `make install`. It is unaffected during the mission (the workers change source only); the new behavior applies after merge + reinstall.

## Test Strategy

- **Runner:** `cargo test` (unit tests in `src/`, integration tests in `tests/`), plus `cargo clippy -- -D warnings` and `cargo fmt --check`. **All run from `tools/darkfactory-dispatch/`** — there is no root `Cargo.toml`.
- **Coverage:** `cargo tarpaulin` for diff coverage. This change is overwhelmingly deletion, so added-line coverage will be low or SKIP; the meaningful gate is that the retained suite stays green with the removed-surface tests deleted, not new-test volume.
- **Tests to delete** (their subjects are removed): async/lifecycle tests in `tests/cli.rs` (help-expects-`--async`, mutual-exclusion, `status`/`wait`/`result` unknown-id, async-flow, orphan, pruning); `control_file_abort_*` in `tests/engine.rs`; `stream_mode_filtering` and `heartbeat_ticker_can_be_cancelled` in `src/event.rs`; `graceful_stop_*` and singular `.arg` uses in `tests/supervisor.rs`.
- **Tests to keep/adapt:** supervisor process-group + two-phase-timeout tests; engine NDJSON/result tests; `tests/smoketest.sh` (sync `--json` only — should survive unchanged).
- **New tests (minimal):** small integration tests for the new observable behaviors — `--async`/lifecycle subcommands rejected, no files under the `/tmp` store, a planted `control.json` is ignored. No-files assertions run against an isolated `HOME`/baseline.

## Existing-codebase assessment

The current structure is sound; the meaningful coupling being removed is the `store_dir` threading (`cli → DispatchSpec → engine`) and the watchdog select-arm. **Compile-safety constraint:** `engine.rs` reads `spec.store_dir` for status-dir selection, cleanup, parent-death, and both status writes, so `DispatchSpec.store_dir` cannot be removed before its engine readers — the field and its consumers must land together in one compile-safe step. The safe removal order (drives task ordering in Phase 4.4): (1) dead test-only code first — risk-free, independent; (2) **one compile-safe step** removing the async subsystem (`--async`, the `Commands` subcommands, store helpers, clap `conflicts_with`, the `tracing` deps) together with the engine's `store_dir` field/branches, the watchdog arm (control read + in-loop status write), the post-dispatch status write, `write_status_json_atomic`/`status_dir`, and the `LiveStatus` type — all coupled through `store_dir`/status — while moving `cli::failed_start_result` (dispatch `Err` → `failed` JSON) into the unified sync path so spawn errors still emit one JSON result; (3) the `/adversarial` consumer; (4) README/CLAUDE.md docs. Every task must compile (`cargo build` green) at its own commit.
