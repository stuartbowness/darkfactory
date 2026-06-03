# Handoff: Task 3 - Task-list and closing-task fragment parsing

## Task

Add `parse_tasks`, `parse_fragment`, and `validate_closing_kinds` to
`core::parse`, and author the standing `templates/closing-task-checks.md`
declaring the five mandatory closing-task kinds. Builds on Task 1
(`core::types`, feb7b283) and Task 2 (`core::parse`, 8c103c64).

## Commit SHA

`bf06a9d0a2371ed4b939c94ad2c8b196e33731d2`

Staged only `tools/darkfactory-render/`:
- `src/core/parse.rs` (the three new functions + helpers + inline tests)
- `templates/closing-task-checks.md` (new standing fragment)
- `tests/closing_task_fragment.rs` (new integration test)
- `tests/fixtures/tasks_sample.md` (real tasks.md copy, fixture)

## Approach

### Signatures (added to `src/core/parse.rs`, matching `parse_specs` style)

- `pub fn parse_tasks(tasks_markdown: &str) -> Result<TaskList, RenderError>`
- `pub fn parse_fragment(fragment_markdown: &str) -> Result<ClosingTaskFragment, RenderError>`
- `pub fn validate_closing_kinds(tasks: &TaskList, fragment: &ClosingTaskFragment) -> Result<(), RenderError>`

No new `RenderError` variants. No file IO (operate on in-memory `&str`, same as
`parse_specs`). Line-slice parsing, fail-loud. The `bracketed` helper from
Task 2 is reused for method-tag extraction.

### `parse_tasks`

A task begins at a `Task N: <name>` heading line. The line may carry a checkbox
prefix `- [ ]`, `- [x]`, or `- [~]`; `strip_checkbox` removes any `- [<one
char or space>]` prefix before reading `Task N:`. Number and name come from the
`Task N:` part (`task_heading` splits on the first `:` after `Task `). A
`Closing: <kind>` line anywhere in the task body (before the next heading) sets
that task's `closing_kind` via `closing_field` (matches the literal `Closing:`
prefix on the trimmed line, so `Depends on:`/`Consumes:`/`Produces:` lines never
false-match). Feature tasks get `closing_kind: None`. Content before the first
heading is ignored. Returns tasks in document order.

### `parse_fragment`

Fragment file syntax I designed (kept simple/parseable):
- `# <title>` and free prose at the top are ignored.
- A `### <kind>` heading opens a per-kind section (e.g. `### smoke-test`).
- Each check is a `- [<method>] <statement>` bullet under its kind.
- The check is STORED VERBATIM as its `[<method>] <statement>` text (the `- `
  bullet prefix is stripped; the `### ` and method brackets stay). The renderer
  (Task 4) will prefix `- [ ] ` and emit it.
- Non-bullet lines and content before the first `### ` heading are ignored.

Each check's `[<method>]` tag is validated via `VerificationMethod::parse` on
the bare word (encouraged-but-optional per the brief, for consistency with
`parse_specs`); an unknown tag fails loud as
`RenderError::UnknownVerificationMethod(<verbatim check>)`.

### `validate_closing_kinds`

Iterates tasks; for each task with `Some(kind)`, requires a `ClosingTaskKind`
of that name in the fragment. First mismatch returns
`RenderError::UnknownClosingKind(<the kind>)`. Feature tasks (`None`) are
skipped. The brief's downstream integration check (task 1 feature + task 7
`smoke-test` vs a `smoke-test` fragment = Ok; `bogus` = `UnknownClosingKind`)
is covered by two inline tests.

### `templates/closing-task-checks.md`

Declares the five mandatory kinds in order: `smoke-test`, `cleanup`,
`module-shape`, `claude-arch`, `documentation`, each with 1-2 generic
`[<method>] <statement>` checks (mostly `[execution]`/`[output]`,
`[inspection]` used sparingly). A committed-file integration test
(`committed_fragment_declares_the_five_mandatory_kinds`) parses the on-disk file
and asserts exactly those five names, each with >=1 check.

### TDD

Red->green->refactor, one vertical slice per behavior: parse_tasks
number/name; checkbox prefixes + Closing field; parse_fragment kinds->verbatim
checks; fragment unknown-method fail-loud; validate_closing_kinds Ok and
UnknownClosingKind; committed-fragment five-kinds; real-tasks end-to-end. The
last test parses the real `tasks.md` (copied to `tests/fixtures/tasks_sample.md`,
11 tasks) and validates all five closing kinds against the committed fragment.

## Commands run + exit codes

- `cargo test --lib <name>` per TDD cycle (red then green) - as expected each cycle
- `cargo fmt` - 0 (applied formatting)
- `cd tools/darkfactory-render && make check` - 0
  - `cargo fmt --check` - 0 (no diffs)
  - `cargo clippy --all-targets -- -D warnings` - 0 (no warnings)
  - `cargo test` - 0 (35 tests pass: 31 lib + 2 closing_task_fragment + 2 openspec_validate)
- `git add tools/darkfactory-render` then `git commit` - 0 (4 files, +468/-2)

## Concerns

- Task 6 (migration) may refine the exact check text in
  `templates/closing-task-checks.md` to match the archived fixture's
  hand-authored closing-task checks. This is a sensible first version per the
  brief's note; the parser does not depend on the exact statement text, only on
  the `### <kind>` / `- [<method>] ...` structure and the five kind names.
- `tests/fixtures/tasks_sample.md` is a verbatim copy of this change's
  `tasks.md` taken at commit time. If `tasks.md` is later edited (e.g. task
  count or closing tags change), `real_tasks_validate_against_the_committed_fragment`
  asserts `tasks.tasks.len() == 11` and specific indices, so the fixture (not
  the live file) is the source of truth for that test and will not drift.
- `parse_fragment`'s method validation is loud-on-unknown but does not require
  at least one check per kind; the committed-file test enforces non-empty checks
  for the standing file specifically. Task 4 can add a stricter rule if needed.
