# Worker Brief: Task 1 - Crate skeleton, domain types, RenderError, Makefile

## Your Task

Scaffold a new, standalone Rust crate at `tools/darkfactory-render` and define the project's domain vocabulary plus its fail-loud error type. This is the foundation every later task builds on; it does no parsing or rendering yet.

Concretely, produce:

1. **A new crate `tools/darkfactory-render`**, a sibling of `tools/darkfactory-dispatch`. It is its own independent package with its own `Cargo.toml` and `Cargo.lock` — it is NOT a Cargo workspace and is NOT a member of any workspace. Mirror the conventions of `tools/darkfactory-dispatch` (read its `Cargo.toml`, `Makefile`, and `src/` layout first).
   - `Cargo.toml`: package name `darkfactory-render`, `version = "1.0.0"` (pinned forever — internal tools are never version-bumped; only the repo release tag moves), `edition = "2021"`. Add the `thiserror` dependency (version `2`, matching dispatch). Include the same `[profile.release]` block dispatch uses (`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`). Do not add dependencies beyond `thiserror` for this task.
   - **Crate shape: a library + a thin binary in one package.**
     - `src/lib.rs` is the library root. It exposes a module named `core` (e.g. `pub mod core;`), and `core` exposes `pub mod types;`. Do NOT create empty `parse`, `render`, or `shadow` modules yet — later tasks add them; empty modules now are dead scaffolding. Naming a local module `core` is allowed in Rust 2021 (the standard `core` crate is reached via `::core`); reference your module as `crate::core::types`.
     - `src/types.rs` (i.e. `core::types`) holds every domain type below and the `RenderError` enum.
     - `src/main.rs` is a thin binary stub that compiles and runs. It must NOT implement the real CLI — the `render`/`shadow` subcommands are Task 5. A minimal `main` (for example, printing a short "darkfactory-render: CLI not yet implemented" line, or trivial no-op arg handling) is correct. Keep it tiny.

2. **The `core::types` domain types**, using these EXACT names (they are the project's Type Catalog and must match design.md):
   - `AcceptanceEntry` — one acceptance entry: a verbatim check statement, a `VerificationMethod`, and a task link (the integer task number it points at). The Spec Reference is inherited from the entry's scenario, so it need not be stored on the entry itself.
   - `VerificationMethod` — enum with exactly three variants for execution / output / inspection.
   - `SpecReference` — a `capability/N` scenario identifier (a capability name string + an integer N).
   - `ScenarioStatus` — enum with exactly five variants: proposed / approved / built / verified / deviated.
   - `RequirementScenario` — a parsed scenario: its `SpecReference`, its status, and its list of `AcceptanceEntry`.
   - `ClosingTaskFragment` — the standing source of operational checks; models the closing-task kinds and their checks. A minimal but real type is fine for this task (later tasks parse into it); do not leave it a bare empty struct if that makes it meaningless — give it the fields the design implies (kinds mapped to their checks).
   - `TaskList` — the ordered tasks from tasks.md, each a feature or closing task (carry task number, name, and the optional closing-kind).
   - `RenderedContract` — the output contract document model (the typed result a later render step produces).
   - `RenderError` — see below.

   Field-level shapes are yours to choose so long as the type NAMES match exactly and the types can carry the data the design describes. Prefer simple, owned data (`String`, `u32`/`usize`, `Vec`, `Option`).

3. **Value parsing for the two small enums** (this is the only behavior Task 1 implements, so this is what your tests target):
   - `VerificationMethod`: parse the bare method token (`execution`, `output`, `inspection`) into the matching variant. An unrecognized token returns `RenderError::UnknownVerificationMethod`. (Bracket/`[...]` handling belongs to the Task 2 line parser — Task 1 parses the inner word only.)
   - `ScenarioStatus`: parse `proposed` / `approved` / `built` / `verified` / `deviated` into the matching variant. An unrecognized token returns `RenderError::UnknownStatus`. Represent "status omitted" as non-deviated (e.g. an absent status modeled as `Option<ScenarioStatus> == None` meaning non-deviated, or an explicit default) — pick one and make it clear in the type. Do not parse an empty/omitted status into an error; only a present-but-unrecognized value is `UnknownStatus`.

4. **`RenderError`** — a single fail-loud error enum in `core::types` using `thiserror::Error` (as dispatch does). Define ALL of these variants now, even though only the first two are exercised in Task 1 (later tasks use the rest, and the enum is the project's complete error surface): `UnknownVerificationMethod`, `EmptyStatement`, `EmptyAcceptanceList`, `MissingSpecRef`, `DuplicateSpecReference`, `UnknownStatus`, `DanglingTaskLink`, `AcceptanceLinksClosingTask`, `FeatureTaskNoChecks`, `UnknownClosingKind`. Give each a clear `#[error("...")]` message. Carry useful context in variants where it is obvious (e.g. the offending token in `UnknownVerificationMethod`/`UnknownStatus`); you don't need to perfect every variant's payload now, but the names and messages must be present. The core never prints and never falls back silently — it returns the typed error.

5. **A `Makefile`** at `tools/darkfactory-render/Makefile` mirroring dispatch's, with targets `build`, `release`, `install`, `test`, `lint`, `fmt`, `check` (and you may include `smoke` even though the smoke test arrives in Task 7 — if you add it, point it at `./tests/smoketest.sh`; otherwise omit it). Substitute the binary name `darkfactory-render` in the `install` target. `lint` must run `cargo clippy --all-targets -- -D warnings`. `fmt` must run `cargo fmt --check`. `check` must run fmt, lint, and test. The crate must pass `make check` clean (zero clippy warnings, zero format diffs, all tests passing).

## Project Context

(from proposal.md) The orchestrator currently hand-authors `contract.md`, restating behaviors the spec scenarios already define; because an LLM regenerates the same facts each time, the contract drifts from the specs. This change (Stage 1 of a larger roadmap) makes the contract a deterministic *projection* of a single authored source: spec scenarios gain an `acceptance` field, and a new deterministic tool renders `contract.md` from those acceptance entries (grouped by task) plus a standing closing-task fragment. The tool runs in read-only shadow mode in this stage; the live pipeline is not wired to it yet. You are building the very first piece: the crate and its type vocabulary.

## Architecture

(from design.md) Chosen architecture: a net-new Rust crate `tools/darkfactory-render` with a pure `core` library (parse → typed model → render → shadow) behind a thin CLI, built/installed/tested exactly like `tools/darkfactory-dispatch`. Module inventory: `core::types` (the domain vocabulary + `RenderError`, the one home every other module shares — this task), `core::parse` (Task 2/3), `core::render` (Task 4), `core::shadow` (Task 5), `cli` bin (Task 5). Error strategy: one `RenderError` `thiserror` enum in `core::types`; `core` returns `Result<_, RenderError>` and never prints; only the future CLI translates errors to stderr + exit codes. Your job is the `core::types` foundation only.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Acceptance (field)** — the requirement field holding the falsifiable check that proves a behavior; carries a Verification method tag, a task link, and a Spec Reference. Modeled here as `AcceptanceEntry`.
- **Verification method** — how a validator checks an acceptance entry: `execution` (run a command), `output` (inspect output/file/response), or `inspection` (read code). Modeled as `VerificationMethod`.
- **Spec Reference** — a `capability/N` scenario identifier (e.g. `add-todo/1`) tracing a check back to a formal spec scenario. Modeled as `SpecReference`.
- **Deviated status** — a scenario status recording an approved divergence; Stage 1 only reads it to mark `(DEVIATED)`. One of the `ScenarioStatus` variants.
- **Renderer** — the deterministic tool that renders the contract from acceptance fields grouped by task, carrying check text verbatim and never inventing checks. The crate you are scaffolding.
- **Closing-task fragment** — the standing authored source of the mandatory operational checks (smoke-test, cleanup, module-shape, claude-arch, documentation) that trace to no spec scenario. Modeled as `ClosingTaskFragment`.

## Domain Types

Use these exact Type Catalog names when modeling domain concepts (all are produced by this task):

- `AcceptanceEntry` — one acceptance entry (verbatim statement, method, task link); owning module `core::types`.
- `VerificationMethod` — enum execution/output/inspection; `core::types`.
- `SpecReference` — a `capability/N` scenario identifier; `core::types`.
- `ScenarioStatus` — enum proposed/approved/built/verified/deviated; `core::types`.
- `RequirementScenario` — a parsed scenario: its Spec Reference, status, and acceptance entries; `core::types`.
- `ClosingTaskFragment` — the standing source of operational checks; declares which tasks are closing tasks; `core::types`.
- `TaskList` — the ordered tasks (feature or closing); `core::types`.
- `RenderedContract` — the output contract document model; `core::types`.
- `RenderError` — the fail-loud error enum; `core::types`.

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

Every later task imports `core::types`:
- Task 2 (`parse_specs`) constructs `AcceptanceEntry`, `VerificationMethod`, `SpecReference`, `ScenarioStatus`, and `RequirementScenario` from spec markdown, and returns `RenderError` on malformed input.
- Task 3 (`parse_tasks`, `parse_fragment`) constructs `TaskList` and `ClosingTaskFragment`.
- Task 4 (`render`) consumes all of the above and produces `RenderedContract`.
- Task 5 (CLI) maps `RenderError` to stderr + exit codes.

Minimum integration check that proves the call path works (and the right target for your tests): given the token `"execution"`, `VerificationMethod` parses to the execution variant; given `"bogus"`, it returns `RenderError::UnknownVerificationMethod`. Likewise `"deviated"` parses to the deviated `ScenarioStatus` variant, an omitted status reads as non-deviated, and `"bogus"` returns `RenderError::UnknownStatus`.

## Procedures

**Before writing any code**, invoke the `/tdd` skill. This is mandatory — do not skip it, do not treat it as optional guidance. The skill contains the workflow you must follow for this task. Invoke it now, read the full instructions it returns, and follow its phases in order (red → green → refactor) for the enum value-parsing behavior, which is the testable surface of this task.

- Read `tools/darkfactory-dispatch/Cargo.toml`, `tools/darkfactory-dispatch/Makefile`, and list `tools/darkfactory-dispatch/src/` first so the new crate matches house conventions.
- Run `make check` (or `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`) in `tools/darkfactory-render` and confirm it is clean before committing. Paste the commands and their exit codes into your handoff.
- Commit your implementation and test files in a single commit with message: `[task-1] crate skeleton, domain types, RenderError, Makefile`. Stage only the files you created under `tools/darkfactory-render/` (including its `Cargo.lock`). Do **not** run `git add -A` or `git add .`, and never stage anything under `openspec/changes/contract-renderer/` — that directory is mission bookkeeping the orchestrator owns; committing it corrupts the mission diff.
- Write your handoff report to `openspec/changes/contract-renderer/mission/handoffs/task-1.md` (write the file, do not commit it).

## Handoff Report Format

- Task: Task 1 - Crate skeleton, domain types, RenderError, Makefile
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
