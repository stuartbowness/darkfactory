# darkfactory-dispatch

Rust process supervisor for Codex CLI subprocesses. Spawns `codex exec` with process group isolation, two-phase timeout, NDJSON heartbeat events, and structured JSON results.

## Repo map

```text
src/
  main.rs         Entrypoint: parse args, dispatch synchronously, print result
  cli.rs          Clap CLI structure, argument parsing
  engine.rs       Core dispatch loop: codex arg construction, NDJSON parsing,
                  timeout select loop, heartbeat, result finalization
  supervisor.rs   Process spawning, process groups, signals, GracefulStop,
                  parent-death detection (kqueue on macOS, prctl on Linux)
  types.rs        DispatchSpec, DispatchResult, DispatchError, HarnessEvent,
                  TokenUsage, Activity
  event.rs        NDJSON event emitter, heartbeat ticker
tests/
  cli.rs          Integration tests (binary smoke tests via assert_cmd)
  supervisor.rs   Process group and signal tests
  engine.rs       Timeout and heartbeat tests with mock child processes
Makefile          build, install (~/.local/bin/), test, lint, fmt, check
```

## Commands

```bash
cargo build
cargo clippy --all-targets -- -D warnings
cargo test
cargo fmt --check
make install     # builds release, copies to ~/.local/bin/
make check       # fmt + lint + test
```

## Rules

- **Gallant pattern** (boundary rule): Engine owns ALL dispatch logic. CLI MUST only parse input, call engine, and format output. This governs which module a function belongs in.
- **Deep modules**: Each module has a small public interface hiding substantial complexity. Prefer fewer entry points over many small helpers. Do not extract pure functions into separate modules just for testability -- test at the module boundary instead. Three similar lines inside one module is better than a premature abstraction split across two.
- **Error types**: `thiserror` in engine and supervisor for typed errors (`DispatchError`). CLI converts at the boundary. NEVER `anyhow` in engine or supervisor.
- **Visibility**: All items MUST use `pub(crate)` unless they are part of the crate's public API (called from `main.rs`).
- **Lint suppression**: Use `#[expect(...)]` instead of `#[allow(...)]` so the compiler warns when the suppression is no longer needed.
- **Serde**: NEVER use `#[serde(flatten)]` or `#[serde(untagged)]` -- both cause silent, hard-to-debug serialization issues.
- **Output**: Stdout for machine-readable data (DispatchResult JSON), stderr for diagnostics (heartbeats, events, warnings).
- **No double-logging**: Propagate errors with `?`. Log only when the error is finally handled. NEVER log and propagate.
- **Exit codes**: 0 = success (completed), 1 = dispatch failure (failed, timed_out), 2 = CLI error (bad args, missing binary).
- **Codex-only**: No adapter trait, no engine flag. Build codex arg construction and NDJSON parsing directly. If a second engine is added later, extract the trait then.

## Reference implementation

The dispatch machinery is modeled on buildoak/agent-mux (`~/Code/agent-mux`). Key reference files:

| Module | agent-mux file | What to learn |
|---|---|---|
| supervisor | `internal/supervisor/supervisor.go` | Setpgid, -pgid signal delivery, GracefulStop, WasSignaled |
| supervisor | `internal/supervisor/reaper_darwin.go` | kqueue EVFILT_PROC + NOTE_EXIT |
| supervisor | `internal/supervisor/reaper_linux.go` | PR_SET_PDEATHSIG via prctl |
| engine | `internal/engine/loop.go:167-981` | Dispatch function, two-phase timeout, heartbeat |
| engine | `internal/engine/adapter/codex.go` | codex exec arg construction + NDJSON event parsing |
| engine | `internal/event/event.go` | Event emitter, heartbeat ticker |
| types | `internal/types/types.go` | DispatchSpec, DispatchResult, HarnessAdapter |
| cli | `cmd/agent-mux/main.go` | CLI routing, flag parsing |

## Testing

Tests required when a change could break silently (process management, NDJSON parsing, timeout logic, CLI argument construction, result serialization). NOT required for docs-only changes.

| Change in | Tests go in | Needs subprocess? |
|---|---|---|
| `src/supervisor.rs` | `tests/supervisor.rs` | Yes (real processes) |
| `src/engine.rs` | `tests/engine.rs` | Yes (mock child binary) |
| `src/cli.rs` or `src/main.rs` | `tests/cli.rs` | Yes (`assert_cmd`) |
| `src/types.rs` | `#[cfg(test)]` same file | No |
| `src/event.rs` | `#[cfg(test)]` same file | No |

- **No mocks for process behavior**: Test through real subprocesses. Use small test helper scripts that simulate codex behavior.
- **CLI tests**: `Command::cargo_bin("darkfactory-dispatch")` via `assert_cmd`.
- **Smoke tests**: `make smoke` runs `tests/smoketest.sh` against real codex (requires codex CLI authenticated). Run after any change to the dispatch lifecycle.
- **Bug fix** -> regression test that fails without the fix.
- **Tokio tests**: All async tests MUST use `#[tokio::test(flavor = "multi_thread")]`.

## Versioning

The Cargo.toml version is pinned at 1.0.0 and never bumped. The darkfactory repo release tag (v1.4.x) is the only version that matters. Do not bump the crate version when making changes or creating releases.

## Secrets -- do not commit

No secrets in this project. The Codex CLI handles its own authentication.
