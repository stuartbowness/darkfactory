# Worker Brief: Task 5 - Shadow compare mode and CLI

## Your Task

Make the tool runnable end to end. Add a read-only shadow/compare mode to `core` and a thin CLI binary with `render` and `shadow` subcommands that does all file IO and maps results to exit codes and stderr.

Produce:

1. **`core::shadow` module** (`src/core/shadow.rs`, add `pub mod shadow;` to the `core` block in `src/lib.rs`). One narrow function:
   `pub fn compare(rendered: &str, existing: &str) -> Diff`
   - `Diff` is a type you define in `core::types` (or `core::shadow`) representing a LINE-BASED comparison result. It must distinguish "identical" from "different" and carry the diff text when different. A simple shape works, e.g. `enum Diff { Identical, Different(String) }` where the `String` is a readable line diff (e.g. lines prefixed `-`/`+`, or `< `/`> `). Implement the diff IN-HOUSE (line-by-line); do NOT add a diff crate or any external dependency.
   - `compare` is pure (no IO). Unit-test it: identical inputs → `Identical`; differing inputs → `Different(...)` whose text reflects the changed lines.

2. **The CLI binary** — replace the `src/main.rs` stub with a real CLI exposing two subcommands. Put the logic where it is testable (recommended: a `pub mod cli;` in the lib with a `run`-style entry the bin calls, OR keep it in main.rs and test the built binary via `std::process::Command`). Use **`std::env::args` (stdlib) for argument parsing — do NOT add `clap` or any new dependency** (the crate's only dep stays `thiserror`). The CLI does ALL file IO; `core` stays IO-free.

   Concrete interface (Tasks 6 and 7 will depend on this — keep these subcommand names and inputs):
   - `darkfactory-render render --specs <specs_dir> --tasks <tasks_md> --fragment <fragment_md>`
     - Reads every `<specs_dir>/<capability>/spec.md`, deriving the `<capability>` from each subdirectory name; calls `parse_specs(capability, content)` per capability and collects the scenarios.
     - Reads `<tasks_md>` → `parse_tasks`; reads `<fragment_md>` → `parse_fragment`.
     - Calls `build_model` then `render`.
     - On success: writes the rendered contract (`RenderedContract::to_markdown()`) to STDOUT; writes any `RenderedContract::warnings()` (incl. the inspection-cap breach) to STDERR; exits 0.
     - On `RenderError`: writes a single actionable message to STDERR and exits NON-ZERO.
   - `darkfactory-render shadow --specs <specs_dir> --tasks <tasks_md> --fragment <fragment_md> --existing <contract_md>`
     - Renders via the same pipeline, reads `<contract_md>`, and `compare`s rendered-vs-existing.
     - Prints the result to STDOUT: when identical, a clear "no differences" line; when different, the diff. Writes render warnings to STDERR.
     - **Read-only: shadow MUST NOT write or modify ANY file** (not the existing contract, not the pipeline, nothing). No auto-cutover.
     - Exit codes: identical → 0; differences found → 1 (so it is scriptable for the future cutover gate); a `RenderError` → a DISTINCT non-zero (e.g. 2) with a stderr message.

   You choose exact flag parsing details, but keep the two subcommands, the `--specs/--tasks/--fragment` inputs, shadow's `--existing`, and the exit-code/stderr behavior above. Give a clear usage/error message on bad args (non-zero exit).

## Project Context

(from proposal.md) The renderer runs in read-only SHADOW mode in this stage: operators run `darkfactory-render shadow` against real missions and inspect the diff against the hand-authored contract, WITHOUT replacing anything. Cutover (wiring render into the pipeline, deleting hand-authoring) is a later, gated change. The CLI is the operator entry point; the shadow mode is the safety mechanism that lets the renderer be trusted before it is relied upon.

## Architecture

(from design.md) Module inventory: `core::shadow` has a narrow interface (`compare(rendered, existing) -> Diff`) hiding the diffing strategy, keeping the read-only comparison separate from rendering. `cli` (bin) is a moderate, thin transport over the pure core: it owns file IO, exit codes, and stderr warnings — NO domain logic. Integration points: cli → parse_* → build_model → render → to_markdown; cli (shadow) → compare; cli → filesystem (reads specs/**/*.md, tasks.md, the fragment, and the existing contract.md). Decision D8: line-based diff, diagnostic only; cutover stays a manual, gated action. Error strategy: the CLI is the ONLY translator of `RenderError` to stderr + a non-zero exit; `core` never prints.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Shadow mode** → running the renderer alongside the existing hand-authored contract and recording the diff WITHOUT replacing anything. Cutover parity is judged later by validator-outcome parity, not text match.
- **Renderer** → the deterministic tool; the CLI `render` subcommand is its entry point.
- **Cutover** → switching from hand-authored to rendered; NOT part of this stage — shadow never auto-cutovers.

## Domain Types (from core::types; add `Diff`)

- Consume: `RequirementScenario`, `TaskList`, `ClosingTaskFragment`, `RenderedContract` (with `to_markdown()` and `warnings()`), `RenderError`.
- Pipeline: `core::parse::{parse_specs, parse_tasks, parse_fragment}` → `core::render::{build_model, render}`.
- Add: `Diff` (the shadow comparison result).

## What Previous Tasks Built

- **Task 1 (feb7b283):** crate `tools/darkfactory-render` (standalone, v1.0.0, dep `thiserror = "2"` ONLY). `src/main.rs` is the stub you will replace. `make check` = fmt + clippy -D warnings + test.
- **Task 2 (8c103c64):** `core::parse::parse_specs(capability: &str, spec_markdown: &str) -> Result<Vec<RequirementScenario>, RenderError>`.
- **Task 3 (bf06a9d0):** `core::parse::parse_tasks(&str) -> Result<TaskList, RenderError>`; `core::parse::parse_fragment(&str) -> Result<ClosingTaskFragment, RenderError>`. Standing fragment at `tools/darkfactory-render/templates/closing-task-checks.md`.
- **Task 4 (a12d37f1):** `core::render::build_model(&[RequirementScenario], &TaskList, &ClosingTaskFragment) -> Result<ContractModel, RenderError>`; `core::render::render(&ContractModel) -> Result<RenderedContract, RenderError>`. `RenderedContract::to_markdown(&self) -> String`, `RenderedContract::warnings(&self) -> &[String]`. Output format: `# Contract` / `## Task N: <name>` / `<!-- Specs: cap/N (DEVIATED) -->` (feature tasks only) / `- [ ] [method] <statement>`. Deterministic byte-identical.
- All parsers + render operate on in-memory strings (no file IO). Tests: inline `#[cfg(test)]` plus `tests/` integration files using `std::process::Command` / `include_str!` (NO assert_cmd — don't add it).

## Downstream Consumers

- Task 6 (migration) and Task 7 (smoke test) drive this CLI end-to-end. They rely on the `render`/`shadow` subcommands and the `--specs/--tasks/--fragment/--existing` inputs and exit codes defined above. Keep them stable.

## Procedures

**Before writing any code**, invoke the `/tdd` skill (Skill tool) and follow red → green → refactor. Mandatory. Red tests first: `compare` identical vs different (pure unit tests); then CLI behavior via `std::process::Command` against the built binary on a small fixture (under `tools/darkfactory-render/tests/fixtures/`): `render` emits the expected format and exits 0; `render` with bad input exits non-zero with a stderr message; the inspection-cap warning goes to stderr with exit 0; `shadow` against an identical contract reports no differences and exits 0; `shadow` against a different contract prints a diff and exits 1; `shadow` leaves the existing contract and the working tree byte-unchanged (capture mtime/bytes before and after, or assert the file content is identical and no new files appeared except an explicit output path).

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git in the foreground. Tests that spawn the binary should use a release/debug build path under the crate's `target/` (cargo provides `env!("CARGO_BIN_EXE_darkfactory-render")` to locate the built binary in an integration test — use it).
3. Edit/create source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Build / test
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` must pass clean. Paste commands + exit codes into the handoff.
- Add NO new dependencies (no clap, no assert_cmd, no diff crate). Stdlib only. If you believe a dependency is truly required, STOP and say so in the handoff instead of adding it.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-5] shadow compare mode + render/shadow CLI"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-5.md`
Format: Task; Commit SHA; Approach (Diff type shape, CLI arg approach, exit-code scheme, how shadow stays read-only); Commands run + exit codes; Concerns.

## Final message
Reply with the commit SHA, the `compare` signature + `Diff` shape, the CLI subcommand/flag interface and exit-code scheme, confirmation no new dependency was added, and the `make check` result.
