# Handoff: Task 6 - Cleanup (closing task)

- **Task:** Task 6 - Cleanup
- **Commit SHA:** 13c6ec2da807dd5dedc506afbf169500299ef4a7
- **Branch:** mission/computed-coverage (tip; parent f778f97, the Task 5 commit)

## Approach

Scope was three things; only (b) and (c) produced changes.

(a) Debris scan — nothing to remove. Audited the coverage-related changes
(`src/core/coverage.rs`, `src/cli.rs`, `src/core/parse.rs`, `src/core/types.rs`,
plus `src/core/render.rs` and `src/lib.rs` which the feature also touched) against
the base `e93a768..f778f97`:
- No dead code, no unused imports, no commented-out code.
- No `#[allow(dead_code)]` / `todo!` / `unimplemented!` / TODO/FIXME markers.
  (The one `dead_code` grep hit in `types.rs:10` is prose in a doc comment
  explaining why no suppression is needed, not an attribute.)
- No backwards-compat shim, re-export, or deprecation wrapper.
- Confirmed the `capability_of` helper noted in the brief is already gone.
- Confirmed the intended posture holds: `analyze` is infallible (unresolved links
  become gap kinds, not errors) and that is documented tolerance, not a silent
  fallback; the `render.rs` `let Some(spec_ref) = ... else { continue }` only skips
  acceptance-less scenarios that genuinely place no entries. Malformed input
  (duplicate task number, bad method, etc.) still exits 2 via `coverage_pipeline`
  mapping `RenderError` to `EXIT_RENDER_ERROR`.

(b) Gitignore build logs — added `*.log` to `tools/darkfactory-render/.gitignore`
(the file already existed with `target/`). Verified with `git check-ignore
build.log` that a log file is now ignored. No stray `.log` files existed in the
repo before or after.

(c) Documented the duplicated capability field — added a doc comment on
`SpecReference.capability` in `src/core/types.rs` noting it always equals the
owning `RequirementScenario::capability` (both set from the same source in
`parse.rs`, so they cannot diverge) and that the field is kept because `render`
reads it for the `<!-- Specs: capability/N -->` comments. The field was NOT
removed.

## Out of scope (left untouched, per brief)

The malformed non-numeric task link (e.g. `[task:two]`) exits 2 but its stderr
says "unknown verification method" instead of naming the malformed link. Behavior
is contract-correct; this is a diagnostic-wording follow-up. Left as-is.

## Commands run

- `make check` (fmt + clippy -D warnings + cargo test) — exit 0 (baseline and
  after the edits).
- `make smoke` (`./tests/smoketest.sh`) — exit 0, 16/16 passed (baseline and
  after the edits).

All build output was redirected to `/tmp`, so no `.log` files were created in the
working tree. `git status` shows no stray `.log` files.

## Staging note

Staged only the two changed files under `tools/darkfactory-render/`
(`.gitignore`, `src/core/types.rs`). Did not stage the pre-existing unrelated
working-tree changes (`UBIQUITOUS_LANGUAGE.md` modification, untracked
`openspec/changes/computed-coverage/`), which were present before this task and
are not mine to commit.

## Concerns

None. Cleanup turned out to be minimal: the prior tasks left the coverage surface
clean, so the change is just the gitignore line and the doc note.
