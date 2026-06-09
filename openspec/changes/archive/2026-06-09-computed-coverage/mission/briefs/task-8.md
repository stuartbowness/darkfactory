# Worker Brief: Task 8 - Documentation (closing task)

## Your Task

Document the new `coverage` subcommand and the `core::coverage` module in three existing docs. Match each file's existing style and structure — read each before editing.

1. **`tools/darkfactory-render/README.md`** — add a section documenting the `coverage` subcommand:
   - Usage: `darkfactory-render coverage --specs <dir> --tasks <md>`.
   - What it does: renders a computed coverage gap report — every spec scenario whose acceptance entries don't link to a valid feature task — from the same parsed specs the `render`/`shadow` commands use. Tolerant (reports gaps, never errors on them).
   - Exit codes: `0` = no gaps, `3` = gaps found (a tolerant, non-failing result), `2` = usage or parse error.
   - An example with REAL pasted output: actually run the command against a fixture (e.g. `cargo run -- coverage --specs tests/fixtures/cli/covered_specs --tasks tests/fixtures/cli/covered_tasks.md` for the no-gaps case, and the migrated fixture `tests/fixtures/migrated/specs` + `tests/fixtures/migrated/tasks.md` for a gaps case) and paste the actual stdout. Do not invent output.

2. **Repo root `README.md`** — add a short mention of the `coverage` subcommand / Stage 2 computed coverage wherever `darkfactory-render` or its commands are described (keep it brief; match the surrounding tone).

3. **Repo root `ARCH.md`** — update the `darkfactory-render` architecture description to mention the new `core::coverage` module (`analyze` builds a `CoverageReport`; `render_report` projects the Gap report markdown; `check_duplicate_tasks`) and the `coverage` subcommand, alongside the existing render/shadow description.

Do NOT modify `CLAUDE.md` (it already exists). Do NOT modify `UBIQUITOUS_LANGUAGE.md` or anything under `openspec/changes/`.

### Done when
- The three docs are updated; any command shown was actually run and its real output pasted.
- `make check` still green (you changed no Rust, but confirm nothing broke).

## Project Context
Mandatory documentation closing task. Stage 2 added a `coverage` subcommand to `darkfactory-render`; users and the architecture doc need to know it exists and how to run it.

## What Previous Tasks Built
- `coverage` subcommand: `darkfactory-render coverage --specs <dir> --tasks <md>`, exit `0`/`3`/`2`, prints the Gap report (`# Coverage Gap Report`, a `## Summary` line `<T> scenarios, <C> covered, <G> gaps`, and a `## Gaps` section) to stdout.
- `core::coverage` module: `analyze(&[RequirementScenario], &TaskList) -> CoverageReport`, `render_report(&CoverageReport) -> String`, `check_duplicate_tasks(&TaskList) -> Result<(), RenderError>`.
- The migrated fixture reports `11 scenarios, 8 covered, 3 gaps` (exit 3); `tests/fixtures/cli/covered_specs` is a no-gaps fixture (exit 0).

## Procedures
**Before writing**, invoke the `/tdd` skill (mandatory, though this is a docs task — mainly run any commands you cite and paste real output, then confirm `make check`).
- You MAY edit: `tools/darkfactory-render/README.md`, repo-root `README.md`, repo-root `ARCH.md`. Stage ONLY those three files. Do NOT run `git add -A`/`git add .`. Do NOT stage `UBIQUITOUS_LANGUAGE.md`, anything under `openspec/changes/`, or any other file.
- Commit once, message: `[task-8] document coverage subcommand (tool README, repo README, ARCH.md)`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-8.md` (write, do not commit).

## Handoff Report Format
- Task: Task 8 - Documentation
- Commit SHA: <full SHA>
- Approach: <what you documented where>
- Commands run: <the example commands you ran for real output; make check exit code>
- Concerns: <any>
