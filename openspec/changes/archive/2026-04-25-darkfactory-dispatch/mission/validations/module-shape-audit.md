# Module Shape Audit: darkfactory-dispatch

## Supervisor (src/supervisor.rs, 354 lines)

**Design.md says:** Interface is `spawn(binary, args, cwd, env) -> RunHandle`, `graceful_stop`, `kill`, `wait -> ExitResult`. Hides process groups, signal delivery, parent-death detection.

**Actual public interface:** `SupervisorBuilder` (builder pattern with `new`, `arg`, `args`, `current_dir`, `env`, `stdin`, `parent_death_watch`, `spawn`), `RunHandle` (`pid`, `stdout`, `stderr`, `wait`, `graceful_stop`, `kill`), `ExitResult` (exit_code, signal), `StdinMode` enum, `SupervisorError` enum, `watch_parent_death` function.

**Drift:** The builder pattern (`SupervisorBuilder`) is slightly wider than design.md's single `spawn()` call -- 8 builder methods vs 1 function. However, this is a standard Rust builder pattern that replaces positional arguments. The hidden complexity (setpgid, -pgid signals, kqueue/prctl reaper, idempotent wait) matches exactly. **Acceptable -- builder pattern is idiomatic Rust, not interface widening.**

## Engine (src/engine.rs, 779 lines)

**Design.md says:** Interface is `dispatch(spec, id, emitter) -> Result<DispatchResult>`. Hides codex arg construction, NDJSON parsing, timeout select loop, heartbeat, watchdog, activity tracking, result finalization.

**Actual public interface:** Single function `pub(crate) async fn dispatch(...)`.

**Drift:** None. One function, 779 lines of hidden complexity. **Deep module as designed.**

## CLI (src/cli.rs + src/main.rs, 593 lines combined)

**Design.md says:** Hides argument parsing, DispatchSpec construction, sync/async routing, dispatch store, lifecycle subcommands, JSON output formatting.

**Actual:** cli.rs has the clap structs, spec construction, async store management, lifecycle handlers. main.rs routes subcommands and wires dispatch. All dispatch logic is in engine.rs.

**Gallant boundary check:** CLI parses flags, builds DispatchSpec, calls `engine::dispatch()`, formats the result. No domain logic in CLI. **Gallant boundary intact.**

## Event (src/event.rs, 466 lines)

**Design.md says:** Not a separate module in design -- part of engine.

**Actual:** Extracted as a separate module with `Emitter`, `DispatchEvent`, `StreamMode`, heartbeat ticker. This is slightly wider than design.md described.

**Drift:** The event emitter was implied as internal to the engine, but was extracted to its own module. This is reasonable -- it's consumed by both the engine (for heartbeat/tool events) and the CLI (for dispatch_start/end). **No drift in behavior, just module boundary adjustment. Acceptable.**

## Types (src/types.rs, 235 lines)

**Design.md says:** Key data structures: DispatchSpec, DispatchResult, HarnessEvent, EventKind, TokenUsage, Activity, LiveStatus.

**Actual:** All listed types present plus DispatchStatus, DispatchError, DispatchErrorInfo. All types have correct fields matching design.md.

**Drift:** None.

## Summary

| Module | Interface drift | Behavior drift |
|---|---|---|
| Supervisor | Minor (builder pattern vs single function) | None |
| Engine | None | None |
| CLI | None | None |
| Event | Minor (separate module vs engine-internal) | None |
| Types | None | None |

**Verdict:** No meaningful drift. All modules are deep (narrow interface, rich hidden behavior). Gallant boundary is intact. Two minor structural adjustments (builder pattern, event module extraction) are idiomatic Rust improvements over the design, not violations.
