# Handoff: Task 11 - Documentation (closing task: documentation)

## Task

Write the darkfactory-render tool documentation and mention the tool in the
repo-root README. Closing task, kind `documentation`.

## Commit SHA

`06d9c2e575b54e2f76150c60b71103b0055327ac`

One commit, exactly two files staged: `tools/darkfactory-render/README.md`
(new) and `README.md` (repo root). No `openspec/changes/`, CLAUDE.md, or ARCH.md
staged.

## Approach

Created `tools/darkfactory-render/README.md` covering:
- What it is: deterministic contract renderer; Stage 1, read-only shadow mode,
  not wired into the live pipeline, never auto-cutovers.
- Build/install: `make build` / `make release` / `make install` / `make check` /
  `make smoke`, plus the no-`--help`/usage-banner behavior.
- `render` usage: `--specs/--tasks/--fragment`; contract to stdout, warnings to
  stderr, exit 0; `RenderError` -> stderr + exit 2. Worked example run against
  the committed fixtures with REAL pasted stdout, plus the exit-2 error path.
- `shadow` usage: `--specs/--tasks/--fragment/--existing`; strictly read-only;
  exit 0 identical (`no differences`) / 1 different (line diff) / 2 error. Both
  the identical and differing examples run against the built binary with REAL
  pasted output.
- Acceptance authoring syntax: `<!-- spec-ref: N -->`, optional
  `<!-- status: proposed|approved|built|verified|deviated -->` (omitted =
  non-deviated; `deviated` -> `(DEVIATED)` marker), `**Acceptance:**` then
  `- [task:N] [<method>] <statement>` with method `execution`/`output`/
  `inspection`; entries inherit the scenario Spec Reference; statements verbatim.
- Closing-task fragment: 5 kinds (smoke-test, cleanup, module-shape, claude-arch,
  documentation); each closing task carries `Closing: <kind>`; renderer joins
  checks; unknown kind -> `UnknownClosingKind`.
- Cutover gate: validator-outcome parity across diverse mission shapes (small
  CLI, large multi-task, frontend, deviation-heavy); independent authoring of
  acceptance source vs control contract; seeded failure caught PLUS one real
  missed/weak requirement caught; a rollback path. Cutover is a deliberate,
  separate human action; renderer never auto-cutovers; the shadow text diff is
  diagnostic only.
- Rollback procedure: regenerate the hand-authored contract (resume Phase 5
  hand-authoring) and restore the prior validator flow; the tool is additive and
  removable without affecting v1.

Repo-root `README.md`: added a `### darkfactory-render` subsection in the
Architecture section (mirroring the darkfactory-dispatch placement/style) and
extended the Rust-toolchain prerequisite line to mention the optional tool. The
subsection links to `tools/darkfactory-render/README.md` for details and does not
overstate (read-only Stage 1, not wired into the pipeline). `darkfactory-render`
now appears 3 times in the repo README.

## Commands run + exit codes

All against the built debug binary at `./target/debug/darkfactory-render`:

- `cargo build` -> exit 0 (already built; no recompile).
- `render --specs tests/fixtures/cli/specs --tasks tests/fixtures/cli/tasks.md
  --fragment templates/closing-task-checks.md` -> exit 0; stdout pasted verbatim
  into the README; stderr empty.
- `render --specs /nonexistent ...` -> exit 2; `error: cannot read specs dir
  ...`; pasted verbatim.
- `shadow ... --fragment tests/fixtures/cli/fragment.md --existing
  tests/fixtures/cli/contract_identical.md` -> exit 0; `no differences`; pasted.
- `shadow ... --existing tests/fixtures/cli/contract_different.md` -> exit 1;
  2-line diff; pasted verbatim.
- All documented commands were re-run after authoring to confirm the pasted
  output matches byte-for-byte.
- `cargo test` -> exit 0, 76 passed (8 suites). `make check` -> exit 0 (fmt,
  clippy -D warnings, tests). `make smoke` -> exit 0, 10/10 passed.
- `git add` (two README files only) + commit -> exit 0; `git show --stat`
  confirms only the two files; no CLAUDE.md/ARCH.md/openspec in the commit.

## Concerns

- The committed `contract_identical.md` fixture was authored against
  `tests/fixtures/cli/fragment.md`, NOT `templates/closing-task-checks.md` (the
  two fragments differ on the smoke-test check wording). So the `render` example
  uses the `templates/` fragment while the `shadow` "no differences" example uses
  the fixture fragment. This is called out explicitly in the README so a reader
  reproducing the commands gets the documented exit codes. The validator should
  re-run with the exact fragment shown in each command block.
- Pre-existing orchestrator changes under `openspec/changes/contract-renderer/`
  (config.yaml, progress.md, tasks.md, design.md) and untracked mission dirs were
  left unstaged, per the brief.
