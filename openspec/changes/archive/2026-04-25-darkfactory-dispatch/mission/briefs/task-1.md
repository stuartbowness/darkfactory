# Worker Brief: Task 1 - Project scaffolding

## Your Task
Initialize the Rust crate at `tools/darkfactory-dispatch/` with:

1. **Cargo.toml** with these dependencies:
   - tokio = { version = "1", features = ["full"] }
   - serde = { version = "1", features = ["derive"] }
   - serde_json = "1"
   - clap = { version = "4", features = ["derive"] }
   - nix = { version = "0.29", features = ["signal", "process"] }
   - ulid = "1"
   - chrono = { version = "0.4", features = ["serde"] }
   - thiserror = "2"
   - tracing = "0.1"
   - tracing-subscriber = { version = "0.3", features = ["json"] }
   
   Dev dependencies:
   - assert_cmd = "2"
   - predicates = "3"

   Binary name: `darkfactory-dispatch`
   Edition: 2021
   
   Release profile (from crucible pattern):
   ```toml
   [profile.release]
   opt-level = "z"
   lto = true
   codegen-units = 1
   strip = true
   panic = "abort"
   ```

2. **src/main.rs** -- minimal stub that parses args and prints version:
   ```rust
   use clap::Parser;
   
   #[derive(Parser)]
   #[command(name = "darkfactory-dispatch", version, about = "Process supervisor for Codex CLI subprocesses")]
   struct Cli {
       #[command(subcommand)]
       command: Option<Commands>,
   }
   
   #[derive(clap::Subcommand)]
   enum Commands {
       Status { id: String },
       Result { id: String },
       Wait { id: String },
   }
   
   #[tokio::main]
   async fn main() {
       let _cli = Cli::parse();
       println!("darkfactory-dispatch stub");
   }
   ```

3. **Makefile** with targets:
   - `build`: `cargo build`
   - `release`: `cargo build --release`
   - `install`: `cargo build --release && cp target/release/darkfactory-dispatch ~/.local/bin/`
   - `test`: `cargo test`
   - `lint`: `cargo clippy --all-targets -- -D warnings`
   - `fmt`: `cargo fmt --check`
   - `check`: fmt + lint + test (all three)

4. **.gitignore**: `target/`

5. **Empty module files** as placeholders: `src/cli.rs`, `src/engine.rs`, `src/supervisor.rs`, `src/types.rs`, `src/event.rs` (just `// TODO: implement` comments for now, or empty)

6. Verify `cargo build` and `make check` pass.

## Project Context
We're building a Rust process supervisor for Codex CLI subprocesses. This is the scaffolding task -- get the project compiling with all dependencies declared.

## Architecture
Single crate with modules: main.rs (entry), cli.rs, engine.rs, supervisor.rs, types.rs, event.rs. See `tools/darkfactory-dispatch/CLAUDE.md` for full code standards.

## Key Terms
- **Dispatcher**: The darkfactory-dispatch binary
- **Gallant pattern**: Engine owns ALL logic, CLI only parses and formats

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
All subsequent tasks build on this scaffolding. Task 2.1 (supervisor) will implement src/supervisor.rs. Task 3.1 (types) will implement src/types.rs.

## Procedures
- Read `tools/darkfactory-dispatch/CLAUDE.md` first for code standards
- Create all files listed above
- Verify `cargo build` and `make check` pass
- Commit with message: "[task-1] Initialize darkfactory-dispatch Rust crate with dependencies and Makefile"
- Write your handoff report to openspec/changes/darkfactory-dispatch/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Project scaffolding
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
