# Task 6.1: CLI Module (Sync Dispatch) -- Scrutiny Validation

**Commit:** 2f23097  
**Validator:** Claude Opus 4.6 (1M context)  
**Date:** 2026-04-23  
**Verdict:** PASS

## 1. Test Presence

Integration tests in `tests/cli.rs` (61 lines, 5 tests):

- `help_lists_dispatch_flags` -- verifies all 14 flags appear in `--help` output
- `version_prints_package_version` -- checks `--version` prints `CARGO_PKG_VERSION`
- `missing_prompt_exits_with_cli_error` -- no args exits with code 2
- `prompt_and_prompt_file_are_mutually_exclusive` -- both flags exits with code 2
- `json_and_async_are_mutually_exclusive` -- both flags exits with code 2

Unit test in `src/engine.rs` (`codex_args_include_expected_flags`) covers effort and config arg construction.

## 2. Build

`cargo build` succeeds with no warnings.

## 3. Tests

`cargo test` passes: 32 tests across 4 suites (6.74s).

## 4. Lint

`cargo clippy --all-targets -- -D warnings` clean. `cargo fmt --check` clean.

## 5. Code Review

**Structure.** The commit moves the Clap `#[derive(Parser)]` struct and subcommand enum from `main.rs` into `cli.rs`, adds `dispatch_spec()` for converting CLI args to `DispatchSpec`, adds `print_result()` for output formatting, and adds `exit_code_for_result()` for mapping dispatch status to exit codes. `main.rs` is rewritten to wire CLI parsing to `engine::dispatch()`.

**Potential issues reviewed:**

- **Unreachable `(Some(_), Some(_))` branch in `dispatch_spec()`.** Clap's `conflicts_with` rejects this combination before `dispatch_spec()` runs, so the match arm is dead code. This is harmless belt-and-suspenders defense. Not a bug.
- **`dispatch_spec()` consumes `self`.** The method takes ownership of `Cli`, which means `json` and `async_mode` must be read before calling it. `main.rs` does this correctly at line 41 (`let json = cli.json;`).
- **`async_mode` is parsed but unused.** The field exists for the `conflicts_with` constraint and future async work. The `#[expect(dead_code)]` is not needed because clap's derive macro references it. Acceptable for an incremental build.

## 6. Module Depth (Gallant Boundary)

CLI module responsibilities (correct):
- Parse input (Clap derive, `dispatch_spec()`)
- Format output (`print_result()`, `exit_code_for_result()`)
- Error boundary (`cli_error()` in `main.rs`)

Engine module responsibilities (correct):
- Codex arg construction (`build_codex_args()`)
- Dispatch loop, timeout, NDJSON parsing
- Result finalization

No logic leakage. CLI does not construct codex args or make dispatch decisions. Engine does not format output or handle CLI errors. Boundary is clean.

## 7. Code Standards

- All items use `pub(crate)` visibility -- correct.
- `#[expect(dead_code)]` used instead of `#[allow(dead_code)]` -- correct.
- `thiserror` for engine errors, no `anyhow` -- correct.
- No `#[serde(flatten)]` or `#[serde(untagged)]` -- correct.
- Stdout for machine-readable data (JSON mode), stderr for diagnostics -- correct.

## 8. Simplification

No unnecessary abstractions. `print_result()` and `exit_code_for_result()` are small helpers that belong in CLI. No premature extraction.

## 9. Security

No secrets. No shell expansion of user input (prompt passed as process arg). No path traversal risk (output/log paths are used directly with fs APIs). Clean.

## 10. Contract Assertions

| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | `--help` lists all 14 flags | PASS | Executed `cargo run -- --help`; output contains --model, --effort, --timeout, --grace, --sandbox, --config, --prompt, --prompt-file, --output, --cwd, --log, --json, --async, --version |
| 2 | `--version` prints a version string | PASS | `darkfactory-dispatch 0.1.0` |
| 3 | `--prompt` + `--prompt-file` exits code 2 with error | PASS | Clap rejects with code 2: "cannot be used with" |
| 4 | `--json` + `--async` exits code 2 with error | PASS | Clap rejects with code 2: "cannot be used with" |
| 5 | Neither `--prompt` nor `--prompt-file` exits code 2 with error | PASS | Exits code 2: "missing prompt: provide --prompt or --prompt-file" |
| 6 | `--effort high` produces `-c model_reasoning_effort="high"` | PASS | `build_codex_args()` at engine.rs:173-174; unit test `codex_args_include_expected_flags` asserts exact args |
| 7 | `--config key=value` produces `-c key=value` | PASS | `build_codex_args()` at engine.rs:176-179; same unit test asserts `-c foo=bar` |
| 8 | Default timeout is 900 seconds | PASS | `cli.rs:26`: `#[arg(long, default_value = "900")]`; `--help` output shows `[default: 900]` |

## 11. Behavioral Conformance

The CLI module conforms to the contract. All execution assertions verified by running the binary. All output assertions verified by code inspection and unit tests. Exit code semantics match CLAUDE.md rules (0 = success, 1 = dispatch failure, 2 = CLI error).
