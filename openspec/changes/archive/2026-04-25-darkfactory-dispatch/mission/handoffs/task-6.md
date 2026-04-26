Implemented Task 6 CLI sync dispatch in:

- [src/cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/cli.rs:10): clap CLI, prompt validation, prompt-file loading, `DispatchSpec` construction, JSON/human output formatting, result exit-code mapping.
- [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs:35): parses args, routes lifecycle stubs, generates ULID, creates `Emitter`, calls `engine::dispatch`.
- [tests/cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/cli.rs:4): assert_cmd smoke tests for help, version, missing prompt, and mutual exclusions.

Verification passed:

- `cargo fmt`: exit 0
- `cargo clippy --all-targets -- -D warnings`: exit 0
- `cargo test`: exit 0

I could not create the commit or write `openspec/changes/darkfactory-dispatch/mission/handoffs/task-6.md`. This session can write the tool directory, but the Git root is `/Users/stuart/.darkfactory`; writing `/Users/stuart/.darkfactory/.git/index.lock` and the OpenSpec handoff path is blocked by the sandbox.

Handoff content to record:

```text
Task: Task 6 - CLI module (sync dispatch)
Commit SHA: not created; sandbox blocked writing /Users/stuart/.darkfactory/.git/index.lock
Approach: Implemented a thin clap-based CLI boundary in src/cli.rs, wired main.rs to validate dispatch flags, build DispatchSpec, generate a ULID, run engine::dispatch for sync mode, and format either JSON or human-readable output. Lifecycle subcommands route to explicit stubs for Task 6.2.
Commands run:
- cargo fmt: exit 0
- cargo clippy --all-targets -- -D warnings: exit 0
- cargo test: exit 0
Concerns: The task brief leaves the default model unspecified even though DispatchSpec requires one, so omitted --model defaults to gpt-5.
```