# Adversarial Review B: darkfactory-dispatch

**Reviewer:** Claude Opus 4.6 (1M context)
**Base SHA:** `870d78e03971fc776bc003875e49bc63a0c529f4`
**Commits reviewed:** 10 (`1d8a7a3..7631a81`)
**Date:** 2026-04-23

---

## Per-Commit Findings

### Commit 1 (`1d8a7a3`) -- Project scaffolding

**Severity: Low**

1. **Unused dependencies shipped.** `tracing` and `tracing-subscriber` are declared in `Cargo.toml` but never used anywhere in the final codebase. No module calls `tracing::info!()` or initializes a subscriber. These add compile time and binary size for zero benefit.

2. **No `Cargo.lock` committed.** The handoff notes explain network was unavailable, but the lock file was never added in any subsequent commit. For a binary crate, the lock file should be committed to ensure reproducible builds.

---

### Commit 2 (`342493a`) -- Supervisor module

**Severity: Medium**

3. **Race between `getpgid` and `setpgid` in `process_group_id`.** After `spawn()`, the child's `pre_exec` closure calls `setpgid(0, 0)` in the child. But the parent immediately calls `getpgid(pid)`. If the parent runs before the child's `pre_exec` completes, `getpgid` returns the parent's process group (not the child's new one). The fallback `pid as i32` partially mitigates this by using the child's pid as the group id (which is what `setpgid(0,0)` sets it to), but only works because the child becomes its own group leader. This is correct by accident rather than by design -- the SAFETY comment claims it is called "with a live child pid" but does not acknowledge the race. If `getpgid` returns the parent's pgid (before the child's `setpgid` runs), the code falls through to `pid as i32` only because `getpgid` of a child that hasn't yet called `setpgid` returns the parent's pgid, which is > 0 -- so the code would use the wrong pgid. **However**, on closer inspection, the fallback only triggers when `pgid <= 0` (error). In the race scenario, `getpgid` returns the parent's pgid (> 0), so the code would store the parent's pgid and later try to `kill(-parent_pgid, signal)`, which would signal the parent's entire process group. This is a real race condition.

   **Impact:** In practice, the child's `pre_exec` runs before `spawn()` returns on most Unix systems (it runs between fork and exec), so this race is extremely unlikely. But it is technically possible with a context switch between fork and pre_exec.

4. **`watch_parent_death` on macOS leaks kqueue fd on error paths.** If `kevent()` returns an error (`n < 0`), the function returns without closing the kqueue descriptor. The `close(kq)` call only happens when `n > 0`. When `n == 0` (timeout with no events, though this should not happen with a null timeout), the kqueue fd is also leaked.

   Fixed version should close kq unconditionally before returning from the thread.

5. **`watch_parent_death` on Linux is called twice.** The `pre_exec` closure calls `prctl(PR_SET_PDEATHSIG)` inside the child, and then after spawn, the parent calls `watch_parent_death(_pgid)` which calls `prctl(PR_SET_PDEATHSIG)` again -- but this time in the *parent* process, not the child. The Linux `watch_parent_death` function sets the parent-death signal on the *current process* (the dispatcher itself), not on the child. This means if the dispatcher's parent dies, the dispatcher itself gets SIGKILL'd. This is probably the intended behavior for async mode, but it is not documented and may be surprising. The `_parent_death_watch` flag in `pre_exec` was added later (task-6b) to conditionally skip this for async mode, but the `watch_parent_death` call in the parent is always made when `self.parent_death_watch` is true.

6. **`stdout()` and `stderr()` can only be called once.** The `take()` pattern means a second call returns `StdoutUnavailable`. This is not a bug per se, but the `RunHandle` is `Clone` (via `Arc`), so a caller could clone it and both clones would race to take stdout. The `Mutex` prevents data corruption but the second caller silently gets an error. No test covers this edge case.

---

### Commit 3 (`c0d1268`) -- Types module

**Severity: Low**

7. **`DispatchError` does not implement `Clone`.** The `SpawnFailed(#[from] std::io::Error)` variant prevents `Clone`. This means `DispatchError` cannot be stored in the watch channel alongside `ExitResult`. This is fine architecturally but means error paths that need to clone the error (e.g., for retry logic) will need `.to_string()` conversion.

8. **`DispatchSpec` is `Serialize + Deserialize` but is never serialized/deserialized.** It is always constructed from CLI args. The serde derives add code for no current use. Minor.

9. **`HarnessEvent` derives `Serialize + Deserialize` but is only constructed internally.** Same as above. The `Deserialize` impl is unused.

---

### Commit 4 (`2b25d51`) -- Event emitter module

**Severity: Low**

10. **`heartbeat_ticker` spawns a tokio task that is never joined.** The `JoinHandle` returned by `tokio::spawn` is dropped. If the heartbeat task panics, the panic is silently swallowed. This is acceptable for a heartbeat (it should never panic), but a `JoinError` check on shutdown would be more robust.

11. **Heartbeat ticker cancellation test is not deterministic.** The test `heartbeat_ticker_can_be_cancelled` sets the interval to 3600 seconds, immediately cancels, and checks that `cancelled()` returns. This does not actually test that the ticker *stops* -- it only tests that the cancellation token works. A race is possible where the spawned task has not yet started by the time `cancel()` is called. The test passes regardless because `cancelled()` returns immediately when the token is already cancelled.

12. **`CancellationToken` reimplements `tokio_util::sync::CancellationToken`.** This is a custom implementation using `watch`. It works, but `tokio-util` is a well-tested alternative. Given the project rule against adding dependencies without asking, this is a reasonable choice.

---

### Commit 5 (`d243014`) -- Engine module

**Severity: High (contains the most critical findings)**

13. **CRITICAL: Double grace period on hard timeout.** When the hard timeout fires (line 115), the engine calls `run_handle.graceful_stop(grace_sec.max(10))`. But `graceful_stop` sends SIGTERM and then waits `grace_secs` before sending SIGKILL. So the actual sequence is:
    - `timeout_sec` elapses -> soft timeout, emit warning
    - `grace_sec` more elapses -> hard timeout fires
    - `graceful_stop(grace_sec.max(10))` -> send SIGTERM, wait *another* `grace_sec.max(10)` seconds before SIGKILL

    This means the total time before SIGKILL is `timeout_sec + grace_sec + grace_sec.max(10)` -- nearly double the intended grace. The hard timeout handler should call `kill()` directly since the grace period has already elapsed, or pass a very short grace to `graceful_stop` (e.g., 1 second).

14. **Unbounded stderr capture.** `capture_stderr` reads all of stderr into a `String` with no size limit. A misbehaving codex process that writes gigabytes to stderr will OOM the dispatcher. Stdout is bounded by `STDOUT_BUFFER_BYTES` (4MB) for the BufReader, but individual lines from `next_line()` are also unbounded. The stdout buffer size is the OS pipe buffer, not a line-length limit.

15. **`DispatchError::SpawnFailed` used for non-spawn errors.** The `open_log`, `capture_stderr`, and `write_log_line` functions all map I/O errors to `DispatchError::SpawnFailed`, which has the message "process spawn failed: {0}". A log write failure would display as "process spawn failed: disk full" which is misleading. A dedicated variant like `IoError` would be clearer.

16. **`finalize_result` requires `output` to exist for `Completed` status.** If `--output` is not provided (`output: None`), then `output.is_some_and(Path::exists)` is `false`, and the status is always `Failed` even when the exit code is 0. This means running `darkfactory-dispatch --prompt "do work" --json` (without `--output`) will always report failure. The mission skill always passes `--output`, so this is not triggered in practice, but it is a footgun for any other caller.

17. **Engine tests mutate global process environment.** `TestPath::install` calls `std::env::set_var("PATH", ...)` and `set_var("CODEX_MOCK_MODE", ...)`. These are process-global mutations. While the `PATH_LOCK` mutex serializes the engine tests, the CLI tests (in `tests/cli.rs`) also manipulate PATH. If `cargo test` runs engine and CLI tests concurrently (they are in different test binaries, so this is fine), but within the engine test binary, any non-engine test running concurrently would see the modified PATH. The `#[tokio::test(flavor = "multi_thread")]` makes this riskier because multiple futures may run concurrently within one test.

18. **`parse_codex_event` does not handle extremely long lines.** `BufReader::lines()` reads until `\n` with no length limit. A single NDJSON line of several GB would be read into memory. This is a DoS vector if the child process is compromised.

19. **`write_log_line` flushes after every single line.** This is correct for crash safety but adds significant I/O overhead. For a high-throughput NDJSON stream, this could slow down the dispatch loop. Since the log is advisory, flushing on heartbeat intervals or on shutdown would be more efficient.

20. **`effective_grace_sec` returns `(timeout_sec / 2).max(1)` when grace is 0.** For a 1800-second timeout, this means a 900-second grace period. Combined with finding #13 (double grace), the worst case is the process running for 1800 + 900 + 900 = 3600 seconds (1 hour) before being killed. The mission skill sets `--timeout 1800` without `--grace`, so it gets a 900-second grace by default.

---

### Commit 6 (`2f23097`) -- CLI module (sync dispatch)

**Severity: Low**

21. **`print_result` in non-JSON mode does not print all fields.** It only prints status, duration_ms, and output path. Token usage, activity, errors, and other fields are silently dropped. The `--json` mode prints everything. This means a human running the dispatcher without `--json` gets minimal information.

22. **`dispatch_spec` does not validate `--cwd` exists.** If the user passes `--cwd /nonexistent`, the error is deferred to the `codex exec` spawn, where it appears as a confusing spawn failure rather than a clear CLI validation error.

23. **Duplicate `status_name` function.** Both `cli.rs` (line 254) and `engine.rs` (line 704) define identical `fn status_name(status: &DispatchStatus) -> &'static str` functions. This is a DRY violation that could lead to drift if one is updated and the other is not. It should be a method on `DispatchStatus` or a shared function in `types.rs`.

---

### Commit 7 (`a2be004`) -- Async dispatch and lifecycle commands

**Severity: Medium**

24. **`print_result_for_id` and `wait_for_id` use blocking `std::thread::sleep` inside an async runtime.** These functions are called from `handle_lifecycle_command`, which is called from `run()` (an async function). `std::thread::sleep` blocks the current tokio worker thread. For `wait_for_id`, this could block a worker thread for the entire dispatch duration (up to 30+ minutes). The functions should use `tokio::time::sleep` and be `async`.

25. **`wait_for_id` prints status to stderr in a loop.** Every 2 seconds it prints `[Ns] running | N tools | N files changed`. For a 30-minute dispatch, this produces ~900 lines of stderr output. There is no way to suppress this.

26. **`host_process_state` uses `kill(pid, 0)` to check liveness.** If the host process dies and the PID is reused by another process, `process_alive` returns `true` and the dispatch appears to be running. PID reuse is rare but possible on long-running systems. A more robust approach would store the process start time alongside the PID.

27. **`prune_old_dispatches` runs synchronously during async store setup.** If there are many old dispatch directories, this blocks the async setup. It uses `fs::read_dir` and `fs::remove_dir_all` which are blocking I/O. This should either be `tokio::fs` or spawned on a blocking thread.

28. **`prune_old_dispatches` fails hard on any entry error.** If one dispatch directory has unexpected permissions, the entire prune operation fails, which causes `prepare_async_store` to fail, which prevents the dispatch from starting. A more robust approach would log the error and continue pruning other entries.

29. **`HostPidGuard` drops the pid file on normal exit.** When the dispatcher finishes normally, the `Drop` impl removes `host.pid`. But `result.json` is written before `HostPidGuard` is dropped. There is a tiny window between writing `result.json` and removing `host.pid` where both files exist, but this is harmless because `read_dispatch_status` checks `result.json` first.

30. **`dispatch_store_root` relies on `HOME` environment variable.** This is standard Unix behavior but will fail if `HOME` is not set (e.g., in some container environments). The error message is clear, but using `dirs::home_dir()` or similar would be more robust.

---

### Commit 8 (`6abecb7`) -- Mission skill integration

**Severity: High**

31. **CRITICAL: Mission skill bash script does not handle dispatcher exit code > 0.** The script uses `set -euo pipefail` (from the shell profile, not the script itself -- actually, looking again, the mission skill scripts do NOT use `set -e`). If `darkfactory-dispatch` exits with code 1 or 2, the `$()` command substitution captures stdout. But if the dispatcher itself crashes (segfault, OOM kill), `DISPATCH_RESULT` will be empty, `jq -r '.status'` will fail with a parse error, and `DISPATCH_STATUS` will be empty. The `if` check then falls through to `TASK_FAILED` with `DISPATCH_STATUS` being empty or "null", which is correct but the error message is unclear.

    **More importantly:** If `darkfactory-dispatch` exits non-zero *and* prints an error to stderr instead of a JSON result to stdout (which it does for `BinaryNotFound` and other startup errors -- see `engine_error` in `main.rs` which returns `Err` which prints to stderr), then `DISPATCH_RESULT` is empty and `jq -r '.status'` outputs "null" (jq treats null input as null, but actually jq will fail with a parse error on empty input). The script will print "TASK_FAILED: dispatch status null" which is confusing.

    **Fix:** Add error handling before the jq call: `if [ -z "$DISPATCH_RESULT" ]; then echo "TASK_FAILED: dispatcher crashed. Log: $LOG_PATH"; exit 0; fi`

32. **`--prompt-file "$BRIEF_PATH"` reads the entire brief into the codex prompt.** The brief content was previously passed via `"$BRIEF_CONTENT"` (a shell variable set by `cat "$BRIEF_PATH"`). The new approach passes `--prompt-file` which is read by the dispatcher's Rust code (`fs::read_to_string`). This is functionally equivalent but the error path is different: a missing brief file now produces a CLI error (exit code 2) rather than a shell error. This is fine but changes the failure mode.

33. **No `--config` flag for writable_roots in the review dispatch.** The task dispatch (line 489) includes `--config 'sandbox_workspace_write.writable_roots=[".git"]'` but the review dispatch (line 744-753) does not. This means the review worker cannot write to `.git/`. If the review worker needs to inspect git state (which it does via `git show`, `git diff` etc.), the read operations should work, but any git operations that modify the worktree would fail. This is likely intentional (reviewers should be read-only) but should be documented.

---

### Commit 9 (`37d5c79`) -- Init skill update

**Severity: Low**

34. **`darkfactory-dispatch --version` may not work.** The `--version` flag is defined in `Cli` via `#[command(version)]` which uses `CARGO_PKG_VERSION`. This works. The init skill runs `darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"`. If the binary is not on PATH, this correctly prints "NOT_INSTALLED". If the binary exists but is corrupt, it prints "NOT_INSTALLED" (stderr goes to /dev/null). This is fine.

35. **Install instructions are macOS-only.** `brew install jq` and `brew install gh` are macOS-specific. The init skill does not provide Linux install alternatives. The CLAUDE.md says "Shell scripts in skills must work on both Debian/Ubuntu and macOS" but install instructions are not shell scripts, so this may be acceptable as guidance rather than automation.

---

### Commit 10 (`7631a81`) -- README update

**Severity: Low**

36. **`make install` requires Rust toolchain.** The README says "build and install the dispatch supervisor" with `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`, but does not mention that Rust/Cargo must be installed. A first-time user without Rust will get a confusing error.

37. **`~/.local/bin` may not be on PATH.** The `make install` copies to `~/.local/bin/darkfactory-dispatch` but the README does not mention adding this directory to PATH. If it is not on PATH, `darkfactory-dispatch` will not be found by the mission skill.

---

## Cross-File Findings

These issues were found by reading the final working-tree state of all source files together.

### C1. Stdout race between engine dispatch loop and process exit (Medium)

In `engine.rs`, the `tokio::select!` loop has two branches that can fire at nearly the same time:

```rust
line = stdout_reader.next_line(), if stdout_open => { ... }
result = run_handle.wait() => { break ... }
```

When the child exits, both the stdout EOF and the `wait()` future become ready. `tokio::select!` picks one randomly. If `wait()` wins, the loop breaks and remaining stdout lines are never read. This means the last NDJSON events (potentially including `turn.completed` with final token counts) are lost. The `stderr_task` is correctly awaited after the loop, but stdout is not drained.

**Impact:** Token counts in the final result could be under-reported. The `turn.completed` event is the one that carries cumulative token usage, so losing it means `tokens.input`, `tokens.output`, and `tokens.reasoning` will all be 0 in the result.

### C2. `capture_stderr` task error is not propagated for timeout path (Medium)

When the dispatch times out, the engine calls `graceful_stop` which kills the child. The stderr pipe closes, `capture_stderr` finishes, and `stderr_task.await` happens after the loop. But if `capture_stderr` returns an `Err`, this error replaces the timeout result. The caller gets a stderr read error instead of the timeout status. The error should be logged and the timeout result should be preserved.

### C3. Log file shared between stdout and stderr with no ordering guarantee (Low)

Both `write_log_line` (for stdout) and `capture_stderr` (for stderr) write to the same `SharedLog` file handle. They share it via an `Arc<Mutex<File>>` so there is no data corruption. But the interleaving is arbitrary. A log reader cannot distinguish stdout lines from stderr lines. Adding a prefix (e.g., `[stdout]` / `[stderr]`) would help debugging.

### C4. `write_status_json_atomic` in engine uses blocking I/O inside async context (Medium)

In the dispatch loop's `watchdog_interval` branch (line 121-131), `write_status_json_atomic` calls `std::fs::create_dir_all`, `std::fs::File::create`, `file.sync_all()`, `std::fs::rename`, and `std::fs::File::open`. All of these are synchronous blocking I/O called from within a `tokio::select!` loop. For the watchdog interval (every 5 seconds), this blocks a tokio worker thread briefly. Under disk pressure or NFS, this could block for seconds and delay processing of NDJSON events.

### C5. `nix` crate is declared as a dependency but only `libc` types are used (Low)

The `nix` crate is in `Cargo.toml` with features `["signal", "process"]` but the code only uses `nix::libc` for raw libc calls (`setpgid`, `getpgid`, `kill`, `kqueue`, `kevent`, `prctl`, `close`, `getppid`). No `nix` high-level wrappers are used. The crate could be replaced with just `libc` to reduce the dependency tree.

### C6. `HarnessEvent` fields are `Option<String>` but many are always set or always None for a given `EventKind` (Low)

For example, `ToolStart` always has `tool: Some(...)` and never has `file`, `command`, `text`, `tokens`, or `session_id`. The flat struct with many Options makes it easy to construct but hard to reason about which fields are valid for which kind. An enum with per-variant data would be more type-safe. This was likely a deliberate design choice for NDJSON serialization simplicity.

### C7. `parse_codex_event` treats unknown event types as `RawPassthrough` (Low)

This increments `parse_errors` in the activity tracker, which is misleading. Unknown event types from future codex versions are not "parse errors" -- they are unrecognized events. The counter should be named `unrecognized_events` or split into `parse_errors` (actually malformed JSON) and `unknown_events` (valid JSON with unrecognized type). Currently, `RawPassthrough` is used for both malformed JSON *and* unknown event types, conflating two different things.

### C8. `parent_death_watch` flag semantics changed between commit 2 and commit 7 (Low)

In commit 2, `parent_death_watch` defaulted to `true` and was used to decide whether to set `PR_SET_PDEATHSIG` and spawn the macOS kqueue watcher. In commit 7 (task-6b), it was repurposed to also control whether to skip `PR_SET_PDEATHSIG` in `pre_exec` (for async mode where the parent should exit without killing the child). The flag now means "kill child when parent dies" which is correct, but the `watch_parent_death` function on Linux sets `PR_SET_PDEATHSIG` on the *parent* process, not the child. So disabling `parent_death_watch` for async mode skips the child's `pre_exec` pdeathsig *and* skips the parent's own pdeathsig -- which is correct but for different reasons.

---

## Summary

| Severity | Count | Key Issues |
|----------|-------|------------|
| Critical | 2 | Double grace period (#13), Mission skill empty-result handling (#31) |
| Medium | 5 | getpgid race (#3), blocking sleep in async context (#24), stdout drain race (#C1), stderr error clobbers timeout (#C2), blocking I/O in select loop (#C4) |
| Low | 15 | Unused deps, duplicate functions, unbounded stderr, misleading error variants, etc. |

### Recommended Immediate Fixes

1. **Fix double grace period** (engine.rs line 115): Change `graceful_stop(grace_sec.max(10))` to `kill()` or `graceful_stop(1)` since the grace period has already elapsed.
2. **Fix mission skill empty-result handling** (SKILL.md): Add a guard before the `jq` call to handle empty `DISPATCH_RESULT`.
3. **Drain stdout before breaking on wait()** (engine.rs): After the select loop breaks on `run_handle.wait()`, drain remaining lines from `stdout_reader`.
4. **Replace `std::thread::sleep` with `tokio::time::sleep`** in `wait_for_id` and `print_result_for_id` (cli.rs).
