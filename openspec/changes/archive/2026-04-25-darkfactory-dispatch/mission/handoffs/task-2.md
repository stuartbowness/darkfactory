Implemented the supervisor module and tests.

Changed:
- [src/supervisor.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/supervisor.rs:1): process group spawning, stdin mode, stdout/stderr pipes, idempotent wait, SIGTERM/SIGKILL group signaling, exit code/signal detection, macOS kqueue and Linux `prctl` parent-death hooks.
- [src/main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs:3): added `mod supervisor;`.
- [tests/supervisor.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/tests/supervisor.rs:15): added process-group kill, graceful-stop timing, and signal detection tests.

Verification passed:
- `cargo fmt --check`
- `cargo test --test supervisor -- --test-threads=1`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`

Blocked:
- I could not update `openspec/.../tasks.md`, write `mission/handoffs/task-2.md`, or create the requested commit because the sandbox cannot write outside `tools/darkfactory-dispatch`; Git’s index is at `/Users/stuart/.darkfactory/.git`, and `git add` failed with `Operation not permitted`.
- Commit SHA is therefore not available.