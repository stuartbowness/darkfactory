# Worker Brief: Task 4 - Coverage CLI subcommand

## Your Task

Add a `coverage` subcommand to the `darkfactory-render` CLI that runs the whole coverage pipeline end-to-end:

`darkfactory-render coverage --specs <dir> --tasks <md>`

Behavior:
1. Read `--specs <dir>`: scan for capability subdirectories containing `spec.md` and parse each with `parse_specs(<capability-name>, <spec.md contents>)`, collecting all scenarios across capabilities. **Reuse the exact spec-directory-reading logic the existing `render` subcommand uses** — read `tools/darkfactory-render/src/cli.rs` first and mirror how `render` discovers and reads capability dirs, so `coverage` and `render` agree on what they read.
2. Read `--tasks <md>` and `parse_tasks` it.
3. Run `core::coverage::check_duplicate_tasks(&tasks)`. If it returns `Err(RenderError::DuplicateTaskNumber(..))`, exit with code `2` and an actionable stderr message.
4. Run `core::coverage::analyze(&scenarios, &tasks)` → `CoverageReport`.
5. Write `core::coverage::render_report(&report)` to **stdout**.
6. Exit code: **`0`** when `report.gaps` is empty, **`3`** when there are one or more gaps.

Error handling / exit `2` (usage or parse error):
- Missing `--specs` or `--tasks` argument → exit `2` (usage).
- An `--specs`/`--tasks` path that is unreadable or missing → exit `2`.
- Any `RenderError` from `parse_specs`/`parse_tasks` (unknown verification method, malformed task link, duplicate Spec Reference within a capability, an acceptance-carrying scenario with no Spec Reference, etc.) → exit `2` with an actionable stderr message.
- A dangling acceptance link (points to a non-existent task) is NOT an error — it is a gap, so it yields exit `3`, not `2`.

Use the crate's existing exit-code convention (look in `cli.rs`: usage/parse errors already exit `2`; shadow uses `1` for "different"). Add a distinct `3` for "gaps found". Do not change `render`/`shadow` exit codes.

### Done when
- Integration tests in a new `tests/coverage.rs` cover: exit `0` (no gaps), exit `3` (gaps, with the report on stdout), and exit `2` for each of: missing `--specs`, missing `--tasks`, an unreadable/missing path, an unrecognised verification method, a malformed task link, a duplicate Spec Reference, an acceptance-carrying scenario with no Spec Reference, and a duplicate task number.
- A test that runs `coverage` twice on the same `--specs`/`--tasks` and asserts byte-identical stdout (full pipeline determinism).
- A test that runs `coverage` against Stage 1's migrated fixture `tools/darkfactory-render/tests/fixtures/migrated/` (specs + `tasks.md`, already in the acceptance-field format) and asserts it completes with a small, stable gap count (that fixture's `domain-type-boundary` capability has acceptance-less scenarios that are gaps) — assert the exact gap count you observe, so the test pins it.
- `make check` is green.

Create small fixtures for the exit-code cases under `tests/fixtures/cli/` (mirror the existing CLI test fixtures there).

## Project Context
Stage 2: this is the user-facing entry point. The orchestrator (or a person) runs `darkfactory-render coverage --specs ... --tasks ...` alongside the hand-authored Phase 4a coverage audit during shadow mode. The three exit codes let CI/the orchestrator mechanically tell clean from gaps-found from a tool error.

## Architecture
The CLI is the thin transport layer: it reads files, calls `parse_specs`/`parse_tasks` then `check_duplicate_tasks` then `analyze` then `render_report`, and maps results to stdout + exit code. All logic already exists in `core` (Tasks 1-3); this task is wiring + the exit-code policy + integration tests. Mirror the structure of the existing `render`/`shadow` subcommands.

## Key Terms
- Exit codes: `0` no gaps, `3` gaps found (tolerant, non-failing), `2` usage/parse error.

## Domain Types
- `core::parse::parse_specs(capability, markdown) -> Result<Vec<RequirementScenario>, RenderError>`.
- `core::parse::parse_tasks(markdown) -> Result<TaskList, RenderError>`.
- `core::coverage::check_duplicate_tasks(&TaskList) -> Result<(), RenderError>`.
- `core::coverage::analyze(&[RequirementScenario], &TaskList) -> CoverageReport`.
- `core::coverage::render_report(&CoverageReport) -> String`.
- `RenderError` (`core::types`) — map any variant to exit `2`.

## What Previous Tasks Built

### Task 1 (cafa105), Task 2 (e644153 + 45e0efd), Task 3 (2a234e7)
- `parse_specs` retains every scenario (incl. acceptance-less) and sets `RequirementScenario.capability` for all of them.
- `core::coverage` (in `src/core/coverage.rs`, wired via `pub mod coverage` in `lib.rs`) has `analyze` (infallible), `check_duplicate_tasks` (fallible → `DuplicateTaskNumber`), and `render_report` (canonical markdown, trailing `\n`).
- `RenderError` includes `DuplicateTaskNumber`, plus the existing parse errors (unknown method, malformed task link, duplicate spec-ref, MissingSpecRef, etc.).
- The existing `render`/`shadow` subcommands and their spec-dir reading live in `src/cli.rs` — read it and reuse the spec-dir reader.

## Downstream Consumers
- Task 5 (smoke test) runs this `coverage` subcommand end-to-end via `make smoke`. Keep the CLI invocation stable (`coverage --specs <dir> --tasks <md>`).

## Procedures
**Before writing any code**, invoke the `/tdd` skill (mandatory) and follow red-green-refactor.
- Read `src/cli.rs` first to mirror the existing subcommand structure, arg parsing, spec-dir reading, and exit-code handling.
- Work only inside `tools/darkfactory-render/`. Verify with `make check` and `make smoke`. Do NOT leave `.log` files in the repo (redirect to /tmp if needed).
- Commit once, code+tests+fixtures only, message: `[task-4] coverage CLI subcommand (--specs/--tasks, exit 0/3/2)`. Stage only files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/computed-coverage/`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-4.md` (write, do not commit).

## Handoff Report Format
- Task: Task 4 - Coverage CLI subcommand
- Commit SHA: <full SHA>
- Approach: <what you did>
- Commands run: <incl. make check + make smoke exit codes>
- Concerns: <any, esp. the migrated-fixture gap count you observed>
