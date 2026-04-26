# Contract

## Task 1.1: Project scaffolding
<!-- PRD Requirements: #10 -->
- [ ] [execution] `cargo build` in `tools/darkfactory-dispatch/` succeeds with exit code 0
- [ ] [execution] `make check` in `tools/darkfactory-dispatch/` succeeds (fmt + clippy + test)
- [ ] [output] `Cargo.toml` lists dependencies: tokio, serde, serde_json, clap, nix, ulid, chrono, thiserror, tracing, tracing-subscriber
- [ ] [output] `Makefile` has targets: build, install, test, lint, fmt, check
- [ ] [output] `.gitignore` includes `target/`

## Task 2.1: Supervisor module
<!-- PRD Requirements: #2, #5, #9, #14 -->
- [ ] [execution] A test spawns a child that spawns a grandchild, kills the process group, and verifies both are dead
- [ ] [execution] A test verifies `graceful_stop` sends SIGTERM, waits, then SIGKILL if the child doesn't exit
- [ ] [execution] A test verifies `ExitResult` correctly reports signal number when a child is killed by SIGTERM vs SIGKILL
- [ ] [output] `src/supervisor.rs` uses `libc::setpgid(0, 0)` in a `pre_exec` hook
- [ ] [output] `src/supervisor.rs` signals the process group via `libc::kill(-pgid, sig)`
- [ ] [inspection] Platform-specific parent-death detection exists: kqueue `EVFILT_PROC` on macOS, `PR_SET_PDEATHSIG` on Linux (conditional compilation)

## Task 3.1: Types module
<!-- PRD Requirements: #3, #9, #13 -->
- [ ] [execution] A test serializes a `DispatchResult` to JSON and deserializes it back, verifying all fields round-trip
- [ ] [output] `DispatchResult` has fields: status, dispatch_id, duration_ms, exit_code, signal, output, output_exists, error, tokens, activity
- [ ] [output] `Activity` has fields: files_changed (Vec<String>), files_read (Vec<String>), commands_run (Vec<String>), tool_calls (u64)
- [ ] [output] `TokenUsage` has fields: input, output, reasoning
- [ ] [output] `DispatchError` is a thiserror enum (not anyhow)

## Task 4.1: Event emitter module
<!-- PRD Requirements: #4 -->
- [ ] [execution] A test verifies heartbeat events are valid NDJSON with type, elapsed_s, and last_activity fields
- [ ] [output] `src/event.rs` emits events to stderr, not stdout
- [ ] [output] Event types include: dispatch_start, dispatch_end, heartbeat, tool_start, tool_end, file_write, file_read, command_run, timeout_warning

## Task 5.1: Engine module
<!-- PRD Requirements: #1, #2, #3, #4, #6, #8, #9, #13 -->
- [ ] [execution] A test dispatches a mock child that outputs codex-format NDJSON, verifies the DispatchResult has status `completed`, correct token usage, and activity file lists
- [ ] [execution] A test dispatches a mock child that runs past the timeout, verifies status `timed_out` and that the child process is dead
- [ ] [execution] A test verifies heartbeat events appear on stderr during a dispatch at ~15s intervals
- [ ] [execution] A test verifies malformed NDJSON lines produce `raw_passthrough` events and increment the parse error count
- [ ] [output] Codex is invoked with `codex exec --json --full-auto -s <sandbox> -m <model> -c model_reasoning_effort="<effort>" -o <output> <prompt>`
- [ ] [output] Child stdin is closed immediately (empty reader, not a pipe)
- [ ] [output] `status.json` is written atomically (tmp file + rename) every 5 seconds during dispatch
- [ ] [output] The `--log` file contains both child stdout and child stderr content
- [ ] [output] Grace period defaults to timeout/2 with minimum 1s when not specified
- [ ] [output] Hard timeout calls graceful_stop with a floor of 10 seconds

## Task 6.1: CLI module (sync dispatch)
<!-- PRD Requirements: #7, #8, #10 -->
- [ ] [execution] `darkfactory-dispatch --help` succeeds and lists all flags: --model, --effort, --timeout, --grace, --sandbox, --config, --prompt, --prompt-file, --output, --cwd, --log, --json, --async, --version
- [ ] [execution] `darkfactory-dispatch --version` prints a version string
- [ ] [execution] Providing both `--prompt` and `--prompt-file` exits with code 2 and an error message
- [ ] [execution] Providing both `--json` and `--async` exits with code 2 and an error message
- [ ] [execution] Providing neither `--prompt` nor `--prompt-file` exits with code 2 and an error message
- [ ] [output] `--effort high` results in codex receiving `-c model_reasoning_effort="high"`
- [ ] [output] `--config key=value` results in codex receiving `-c key=value`
- [ ] [output] Default timeout is 900 seconds when `--timeout` is not specified

## Task 6.2: Async dispatch and lifecycle subcommands
<!-- PRD Requirements: #11, #12 -->
- [ ] [execution] `darkfactory-dispatch --async --prompt "test"` emits `{"kind":"async_started","dispatch_id":"..."}` to stdout and returns immediately
- [ ] [execution] `darkfactory-dispatch status <id>` returns JSON with state field after an async dispatch
- [ ] [execution] `darkfactory-dispatch result <id>` returns the full DispatchResult JSON after async dispatch completes
- [ ] [execution] `darkfactory-dispatch wait <id>` blocks until the async dispatch completes, then exits 0
- [ ] [output] Async dispatch creates `~/.darkfactory/dispatches/<ulid>/` with host.pid, status.json, events.jsonl
- [ ] [output] Async dispatch writes result.json to the dispatch directory on completion
- [ ] [execution] Status reports `orphaned` when host.pid references a dead process and no result.json exists
- [ ] [execution] Dispatch directories older than 7 days are pruned when a new async dispatch starts

## Task 7.1: Mission skill integration
<!-- PRD Requirements: #1-#9 (integration) -->
- [ ] [output] `.claude/skills/mission/SKILL.md` worker dispatch block (~line 486) calls `darkfactory-dispatch` instead of raw `codex exec`
- [ ] [output] `.claude/skills/mission/SKILL.md` adversarial reviewer A block (~line 745) calls `darkfactory-dispatch` instead of raw `codex exec`
- [ ] [output] The mission skill parses JSON result for `status` field instead of checking for `TASK_COMPLETE`/`TASK_FAILED` strings
- [ ] [output] The fallback logic triggers on `status: "failed"` or `status: "timed_out"` from the JSON result

## Task 8.1: Init skill integration
<!-- PRD Requirements: #10 (discovery) -->
- [ ] [output] `.claude/skills/darkfactory-init/SKILL.md` includes `darkfactory-dispatch` in the required CLI tools check
- [ ] [output] The install instructions reference `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`

## Task 9.1: Smoke test
<!-- PRD Requirements: all (verification) -->
- [ ] [execution] `cargo build --release` in `tools/darkfactory-dispatch/` succeeds
- [ ] [execution] The release binary dispatches a codex task and returns valid JSON with status, dispatch_id, duration_ms, and exit_code fields

## Task 9.2: Cleanup
<!-- PRD Requirements: all (hygiene) -->
- [ ] [execution] `make check` passes with zero warnings (fmt + clippy -D warnings + tests)
- [ ] [execution] `grep -r "TODO\|FIXME\|HACK\|XXX" src/` returns no matches (or all are intentional and documented)

## Task 9.3: Module shape re-check
<!-- PRD Requirements: design conformance -->
- [ ] [output] `mission/validations/module-shape-audit.md` exists and documents each module's interface vs design.md
- [ ] [output] The audit confirms the Gallant boundary: engine owns all dispatch logic, CLI only parses and formats

## Task 9.4: Documentation
<!-- PRD Requirements: #10 (installation) -->
- [ ] [output] `README.md` documents darkfactory-dispatch: what it is, how to install, how it integrates with missions
- [ ] [output] `tools/darkfactory-dispatch/CLAUDE.md` repo map matches the actual file structure
