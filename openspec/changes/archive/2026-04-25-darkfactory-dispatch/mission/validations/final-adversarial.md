# Final Adversarial Review -- darkfactory-dispatch

Reviewer: Claude Opus 4.6 (1M context)
Date: 2026-04-23
Scope: Verify previous fixes landed correctly, check for regressions, validate mission skill bash blocks.

## Verdict: CLEAN -- ready to merge

All previously reported issues were fixed correctly. No new production-breaking bugs found.

---

## 1. Two-phase timeout (engine.rs lines 64-161)

**Fixed correctly.** The select loop now has three distinct phases:

1. `soft_sleep` fires at `timeout_sec` -- sets `soft_armed = false`, sends SIGTERM via `kill_soft()`, arms `hard_sleep`.
2. `hard_sleep` fires after `grace_sec` -- sends SIGKILL via `kill()`, waits, drains, breaks with `timed_out = true`.
3. Normal `run_handle.wait()` arm -- if the child exits on its own, `timed_out` is set to `!soft_armed` (true only if SIGTERM was already sent).

No double grace period. SIGTERM and SIGKILL are distinct signals to the process group. The `graceful_stop()` method on `RunHandle` (which had the old double-grace issue) is no longer called from the engine -- it still exists as a public API but the engine uses the two-phase loop directly.

## 2. Stdout drain after child exit (engine.rs lines 96-117)

**Fixed correctly.** When `run_handle.wait()` resolves, a bounded drain loop reads remaining stdout lines with a 2-second deadline. The `tokio::select!` between `next_line()` and the deadline means the drain cannot hang on a grandchild holding stdout open. The hard-timeout path has its own 1-second drain (lines 142-160), which is appropriate since the process was just SIGKILL'd.

## 3. Empty dispatcher output guard (SKILL.md lines 496-507, 762-773)

**Fixed correctly.** Both bash blocks check `if [ -z "$DISPATCH_RESULT" ]` before piping to jq. This prevents the `jq: error (at <stdin>:0): null` crash that would occur if the dispatcher segfaults or produces no output.

## 4. Stderr routing to log file (SKILL.md lines 494, 758)

**Fixed correctly.** Both dispatch commands end with `2>>"$LOG_PATH"`, which appends dispatcher stderr to the log file. This keeps only the final `TASK_COMPLETE`/`TASK_FAILED` line on stdout for the Bash tool to read.

## 5. Makefile mkdir -p (Makefile line 10)

**Fixed correctly.** `mkdir -p ~/.local/bin` before `cp`.

## 6. Heartbeat visibility text (SKILL.md line 515)

**Fixed correctly.** Documents that heartbeats are every 15 seconds, and that the dispatcher kills the codex process automatically with SIGTERM at soft timeout and SIGKILL after grace period. Matches the engine implementation.

---

## Minor observations (not blocking)

**jq dependency assumed.** The bash blocks use `jq -r` without checking if jq is installed. This is a pre-existing dependency across darkfactory skills, not a regression. If jq is missing the block falls through to the `TASK_FAILED` path, which is acceptable degradation.

**`set_var` in tests is unsound under Rust 2024 edition.** `tests/engine.rs` lines 84, 93, 223-224 use `std::env::set_var` which is `unsafe` starting in edition 2024. The `Cargo.toml` would need to specify edition 2024 for this to be a compile error, and the `PATH_LOCK` mutex serializes tests that modify env vars. Not a production issue, but worth noting for a future edition bump.

---

No blocking issues. Ship it.
