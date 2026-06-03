# Worker Brief: Task 3 - Task-list and closing-task fragment parsing

## Your Task

Extend `core::parse` with two more parsers and author the standing closing-task fragment. After this task, the crate can parse a `tasks.md` into a `TaskList`, parse the closing-task fragment into a `ClosingTaskFragment`, and detect a closing task whose kind is unknown to the fragment.

Concretely, produce:

1. **`core::parse::parse_tasks`** — parses `tasks.md` content into a `TaskList`. Operates on an in-memory string (NO file IO — same rule as `parse_specs`). For each task entry it extracts:
   - the task number N from a `Task N: <name>` heading line (the line may have a checkbox prefix `- [ ]`, `- [x]`, or `- [~]` — handle all three; the number and name come from the `Task N:` part),
   - the task `name` (the text after `Task N:`),
   - the optional `Closing: <kind>` topology field (a line in the task's body), stored as `Task::closing_kind: Option<String>` (`None` for feature tasks).
   Recommended signature: `pub fn parse_tasks(tasks_markdown: &str) -> Result<TaskList, RenderError>`.

2. **`core::parse::parse_fragment`** — parses the closing-task fragment content into a `ClosingTaskFragment`. Operates on an in-memory string (NO file IO). It yields the closing-task kinds mapped to their checks (`Vec<ClosingTaskKind>`, each `{ name, checks }`). Recommended signature: `pub fn parse_fragment(fragment_markdown: &str) -> Result<ClosingTaskFragment, RenderError>`.
   - Design the fragment file syntax yourself, but keep it simple and parseable: a per-kind section naming the kind, followed by its check lines. Each check is a `[<method>] <statement>` line. STORE each check as its verbatim `[<method>] <statement>` text in `ClosingTaskKind::checks` (which is `Vec<String>` from Task 1) — the renderer (Task 4) will prefix `- [ ] ` and emit it. (You MAY validate that each check's `[<method>]` tag is a recognized verification method via `VerificationMethod::parse` on the bare word, failing loud on an unknown one, for consistency with `parse_specs`; this is encouraged but not required by the contract.)

3. **`templates/closing-task-checks.md`** — author this standing file at `tools/darkfactory-render/templates/closing-task-checks.md`. It declares the FIVE mandatory closing-task kinds, each with its operational checks. The kinds (exact names) are: `smoke-test`, `cleanup`, `module-shape`, `claude-arch`, `documentation`. Author 1-2 generic `[<method>] <statement>` checks per kind that capture that kind's verification essence (these are generic across missions, not specific to this crate). Use the mission's mandatory closing-task definitions as your guide for what each kind verifies:
   - `smoke-test`: an end-to-end test runs the project's primary user flow against real dependencies (no internal mocks) and passes.
   - `cleanup`: no dead code, unused dependencies, leftover scaffolding, or temporary artifacts remain (e.g. the linter is clean).
   - `module-shape`: a record exists comparing the design's module inventory and type catalog to the committed code, noting drift or none.
   - `claude-arch`: CLAUDE.md and ARCH.md exist and reflect the built codebase (created if absent; a pre-existing one is left unmodified).
   - `documentation`: documentation for what was built exists, and any commands shown were run against the built artifacts with real pasted output.
   Pick verification-method tags sensibly (most will be `[execution]` or `[output]`; keep `[inspection]` rare). NOTE: Task 6 (migration) may refine the exact check text later to match an archived fixture's hand-authored closing-task checks; author a sensible first version now.

4. **`UnknownClosingKind` detection** — a function that, given a parsed `TaskList` and a `ClosingTaskFragment`, returns `RenderError::UnknownClosingKind(<the kind>)` for any closing task whose `Closing: <kind>` is not among the fragment's kind names; otherwise `Ok`. Recommended signature: `pub fn validate_closing_kinds(tasks: &TaskList, fragment: &ClosingTaskFragment) -> Result<(), RenderError>` (name is yours). Task 4's render will call this; here it must be independently testable.

## Project Context

(from proposal.md) The contract is being made a deterministic projection of two authored sources: spec `acceptance` (feature-task checks, Task 2) and a standing closing-task fragment (operational checks for the mandatory closing tasks — smoke test, cleanup, module-shape, CLAUDE/ARCH, documentation — that trace to no spec scenario). Your task parses the task list (to know each task's number/name and which are closing tasks) and the fragment (the operational-check source), and detects a closing task tagged with a kind the fragment doesn't know.

## Architecture

(from design.md) `core::parse` is a deep module hiding all markdown parsing; it returns typed values or `RenderError`, no file IO. Decision D5: tasks.md tags each closing task with a `Closing: <kind>` field; a task is a closing task iff it carries a fragment-known kind; the renderer joins each `Closing: <kind>` task to the fragment's checks for that kind; a `Closing:` kind absent from the fragment is `UnknownClosingKind`. Decision D2: tasks are keyed by their number N. The standing `templates/closing-task-checks.md` declares the mandatory kinds with their `[<method>] <statement>` checks.

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Closing-task fragment** → `ClosingTaskFragment` (the standing operational-check source), modeled as kinds (`ClosingTaskKind { name, checks }`).
- **Task / TaskList** → `Task { number, name, closing_kind }`, `TaskList { tasks }`.
- **Verification method** → `VerificationMethod` (execution/output/inspection), tag on each check.

## Domain Types

All already defined in `core::types` (Task 1) — import and use, do not redefine:
- `Task { number: u32, name: String, closing_kind: Option<String> }`.
- `TaskList { tasks: Vec<Task> }` (Default).
- `ClosingTaskFragment { kinds: Vec<ClosingTaskKind> }` (Default).
- `ClosingTaskKind { name: String, checks: Vec<String> }` — `checks` holds verbatim `[<method>] <statement>` strings.
- `VerificationMethod` with `VerificationMethod::parse(&str)` (if you choose to validate fragment method tags).
- `RenderError` — use `UnknownClosingKind(String)` (already defined). Do NOT add new variants.

## What Previous Tasks Built

### Task 1: Crate skeleton + core::types (commit feb7b283)
- Crate `tools/darkfactory-render` (standalone, v1.0.0, dep `thiserror = "2"`). `make check` = fmt + clippy -D warnings + test; keep it green.
- `src/lib.rs` declares `pub mod core { pub mod types; pub mod parse; }`. All domain types in `crate::core::types`. `src/main.rs` is a stub (real CLI is Task 5).
- Types above are all defined. `RenderError` variants include `UnknownClosingKind(String)`, `DanglingTaskLink(u32)`, `AcceptanceLinksClosingTask(u32)`, `FeatureTaskNoChecks(u32)` (reserved for Task 4).

### Task 2: Spec & acceptance parsing (commit 8c103c64)
- `src/core/parse.rs` exists with `pub fn parse_specs(capability: &str, spec_markdown: &str) -> Result<Vec<RequirementScenario>, RenderError>` and 8 private helpers. ADD your `parse_tasks`, `parse_fragment`, and `validate_closing_kinds` to THIS module (`src/core/parse.rs`). Match its style (line-slice parsing, fail-loud, verbatim statements, no file IO).
- NOTE: `RenderError::EmptyStatement` now carries a `String` payload (`EmptyStatement(String)`). Not relevant to your task but be aware the variant is not unit-shaped.
- Tests live inline in `#[cfg(test)] mod tests` in parse.rs plus `tests/` integration files. Fixtures live under `tools/darkfactory-render/tests/fixtures/`.

## Downstream Consumers

- Task 4 (`core::build_model` / `core::render`) consumes `parse_tasks` output (to order task sections and join closing-kind tasks to fragment checks) and `parse_fragment` output, and calls your `validate_closing_kinds` for the `UnknownClosingKind` failure. Minimum integration check: a `TaskList` with a feature task (number 1, no closing kind) and a closing task (number 7, `Closing: smoke-test`) plus a fragment declaring `smoke-test` validates OK; the same closing task with `Closing: bogus` returns `UnknownClosingKind("bogus")`.
- Task 5 (CLI) reads `templates/closing-task-checks.md` from disk and passes its content to `parse_fragment`. Task 7 (smoke test) exercises the file end-to-end.

## Procedures

**Before writing any code**, invoke the `/tdd` skill (Skill tool) and follow red → green → refactor. Mandatory. Red tests first: parse_tasks extracting number/name/closing-kind (incl. checkbox prefixes); parse_fragment yielding kinds→checks; validate_closing_kinds returning Ok for known kinds and `UnknownClosingKind` for an unknown one; and a test that `parse_fragment` on the committed `templates/closing-task-checks.md` yields all five mandatory kinds.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git in the foreground.
3. Edit/create source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Build / test
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` must pass clean (zero fmt diffs, zero clippy -D warnings, all tests). Paste commands + exit codes into the handoff.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/` (incl. the new `templates/closing-task-checks.md` and any new fixtures). Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-3] task-list + closing-task fragment parsing"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-3.md`
Format: Task; Commit SHA; Approach (incl. the parse_tasks/parse_fragment/validate_closing_kinds signatures and the fragment file syntax you chose); Commands run + exit codes; Concerns.

## Final message
Reply with the commit SHA, the three new function signatures, confirmation that `templates/closing-task-checks.md` declares the five kinds, and the `make check` result.
