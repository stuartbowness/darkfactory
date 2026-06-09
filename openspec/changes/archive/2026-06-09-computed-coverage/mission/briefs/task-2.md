# Worker Brief: Task 2 - Coverage analysis

## Your Task

Add the tolerant coverage analysis to the `darkfactory-render` crate: classify which spec scenarios have no task covering them.

1. **Add domain types to `core::types`** (`tools/darkfactory-render/src/core/types.rs`):
   - `GapKind` — an enum with exactly three variants: `NoAcceptance`, `DanglingTaskLink`, `ClosingTaskLink`.
   - `CoverageGap` — one uncovered scenario: its capability, its `Option<SpecReference>`, its scenario title, its behavior text (`Vec<String>`), and the distinct `GapKind`s that apply to it.
   - `CoverageReport` — the computed result: the ordered list of `CoverageGap`s plus the total number of scenarios considered and the number covered.
   - Add a new `RenderError::DuplicateTaskNumber` variant (carry the offending number/name in the message).

2. **Create the `core::coverage` module** (`src/core/coverage.rs`, wired into `core/mod.rs` or `lib.rs` as the crate does for other core modules) owning:
   - `analyze(scenarios: &[RequirementScenario], tasks: &TaskList) -> CoverageReport` — **infallible** (never returns an error). For each scenario:
     - It is a **gap** when NONE of its acceptance entries links to a valid **feature task**. A *feature task* is a `Task` in `tasks` whose `closing_kind == None`. A scenario with at least one acceptance entry whose `task_link` resolves to a feature task is **covered** — even if other entries of that scenario have unresolved links (those are not coverage's concern; they surface later at render time).
     - The gap's `GapKind`s are the **distinct** reasons present among its acceptance entries, in this fixed order: `NoAcceptance`, `DanglingTaskLink`, `ClosingTaskLink`. `NoAcceptance` applies when the scenario has zero acceptance entries. `DanglingTaskLink` applies when an entry's `task_link` matches no task at all. `ClosingTaskLink` applies when an entry's `task_link` matches a task whose `closing_kind` is `Some`.
     - Order the gaps by capability name ascending, then by the scenario's document position within that capability (the order they came out of `parse_specs`).
     - `CoverageReport.covered` = scenarios that are not gaps; `total` = all scenarios considered.
   - `check_duplicate_tasks(tasks: &TaskList) -> Result<(), RenderError>` — returns `Err(RenderError::DuplicateTaskNumber(..))` if two tasks share a number. This is used by the CLI path in Task 4; **do NOT put this check inside `parse_tasks`** (it must stay permissive so `render`/`shadow` behavior is unchanged).

### Done when
- Unit tests cover: a no-acceptance gap, a dangling-link gap, a closing-task-link gap, a scenario with one valid feature-task link (not a gap) even with an unresolved sibling entry, a gap with two distinct kinds (listed once each in fixed order), ordering across and within capabilities, and `check_duplicate_tasks` detecting a duplicate.
- `make check` is green.

## Project Context

Stage 2: a new `coverage` subcommand (Task 4) will compute which requirements have no task. This task is the engine behind it — the gap classification — kept tolerant (it reports, never errors) so it can run on incomplete planning input. The CLI wiring, the markdown rendering, and the exit codes are separate later tasks.

## Architecture

`analyze` is the tolerant counterpart to the strict `build_model`: where `build_model` fails loud on a dangling or closing-task link, `analyze` records it as a `GapKind`. Coverage reuses the same parsed scenarios (`parse_specs`) the renderer uses — do not re-parse. Keep `analyze` infallible; malformed *input* (handled in Task 4 via `check_duplicate_tasks` and the parser) is separate from unresolved *links* (which are gaps).

## Key Terms
- **Coverage gap**: a scenario none of whose acceptance entries links to a valid feature task.
- **Gap kind**: why it is uncovered — no acceptance, dangling task link, or closing-task link.
- **Feature task** / **Closing task**: a `Task` with `closing_kind == None` is a feature task; `Some` is a closing task.

## Domain Types
- `CoverageGap` (new, `core::types`) — an uncovered scenario + its gap kinds.
- `GapKind` (new, `core::types`) — NoAcceptance / DanglingTaskLink / ClosingTaskLink.
- `CoverageReport` (new, `core::types`) — ordered gaps + total/covered counts.
- `RequirementScenario` (`core::types`, from Task 1) — has `title`, `behavior`, `spec_ref: Option<SpecReference>`, `acceptance: Vec<AcceptanceEntry>`.
- `AcceptanceEntry` (`core::types`) — has `task_link: u32` (and method/statement).
- `Task` / `TaskList` (`core::types`) — `Task { number: u32, name: String, closing_kind: Option<String> }`.
- `SpecReference` (`core::types`) — `{ capability: String, number: u32 }`.
- `RenderError` (`core::types`) — add `DuplicateTaskNumber`.

## What Previous Tasks Built

### Task 1: Scenario model and parser retention (commit cafa105)
- Modified: src/core/types.rs, src/core/parse.rs, src/core/render.rs.
- `RequirementScenario` now `{ title: String, behavior: Vec<String>, spec_ref: Option<SpecReference>, status: Option<ScenarioStatus>, acceptance: Vec<AcceptanceEntry> }`. `behavior` is the verbatim WHEN/THEN/AND bullets before `**Acceptance:**`.
- `parse_specs(capability, markdown) -> Result<Vec<RequirementScenario>, RenderError>` retains EVERY scenario, including acceptance-less ones (which have `spec_ref` possibly `None` and `acceptance` empty). They come out in document order.
- `AcceptanceEntry.task_link` is `u32` (mandatory). A scenario with acceptance but no spec-ref still errors `MissingSpecRef` at parse.

## Downstream Consumers
- Task 3 (gap report rendering) consumes `CoverageReport`/`CoverageGap`/`GapKind` to render markdown. Minimum integration test proving the path: construct a `CoverageReport` from `analyze` and confirm its gaps/counts are as expected.
- Task 4 (coverage CLI) calls `analyze` and `check_duplicate_tasks`.

## Procedures

**Before writing any code**, invoke the `/tdd` skill (mandatory) and follow its red-green-refactor workflow.

- Work only inside `tools/darkfactory-render/`. Verify with `make check`.
- A nice-to-have: include a test for an acceptance-less scenario that DOES carry a `<!-- spec-ref: N -->` comment (it should be a `NoAcceptance` gap identified by its Spec Reference, not just by title) — this closes a small coverage gap left from Task 1.
- Commit your implementation and tests in a single commit with message: `[task-2] coverage analysis (analyze, CoverageGap/GapKind/CoverageReport, DuplicateTaskNumber)`. Stage only files under `tools/darkfactory-render/`. Never run `git add -A`/`git add .`, never stage anything under `openspec/changes/computed-coverage/`.
- Write your handoff report to `openspec/changes/computed-coverage/mission/handoffs/task-2.md` (write the file, do not commit it).

## Handoff Report Format
- Task: Task 2 - Coverage analysis
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes, including `make check`>
- Concerns: <anything uncertain>
