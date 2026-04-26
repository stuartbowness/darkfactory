## Context

Darkfactory dispatches Codex workers as raw `codex exec` subprocesses from inlined bash blocks in the mission skill. These subprocesses have no timeout, no heartbeat, no process group isolation, and no structured result reporting. The orchestrator checks for `TASK_COMPLETE` or `TASK_FAILED` on stdout and manually tails log files for progress.

buildoak/agent-mux (`~/Code/agent-mux`, 200+ commits) solves this for codex/claude/gemini CLIs. We're building a lightweight Rust clone of its core codex dispatch machinery.

### Current state

- Mission skill worker dispatch: `SKILL.md:486-494` -- raw `codex exec` with `< /dev/null`, stdout+stderr redirected to log file, checks for string on last line
- Mission skill adversarial reviewer: `SKILL.md:745-752` -- same pattern
- No compiled binaries in the darkfactory repo today
- Existing Rust projects (irontodo-v2, ironhog, crucible) at `~/Code/` use clap derive, tokio, thiserror+anyhow

## Goals / Non-Goals

**Goals:**
- Supervised Codex subprocess dispatch with process group isolation
- Two-phase timeout with configurable soft/hard timers
- NDJSON heartbeat and event stream on stderr
- Structured JSON result on stdout
- Async dispatch with status/result/wait lifecycle
- Parent-death detection (macOS kqueue, Linux prctl)
- Integration with mission skill (replace both bash blocks)
- Required dependency in darkfactory-init

**Non-Goals:**
- Claude adapter (Agent tool handles Claude lifecycle)
- Adapter trait abstraction (codex-only, no polymorphism needed)
- Steering/inbox (no mid-flight message injection)
- Recovery/resume (orchestrator handles retries at mission level)
- Profile/prompt system (orchestrator owns role definitions)

## Architecture: Single binary, three modules

The original PRD proposed four modules including engine adapters. Since we're codex-only, the adapter collapses into the engine module. Three modules remain.

### Module 1: Supervisor (`src/supervisor.rs`)

**Interface:** `Supervisor::spawn(binary, args, cwd, env) -> Result<RunHandle>`, `RunHandle::graceful_stop(grace_secs)`, `RunHandle::kill()`, `RunHandle::wait() -> ExitResult`

**What it hides:**
- Process group setup via `Command::pre_exec` with `libc::setpgid(0, 0)`
- Signal delivery to `-pgid` via `libc::kill(-pgid, sig)`
- SIGTERM -> wait grace -> SIGKILL sequence in `graceful_stop`
- Platform-specific parent-death detection (kqueue on macOS, prctl on Linux)
- Idempotent wait via `tokio::sync::OnceCell`
- Exit status unwrapping: normal exit code vs signal number via `ExitStatusExt::signal()`

**Why this boundary:** Process management is the most OS-specific and hardest-to-test part. Isolating it lets us test signal handling, process groups, and parent-death detection independently of NDJSON parsing or timeout logic.

**Reference:** `agent-mux/internal/supervisor/supervisor.go` (process groups, GracefulStop), `reaper_darwin.go` (kqueue), `reaper_linux.go` (prctl)

### Module 2: Engine (`src/engine.rs`)

**Interface:** `Engine::dispatch(spec: DispatchSpec) -> Result<DispatchResult>`

**What it hides:**
- Codex arg construction (`codex exec --json -m <model> -s <sandbox> -c <config...> -o <output> <prompt>`)
- Stdin closure (empty reader for codex)
- NDJSON stdout parsing with 4MB line buffer
- Codex event translation (thread.started, item.started/completed, turn.completed -> unified events)
- Two-phase timeout select loop (stdout events, process exit, soft timer, hard timer, watchdog ticker, heartbeat ticker)
- Heartbeat emission to stderr every 15 seconds
- Watchdog writing `status.json` atomically every 5 seconds
- Activity tracking (files changed/read, commands run, tool calls, tokens)
- Last-activity updating for heartbeat context
- Result finalization (completed, timed_out, failed classification)

**Why this boundary:** This is the core event loop -- the biggest module (~400-500 lines). It consumes a `Supervisor::RunHandle` and produces a `DispatchResult`. All the complexity of multiplexing timers, events, and process lifecycle lives here. Callers just call `dispatch()` and get a result.

**Reference:** `agent-mux/internal/engine/loop.go:167-981` (dispatch function), `adapter/codex.go:57-99` (arg construction), `adapter/codex.go:174-337` (event parsing), `event/event.go:185-214` (heartbeat ticker)

### Module 3: CLI (`src/main.rs`, `src/cli.rs`)

**Interface:** The binary's command-line interface and subcommands.

**What it hides:**
- Argument parsing (clap derive)
- DispatchSpec construction from CLI flags
- Sync vs async dispatch routing
- Async dispatch lifecycle (host.pid, status.json, result.json persistence)
- Status/result/wait subcommand implementation
- Dispatch store management (`~/.darkfactory/dispatches/<ulid>/`)
- Dispatch store cleanup (prune >7 days)
- ULID generation
- JSON output formatting (result to stdout, events to stderr)
- Signal handling (SIGINT/SIGTERM -> graceful stop)

**Why this boundary:** The CLI is the entry point and owns all I/O formatting. It translates between human-facing flags and the engine's `DispatchSpec`. The async lifecycle (persist/query/wait) is fundamentally a CLI concern, not an engine concern.

**Reference:** `agent-mux/cmd/agent-mux/main.go` (CLI routing), `async.go` (async dispatch), `lifecycle.go` (status/result/wait)

## Code Standards (from irontodo-v2)

### Gallant pattern (boundary rule)
Engine owns ALL dispatch logic. CLI MUST only parse input, call engine, and format output. This governs which module a function belongs in. If logic could live in the engine, it goes in the engine -- not the CLI.

### Additional rules
- **Deep modules**: Each module has a small public interface hiding substantial complexity. Prefer fewer entry points over many small helpers. Do not extract pure functions into separate modules just for testability -- test at the module boundary instead.
- **Error types**: `thiserror` in engine/supervisor for typed errors (`DispatchError`). CLI converts at the boundary. No `anyhow` in engine or supervisor.
- **Visibility**: All items use `pub(crate)` unless they are part of the crate's public API.
- **Lint suppression**: Use `#[expect(...)]` instead of `#[allow(...)]`.
- **Serde**: No `#[serde(flatten)]` or `#[serde(untagged)]`.
- **Output**: Stdout for machine-readable data (DispatchResult JSON), stderr for diagnostics (heartbeats, events, warnings).
- **No double-logging**: Propagate errors with `?`. Log only when the error is finally handled.
- **Exit codes**: 0 = success (completed), 1 = dispatch failure (failed, timed_out), 2 = CLI error (bad args, missing binary).

## Key Data Structures

### DispatchSpec
```
model: String
effort: Option<String>
timeout_sec: u64
grace_sec: u64
sandbox: String
configs: Vec<String>
prompt: String
output: Option<PathBuf>
cwd: PathBuf
log: Option<PathBuf>
```

### DispatchResult
```
status: "completed" | "timed_out" | "failed"
dispatch_id: String (ULID)
duration_ms: u64
exit_code: Option<i32>
signal: Option<i32>
output: Option<String> (path)
output_exists: bool
error: Option<DispatchError>
tokens: TokenUsage { input, output, reasoning }
activity: Activity { files_changed: Vec<String>, files_read: Vec<String>, commands_run: Vec<String>, tool_calls: u64 }
```

### HarnessEvent (internal, not exposed)
```
kind: EventKind (ToolStart, ToolEnd, FileWrite, FileRead, CommandRun, Response, TurnComplete, SessionStart, Error, RawPassthrough)
tool: Option<String>
file: Option<String>
command: Option<String>
text: Option<String>
tokens: Option<TokenUsage>
```

## Decisions

### 1. No adapter trait -- build codex directly

**Why:** We're codex-only. An adapter trait adds indirection for zero benefit today. If we later add a gemini adapter, we extract the trait then. YAGNI.

**Alternative considered:** Adapter trait matching agent-mux's `HarnessAdapter` interface. Rejected because it adds a trait, a registry, and a vtable dispatch for one implementation.

### 2. tokio for the event loop

**Why:** The engine module needs to multiplex timers (soft timeout, hard timeout, heartbeat, watchdog), process exit, and stdout line reading. `tokio::select!` handles this cleanly. Matches crucible's async pattern.

**Alternative considered:** Synchronous threads with channels. Rejected because the select! pattern is more readable and avoids thread coordination bugs.

### 3. Atomic status.json via tmp+fsync+rename

**Why:** The watchdog writes status.json every 5 seconds. Concurrent readers (status subcommand) must never see a partial write. Write to a temp file, fsync, rename. Matches agent-mux at `dispatch/status.go:30-70`.

### 4. ULID for dispatch IDs

**Why:** Time-ordered, lexicographically sortable, no coordination needed. Matches agent-mux's choice. The `ulid` crate is well-maintained.

## Risks / Trade-offs

**[Risk] Codex NDJSON format changes across versions** -> Mitigated by parsing known event types and treating unknown types as `RawPassthrough`. The parser is forward-compatible.

**[Risk] Process group behavior differs across macOS/Linux** -> Mitigated by testing on both platforms. The `nix` crate abstracts most differences. Parent-death detection has platform-specific implementations (kqueue vs prctl), matching agent-mux's approach.

**[Risk] 4MB line buffer may not be enough for very large Codex responses** -> Matched to agent-mux's choice at `loop.go:89`. If a single NDJSON line exceeds 4MB, it's treated as a scan error. This hasn't been observed in practice.

**[Risk] Async dispatch store at ~/.darkfactory/dispatches/ grows unbounded** -> Mitigated by pruning dispatches older than 7 days on each new async dispatch.
