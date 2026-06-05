# Worker Brief: Task 3 - Gap report rendering

## Your Task

Add `core::coverage::render_report(report: &CoverageReport) -> String` to `tools/darkfactory-render/src/core/coverage.rs`. It projects a `CoverageReport` (built by `analyze`, already present) into the **canonical Gap report markdown** below, deterministically and byte-identically.

### Canonical format (produce EXACTLY this layout)
- A `# Coverage Gap Report` title line.
- A `## Summary` section whose single line is `<T> scenarios, <C> covered, <G> gaps` (T=`report.total`, C=`report.covered`, G=`report.gaps.len()`).
- A `## Gaps` section. When there are no gaps, its content is the single line `None.`. Otherwise, one block per gap.
- Each gap block:
  - `### <identifier> — <title>` where identifier is `<capability>/<N>` when the gap has a Spec Reference, or just `<capability>` when it has none. (Note the em dash `—`, U+2014, with a single space on each side.)
  - `- Gap kind: <kinds>` — the gap's distinct kinds, comma-separated, in the order they are stored on the gap (already the fixed order no-acceptance, dangling, closing-task). Labels map EXACTLY: `GapKind::NoAcceptance` -> `no acceptance`, `GapKind::DanglingTaskLink` -> `dangling task link`, `GapKind::ClosingTaskLink` -> `closing-task link`.
  - `- Behavior:` followed by each stored behavior line prefixed with **two spaces**. `CoverageGap.behavior` already contains the verbatim bullets INCLUDING the leading `- ` (e.g. `"- **WHEN** a gadget is edited"`), so a rendered behavior line is `"  " + that string` -> `"  - **WHEN** a gadget is edited"`.
- Sections and gap blocks are separated by exactly one blank line.

### Exact worked example (this is the byte-identical test target)
For a report of 3 scenarios, 1 covered, with two gaps — `gadget/1` "Sync gadget" (dangling task link) and `widget/2` "Delete widget" (no acceptance) — `render_report` returns exactly:

```
# Coverage Gap Report

## Summary
3 scenarios, 1 covered, 2 gaps

## Gaps

### gadget/1 — Sync gadget
- Gap kind: dangling task link
- Behavior:
  - **WHEN** a gadget is edited
  - **THEN** it syncs

### widget/2 — Delete widget
- Gap kind: no acceptance
- Behavior:
  - **WHEN** the user deletes a widget
  - **THEN** it is removed
```

The no-gaps form (3 scenarios, all covered):

```
# Coverage Gap Report

## Summary
3 scenarios, 3 covered, 0 gaps

## Gaps

None.
```

Decide the trailing newline by matching these examples in your test (assert the full `String` equals the expected literal); keep it consistent between the gaps and no-gaps cases.

### Done when
- A unit test asserts `render_report` returns the exact gaps example above (build the input `CoverageReport`/`CoverageGap`s by hand or via `analyze`).
- A unit test asserts the exact no-gaps form.
- `make check` is green.

## Project Context
Stage 2 coverage tool. `analyze` (Task 2) computes the gaps; this task renders them as the human-readable report the `coverage` subcommand (Task 4) will print. Deterministic, byte-identical output is required so fixtures and diffs are reliable.

## Architecture
`render_report` is a pure formatter: `CoverageReport -> String`, no I/O, no errors. It mirrors how the existing `render::render` projects a contract model to a string. Keep it in `core::coverage`.

## Key Terms
- **Gap report**: the rendered markdown (this task's output).
- **Gap kind**: NoAcceptance / DanglingTaskLink / ClosingTaskLink — rendered with the exact labels above.

## Domain Types
- `CoverageReport { gaps: Vec<CoverageGap>, total: usize, covered: usize }` (`core::types`, from Task 2).
- `CoverageGap { capability: String, spec_ref: Option<SpecReference>, title: String, behavior: Vec<String>, kinds: Vec<GapKind> }` (`core::types`, from Task 2). (Field names: confirm exact names in types.rs.)
- `GapKind` (`core::types`) — NoAcceptance / DanglingTaskLink / ClosingTaskLink.
- `SpecReference { capability: String, number: u32 }` (`core::types`).

## What Previous Tasks Built

### Task 1 (commit cafa105) + Task 2 (commits e644153, 45e0efd)
- `RequirementScenario { title, behavior: Vec<String>, capability: String, spec_ref: Option<SpecReference>, status, acceptance }`. `behavior` strings include the leading `- ` (e.g. `"- **WHEN** ..."`).
- `core::coverage::analyze(&[RequirementScenario], &TaskList) -> CoverageReport` (infallible) classifies gaps; gaps are ordered by capability then document position; each `CoverageGap` carries its `capability`, `spec_ref`, `title`, `behavior`, and `kinds` (distinct, fixed order).
- Types `CoverageGap`, `CoverageReport`, `GapKind`, `RenderError::DuplicateTaskNumber` are in `core::types`. Confirm the exact field names by reading `src/core/types.rs` before coding.

## Downstream Consumers
- Task 4 (coverage CLI) calls `render_report` and writes the result to stdout. Minimum integration test proving the path: render a `CoverageReport` and confirm the string matches the canonical layout.

## Procedures
**Before writing any code**, invoke the `/tdd` skill (mandatory) and follow red-green-refactor.
- Read `src/core/types.rs` to confirm the exact field names on `CoverageGap`/`CoverageReport`/`GapKind` before coding.
- Work only inside `tools/darkfactory-render/`. Verify with `make check`.
- Commit once, code+tests only, message: `[task-3] gap report rendering (render_report, canonical format)`. Stage only files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/computed-coverage/`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-3.md` (write, do not commit).

## Handoff Report Format
- Task: Task 3 - Gap report rendering
- Commit SHA: <full SHA>
- Approach: <what you did>
- Commands run: <incl. make check exit code>
- Concerns: <any, esp. trailing-newline / indent decisions>
