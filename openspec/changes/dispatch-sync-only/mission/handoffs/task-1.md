Done.

Committed the scoped cleanup as:

`07833a0773c08cbe7fa4745d18c4d05180671815`  
`[task-1] Delete dead test-only code`

Handoff written and left uncommitted at:

[task-1.md](/home/ironbank/.darkfactory/openspec/changes/dispatch-sync-only/mission/handoffs/task-1.md)

Verification passed from `tools/darkfactory-dispatch/`:

- `cargo build`
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`

Two environment notes are in the handoff: I had to use a writable `CARGO_HOME` under `/tmp`, and `TMPDIR=target/tmp` because this sandbox’s `/tmp` won’t execute the copied mock `codex` script.