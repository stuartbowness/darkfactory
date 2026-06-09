# Handoff: Task 8 - Documentation (closing task)

- Task: Task 8 - Documentation
- Commit SHA: 9f0bf79993ceb64c03fdb42b8018cc7dfae55cce

## Approach

Documented the Stage-2 `coverage` subcommand and the `core::coverage` module
across the three existing docs named in the brief. Read each file first and
matched its style. No Rust source was touched.

- `tools/darkfactory-render/README.md`
  - Updated the stale usage banner (the no-subcommand example block) to add the
    `coverage --specs <dir> --tasks <md>` line, matching the binary's real
    output.
  - Added a full `## coverage` section after `## shadow`: usage, the tolerant
    semantics (gaps reported, never a `RenderError`), the report layout
    (`# Coverage Gap Report` / `## Summary` / `## Gaps`), the gap-kind labels,
    and the exit codes (`0` no gaps, `3` gaps found = tolerant non-failing, `2`
    usage/parse error). Two worked examples with REAL pasted stdout: the no-gaps
    `covered_specs`/`covered_tasks.md` case (exit 0) and the migrated fixture
    gaps case (exit 3).

- Repo-root `README.md`
  - Extended the existing `### darkfactory-render` paragraph with a brief Stage-2
    mention: the `coverage` subcommand, its one-line invocation, what it does
    (tolerant computed coverage over the same parsed specs/tasks), and the
    `3`/`0` exit signal. Kept it to one added sentence to match the surrounding
    tone.

- Repo-root `ARCH.md`
  - Added a `core::coverage` bullet to the module inventory (`analyze` builds a
    `CoverageReport`, the infallible counterpart to `build_model`; `render_report`
    projects the Gap report markdown; `check_duplicate_tasks` fails loud on a
    duplicate task number, kept out of `parse_tasks`).
  - Added `coverage` to the `cli` subcommand bullet (`render`, `shadow`, and
    `coverage`).
  - Added a short **Computed coverage (Stage 2)** paragraph before the existing
    **Shadow mode / cutover** paragraph, describing the Gap report and the
    `3`/`0`/`2` exit-code semantics.

## Commands run (real output pasted into the tool README)

All from `tools/darkfactory-render/` against the debug binary (`make build`):

- `./target/debug/darkfactory-render coverage --specs tests/fixtures/cli/covered_specs --tasks tests/fixtures/cli/covered_tasks.md`
  -> `1 scenarios, 1 covered, 0 gaps`, exit 0.
- `./target/debug/darkfactory-render coverage --specs tests/fixtures/migrated/specs --tasks tests/fixtures/migrated/tasks.md`
  -> `11 scenarios, 8 covered, 3 gaps`, exit 3 (matches the brief).
- `./target/debug/darkfactory-render` (no subcommand) -> usage banner now
  including the `coverage` line, exit 2 (used to refresh the README banner).
- `make check` -> exit 0 (format check + clippy deny-warnings + full test
  suite, all green).

## Verification

- `make check` passed (exit 0). No Rust changed; confirmed nothing broke.
- Staged ONLY the three doc files; committed once with the exact required
  message. Did not stage `UBIQUITOUS_LANGUAGE.md` (a pre-existing,
  unrelated modification on this branch from an earlier task) or anything under
  `openspec/changes/`.
- No `.log` files left in the repo.

## Concerns

- `UBIQUITOUS_LANGUAGE.md` carries an uncommitted modification (17 insertions,
  2 deletions) that predates this task and is unrelated to the docs work. Per
  the brief I left it untouched and unstaged. Whoever owns that change should
  reconcile it; it is NOT part of this commit.
