# Handoff: Task 2 - Coverage analysis

- **Task**: Task 2 - Coverage analysis
- **Commit SHA**: e6441537f192401c4d13e1567283dd255ab8d2cb
- **Branch**: mission/computed-coverage (parent HEAD was cafa105, the Task 1 commit)

## Approach

Added the tolerant coverage engine in three changed files under
`tools/darkfactory-render/`:

- `src/core/types.rs`: new domain types `GapKind` (NoAcceptance /
  DanglingTaskLink / ClosingTaskLink), `CoverageGap` (capability, optional
  spec_ref, title, behavior, distinct kinds), `CoverageReport` (ordered gaps +
  total + covered), and a new `RenderError::DuplicateTaskNumber(u32, String)`
  variant carrying the offending number and task name.
- `src/lib.rs`: wired `pub mod coverage;` into the inline `core` module tree
  (alphabetically, before `parse`).
- `src/core/coverage.rs` (new): `analyze` and `check_duplicate_tasks`.

`analyze` is infallible. For each scenario, a private `classify` helper decides
coverage: a scenario is covered the moment any acceptance entry resolves to a
feature task (`closing_kind == None`) — it returns `None` (not a gap) and short-
circuits, so an unresolved sibling entry is ignored (those surface later at
render time). Otherwise it is a gap whose distinct kinds are collected in the
fixed order NoAcceptance, DanglingTaskLink, ClosingTaskLink: zero acceptance
entries gives `[NoAcceptance]`; otherwise a scan sets `dangling`/`closing`
booleans (each contributes its kind at most once). Gaps are ordered by capability
name ascending, then by global document position, using a stable sort on
`(capability, source_index)` — the index is the order out of `parse_specs`, so
within a capability the document order is preserved. `covered = total - gaps`.

`check_duplicate_tasks` is a separate fallible helper (for the Task 4 CLI path)
that returns `Err(DuplicateTaskNumber(number, name))` on the first repeated task
number. It is deliberately NOT inside `parse_tasks`, so render/shadow stay
permissive and unchanged.

Followed the `/tdd` red-green-refactor loop: stubbed `analyze`/`check_duplicate_tasks`,
drove the first no-acceptance test RED, implemented to GREEN, then added the
remaining behavior tests. `check_duplicate_tasks` was genuinely RED against its
stub before implementing. No refactor step was needed; the implementation is two
deep helpers behind a small interface.

## Tests (9 new, all in `src/core/coverage.rs`)

- no-acceptance gap (NoAcceptance), checks capability + title
- dangling-only gap (DanglingTaskLink)
- closing-task-only gap (ClosingTaskLink)
- covered: one valid feature link wins despite a dangling sibling (covered == 1)
- two distinct kinds listed once each in fixed order (Dangling before Closing)
- ordering: beta/1, alpha/2, alpha/1 in source order -> alpha/2, alpha/1, beta/1
- nice-to-have: acceptance-less scenario carrying a spec-ref is a NoAcceptance
  gap that retains its SpecReference (closes a Task 1 coverage gap)
- check_duplicate_tasks accepts distinct numbers
- check_duplicate_tasks rejects a shared number with the right error payload

## Commands run (exit codes)

- `cargo build` — 0 (baseline before changes)
- `cargo test` (baseline) — 0
- `cargo test --lib coverage::` (RED then GREEN iterations) — 1 then 0
- `cargo fmt` — 0 (applied formatting flagged by fmt --check)
- `make check` (final, from `tools/darkfactory-render/`) — **0 (PASS)**
  - cargo fmt --check: clean
  - cargo clippy --all-targets -D warnings: clean
  - cargo test: 69 unit tests pass (incl. 9 new coverage tests) plus integration
    + doc tests, all green

## Concerns

- An unrelated working-tree modification to `UBIQUITOUS_LANGUAGE.md` was present
  before I started (not made by me). I did NOT stage it. Only the three
  `tools/darkfactory-render/` files were committed. Flagging so it is not lost or
  mistaken for my change.
- `analyze` does not validate or dedupe task numbers itself (by design — that is
  `check_duplicate_tasks`, called from the CLI in Task 4). If a `TaskList` had a
  duplicate number, `analyze` would resolve a link to the first match; the CLI is
  expected to reject duplicates before calling `analyze`.
- `capability_of` falls back to an empty string when a gap scenario has no
  spec_ref (only possible for an acceptance-less scenario without a spec-ref
  comment). Such gaps sort first under the empty-string capability. Task 3's
  renderer should decide how to label a gap with no capability/spec-ref.
