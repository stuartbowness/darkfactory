# Handoff: Task 5 - Shadow compare mode and CLI

## Task

Add a read-only `core::shadow` compare mode and a real `darkfactory-render` CLI
binary (replacing the `src/main.rs` stub) with `render` and `shadow`
subcommands. The CLI does all file IO and maps results to exit codes and stderr;
`core` stays IO-free; `shadow` is strictly read-only.

## Commit SHA

`24fe71ec3ebcbd1d8194fa10ef236e9ad32f9dbe`

## Approach

### `Diff` type shape and `compare`

- `pub enum Diff { Identical, Different(String) }` in `core::shadow`.
- `pub fn compare(rendered: &str, existing: &str) -> Diff`. Pure, no IO.
  Identical inputs -> `Identical`; otherwise `Different(diff_text)`.
- The diff is computed in-house with a line-based longest-common-subsequence
  (LCS) walk (no `diff` crate, no external dep). Unchanged lines are omitted;
  a line only in `existing` (the old side) is emitted `- <line>`, a line only in
  `rendered` (the new side) is emitted `+ <line>`, joined with `\n`. Inputs are
  short contract documents, so the quadratic LCS table is fine.

### CLI arg approach

- All logic lives in `pub mod cli` in the library; `src/main.rs` is a thin shell
  that collects `std::env::args().skip(1)`, locks real stdout/stderr, calls
  `cli::run(&args, &mut out, &mut err) -> i32`, and exits with that code. Keeping
  `run` in the lib over generic `Write` streams lets it be unit-tested
  in-process AND end-to-end via `std::process::Command`.
- Argument parsing is stdlib-only (`std::env::args`). A tiny `Flags` parser
  collects `--flag value` pairs; `take(name)` consumes a required flag (order
  independent) and `finish()` fails loud on any leftover (unknown or duplicate)
  flag. No `clap`.
- Subcommands and flags:
  - `render --specs <dir> --tasks <md> --fragment <md>`
  - `shadow --specs <dir> --tasks <md> --fragment <md> --existing <contract_md>`
- `--specs <dir>`: each immediate subdirectory `<capability>/` whose `spec.md`
  exists is parsed with `parse_specs(<capability>, content)`; the capability is
  the directory name. Subdirs are visited in sorted name order for deterministic
  output. A subdir without `spec.md` is skipped.

### Exit-code scheme

- `EXIT_USAGE = 2`: missing/unknown subcommand, missing required flag, unknown
  or duplicate flag, or a flag missing its value. Usage text printed to stderr.
- `EXIT_RENDER_ERROR = 2`: any `RenderError` OR a failed file read. A single
  actionable `error: <message>` line to stderr; nothing on stdout (no partial
  contract).
- `EXIT_DIFFERENT = 1`: `shadow` only, when the rendered contract differs from
  the existing one (so the future cutover gate can script it). Distinct from
  `EXIT_RENDER_ERROR` so "differences found" is not confused with "render
  failed".
- Success: `0`. `render` prints the contract to stdout, render warnings (incl.
  the inspection-cap breach) to stderr as `warning: <msg>`, exit 0. `shadow`
  prints `no differences` (identical) or the diff (different) to stdout, render
  warnings to stderr.

### How shadow stays read-only

- `run_shadow` only ever READS: it runs the same render pipeline (which reads
  specs/tasks/fragment), reads `--existing` with `fs::read_to_string`, then
  `compare`s the two in-memory strings and prints the result. There is no
  `fs::write`, no rename, no create anywhere on the shadow path. No auto-cutover.
- Proven by an integration test that copies the fixture tree to a temp dir,
  snapshots every file's (relative path, bytes, mtime), runs `shadow` against
  the DIFFERING contract (the most work path), and asserts the snapshot is
  byte-and-mtime identical afterward and no file appeared.

## Files added / changed (all under `tools/darkfactory-render/`)

- `src/core/shadow.rs` (new): `Diff`, `compare`, in-house LCS line diff, 5 unit
  tests.
- `src/cli.rs` (new): the CLI transport (`run`, `render`/`shadow` handlers, file
  IO, `Flags` parser), 5 unit tests for arg handling.
- `src/main.rs`: replaced the stub with the thin shell calling `cli::run`.
- `src/lib.rs`: added `pub mod cli;` and `pub mod shadow;` under `core`.
- `tests/cli.rs` (new): 9 end-to-end tests driving the built binary via
  `std::process::Command` + `env!("CARGO_BIN_EXE_darkfactory-render")`.
- `tests/fixtures/cli/**` (new): specs/tasks/fragment, captured identical and
  hand-edited different contracts, a bad-spec dir (dangling task link) for the
  RenderError path, and an inspection-heavy spec+tasks for the warning path.

## Commands run + exit codes

- `cargo test --lib shadow` -> 5 passed.
- `cargo test --lib` -> 55 passed.
- `cargo test --test cli` -> 9 passed.
- `cargo fmt --check` -> exit 0 (clean).
- `cargo clippy --all-targets -- -D warnings` -> exit 0 (no issues).
- `cargo test` -> 70 passed across 7 suites, exit 0.
- `make check` (fmt + lint + test) -> exit 0.

## Dependencies

No new dependency. Cargo.toml's only `[dependencies]` entry is still
`thiserror = "2"`. Cargo.lock holds only the `thiserror` chain (proc-macro2,
quote, syn, thiserror, thiserror-impl, unicode-ident).

## Concerns

- `compare(rendered, existing)` maps to `line_diff(existing, rendered)`, so in
  the diff the EXISTING (hand-authored) contract is the `-` side and the RENDERED
  contract is the `+` side. Read it as "to match the renderer, change `-` to
  `+`." Downstream tooling that parses the diff direction should rely on this.
- The `--specs` reader skips subdirectories with no `spec.md` silently (it is not
  an error to have a non-spec subdir). A completely empty/absent specs dir is a
  read error (loud). This matches the OpenSpec `specs/<cap>/spec.md` layout.
- `EXIT_USAGE` and `EXIT_RENDER_ERROR` are both `2` by design: usage errors and
  render failures are both "the command could not produce a contract," and
  shadow's only special code is `1` for "rendered != existing." If Task 6/7 need
  to distinguish a usage error from a render failure, split these constants.
