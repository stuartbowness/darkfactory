# Worker Brief: Task 6 - CLI module (sync dispatch)

## Your Task
Implement `src/cli.rs` and update `src/main.rs` to wire up the full CLI with sync dispatch. This task makes `darkfactory-dispatch` a working binary that can dispatch codex workers.

### What to implement

**CLI structure (clap derive):**
```rust
#[derive(Parser)]
#[command(name = "darkfactory-dispatch", version, about = "Process supervisor for Codex CLI subprocesses")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    // Dispatch flags (used when no subcommand):
    #[arg(long)]
    pub(crate) model: Option<String>,
    
    #[arg(long)]
    pub(crate) effort: Option<String>,
    
    #[arg(long, default_value = "900")]
    pub(crate) timeout: u64,
    
    #[arg(long)]
    pub(crate) grace: Option<u64>,
    
    #[arg(long, default_value = "workspace-write")]
    pub(crate) sandbox: String,
    
    #[arg(long = "config", num_args = 1)]
    pub(crate) configs: Vec<String>,
    
    #[arg(long, conflicts_with = "prompt_file")]
    pub(crate) prompt: Option<String>,
    
    #[arg(long, conflicts_with = "prompt")]
    pub(crate) prompt_file: Option<PathBuf>,
    
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    
    #[arg(long)]
    pub(crate) cwd: Option<PathBuf>,
    
    #[arg(long)]
    pub(crate) log: Option<PathBuf>,
    
    #[arg(long, conflicts_with = "async_mode")]
    pub(crate) json: bool,
    
    #[arg(long = "async", conflicts_with = "json")]
    pub(crate) async_mode: bool,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Status { id: String },
    Result { id: String },
    Wait { id: String },
}
```

**main.rs flow:**
1. Parse CLI args
2. If subcommand (status/result/wait) -> route to lifecycle handler (stub for now, task 6.2)
3. If no subcommand -> validate dispatch flags, build DispatchSpec, run sync dispatch
4. Sync dispatch: generate ULID, create Emitter, call `engine::dispatch(spec, id, emitter)`, emit DispatchResult JSON to stdout, exit with code 0/1/2

**Validation:**
- If neither `--prompt` nor `--prompt-file` is given -> exit 2 with error
- If `--prompt-file` is given, read the file contents as the prompt
- Grace default: if not specified, use timeout/2 with minimum 1s
- `--effort` maps to DispatchSpec.effort (engine handles the codex `-c` flag)

**Exit codes:**
- 0: dispatch completed (status == Completed)
- 1: dispatch failed or timed out (status == Failed or TimedOut)
- 2: CLI error (bad args, missing prompt, file read error)

**JSON output (--json flag):**
When `--json` is set, stdout is the DispatchResult serialized as JSON. When not set, print a human-readable summary (status, duration, output path).

### Tests to write

Write CLI smoke tests in `tests/cli.rs` via `assert_cmd`:

1. **--help**: succeeds, output contains all flag names
2. **--version**: succeeds, prints version
3. **Missing prompt**: no --prompt or --prompt-file -> exit 2
4. **Mutual exclusion (prompt)**: both --prompt and --prompt-file -> exit 2
5. **Mutual exclusion (json/async)**: both --json and --async -> exit 2

### Code standards
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Gallant boundary: CLI only parses input, calls engine::dispatch, formats output
- `pub(crate)` visibility
- Exit codes: 0 success, 1 dispatch failure, 2 CLI error
- Update main.rs to use cli module

## What Previous Tasks Built
### Task 1-4: Foundation
- Cargo.toml, Makefile, supervisor, types, event emitter

### Task 5: Engine
- src/engine.rs: `dispatch(spec, id, emitter) -> Result<DispatchResult>`
- Codex arg construction, NDJSON parsing, two-phase timeout, heartbeat, watchdog, activity tracking
- 27 tests passing

## Downstream Consumers
Task 6.2 (async mode) will add the async dispatch path and lifecycle subcommands to this CLI.
Task 9.1 (smoke test) will run the binary end-to-end.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md`
- Implement src/cli.rs
- Rewrite src/main.rs to use cli module
- Write tests in tests/cli.rs
- Verify `cargo test` and `cargo clippy --all-targets -- -D warnings` pass
- Commit: "[task-6] Implement CLI module with sync dispatch, flag parsing, and exit codes"
- Write handoff to openspec/changes/darkfactory-dispatch/mission/handoffs/task-6.md

## Handoff Report Format
- Task: Task 6 - CLI module (sync dispatch)
- Commit SHA: <full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
