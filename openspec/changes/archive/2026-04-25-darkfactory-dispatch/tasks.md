## 1. Project scaffolding

- [x] 1.1 Initialize the Rust crate at `tools/darkfactory-dispatch/` with Cargo.toml (tokio, serde, serde_json, clap, nix, ulid, chrono, thiserror, tracing, tracing-subscriber), src/main.rs stub, Makefile (build, install, test, lint, fmt, check), and .gitignore. Verify `cargo build` and `make check` pass on the empty project.

## 2. Supervisor module

- [x] 2.1 Implement `src/supervisor.rs`: process spawning with `setpgid`, pgid tracking, `graceful_stop` (SIGTERM to group, wait grace, SIGKILL), `kill`, idempotent `wait`, `ExitResult` with exit code and signal detection. Include platform-specific parent-death detection: kqueue `EVFILT_PROC`+`NOTE_EXIT` on macOS, `PR_SET_PDEATHSIG` on Linux. Write tests in `tests/supervisor.rs` for process group kill (child + grandchild), graceful stop timing, and signal detection.

## 3. Types module

- [x] 3.1 Implement `src/types.rs`: `DispatchSpec`, `DispatchResult`, `DispatchError` (thiserror enum), `HarnessEvent`, `EventKind`, `TokenUsage`, `Activity`, `LiveStatus`. Include serde Serialize/Deserialize on all public types. Write inline `#[cfg(test)]` tests for JSON round-trip serialization of DispatchResult and DispatchError.

## 4. Event emitter module

- [x] 4.1 Implement `src/event.rs`: NDJSON event emitter to stderr, heartbeat ticker (configurable interval, default 15s, carries elapsed time and last activity), `timeout_warning` emission, `dispatch_start`/`dispatch_end` events, stream mode filtering (silent/normal/verbose). Write inline `#[cfg(test)]` tests for event serialization format.

## 5. Engine module

- [x] 5.1 Implement `src/engine.rs`: `dispatch(spec) -> Result<DispatchResult>` function. Codex arg construction (`codex exec --json -m <model> --full-auto -s <sandbox> -c <config...> -o <output> <prompt>`), stdin as empty reader, stdout pipe with 4MB BufReader for NDJSON parsing, stderr capture to separate buffer, log file writing (both stdout and stderr), Codex event translation (thread.started, item.started/completed, turn.completed -> HarnessEvent), activity tracking (files_changed, files_read, commands_run as path lists, tool_calls as count, token accumulation from turn.completed), last-activity updater for heartbeat context, two-phase timeout via `tokio::select!` (soft timer -> timeout_warning event + arm hard timer, hard timer -> graceful_stop with 10s floor), watchdog ticker (5s, writes status.json atomically via tmp+fsync+rename), result finalization (completed/timed_out/failed classification with exit_code, signal, output_exists check). Write tests in `tests/engine.rs` using a small test helper script that simulates codex NDJSON output, testing: successful completion, timeout (soft + hard), heartbeat emission, malformed NDJSON handling.

## 6. CLI module

- [x] 6.1 Implement `src/cli.rs` and update `src/main.rs` and update `src/main.rs`: clap derive CLI with flags (--model, --effort, --timeout, --grace, --sandbox, --config repeatable, --prompt, --prompt-file, --output, --cwd, --log, --json, --async, --version), mutual exclusion validation (--prompt vs --prompt-file, --json vs --async), DispatchSpec construction from flags, effort mapping to codex `-c model_reasoning_effort`, default timeout 900s, default grace = timeout/2 min 1s. Sync dispatch path: call engine::dispatch, emit DispatchResult JSON to stdout, exit with appropriate code. Write CLI smoke tests in `tests/cli.rs` via assert_cmd: --help, --version, missing prompt error, mutual exclusion errors.

- [x] 6.2 Implement async dispatch mode and lifecycle subcommands: async dispatch (write host.pid, emit async_started JSON to stdout, run dispatch in background, write status.json and result.json to `~/.darkfactory/dispatches/<ulid>/` on completion), `status <id>` subcommand (read status.json, check host.pid liveness, report orphaned if dead + no result), `result <id>` subcommand (read result.json, block if still running), `wait <id>` subcommand (poll until terminal state, emit periodic status lines to stderr), dispatch store cleanup (prune >7 days on new async dispatch). Write tests in `tests/cli.rs` for async dispatch + status + result + wait flow.

## 7. Mission skill integration

- [x] 7.1 Update `.claude/skills/mission/SKILL.md`: replace the Codex worker dispatch bash block (~line 486) and the adversarial reviewer A dispatch bash block (~line 745) with `darkfactory-dispatch` calls. Update the result parsing logic to read JSON status instead of checking for TASK_COMPLETE/TASK_FAILED strings. Preserve the existing Claude fallback logic (trigger on status `failed` or `timed_out` instead of TASK_FAILED).

## 8. Init skill integration

- [x] 8.1 Update `.claude/skills/darkfactory-init/SKILL.md`: add `darkfactory-dispatch` to the required CLI tools check in Step 4, with install instructions pointing to `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`.

## 9. Closing tasks

- [x] 9.1 Smoke test: run `cargo build --release` and verify the binary works end-to-end by dispatching a simple `codex exec --json "echo hello"` through darkfactory-dispatch with a short timeout, verifying the JSON result on stdout contains status, dispatch_id, duration_ms, exit_code, and token usage.
- [x] 9.2 Cleanup: remove any dead code, unused dependencies, leftover TODOs, debug prints. Verify `make check` passes (fmt + clippy + tests). Verify no compiler warnings.
- [x] 9.3 Module shape re-check: compare design.md module descriptions against the actual code. Verify the Gallant boundary (engine owns all logic, CLI only parses and formats). Check that public interfaces match what design.md describes. Write findings to `mission/validations/module-shape-audit.md`.
- [x] 9.4 Documentation: update darkfactory README.md to document darkfactory-dispatch (what it is, how to install, how it integrates with missions). Update the CLAUDE.md in tools/darkfactory-dispatch/ if the repo map or module structure drifted during implementation.
