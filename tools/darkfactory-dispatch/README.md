# darkfactory-dispatch

A process supervisor for Codex CLI subprocesses. Sits between the orchestrator and a Codex worker, handling process group isolation, two-phase timeout, heartbeat events, and structured result reporting.

## Why

When Darkfactory dispatches a Codex worker, the orchestrator needs to know: is the worker still alive? Did it finish? Did it time out? What files did it change? How many tokens did it use?

Without the dispatcher, the orchestrator calls `codex exec` directly as a raw subprocess. If Codex hangs (stdin stall, long reasoning silence, subprocess wedge), the orchestrator waits forever. There are no heartbeats, no timeout, no structured results -- just a log file and hope.

The dispatcher solves this by wrapping the Codex subprocess in a supervisor that:

- **Isolates the process group** so killing Codex also kills any shell commands it spawned
- **Enforces a two-phase timeout** -- SIGTERM at the soft timeout, SIGKILL after the grace period
- **Emits heartbeat events** every 15 seconds so the orchestrator can distinguish "thinking" from "dead"
- **Parses Codex NDJSON events** in real-time, tracking files changed, commands run, and tokens consumed
- **Returns a structured JSON result** with status, exit code, signal, token usage, and activity summary

Claude workers don't use the dispatcher. They use Claude Code's built-in Agent tool, which provides its own lifecycle management.

## Architecture

```
Orchestrator (Claude Code /mission)
  │
  │ calls darkfactory-dispatch --json --prompt-file brief.md ...
  │
  ▼
Dispatcher (this binary)
  │
  │ spawns codex exec --json ... in its own process group
  │ parses NDJSON on stdout, captures stderr
  │ emits heartbeats to stderr every 15s
  │ enforces two-phase timeout (SIGTERM → grace → SIGKILL)
  │
  ▼
Worker (codex exec process)
  │
  │ reads files, writes code, runs commands, commits
  │ writes handoff report to --output path
  │ emits NDJSON events on stdout
  │
  ▼
Dispatch Result (JSON on stdout)
  status, exit_code, signal, duration_ms, tokens, activity
```

Three layers, each with a single responsibility. The orchestrator owns planning and decisions. The dispatcher owns process supervision. The worker owns code generation.

## Installation

Requires the [Rust toolchain](https://rustup.rs/).

```bash
cd ~/.darkfactory/tools/darkfactory-dispatch
make install
```

This compiles a release binary and copies it to `~/.local/bin/darkfactory-dispatch`. Ensure `~/.local/bin` is on your PATH:

```bash
# Add to ~/.zshrc or ~/.bashrc if not already present
export PATH="$HOME/.local/bin:$PATH"
```

Verify:

```bash
darkfactory-dispatch --version
```

## Usage

### Sync dispatch (most common)

Dispatch a task and wait for the result:

```bash
darkfactory-dispatch \
  --model gpt-5.5 \
  --effort high \
  --timeout 1800 \
  --sandbox workspace-write \
  --config 'sandbox_workspace_write.writable_roots=[".git"]' \
  --prompt-file brief.md \
  --output handoff.md \
  --cwd /path/to/repo \
  --log dispatch.log \
  --json 2>>dispatch.log
```

The dispatch result is a single JSON object on stdout:

```json
{
  "status": "completed",
  "dispatch_id": "01KQ3XPQZM6R34H360BF6B8W82",
  "duration_ms": 53311,
  "exit_code": 0,
  "signal": null,
  "output": "handoff.md",
  "output_exists": true,
  "error": null,
  "tokens": {
    "input": 136538,
    "output": 1407,
    "reasoning": 0
  },
  "activity": {
    "files_changed": ["src/lib.rs", "src/main.rs"],
    "files_read": ["Cargo.toml", "README.md"],
    "commands_run": ["cargo test", "cargo clippy"],
    "tool_calls": 5,
    "parse_errors": 0
  }
}
```

### Async dispatch

Dispatch a task and return immediately:

```bash
darkfactory-dispatch \
  --async \
  --model gpt-5.5 \
  --effort high \
  --timeout 1800 \
  --prompt "Refactor the auth module"
```

Returns `{"kind":"async_started","dispatch_id":"..."}` on stdout. The dispatch continues in the background. Manage the lifecycle with subcommands:

```bash
darkfactory-dispatch status <dispatch-id>   # Check state (running/completed/failed/timed_out/orphaned)
darkfactory-dispatch wait <dispatch-id>     # Block until complete
darkfactory-dispatch result <dispatch-id>   # Get the full dispatch result JSON
```

Async state lives in the dispatch store at `~/.darkfactory/dispatches/<dispatch-id>/`, containing `status.json`, `result.json`, `host.pid`, and `events.jsonl`. Dispatch directories older than 7 days are pruned automatically.

### Monitoring progress

The dispatcher emits NDJSON events to stderr during a dispatch:

```jsonl
{"type":"dispatch_start","dispatch_id":"...","ts":"...","engine":"codex","model":"gpt-5.5"}
{"type":"heartbeat","dispatch_id":"...","ts":"...","elapsed_s":15,"last_activity":"running: cargo test"}
{"type":"command_run","dispatch_id":"...","ts":"...","command":"cargo test"}
{"type":"file_write","dispatch_id":"...","ts":"...","file":"src/lib.rs"}
{"type":"heartbeat","dispatch_id":"...","ts":"...","elapsed_s":30,"last_activity":"writing: src/lib.rs"}
{"type":"timeout_warning","dispatch_id":"...","ts":"...","elapsed_s":1800}
{"type":"dispatch_end","dispatch_id":"...","ts":"...","status":"completed","duration_ms":53311}
```

To watch progress live, redirect stderr to the log file and tail it:

```bash
darkfactory-dispatch ... --json 2>>dispatch.log &
tail -f dispatch.log
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--model` | `gpt-5` | Codex model to use |
| `--effort` | (none) | Reasoning effort (maps to `-c model_reasoning_effort`) |
| `--timeout` | `900` | Seconds before soft timeout fires (SIGTERM) |
| `--grace` | half of timeout, min 1s | Seconds between soft timeout and hard timeout (SIGKILL) |
| `--sandbox` | `workspace-write` | Codex sandbox mode |
| `--config` | (none) | Repeatable. Key=value passed to Codex `-c` flag |
| `--prompt` | (none) | Task prompt (mutually exclusive with `--prompt-file`) |
| `--prompt-file` | (none) | Read prompt from file (for large briefs) |
| `--output` | (none) | Path for Codex to write results (passed to `-o`) |
| `--cwd` | current dir | Working directory for the Codex process |
| `--log` | (none) | File to capture Codex stdout and stderr |
| `--json` | false | Emit dispatch result as JSON on stdout (mutually exclusive with `--async`) |
| `--async` | false | Return immediately with dispatch ID (mutually exclusive with `--json`) |

## Two-phase timeout

The dispatcher enforces a configurable two-phase timeout:

1. **Soft timeout** (at `--timeout` seconds): Sends SIGTERM to the process group and emits a `timeout_warning` event. The worker has a chance to flush partial results.
2. **Hard timeout** (at `--timeout` + `--grace` seconds): Sends SIGKILL to the process group. The worker is forcibly terminated.

The grace period defaults to half the timeout with a minimum of 1 second. The hard timeout has a floor of 10 seconds to allow the worker time to flush final events.

If the worker exits cleanly after SIGTERM (during the grace period), the dispatch result reports `timed_out` with the worker's final output preserved.

## Process group isolation

Every Codex worker runs in its own process group (via `setpgid`). When the dispatcher sends SIGTERM or SIGKILL, the signal goes to the entire group -- not just the Codex process, but every shell command and subprocess it spawned. This prevents orphaned processes from accumulating.

## Parent-death detection

In sync mode, the dispatcher detects when its parent process (the orchestrator) dies and kills the worker to prevent orphans:

- **macOS**: Uses kqueue with `EVFILT_PROC` + `NOTE_EXIT` to watch the parent PID
- **Linux**: Uses `PR_SET_PDEATHSIG` to arrange SIGKILL on parent death

In async mode, parent-death detection is disabled so the dispatcher survives the caller exiting.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Dispatch completed successfully |
| 1 | Dispatch failed or timed out |
| 2 | CLI error (bad arguments, missing prompt, file read error) |

## Dispatch result status

| Status | Meaning |
|--------|---------|
| `completed` | Worker exited 0 and output file exists (if `--output` was specified) |
| `timed_out` | Two-phase timeout fired. `error.retryable` is true. |
| `failed` | Worker exited non-zero, output file missing, or spawn error |

## Development

```bash
make build     # Debug build
make test      # Run 66 Rust tests
make lint      # cargo clippy -D warnings
make fmt       # cargo fmt --check
make check     # fmt + lint + test
make smoke     # Live-codex end-to-end tests (requires codex CLI)
make install   # Release build + copy to ~/.local/bin/
```

See [CLAUDE.md](CLAUDE.md) for code standards, module structure, and testing rules.
