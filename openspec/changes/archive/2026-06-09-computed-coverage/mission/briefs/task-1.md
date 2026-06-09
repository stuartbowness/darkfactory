# Worker Brief: Task 1 - Scenario model and parser retention

## Your Task

Extend the `darkfactory-render` crate's parsed scenario model so a later coverage feature can see every scenario, while keeping the existing `render`/`shadow` output byte-identical. Specifically:

1. **Extend `RequirementScenario`** (`tools/darkfactory-render/src/core/types.rs`):
   - Add a field for the scenario **title** (the text after `#### Scenario:`).
   - Add a field for the scenario **behavior text**: the verbatim, ordered list of the scenario's behavior bullets — the lines beginning `- **WHEN**`, `- **THEN**`, or `- **AND**` that appear before the `**Acceptance:**` block — excluding HTML comments and blank lines.
   - Change `spec_ref` from `SpecReference` to `Option<SpecReference>`. It is `None` only for an acceptance-less scenario that has no `<!-- spec-ref: N -->` comment.

2. **Extend `parse_specs`** (`src/core/parse.rs`) to **retain every scenario**, including scenarios that have no `**Acceptance:**` block (these are currently dropped). Capture each scenario's title and behavior text. Keep the existing rule that a scenario which **does** declare acceptance but has **no** Spec Reference still fails loudly with `MissingSpecRef`.

3. **Adapt the strict path** (`src/core/render.rs` `build_model`/`render`, and `src/core/shadow.rs`) to the new `Option<SpecReference>` **without changing their output**. The renderer only processes acceptance-carrying scenarios, which always carry `Some(spec_ref)` (the parser guarantees it), so you can rely on that invariant. The rendered contract and shadow comparison must be byte-identical to before for every input.

### Done when
- All pre-existing `render` and `shadow` tests still pass **unmodified** (do not weaken or edit them to accommodate the change).
- A new test proves `parse_specs` retains a scenario with no `**Acceptance:**` block and exposes its title and behavior text.
- `make check` is green (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`).

## Project Context

Stage 2 of a planning-pipeline overhaul. A new `coverage` subcommand (built in later tasks) will compute which spec scenarios have no task linked. To do that it must see **all** scenarios — including ones an author wrote with no acceptance/task yet — and show their behavior text. This task is the foundational model+parser change the rest build on. It is additive and read-only; nothing about the live mission flow changes.

## Architecture

Single crate `tools/darkfactory-render`: `core::parse` → `core::render`/`core::shadow`, fail-loud via `RenderError`. The design choice for this work is **one extended parsed model** (not a second parser): `parse_specs` returns the richer scenarios and both render and coverage consume them. The `spec_ref → Option` change ripples into render/shadow only as mechanical `Option` handling; render's behavior is unchanged because it only ever processes acceptance-carrying scenarios, which always have `Some(spec_ref)`.

## Key Terms
- **Scenario**: one `#### Scenario:` block (a WHEN/THEN behavior) inside a capability's `spec.md`.
- **Spec Reference**: the `capability/N` identifier from a `<!-- spec-ref: N -->` comment.
- **Behavior text**: the verbatim WHEN/THEN/AND bullets before `**Acceptance:**`.

## Domain Types
- `RequirementScenario` (`core::types`) — a spec scenario; you are adding title + behavior text and making `spec_ref` optional.
- `SpecReference` (`core::types`) — the `capability/N` identifier.
- `AcceptanceEntry` (`core::types`) — an acceptance check under a scenario (unchanged here).

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
- Task 2 (coverage analysis) consumes the retained scenarios (incl. acceptance-less) and their behavior text. Minimum integration test proving the path: a test that `parse_specs` returns an acceptance-less scenario with its title and behavior captured.
- Task 4 (coverage CLI) ultimately reads these via the same parser.

## Procedures

**Before writing any code**, invoke the `/tdd` skill. This is mandatory — do not skip it. Read the full workflow it returns and follow its phases (red-green-refactor) in order.

- Work only inside `tools/darkfactory-render/`. Run `make check` from that directory to verify.
- Commit your implementation and test files in a single commit with message: `[task-1] scenario model + parser retention (title, behavior, spec_ref Option)`. Stage only the files you changed under `tools/darkfactory-render/`. Do **not** run `git add -A` or `git add .`, and never stage anything under `openspec/changes/computed-coverage/` — that is mission bookkeeping the orchestrator owns.
- Write your handoff report to `openspec/changes/computed-coverage/mission/handoffs/task-1.md` (write the file, do not commit it).

## Handoff Report Format
- Task: Task 1 - Scenario model and parser retention
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <commands and exit codes, including `make check`>
- Concerns: <anything you're uncertain about, especially any render/shadow test you had to look at>
