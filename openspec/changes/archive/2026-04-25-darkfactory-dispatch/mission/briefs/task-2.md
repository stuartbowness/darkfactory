# Worker Brief: Task 2 - Supervisor module

## Your Task
Implement `src/supervisor.rs` -- the process spawning and lifecycle management module. This module hides all OS-level process management behind a clean interface.

### What to implement

**Public interface:**
- `SupervisorBuilder` -- configures the child process (binary, args, cwd, env)
- `RunHandle` -- returned after spawn, provides lifecycle control
  - `wait() -> ExitResult` -- idempotent wait for child exit
  - `graceful_stop(grace_secs: u64)` -- SIGTERM to group, wait, SIGKILL
  - `kill()` -- immediate SIGKILL to group
  - `pid() -> u32` -- child PID
  - `stdout() -> ChildStdout` -- for the engine to read NDJSON
  - `stderr() -> ChildStderr` -- for the engine to capture
- `ExitResult` -- exit code (Option<i32>) and signal (Option<i32>)
- `watch_parent_death(pgid: i32)` -- platform-specific orphan prevention

**Implementation details:**

1. **Process group isolation**: Use `Command::pre_exec` with `libc::setpgid(0, 0)` to put the child in its own process group. After start, get pgid via the child's PID (since setpgid(0,0) makes pgid = pid).

   Reference: `~/Code/agent-mux/internal/supervisor/supervisor.go:23-45`

2. **Signal delivery**: Send signals to `-pgid` via `libc::kill(-pgid, sig)` to target the entire group. Ignore ESRCH (process already dead).

   Reference: `~/Code/agent-mux/internal/supervisor/supervisor.go:94-112`

3. **GracefulStop sequence**: SIGTERM to group -> sleep grace_secs -> if still alive, SIGKILL to group. Use `tokio::select!` with `tokio::time::sleep` and the wait future.

   Reference: `~/Code/agent-mux/internal/supervisor/supervisor.go:60-81`

4. **Idempotent wait**: Use `tokio::sync::OnceCell` or similar to ensure multiple callers can await the same exit result.

   Reference: `~/Code/agent-mux/internal/supervisor/supervisor.go:51-58`

5. **Exit status detection**: Use `std::os::unix::process::ExitStatusExt::signal()` to detect whether the child was killed by a signal vs exited normally.

   Reference: `~/Code/agent-mux/internal/supervisor/supervisor.go:140-150`

6. **Parent-death detection**: Platform-specific:
   - **macOS**: Spawn a tokio task that uses kqueue with `EVFILT_PROC` + `NOTE_EXIT` + `EV_ONESHOT` watching the parent PID. When the event fires, send SIGKILL to -pgid.
   - **Linux**: Call `libc::prctl(PR_SET_PDEATHSIG, libc::SIGKILL)` in the current process.
   - Guard with `#[cfg(target_os = "macos")]` and `#[cfg(target_os = "linux")]`.

   Reference: `~/Code/agent-mux/internal/supervisor/reaper_darwin.go:13-43`, `reaper_linux.go:11-18`

7. **Stdin handling**: The supervisor configures stdin via `Command::stdin(Stdio::null())` for codex (immediate EOF) or `Stdio::piped()` for potential future use. The engine tells the supervisor which to use.

### Tests to write

Write tests in `tests/supervisor.rs`:

1. **Process group kill**: Spawn a child (`sleep 300`) that also spawns a grandchild (use a shell script: `sh -c "sleep 300 & sleep 300"`). Kill the group. Verify both processes are dead via `libc::kill(pid, 0)` returning ESRCH.

2. **GracefulStop timing**: Spawn a child that ignores SIGTERM (use `sh -c "trap '' TERM; sleep 300"`). Call `graceful_stop(1)`. Verify the child is killed within ~2 seconds (SIGTERM ignored, then SIGKILL after 1s grace).

3. **Signal detection**: Spawn a child, send SIGTERM, wait. Verify `ExitResult.signal == Some(15)`. Spawn another, send SIGKILL, verify `signal == Some(9)`.

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md` first
- All items `pub(crate)` unless called from main.rs
- `thiserror` for errors, no `anyhow`
- Use `#[expect(...)]` not `#[allow(...)]`
- Update `src/main.rs` to `mod supervisor;` so the module compiles
- Verify `cargo clippy --all-targets -- -D warnings` passes

## Project Context
darkfactory-dispatch is a Rust process supervisor for Codex CLI subprocesses. The supervisor module is the OS-level foundation that the engine module will consume.

## Architecture
The supervisor's public interface is small (spawn, wait, graceful_stop, kill) but hides significant OS complexity (process groups, signal delivery, platform-specific reaping). The engine module will call `SupervisorBuilder::new("codex").args(...).spawn()` and get a `RunHandle` to work with.

## Key Terms
- **Process group**: OS-level grouping via setpgid so signals kill the whole tree
- **GracefulStop**: SIGTERM -> wait grace -> SIGKILL sequence
- **Parent-death detection**: OS mechanism to kill orphaned workers when the dispatcher dies

## What Previous Tasks Built
### Task 1: Project scaffolding
- Created: tools/darkfactory-dispatch/ with Cargo.toml, Makefile, src/main.rs stub, placeholder modules
- Dependencies: tokio, nix, clap, serde, thiserror, etc.
- `cargo build` and `make check` pass

## Downstream Consumers
Task 5.1 (engine) will use the supervisor to spawn and manage the codex process.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md` for code standards
- Implement src/supervisor.rs
- Write tests in tests/supervisor.rs
- Update src/main.rs to include `mod supervisor;`
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit with message: "[task-2] Implement supervisor module with process groups, signals, and parent-death detection"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2 - Supervisor module
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
