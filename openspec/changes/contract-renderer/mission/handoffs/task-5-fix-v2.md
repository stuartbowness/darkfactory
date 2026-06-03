# Handoff: Task 5 Fix (Attempt 2)

## Task
Fix the empty `Different("")` shadow diff on trailing-newline / line-ending-only
differences (commit 24fe71ec), and close two test-coverage gaps: a unit test for
the newline-only `compare` case and a `render` integration test with two
capability subdirectories asserting deterministic sorted ordering.

## Commit SHA
d5e5a4f47e494806fb44df768c01af5831252c8c (branch mission/darkfactory-2)

Staged only files under `tools/darkfactory-render/`; no `openspec/changes/` files
were staged.

## Approach

### The defect
`core/shadow.rs::compare` did a raw byte-equality check, then built the line diff
with `str::lines()`, which strips the trailing newline and any `\r`. So two
contracts with identical line content but a trailing-newline or LF-vs-CRLF
difference produced `Diff::Different("")` — an empty diff the CLI printed as a
blank line while exiting 1.

### The fix
`compare` now computes the line diff and, when the bytes differ but the line diff
comes back empty, returns an EXPLICIT message instead of the empty string:

    Diff::Different(NEWLINE_ONLY_MESSAGE)

where `NEWLINE_ONLY_MESSAGE` is a new public const:

    "contents are identical; the files differ only in trailing newline or line endings"

Real content differences are unchanged: a non-empty line diff is returned as
before. The operator can now NEVER see a blank `Different` diff.

### Exit behavior (the choice)
Exit code is UNCHANGED: a newline-only difference still flows through the CLI's
`Diff::Different` arm, prints the explicit message to stdout, and exits 1
(`EXIT_DIFFERENT`). Rationale: shadow's exit-1 contract means "the rendered and
existing contracts are not byte-identical, inspect the diff." A trailing-newline
or line-ending mismatch IS a real byte difference the cutover gate should see
(a hand-authored contract lacking a final newline is something the operator
should reconcile before cutover), so keeping exit 1 preserves the scriptable
"differences found" signal. The fix is only that the message is now explicit and
human-readable rather than blank. No new exit code was introduced.

### Tests added (TDD: red -> green, one slice at a time)
- `core::shadow::tests::a_trailing_newline_only_difference_is_reported_explicitly_not_empty`
  — existing input lacks the final newline; asserts `compare` returns
  `Different(NEWLINE_ONLY_MESSAGE)` and the diff is non-empty (RED first: the
  const did not exist, then GREEN after the fix).
- `core::shadow::tests::a_crlf_vs_lf_only_difference_is_reported_explicitly_not_empty`
  — CRLF existing vs LF rendered; same explicit message.
- `cli.rs::render_orders_two_capabilities_deterministically_alpha_before_zulu`
  (integration) — new fixtures `multi_specs/alpha/spec.md` and
  `multi_specs/zulu/spec.md` (each a scenario linking acceptance to feature
  Task 2) plus `multi_tasks.md`; asserts the rendered Specs comment reads
  `<!-- Specs: alpha/1, zulu/1 -->` and alpha's check line precedes zulu's.

## Commands run + exit codes
- `cargo fmt --check` -> exit 0 (zero diffs)
- `cargo clippy --all-targets -- -D warnings` -> exit 0 (no issues)
- `cargo test` -> exit 0 (73 passed across 7 suites; was 70, +3 new tests)
- `make check` (fmt + lint + test) -> exit 0
- `git rev-parse HEAD` -> d5e5a4f47e494806fb44df768c01af5831252c8c

## Concerns
- Honest scope of the ordering test: the rendered output is deterministically
  sorted (alpha before zulu) as asserted, but that ordering is enforced by
  `feature_section`'s sort on the Spec Reference key (`capability` ascending),
  NOT directly by `read_scenarios`'s `capabilities.sort()`. I verified this by
  temporarily reversing the sorted capability list in `read_scenarios`: the test
  still passed, because `feature_section` re-sorts checks by spec-ref. So
  `capabilities.sort()` in `read_scenarios` is effectively redundant for output
  ordering given the current architecture; the real guardrail for deterministic
  alpha-before-zulu output is the spec-ref sort in `feature_section`. The new
  integration test locks in the contractual sorted output (the brief's literal
  requirement), and there is already a unit test
  (`feature_checks_order_by_spec_reference_then_source_order`) covering the
  spec-ref sort directly. No code change was made to `read_scenarios`; its sort
  was left in place as a defensive, intention-revealing step.
- No new dependencies; stdlib only. No unrelated refactoring.
