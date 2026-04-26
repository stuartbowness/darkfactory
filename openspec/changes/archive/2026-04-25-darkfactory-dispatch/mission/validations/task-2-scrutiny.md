# Task 2.1 Scrutiny: Supervisor Module

**Commit:** `342493a`
**Validator:** scrutiny
**Date:** 2026-04-23

## Build & Checks

| Step | Result |
|---|---|
| `cargo build` | PASS (0 crates compiled, clean) |
| `cargo test --test supervisor -- --test-threads=1` | PASS (3/3 tests, 1.10s) |
| `cargo clippy --all-targets -- -D warnings` | PASS (no issues) |

## Contract Assertions

### 1. [execution] Process group kill test
**PASS.** `kill_terminates_the_whole_process_group` spawns a shell that backgrounds two `sleep 300` children (grandchildren of the test), then calls `handle.kill()` on the process group and verifies all three PIDs are dead via `assert_process_exits`.

### 2. [execution] Graceful stop SIGTERM -> SIGKILL test
**PASS.** `graceful_stop_kills_after_grace_when_sigterm_is_ignored` spawns a Python process that ignores SIGTERM, calls `graceful_stop(1)`, and verifies: (a) the result reports `SIGKILL`, (b) elapsed time is between 1s and 3s -- confirming it waited the grace period then escalated.

### 3. [execution] ExitResult reports correct signal for SIGTERM vs SIGKILL
**PASS.** `wait_reports_sigterm_and_sigkill` spawns two separate `sleep 300` processes. The first is stopped with `graceful_stop(10)` and asserts `signal == Some(SIGTERM)`. The second is killed with `kill()` and asserts `signal == Some(SIGKILL)`. Both paths confirmed distinct.

### 4. [output] setpgid(0, 0) in pre_exec hook
**PASS.** Line 134 of `src/supervisor.rs`:
```rust
unsafe {
    command.pre_exec(|| {
        if libc::setpgid(0, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
        ...
    });
}
```

### 5. [output] Signals sent via kill(-pgid, sig)
**PASS.** `signal_group` at line 281:
```rust
let rc = unsafe { libc::kill(-pgid, signal) };
```

### 6. [inspection] Platform-specific parent-death detection
**PASS.** Both paths exist with conditional compilation:
- **macOS** (line 298): `#[cfg(target_os = "macos")]` -- uses `kqueue` with `EVFILT_PROC` + `NOTE_EXIT` to watch for parent PID exit, then signals the child process group.
- **Linux** (line 343): `#[cfg(target_os = "linux")]` -- calls `libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL)`. Also called in `pre_exec` (line 140) for the child itself.
- **Other** (line 350): No-op fallback for unsupported platforms.

## Code Standards

| Check | Result | Notes |
|---|---|---|
| Gallant pattern | PASS | Supervisor owns process management only. No CLI or engine logic. |
| `pub(crate)` visibility | PASS | All public items are `pub(crate)`. |
| `thiserror` errors | PASS | `SupervisorError` uses `#[derive(Error)]`. |
| No `anyhow` | PASS | Zero occurrences. |
| `#[expect(...)]` over `#[allow(...)]` | PASS | Used in both `main.rs` and `tests/supervisor.rs`. |
| Serde rules | N/A | No serialization in this module. |
| Tokio test flavor | PASS | All three tests use `#[tokio::test(flavor = "multi_thread")]`. |

## Module Depth

The supervisor module has a small public interface (3 types: `SupervisorBuilder`, `RunHandle`, `ExitResult`) hiding 352 lines of process management complexity including process groups, signal delivery, two-platform parent-death detection, and async wait channels. The interface is appropriately deep per the project rules.

## Security Review (unsafe code)

Five `unsafe` blocks, all reviewed:

1. **`pre_exec` closure (line 132-147):** Runs `setpgid(0, 0)` and (on Linux) `prctl(PR_SET_PDEATHSIG)` between fork and exec. Both are async-signal-safe. Correct.

2. **`process_group_id` (line 266):** Calls `getpgid(pid)` with a live child PID. Falls back to using `pid` as the group if the call fails. Correct.

3. **`signal_group` (line 281):** Calls `kill(-pgid, signal)` with a guard that `pgid > 0` (line 275-277), preventing signaling pid 0 (all processes in the caller's group). ESRCH is treated as success. Correct.

4. **`watch_parent_death` macOS (line 298-341):** Creates a kqueue, registers `EVFILT_PROC` + `NOTE_EXIT` on the parent PID, blocks until notification, then kills the child process group. Resources are cleaned up (`close(kq)`). Runs on a dedicated named thread. Correct.

5. **`watch_parent_death` Linux (line 343-348):** Redundant `prctl` call (also done in `pre_exec`). Harmless but belt-and-suspenders. Correct.

**No issues found.** The `pgid > 0` guard in `signal_group` prevents the most dangerous foot-gun (signaling group 0). All libc calls are well-documented with SAFETY comments.

## Minimality Check

The implementation is minimal. No dead code, no unused features, no over-abstraction. `StdinMode` and `parent_death_watch` builder options are unused by tests but are documented as needed by the upcoming engine module. The `#[expect(dead_code)]` annotation in `main.rs` makes this explicit.

One minor note: the Linux `watch_parent_death` function (line 343) calls `prctl(PR_SET_PDEATHSIG)` again even though the same call happens in `pre_exec` (line 140). This is intentional -- `pre_exec` sets it for the child process itself, while `watch_parent_death` is called in the parent context. On Linux these are different processes. Not redundant.

## Verdict

**PASS.** All 6 contract assertions verified. Build, tests, and lint all clean. Code follows project standards. Unsafe code is correct and well-documented.
