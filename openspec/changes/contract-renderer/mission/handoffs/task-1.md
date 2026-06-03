# Handoff: Task 1 - Crate skeleton, domain types, RenderError, Makefile

- **Task:** Task 1 - Crate skeleton, domain types, RenderError, Makefile
- **Commit SHA:** feb7b283218539a525ccc90d552a78c666988120

## Approach

Scaffolded a new standalone (non-workspace) crate at `tools/darkfactory-render`,
mirroring `tools/darkfactory-dispatch` conventions:

- `Cargo.toml`: package `darkfactory-render`, `version = "1.0.0"` (pinned),
  `edition = "2021"`, single dependency `thiserror = "2"`, and the identical
  `[profile.release]` block dispatch uses (`opt-level = "z"`, `lto`,
  `codegen-units = 1`, `strip`, `panic = "abort"`).
- `.gitignore` with `target/`, and a `Makefile` mirroring dispatch's targets
  (`build`, `release`, `install`, `test`, `lint`, `fmt`, `check`, `smoke`).
  `install` copies the `darkfactory-render` binary; `lint` runs
  `cargo clippy --all-targets -- -D warnings`; `fmt` runs `cargo fmt --check`;
  `check` runs fmt + lint + test. I included a `smoke` target pointing at
  `./tests/smoketest.sh` per the brief (the script itself lands in Task 7).

Crate shape is a library + thin binary in one package:

- `src/lib.rs` is the library root, exposing `pub mod core { pub mod types; }`,
  so the catalog is reached as `crate::core::types`.
- `src/core/types.rs` holds the whole Type Catalog and `RenderError`. I used the
  idiomatic Cargo directory layout (`src/core/types.rs`) rather than a `#[path]`
  hack on `src/types.rs`. Rust's default module resolution maps the inline
  `pub mod types;` under `pub mod core` to `src/core/types.rs`, which is the
  clean way to honor "lib exposes `core`; `core` exposes `types`". (See
  Concerns.)
- `src/main.rs` is a tiny stub that prints
  `darkfactory-render: CLI not yet implemented` and exits 0. The real CLI is
  Task 5.

All nine catalog types exist with the exact required names: `AcceptanceEntry`,
`VerificationMethod`, `SpecReference`, `ScenarioStatus`, `RequirementScenario`,
`ClosingTaskFragment`, `TaskList`, `RenderedContract`, `RenderError`. Field
shapes use simple owned data (`String`, `u32`, `Vec`, `Option`). Two small
supporting structs were added where the design implied structure:
`ClosingTaskKind` (a kind name plus its checks, held by `ClosingTaskFragment`)
and `Task` (number, name, optional closing-kind, held by `TaskList`).

Behavior implemented (the only behavior in this task) and the TDD target:

- `VerificationMethod::parse(&str)` parses `execution` / `output` / `inspection`;
  an unknown token returns `RenderError::UnknownVerificationMethod(token)`.
- `ScenarioStatus::parse(&str)` parses the five status tokens; an unknown token
  returns `RenderError::UnknownStatus(token)`.
- `ScenarioStatus::parse_optional(Option<&str>)` models omitted status as
  `Ok(None)` (non-deviated); a present-but-unknown value is `UnknownStatus`. An
  `is_deviated()` helper makes the non-deviated semantics explicit.

`RenderError` is a single `thiserror::Error` enum with all ten variants
(`UnknownVerificationMethod`, `EmptyStatement`, `EmptyAcceptanceList`,
`MissingSpecRef`, `DuplicateSpecReference`, `UnknownStatus`, `DanglingTaskLink`,
`AcceptanceLinksClosingTask`, `FeatureTaskNoChecks`, `UnknownClosingKind`), each
with a clear `#[error("...")]` message and context payloads where obvious. The
core returns the typed error and never prints.

I followed the `/tdd` red->green workflow on the parsing behavior: wrote the
parse tests as the spec of observable behavior, drove the `parse` /
`parse_optional` implementations to green, then confirmed the full suite. 11
unit tests cover both enums' happy paths, unknown-token errors, the
omitted-status-is-non-deviated rule, and stable `RenderError` Display messages.

## Commands run

- `cd .../tools/darkfactory-render && make check` -> exit 0
  - `cargo fmt --check` -> clean (no diffs)
  - `cargo clippy --all-targets -- -D warnings` -> 0 warnings
  - `cargo test` -> 11 passed, 0 failed (plus 0 doc-tests, 0 bin tests)
- `cargo run -q` -> printed `darkfactory-render: CLI not yet implemented`, exit 0
- `git add tools/darkfactory-render` (staged 7 files; `target/` ignored;
  nothing under `openspec/changes/` staged)
- `git commit -m "[task-1] crate skeleton, domain types, RenderError, Makefile"`
  -> commit feb7b283218539a525ccc90d552a78c666988120
- `git show --stat HEAD` -> 7 files, all under `tools/darkfactory-render/`

## Concerns

- File location nuance: the brief's prose says `src/types.rs`, but the same
  brief requires the module path `crate::core::types` and the standard Cargo
  layout for an inline `pub mod core { pub mod types; }` resolves to
  `src/core/types.rs`. I chose the idiomatic directory layout
  (`src/core/types.rs`) to keep module resolution clean and avoid a `#[path]`
  attribute. If the orchestrator strictly requires the file to sit at
  `src/types.rs`, it would need a `#[path]` override; I judged the clean layout
  the better call. All type names and the `crate::core::types` path are exactly
  as specified.
- `ClosingTaskFragment`, `TaskList`, `RenderedContract`, and several fields are
  not read yet (later tasks consume them). They compile warning-free without any
  `dead_code` suppression because the whole catalog is `pub` from the library
  root and is therefore reachable public API.
