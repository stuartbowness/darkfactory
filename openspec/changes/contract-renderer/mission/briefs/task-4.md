# Worker Brief: Task 4 - Contract render (the deep core)

## Your Task

This is the heart of the tool: assemble the parsed inputs into a typed model and render `contract.md` deterministically, byte-identically, in the existing contract format, failing loud on every malformed case. It RENDERS authored checks verbatim — it never invents or rewords a check.

Produce, in a new `core::render` module (`src/core/render.rs`, add `pub mod render;` to the `core` block in `src/lib.rs`):

1. **`build_model`** — assemble a typed model from the three parsed inputs: the spec scenarios (`Vec<RequirementScenario>` from `parse_specs`), the `TaskList` (from `parse_tasks`), and the `ClosingTaskFragment` (from `parse_fragment`). Recommended: `pub fn build_model(scenarios: &[RequirementScenario], tasks: &TaskList, fragment: &ClosingTaskFragment) -> Result<ContractModel, RenderError>` (you name the model type, e.g. `ContractModel`). This is where grouping, joining, and fail-loud validation happen (see below).

2. **`render`** — `pub fn render(model: &ContractModel) -> Result<RenderedContract, RenderError>` (or take the model by value). Produces the `RenderedContract`. The contract markdown lives in `RenderedContract.body`; warnings live in the new warnings surface.

3. **`RenderedContract::to_markdown(&self) -> String`** — returns the rendered contract body.

4. **`RenderedContract::warnings`** — a named warning surface carrying the inspection-cap warning(s). Task 1 defined `RenderedContract { body: String }`; ADD a warnings field (e.g. `warnings: Vec<String>`) and an accessor (`pub fn warnings(&self) -> &[String]`) in `core::types`. Warnings are non-fatal (rendering still succeeds, exit 0).

You may split build_model/render differently if cleaner, but keep these public entry points and behaviors.

## Exact output format (match it — this is "existing-format fidelity")

```
# Contract

## Task <N>: <name>
<!-- Specs: <cap>/<n>, <cap>/<m> (DEVIATED) -->
- [ ] [<method>] <statement>
- [ ] [<method>] <statement>

## Task <N>: <name>
- [ ] [<method>] <statement>
```

Rules:
- First line `# Contract`, then a blank line.
- One `## Task <N>: <name>` section per task, in TASK-LIST order (the order tasks appear in tasks.md).
- FEATURE tasks: emit exactly ONE `<!-- Specs: ... -->` comment listing the spec references of that task's checks, de-duplicated and ordered by (capability ascending, then N ascending). A reference whose scenario `status` is `deviated` gets ` (DEVIATED)` appended to that ref (e.g. `acceptance-field/4 (DEVIATED)`). Then the check lines.
- CLOSING tasks: NO `<!-- Specs -->` comment — just the check lines.
- Each check line: `- [ ] [<method>] <statement>` where `<method>` is `execution`/`output`/`inspection`.
- One blank line between task sections.

## Grouping, joining, ordering

- **Feature-task checks** come from spec `acceptance`: each `AcceptanceEntry` is placed under the task named by its `task_link`. The check's `<statement>` is the entry's statement VERBATIM (no rewording); `<method>` is the entry's `VerificationMethod`. The entry's Spec Reference (inherited from its scenario) feeds the task's Specs comment.
- **Closing-task checks** come from the fragment: a task with `Closing: <kind>` gets the fragment's checks for that kind, joined by kind. Each fragment check is already stored verbatim as `[<method>] <statement>` — render it as `- [ ] ` + that string.
- **Ordering within a feature task:** by Spec Reference (capability asc, then N asc); ties (multiple acceptance entries sharing one scenario's Spec Reference) broken by SOURCE order — scenario order as returned by `parse_specs`, then entry order within the scenario. Closing-task checks render in fragment order for that kind.
- **Determinism:** rendering the same inputs twice MUST produce byte-identical output. Do not let HashMap iteration order (or any nondeterministic source) leak into the output — use sorted/stable structures.

## Fail-loud (the pipeline returns a RenderError and NO partial contract)

- `DanglingTaskLink(N)` — an acceptance entry's `task_link` N is not a task number in the `TaskList`.
- `AcceptanceLinksClosingTask(N)` — an acceptance entry's `task_link` points to a task that is a closing task (has a `Closing: <kind>`). Acceptance attaches only to feature tasks.
- `FeatureTaskNoChecks(N)` — a feature task (no closing kind) has zero acceptance checks.
- `UnknownClosingKind(<kind>)` — a closing task's kind is absent from the fragment. Enforce this in the render pipeline (you may call the existing `core::parse::validate_closing_kinds`, or fold the check into build_model — see the placement note below).
- `DuplicateSpecReference(...)` — already detected upstream by `parse_specs` (per capability); the full pipeline (parse -> build_model -> render) naturally surfaces it. Add a pipeline-level test that feeding duplicate spec-refs yields `DuplicateSpecReference` (you do not need to re-detect it in render).

All of these already exist as `RenderError` variants (Task 1). Do NOT add new variants.

## Inspection-cap warning

For EACH task, compute (number of `[inspection]` checks) / (total checks in that task). If it is STRICTLY GREATER THAN 20%, push a warning string to `RenderedContract.warnings` naming the task (e.g. `Task 9: 50% of checks are [inspection] (exceeds 20% cap)`). At or under 20% → no warning for that task. This is non-fatal: the contract still renders and the pipeline returns `Ok`. For feature checks the method is `AcceptanceEntry::method`; for closing checks, read the `[method]` tag from the verbatim fragment string.

## Project Context

(from proposal.md) The renderer composes `contract.md` from two authored sources — spec `acceptance` grouped by task, plus the closing-task fragment joined to each `Closing: <kind>` task — in the existing contract format, carrying check statements verbatim and never inventing checks. This is the deterministic projection that removes duplicated-artifact drift.

## Architecture

(from design.md) `core::render` is the deep core: the deterministic projection lives behind one call; it hides grouping by task, the Spec Reference comment grammar (dedupe/order/DEVIATED), check formatting, the inspection-cap warning, and the closing-kind->task join. Relevant decisions: D2 (tasks keyed by number N; `[task:N]` links; an N absent from tasks.md is DanglingTaskLink; this single rule matches both acceptance entries and closing tasks — no name matching). D6 (every feature-task check traces to a scenario; a feature task with zero acceptance checks is FeatureTaskNoChecks; an acceptance entry linking to a closing task is AcceptanceLinksClosingTask). D7 (deterministic ordering: task-list order, then Spec Reference, then source order). D5 (closing-kind->fragment join; UnknownClosingKind for an unknown kind).

### Placement note (carry-forward from Task 3)
`core::parse::validate_closing_kinds` currently performs the `UnknownClosingKind` detection, but design D5 implies UnknownClosingKind belongs to the renderer. For this task: have the render pipeline ENFORCE UnknownClosingKind (the simplest path is to call the existing `validate_closing_kinds` from build_model). You do NOT need to relocate it — the Task 9 module-shape re-check will reconcile whether it stays in `core::parse` or moves to `core::render` and update design.md accordingly. Just make sure the render pipeline fails loud on an unknown closing kind.

## Domain Types (all in core::types from Task 1 — use them; the only change is adding the warnings surface to RenderedContract)

- `RequirementScenario { spec_ref: SpecReference, status: Option<ScenarioStatus>, acceptance: Vec<AcceptanceEntry> }`.
- `AcceptanceEntry { statement: String, method: VerificationMethod, task_link: u32 }`.
- `SpecReference { capability: String, number: u32 }`.
- `ScenarioStatus` with `is_deviated(self) -> bool` (use it for the `(DEVIATED)` marker).
- `VerificationMethod { Execution, Output, Inspection }`.
- `Task { number: u32, name: String, closing_kind: Option<String> }`; `TaskList { tasks }`.
- `ClosingTaskFragment { kinds: Vec<ClosingTaskKind> }`; `ClosingTaskKind { name: String, checks: Vec<String> }` (checks are verbatim `[<method>] <statement>`).
- `RenderedContract { body: String }` — ADD a `warnings` surface here (Task 4 owns the render output shape).
- `RenderError` — use existing variants `DanglingTaskLink(u32)`, `AcceptanceLinksClosingTask(u32)`, `FeatureTaskNoChecks(u32)`, `UnknownClosingKind(String)`, `DuplicateSpecReference(String)`. No new variants.

## What Previous Tasks Built

- **Task 1 (feb7b283):** crate `tools/darkfactory-render`, all `core::types`, `RenderError`. `make check` = fmt + clippy -D warnings + test.
- **Task 2 (8c103c64):** `core::parse::parse_specs(capability, spec_markdown) -> Result<Vec<RequirementScenario>, RenderError>`. `EmptyStatement` carries a `String`.
- **Task 3 (bf06a9d0):** `core::parse::parse_tasks(tasks_markdown) -> Result<TaskList, RenderError>`; `core::parse::parse_fragment(fragment_markdown) -> Result<ClosingTaskFragment, RenderError>`; `core::parse::validate_closing_kinds(&TaskList, &ClosingTaskFragment) -> Result<(), RenderError>`. Standing fragment at `tools/darkfactory-render/templates/closing-task-checks.md` (5 kinds). Fixtures under `tests/fixtures/`.
- All parsers operate on in-memory strings (no file IO). Tests are inline `#[cfg(test)]` in parse.rs plus `tests/` integration files.

## Downstream Consumers

- Task 5 (CLI) calls `parse_*` then `build_model` then `render`, writes/prints `to_markdown()`, and emits `warnings()` to stderr. Minimum integration: parse a small specs+tasks+fragment set, build_model, render, and assert the markdown has the `## Task N`/`<!-- Specs -->`/`- [ ] [method]` shape.
- Task 6 (migration) renders the archived fixture and asserts equivalence to its hand-authored contract.

## Procedures

**Before writing any code**, invoke the `/tdd` skill (Skill tool) and follow red → green → refactor. Mandatory. Write red tests first for: placement+format of a feature-task check; verbatim statement; ordering by Spec Reference with a source-order tie-break; one deduped+ordered Specs comment; the `(DEVIATED)` marker; closing-task checks with NO Specs comment; `AcceptanceLinksClosingTask`; byte-identical repeat render; task-list ordering; `DanglingTaskLink`/`FeatureTaskNoChecks`/`DuplicateSpecReference`; format-fidelity; and the inspection-cap warning boundary (a task >20% inspection warns; a task at/under 20% does not).

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git in the foreground.
3. Edit/create source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.
4. Do NOT read `openspec/changes/contract-renderer/contract.md` — the exact format you need is specified in this brief.

### Build / test
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` must pass clean. Paste commands + exit codes into the handoff.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-4] contract render (build_model + render + format/ordering + fail-loud + inspection-cap)"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-4.md`
Format: Task; Commit SHA; Approach (signatures, model type, how you achieved determinism, where UnknownClosingKind is enforced); Commands run + exit codes; Concerns.

## Final message
Reply with the commit SHA, the build_model/render signatures and model type name, how you guaranteed byte-identical output, and the `make check` result.
