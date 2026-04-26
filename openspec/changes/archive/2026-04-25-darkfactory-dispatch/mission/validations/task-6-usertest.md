# Task 6.1 User-Test Validation

**Commit:** 2f23097  
**Date:** 2026-04-23  
**Test command:** `cargo test` -- 32 passed (4 suites, 6.77s)

## Execution Assertions

### [PASS] `darkfactory-dispatch --help` succeeds and lists all flags

`cargo run -- --help` exits 0 and the output contains every required flag:
`--model`, `--effort`, `--timeout`, `--grace`, `--sandbox`, `--config`, `--prompt`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json`, `--async`, `--version`.

Integration test: `help_lists_dispatch_flags` (tests/cli.rs:5) asserts all 14 flags with `predicate::str::contains`.

### [PASS] `darkfactory-dispatch --version` prints a version string

`cargo run -- --version` exits 0 and prints `darkfactory-dispatch 0.1.0`.

Integration test: `version_prints_package_version` (tests/cli.rs:27) asserts the output contains `CARGO_PKG_VERSION`.

### [PASS] Providing both `--prompt` and `--prompt-file` exits with code 2 and an error message

`cargo run -- --prompt test --prompt-file test.md` exits with code 2 and prints:
`error: the argument '--prompt <PROMPT>' cannot be used with '--prompt-file <PROMPT_FILE>'`

Clap enforces this via `conflicts_with` on both fields (src/cli.rs:38,41). Integration test: `prompt_and_prompt_file_are_mutually_exclusive` (tests/cli.rs:46).

### [PASS] Providing both `--json` and `--async` exits with code 2 and an error message

`cargo run -- --json --async --prompt test` exits with code 2 and prints:
`error: the argument '--json' cannot be used with '--async'`

Clap enforces this via `conflicts_with` on both fields (src/cli.rs:53,56). Integration test: `json_and_async_are_mutually_exclusive` (tests/cli.rs:54).

### [PASS] Providing neither `--prompt` nor `--prompt-file` exits with code 2 and an error message

`cargo run --` (no arguments) exits with code 2 and prints:
`error: missing prompt: provide --prompt or --prompt-file`

The `dispatch_spec()` method returns `Err` for the `(None, None)` case (src/cli.rs:73-75), and `main.rs` maps CLI errors to exit code 2 via `cli_error` (src/main.rs:62-64). Integration test: `missing_prompt_exits_with_cli_error` (tests/cli.rs:37).

## Output Assertions

### [PASS] `--effort high` results in codex receiving `-c model_reasoning_effort="high"`

`build_codex_args` in src/engine.rs:172-175 pushes `-c` followed by `model_reasoning_effort="high"` when effort is set. Unit test `codex_args_include_expected_flags` (src/engine.rs:709) asserts the exact argument vector includes `"-c", "model_reasoning_effort=\"high\""`.

### [PASS] `--config key=value` results in codex receiving `-c key=value`

`build_codex_args` in src/engine.rs:176-179 iterates over configs and pushes `-c` followed by the raw config string. Unit test `codex_args_include_expected_flags` (src/engine.rs:709) asserts `"-c", "foo=bar"` in the argument vector.

### [PASS] Default timeout is 900 seconds when `--timeout` is not specified

The `timeout` field in `Cli` has `#[arg(long, default_value = "900")]` (src/cli.rs:27). The `--help` output confirms `[default: 900]`. This value flows through `dispatch_spec()` to `DispatchSpec.timeout_sec`.

## Summary

All 8 assertions pass. 32 tests pass across 4 test suites. No failures.
