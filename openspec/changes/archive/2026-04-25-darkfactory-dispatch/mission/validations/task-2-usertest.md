# Task 2.1 User-Test Validation: Supervisor Module

**Commit:** 342493a
**Date:** 2026-04-23
**Test command:** `cargo test --test supervisor -- --test-threads=1`
**Test result:** 3 passed, 0 failed (1.10s)

## Contract Assertions

- [x] **[execution]** A test spawns a child that spawns a grandchild, kills the process group, and verifies both are dead
  - `kill_terminates_the_whole_process_group` spawns `sh -c "sleep 300 & ...; echo $$ $bg $fg; wait"`, producing a shell plus two background sleeps (3 PIDs). After `handle.kill()`, all three are verified dead via `assert_process_exits`.

- [x] **[execution]** A test verifies `graceful_stop` sends SIGTERM, waits, then SIGKILL if the child doesn't exit
  - `graceful_stop_kills_after_grace_when_sigterm_is_ignored` spawns a Python process that traps and ignores SIGTERM. `graceful_stop(1)` is called; the test asserts signal is SIGKILL, elapsed time >= 1s and < 3s, confirming SIGTERM was sent, the grace period was honored, and SIGKILL followed.

- [x] **[execution]** A test verifies `ExitResult` correctly reports signal number when a child is killed by SIGTERM vs SIGKILL
  - `wait_reports_sigterm_and_sigkill` spawns two `sleep 300` processes. The first dies to `graceful_stop(10)` (sleep does not trap SIGTERM) and asserts `signal == Some(SIGTERM)`. The second is killed via `handle.kill()` and asserts `signal == Some(SIGKILL)`.

- [x] **[output]** `src/supervisor.rs` uses `libc::setpgid(0, 0)` in a `pre_exec` hook
  - Lines 132-147: `command.pre_exec(|| { libc::setpgid(0, 0); ... })`.

- [x] **[output]** `src/supervisor.rs` signals the process group via `libc::kill(-pgid, sig)`
  - `signal_group` function, line 281: `libc::kill(-pgid, signal)`.

- [x] **[inspection]** Platform-specific parent-death detection exists: kqueue `EVFILT_PROC` on macOS, `PR_SET_PDEATHSIG` on Linux (conditional compilation)
  - `#[cfg(target_os = "macos")]` `watch_parent_death` (lines 298-341): kqueue with `EVFILT_PROC` / `NOTE_EXIT`, signals group on parent exit.
  - `#[cfg(target_os = "linux")]` `watch_parent_death` (lines 343-348): `prctl(PR_SET_PDEATHSIG, SIGKILL)`.
  - `#[cfg(not(any(...)))]` no-op fallback (lines 350-351).
  - Linux `pre_exec` hook (lines 138-143) also sets `PR_SET_PDEATHSIG` for defense-in-depth.

## Result

**6/6 assertions passed.** All tests execute successfully. Source code matches all output and inspection assertions.
