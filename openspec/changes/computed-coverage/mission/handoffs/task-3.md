# Handoff: Task 3 - Gap report rendering

- **Task**: Task 3 - Gap report rendering
- **Commit SHA**: 2a234e765f41078dfa9ca86ec1eaaf44c21c4ed3
- **Branch**: mission/computed-coverage (parent commit 45e0efd, the Task 2 commit)

## Approach

Added the pure formatter `core::coverage::render_report(report: &CoverageReport) -> String` to
`tools/darkfactory-render/src/core/coverage.rs`. It projects a `CoverageReport` into the canonical
Gap report markdown, deterministically and byte-identically, with no I/O and no errors. It mirrors
the existing `render::render` string-building idiom (push each line terminated with `\n`).

Layout produced:

- `# Coverage Gap Report` title line.
- `## Summary` section, single line `<total> scenarios, <covered> covered, <gaps> gaps`
  (total = `report.total`, covered = `report.covered`, gaps = `report.gaps.len()`).
- `## Gaps` section. No gaps -> body is the single line `None.`. Otherwise one block per gap,
  each block preceded by one blank line.
- Each gap block:
  - `### <identifier> — <title>` heading, em dash U+2014 with one space each side. Identifier is
    `<capability>/<N>` when the gap has a `spec_ref` (uses `spec_ref.number`), else the bare
    `<capability>`.
  - `- Gap kind: <kinds>` where kinds are the distinct `GapKind`s in stored order, comma-separated,
    mapped exactly: `NoAcceptance` -> `no acceptance`, `DanglingTaskLink` -> `dangling task link`,
    `ClosingTaskLink` -> `closing-task link`.
  - `- Behavior:` followed by each stored `behavior` string prefixed with two spaces. The stored
    strings already include the leading `- `, so a line renders as `"  " + stored` ->
    `"  - **WHEN** ..."`.

Two private helpers: `gap_identifier` and `gap_kinds`.

Field names were confirmed against `src/core/types.rs` before coding: `CoverageGap { capability,
spec_ref: Option<SpecReference>, title, behavior: Vec<String>, kinds: Vec<GapKind> }`,
`CoverageReport { gaps, total, covered }`, `SpecReference { capability, number: u32 }`,
`GapKind::{NoAcceptance, DanglingTaskLink, ClosingTaskLink}`.

## Tests added (in the existing `#[cfg(test)] mod tests`)

- `render_report_emits_the_canonical_gaps_layout` — asserts the full `String` equals the exact
  worked example from the brief (gadget/1 + widget/2), byte-identical.
- `render_report_emits_the_canonical_no_gaps_layout` — asserts the exact no-gaps form.
- `render_report_uses_bare_capability_when_a_gap_has_no_spec_ref` — heading is `### widget — ...`
  with no `/N`.
- `render_report_lists_multiple_kinds_comma_separated_in_stored_order` — `dangling task link,
  closing-task link` in stored order with exact labels.

A `gap(...)` test helper builds `CoverageGap`s by hand. `analyze`, `parse`, `render`, and `shadow`
were not touched; all pre-existing tests stay green.

## Commands run

- `cargo test coverage` -> 14 passed (the 10 prior coverage tests + 4 new render_report tests).
- `cargo fmt` (one formatting fix on a long assert line).
- `make check` (from `tools/darkfactory-render/`) -> **exit 0**: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, and the full test suite (45 passed; 0 failed).

## Concerns / decisions

- **Trailing newline**: output ends with a single trailing `\n` in both the gaps and no-gaps cases,
  matching the brief's examples and the existing `render` convention (every line terminated with
  `\n`). Tests assert the full string including this trailing newline.
- **Indentation**: behavior bullets render as `"  " + stored_string`; the stored strings already
  carry the leading `- `, giving `"  - **WHEN** ..."` (two-space indent before the dash). Verified
  byte-identically against the brief's worked example.
- Scope held: implemented only `render_report` (Task 3). Did not start Task 4 (CLI). The existing
  CLI/parse/render/shadow modules are unchanged.
- Working tree has untracked `tools/darkfactory-render/*.log` files (build/clippy/fmt/test logs from
  a prior make-check run) and a pre-existing unstaged `UBIQUITOUS_LANGUAGE.md` modification. Neither
  was created by this task and neither was staged. Only `src/core/coverage.rs` was committed.

## Downstream (Task 4)

Task 4 (coverage CLI) calls `render_report` and writes the result to stdout. The function is
`pub`, infallible, and lives in `core::coverage`.
