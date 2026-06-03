# User-Test Validation: Task 3

## Result: PASS

## Test Strategy
The commit delivers a Rust parser layer in `tools/darkfactory-render/src/core/parse.rs`
(public functions `parse_tasks`, `parse_fragment`, `validate_closing_kinds`),
a standing template `templates/closing-task-checks.md`, and integration tests.
There is no HTTP API and no browser UI, so this is verified by running the code:
`cargo test` against the committed suite, a validator-authored throwaway
integration test exercising each contract check with my own fixtures against the
public API, and direct inspection of the committed `templates/closing-task-checks.md`.
No check was SKIP'd; all four are testable via cargo. The throwaway test was
deleted after the run and the crate worktree is clean.

I confirmed HEAD is at the commit under validation
(`bf06a9d0a2371ed4b939c94ad2c8b196e33731d2`). The only dirty working-tree paths
are mission bookkeeping files outside the crate; no crate source was modified.

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | Committed crate suite (`cargo test`) | Manual (cargo) | PASS |
| 2 | Throwaway: parse a validator-authored tasks.md, assert number/name/`Closing:` extracted | Manual (cargo) | PASS |
| 3 | Throwaway: parse committed `templates/closing-task-checks.md`, assert kinds map to non-empty verbatim `[<method>]` checks | Manual (cargo) | PASS |
| 4 | Throwaway: assert committed fragment declares all five mandatory kinds | Manual (cargo) | PASS |
| 5 | Throwaway: a `Closing:` kind absent from the fragment yields `UnknownClosingKind`; a known kind validates OK | Manual (cargo) | PASS |
| 6 | Inspect `templates/closing-task-checks.md` on disk | Manual (Read) | PASS |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Parsing a tasks.md fixture extracts each task's number, name, and `Closing: <kind>` field where present. | PASS | `parse_tasks` returns a `TaskList`; my fixture with non-sequential numbers (1, 4, 9) and a name containing an embedded colon parsed to `number`/`name`/`closing_kind = None` for the two feature tasks and `closing_kind = Some("smoke-test")` for the closing task. Committed test `parse_tasks_extracts_number_and_name_from_a_feature_task` and `parse_tasks_handles_all_checkbox_prefixes_and_closing_kind` confirm the same. |
| 2 | Parsing `templates/closing-task-checks.md` yields the closing-task kinds mapped to their checks. | PASS | `parse_fragment` over the committed on-disk template (via `include_str!`) produced 5 `ClosingTaskKind`s, each with >= 1 check stored verbatim as `[<method>] <statement>` (the `- ` bullet stripped). Spot-check: `cleanup` maps to 2 checks (`[execution]...`, `[inspection]...`). |
| 3 | `templates/closing-task-checks.md` exists and declares kinds `smoke-test`, `cleanup`, `module-shape`, `claude-arch`, and `documentation`. | PASS | File exists at `tools/darkfactory-render/templates/closing-task-checks.md` (committed in this diff). Direct read shows `### smoke-test`, `### cleanup`, `### module-shape`, `### claude-arch`, `### documentation`. `parse_fragment` reports exactly those five names; committed test `committed_fragment_declares_the_five_mandatory_kinds` pins the order. |
| 4 | A task whose `Closing: <kind>` is absent from the fragment returns an `UnknownClosingKind` error. | PASS | `validate_closing_kinds` on a task carrying `Closing: not-a-real-kind` (a kind absent from the committed fragment) returned `Err(RenderError::UnknownClosingKind("not-a-real-kind"))`. A task tagged `Closing: smoke-test` (declared by the fragment) returned `Ok(())`. Committed test `validate_closing_kinds_rejects_a_kind_absent_from_the_fragment` confirms the error variant carries the offending kind. |

## Failed Checks (if any)
None.

## Recommendations (if FAIL)
N/A.
