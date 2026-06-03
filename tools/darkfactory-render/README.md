# darkfactory-render

A deterministic tool that renders a mission's `contract.md` from authored
sources instead of having a human hand-write it. The sources are the per-scenario
`**Acceptance:**` blocks in each `spec.md` plus one standing closing-task
fragment. Rendering from a single source removes the drift that creeps in when
the same checks are duplicated by hand into both the specs and the contract.

This is Stage 1. The tool runs in read-only **shadow mode** only: it renders a
contract and diffs it against the existing hand-authored one for inspection. It
is **not wired into the live mission pipeline**, it never replaces `contract.md`,
and it never auto-cutovers. Switching the pipeline to rendered contracts is a
separate, gated human action (see [Cutover gate](#cutover-gate) below).

## Build and install

Run these from this directory (`tools/darkfactory-render`):

- `make build` — debug build at `./target/debug/darkfactory-render`
- `make release` — optimized build at `./target/release/darkfactory-render`
- `make install` — release build copied to `~/.local/bin/darkfactory-render`
- `make check` — format check, clippy (deny warnings), and the test suite
- `make smoke` — end-to-end smoke tests against the built binary

The examples below use the debug binary `./target/debug/darkfactory-render`.
After `make install` you can drop the path prefix and call `darkfactory-render`.

The CLI is stdlib-only (no `clap`). There is no `--help` subcommand; running with
no subcommand, an unknown subcommand, or a missing flag prints the usage banner
to stderr and exits 2:

```
$ ./target/debug/darkfactory-render
error: missing subcommand
usage:
  darkfactory-render render --specs <dir> --tasks <md> --fragment <md>
  darkfactory-render shadow --specs <dir> --tasks <md> --fragment <md> --existing <contract_md>
```

## `render`

```
darkfactory-render render --specs <specs_dir> --tasks <tasks.md> --fragment <fragment.md>
```

`render` parses the specs directory, the tasks file, and the closing-task
fragment, builds the contract model, and prints the rendered `contract.md` to
**stdout**. Any non-fatal warnings go to **stderr**, and a clean run exits **0**.
A failure (a bad path, a malformed document, a dangling task link, an unknown
closing-task kind, and so on) is a `RenderError`: a single `error: <message>`
line on stderr and exit **2**, never a partial contract.

`--specs` points at a directory whose immediate subdirectories are capabilities;
each `<capability>/spec.md` is parsed and its scenarios contribute acceptance
entries. Subdirectories are visited in sorted order so the output is
deterministic across filesystems; a subdirectory without a `spec.md` is skipped.

### Example (run against the committed fixtures)

This command was run against the built debug binary; the output below is its
actual stdout, pasted verbatim:

```
$ ./target/debug/darkfactory-render render \
    --specs tests/fixtures/cli/specs \
    --tasks tests/fixtures/cli/tasks.md \
    --fragment templates/closing-task-checks.md
# Contract

## Task 2: Greet command
<!-- Specs: widget/1 -->
- [ ] [execution] `widget greet` prints a greeting and exits 0

## Task 3: Greeting output
<!-- Specs: widget/1 -->
- [ ] [output] the greeting output contains the caller's name

## Task 7: Smoke test
- [ ] [execution] an end-to-end test runs the project's primary user flow against real dependencies (no internal mocks) and passes
```

Exit code 0; stderr empty (no warnings for this input). Note that Task 7 is a
closing task (`Closing: smoke-test` in `tasks.md`), so its checks come from the
`smoke-test` kind in `templates/closing-task-checks.md`, and a closing task
carries no `<!-- Specs: ... -->` comment.

To trigger the error path, point `--specs` at a directory that does not exist:

```
$ ./target/debug/darkfactory-render render \
    --specs /nonexistent \
    --tasks tests/fixtures/cli/tasks.md \
    --fragment templates/closing-task-checks.md
error: cannot read specs dir `/nonexistent`: No such file or directory (os error 2)
```

Exit code 2; nothing on stdout.

## `shadow`

```
darkfactory-render shadow --specs <specs_dir> --tasks <tasks.md> --fragment <fragment.md> --existing <contract.md>
```

`shadow` renders the contract the same way `render` does, then diffs it against
the existing hand-authored `contract.md` passed with `--existing`. It is
strictly **read-only**: the only filesystem touch is reading `--existing`, and it
never writes or modifies any file. The diff is diagnostic only; it is not the
cutover criterion.

Exit codes:

- **0** — the rendered contract is identical to the existing one; prints
  `no differences` to stdout.
- **1** — they differ; prints a line diff to stdout (existing-only lines prefixed
  `- `, rendered-only lines prefixed `+ `, shared lines omitted). If the line
  content matches but the files differ only in a trailing newline or in line
  endings, it prints an explicit message instead of a blank diff.
- **2** — a `RenderError` (a render failure or a failed read of `--existing`);
  prints `error: <message>` to stderr.

These shadow examples use `tests/fixtures/cli/fragment.md` (not the
`templates/` fragment), because the committed `contract_identical.md` fixture was
authored against that fragment.

### Identical contract (exit 0)

Run against the built binary; output pasted verbatim:

```
$ ./target/debug/darkfactory-render shadow \
    --specs tests/fixtures/cli/specs \
    --tasks tests/fixtures/cli/tasks.md \
    --fragment tests/fixtures/cli/fragment.md \
    --existing tests/fixtures/cli/contract_identical.md
no differences
```

Exit code 0.

### Differing contract (exit 1)

Run against the built binary; output pasted verbatim:

```
$ ./target/debug/darkfactory-render shadow \
    --specs tests/fixtures/cli/specs \
    --tasks tests/fixtures/cli/tasks.md \
    --fragment tests/fixtures/cli/fragment.md \
    --existing tests/fixtures/cli/contract_different.md
- - [ ] [execution] `widget greet` prints a DIFFERENT greeting and exits 0
+ - [ ] [execution] `widget greet` prints a greeting and exits 0
```

Exit code 1. The `- ` line is what the hand-authored contract has; the `+ ` line
is what the renderer produced.

## Acceptance authoring syntax

This is the input the renderer reads out of each `spec.md`. Under a
`#### Scenario:` heading, two optional HTML comments may appear:

- `<!-- spec-ref: N -->` — a stable integer, unique per capability. It sets the
  scenario's **Spec Reference** to `<capability>/<N>` (the capability is the
  spec's directory name). Any scenario that carries acceptance entries must have
  a `spec-ref`; a duplicate `N` within one capability is a `RenderError`.
- `<!-- status: proposed|approved|built|verified|deviated -->` — optional. An
  omitted status means non-deviated. A `deviated` scenario's Spec Reference is
  rendered with a `(DEVIATED)` marker so an approved divergence is visible in the
  contract. An unrecognized value is a `RenderError`.

After the `WHEN`/`THEN` bullets, an `**Acceptance:**` line introduces the
entries, one per bullet:

```
- [task:N] [<method>] <statement>
```

- `[task:N]` — the integer feature task this check links to. A link to a task
  that does not exist, or to a closing task, is a `RenderError`.
- `[<method>]` — `execution` (run a command), `output` (inspect output, a file,
  or a response), or `inspection` (read code). Any other word is a `RenderError`.
- `<statement>` — the falsifiable check, rendered verbatim.

Each entry inherits its scenario's Spec Reference; the reference is not repeated
on the entry. A worked example lives in `tests/fixtures/cli/specs/widget/spec.md`:

```
#### Scenario: Widget greets a caller
<!-- spec-ref: 1 -->
- **WHEN** a caller asks the widget to greet
- **THEN** the widget returns a greeting

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0
- [task:3] [output] the greeting output contains the caller's name
```

In the rendered contract, each feature task carries a `<!-- Specs: <capability>/<N> -->`
comment summarizing the spec references that fed its checks (see the `render`
example above).

## Closing-task fragment

`templates/closing-task-checks.md` is the standing source for the mandatory
closing tasks. These checks trace to no spec scenario; they are the operational
gates every mission ends with. The fragment declares the five mandatory KINDS,
each with its own `- [<method>] <statement>` checks:

- `smoke-test`
- `cleanup`
- `module-shape`
- `claude-arch`
- `documentation`

In `tasks.md`, each closing task carries a `Closing: <kind>` line in its body.
The renderer joins each closing task to the checks under its declared kind, so a
closing task needs no acceptance entries of its own. A `Closing: <kind>` whose
kind is not declared in the fragment is a `RenderError`
(`UnknownClosingKind`).

## Cutover gate

Cutover means switching the live pipeline from hand-authored contracts to
rendered ones (wiring `render` into the mission flow and retiring hand-authoring).
The renderer **never** does this on its own. Cutover is a deliberate, separate
human action, and it is gated on all of the following:

- **Validator-outcome parity across diverse mission shapes.** The rendered
  contract and the hand-authored control contract must drive the same validator
  outcomes across a range of missions: a small CLI, a large multi-task build, a
  frontend mission, and a deviation-heavy mission.
- **Independent authoring.** The acceptance source and the control
  (hand-authored) contract must be written in separate sessions, so parity is not
  an artifact of one author writing both.
- **A seeded failure is caught, plus at least one real miss.** A deliberately
  seeded bad requirement must be caught, AND the rendered-contract flow must
  catch at least one real missed-or-weak requirement.
- **A working rollback path** (below).

The text diff `shadow` prints is diagnostic only; it is not itself the cutover
criterion. Parity is judged on validator outcomes, not on a clean diff.

## Rollback procedure

The tool is additive and removable without affecting the v1 pipeline. If a
cutover proves wrong, roll back by:

1. **Regenerate the hand-authored contract** — resume the Phase 5 hand-authoring
   step to produce `contract.md` by hand again.
2. **Restore the prior validator flow** — point the pipeline back at the
   hand-authored contract, undoing the cutover wiring.

Because nothing in v1 depends on `darkfactory-render`, removing it leaves the
hand-authored flow intact.
