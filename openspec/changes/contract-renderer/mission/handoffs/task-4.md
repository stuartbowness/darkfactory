# Handoff: Task 4 - Contract render (the deep core)

## Task

Task 4: the `core::render` module that assembles parsed inputs into a typed
model and renders `contract.md` deterministically, byte-identically, in the
existing contract format, failing loud on every malformed case and emitting a
non-fatal inspection-cap warning.

## Commit SHA

`a12d37f1eaef254b40a9d2ada4e953560f5df827`

Staged ONLY files under `tools/darkfactory-render/` (4 files: `src/core/render.rs`,
`src/core/types.rs`, `src/lib.rs`, `tests/contract_render.rs`). `openspec/changes/`
was not staged.

## Approach

### Public entry points and signatures

- `pub fn build_model(scenarios: &[RequirementScenario], tasks: &TaskList, fragment: &ClosingTaskFragment) -> Result<ContractModel, RenderError>`
- `pub fn render(model: &ContractModel) -> Result<RenderedContract, RenderError>`
- `RenderedContract::to_markdown(&self) -> String` (returns `body.clone()`)
- `RenderedContract::warnings(&self) -> &[String]` (the inspection-cap warning surface)

### Model type

`pub struct ContractModel { sections: Vec<TaskSection> }` (private field). Two
private support types: `TaskSection { number, name, specs_comment: Option<String>,
checks: Vec<CheckLine> }` and `CheckLine { body: String, method: VerificationMethod }`.

`build_model` does ALL the work — grouping by task, the Spec Reference comment
grammar (dedupe/order/DEVIATED), the closing-kind -> fragment join, deterministic
ordering, and fail-loud validation. The model carries each section already fully
ordered with its comment and check lines pre-formatted as `[<method>] <statement>`,
so `render` is a pure string join with no remaining decisions. `CheckLine` keeps
the typed `method` so the inspection-cap warning is computed without re-parsing
the rendered text.

### Warnings surface (types change)

Added `warnings: Vec<String>` to `RenderedContract` in `core::types`, plus
`to_markdown()` and `warnings()` accessors. Warnings are non-fatal: render returns
`Ok` and exit 0 is unaffected.

### Determinism (byte-identical guarantee)

No HashMap or any nondeterministic source touches the output. `placed_entries`
flattens scenarios in source order (recording scenario index + entry index).
Feature-task checks are ordered with a single stable `sort_by` on the
source-ordered vec, keyed by `(capability, number)` then `(scenario_index,
entry_index)`, so equal Spec References keep source order. The Specs comment is
deduped in one forward pass over the already-sorted vec (skip a key equal to the
previous one). Closing-task checks render in fragment order. `render` builds the
body by sequential `push_str` with fixed separators. A
`repeat_render_is_byte_identical` unit test asserts two renders produce identical
bytes and that two `build_model` calls produce equal models.

### Where UnknownClosingKind is enforced

In the render pipeline: `build_model` calls the existing
`core::parse::validate_closing_kinds(tasks, fragment)?` as its first step (the
carry-forward placement note from Task 3). Not relocated — Task 9's module-shape
re-check reconciles whether it stays in `core::parse` or moves to `core::render`.
A `core::render` unit test
(`an_unknown_closing_kind_fails_loud_in_the_pipeline`) asserts the pipeline fails
loud.

### Fail-loud (no partial output)

`build_model` validates every acceptance entry's `task_link` before grouping:
no matching task -> `DanglingTaskLink(N)`; matching task is a closing task ->
`AcceptanceLinksClosingTask(N)`. A feature task that no acceptance entry links to
-> `FeatureTaskNoChecks(N)`. `UnknownClosingKind` via `validate_closing_kinds`.
`DuplicateSpecReference` is detected upstream by `parse_specs`; a pipeline test
(`duplicate_spec_reference_surfaces_through_the_full_pipeline`) feeds duplicate
spec-refs through `parse_specs -> build_model -> render` and asserts the error
surfaces unchanged. No new `RenderError` variants were added.

### Inspection-cap warning

Per section: `inspection * 100 > total * 20` (integer math, exact boundary;
strictly greater than 20% warns, at/under does not; zero-check sections never
warn). Message: `Task <N>: <pct>% of checks are [inspection] (exceeds 20% cap)`.
For feature checks the method is the typed `AcceptanceEntry::method`; for closing
checks it is read from the `[<method>]` tag of the verbatim fragment string via
`fragment_check_method`. Tested at both sides of the boundary (1 of 2 = 50% warns;
1 of 5 = 20% does not) plus a closing-task path (2 of 2 = 100% warns).

### Tests (TDD red->green, vertical slices)

14 unit tests in `core::render` covering: feature-check placement+format,
ordering by Spec Reference with source-order tie-break, the deduped/ordered Specs
comment, the `(DEVIATED)` marker, closing-task checks with no Specs comment,
byte-identical repeat render, task-list ordering, verbatim statements,
`DanglingTaskLink`, `AcceptanceLinksClosingTask`, `FeatureTaskNoChecks`,
`UnknownClosingKind`, and the inspection-cap boundary (over / at / closing-task).
Plus 2 pipeline integration tests in `tests/contract_render.rs` (the Task 5
minimum integration shape, and `DuplicateSpecReference` through the full
pipeline).

## Commands run + exit codes

- `cd tools/darkfactory-render && make check` -> exit 0 (fmt clean, clippy
  `-D warnings` clean, all tests pass). Ran twice; first run flagged fmt diffs in
  the new files, fixed with `cargo fmt`, then re-ran clean.
- `cargo fmt --check` -> exit 0
- `cargo clippy --all-targets -- -D warnings` -> exit 0 (No issues found)
- `cargo test --no-fail-fast` -> exit 0: 51 tests passed across 6 suites (45 lib
  unit tests, 2 contract_render, 2 closing_task_fragment, 2 openspec_validate).
- `git add tools/darkfactory-render && git commit ...` -> exit 0, 4 files changed,
  974 insertions.

## Concerns

- The Specs-comment dedupe relies on `placed` being sorted by Spec Reference
  first so equal keys are adjacent. This holds because feature_section sorts
  before calling specs_comment. A given spec-ref maps to exactly one scenario
  (parse_specs enforces per-capability uniqueness), so the `deviated` flag is
  consistent across all entries sharing a ref — no risk of one ref rendering both
  with and without `(DEVIATED)`.
- `fragment_check_method` falls back to `Execution` if a fragment check's tag is
  somehow unparseable. In practice this never fires: `parse_fragment` already
  rejects an unknown method tag. The fallback is conservative for the cap count
  (it cannot inflate the inspection count). This is the one non-fail-loud spot,
  and it is unreachable given upstream validation. Flag for Task 9 review if a
  stricter stance is wanted.
- `UnknownClosingKind` enforcement still lives in `core::parse::validate_closing_kinds`
  and is merely called from `build_model`. Design D5 implies it belongs to the
  renderer; the brief explicitly defers the relocation decision to Task 9's
  module-shape re-check.
