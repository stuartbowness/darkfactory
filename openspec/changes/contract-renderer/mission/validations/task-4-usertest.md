# User-Test Validation: Task 4

## Result: PASS

## Test Strategy

Task 4 delivers the renderer library only (no CLI yet — that's Task 5), so this
is a library-behavior validation, not an HTTP or browser test. I exercised the
real public API end to end: `core::parse::{parse_specs, parse_tasks,
parse_fragment}` -> `core::render::{build_model, render}` ->
`RenderedContract::to_markdown()` / `::warnings()`.

I first ran the existing suite as a baseline (51 passed), then wrote a throwaway
integration test under `tests/` (`zz_usertest_task4.rs`) driving the full
pipeline on my OWN fixtures, with hard `assert_eq!` on exact rendered markdown,
de-duplicated/ordered Specs comments, the `(DEVIATED)` marker, closing-task
no-Specs rendering, every fail-loud variant, byte-identical repeat render, and
the inspection-cap boundary (over vs at exactly 20%). All 14 throwaway tests
passed (full suite 65 = 51 baseline + 14). I then deleted the throwaway test
and confirmed no source/test code remains modified in the worktree. No checks
were SKIP'd: every check here is testable via cargo and was tested.

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | Baseline: full existing crate suite builds and passes | cargo test | PASS (51) |
| 2 | parse->build->render places a check under `## Task N` named by its task link, format `- [ ] [execution] <stmt>`, statement verbatim incl. backticks/brackets/`%` | cargo test (pipeline) | PASS |
| 3 | Checks within a task ordered by Spec Reference (N asc) then source order; multi-entry scenario keeps entry order | cargo test (pipeline) | PASS |
| 4 | Cross-capability order: capability asc drives placement (alpha/9 before zulu/1 despite source order and larger N) | cargo test (pipeline) | PASS |
| 5 | Exactly one `<!-- Specs -->` per feature task, refs deduped + ordered (b/3 contributing 2 checks collapses to one ref) | cargo test (pipeline) | PASS |
| 6 | Deviated scenario ref marked `(DEVIATED)` and deduped to one occurrence | cargo test (pipeline) | PASS |
| 7 | Closing-task checks render under closing task with NO Specs comment | cargo test (pipeline) | PASS |
| 8 | Acceptance entry linking to a closing task returns `AcceptanceLinksClosingTask` | cargo test (pipeline) | PASS |
| 9 | Same input rendered twice is byte-identical | cargo test (pipeline) | PASS |
| 10 | Task sections render in task-list order (1 before 3) not link order | cargo test (pipeline) | PASS |
| 11 | Dangling task link returns `DanglingTaskLink`; feature task with zero checks returns `FeatureTaskNoChecks`; duplicate spec-ref returns `DuplicateSpecReference` — each with no partial contract | cargo test (pipeline) | PASS |
| 12 | Inspection > 20% (2 of 4 = 50%) warns; inspection at exactly 20% (1 of 5) does not warn; render still succeeds | cargo test (pipeline) | PASS |
| 13 | Throwaway test deleted, worktree code clean afterward | git status | PASS |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Rendering places each spec-derived check under the `## Task N` section named by its acceptance entry's task link, formatted `- [ ] [<method>] <statement>` | PASS | `[task:2]` entry renders under `## Task 2: Greet command` as `- [ ] [execution] ...`; exact-string match in pipeline test. |
| 2 | Rendered check text equals the acceptance statement verbatim (no rewording) | PASS | Statement `` `tool greet --loud` prints [GREETING] at 100% volume `` survived char-for-char through parse->render. |
| 3 | Checks within a task render in Spec Reference order (capability, then N), ties broken by source order | PASS | Within one capability: c/1, c/2 (entry1, entry2), c/9 despite source order 9,2,2,1. Cross-capability: alpha/9 before zulu/1. |
| 4 | Each task emits exactly one `<!-- Specs: ... -->` comment whose references are de-duplicated and ordered | PASS | `md.matches("<!-- Specs:").count() == 1`; comment is `<!-- Specs: beta/1, beta/3 -->` though beta/3 has two checks. |
| 5 | A check whose scenario has `status: deviated` is marked `(DEVIATED)` in its task's Specs comment | PASS | `<!-- Specs: acc/4 (DEVIATED) -->`, with `acc/4` appearing exactly once. |
| 6 | Closing-task (operational) checks render under their closing tasks and carry no `<!-- Specs -->` comment | PASS | `## Task 2: Cleanup` section contains the fragment checks and no `<!-- Specs:` substring. |
| 7 | An acceptance entry linking to a closing task returns an `AcceptanceLinksClosingTask` error | PASS | `build_model` with a `[task:2]` entry against a `Closing: cleanup` task returns `Err(AcceptanceLinksClosingTask(2))`. |
| 8 | Rendering the same input twice produces byte-identical output | PASS | `a.as_bytes() == b.as_bytes()` for two full pipeline renders with mixed refs, deviated status, and a closing task. |
| 9 | Task sections render in task-list order | PASS | Scenarios link 3 then 1; tasks.md lists 1 then 3; `## Task 1` appears before `## Task 3`. |
| 10 | A dangling task link, a feature task with zero checks, and a duplicate Spec Reference each return the corresponding RenderError rather than a partial contract | PASS | `DanglingTaskLink(99)`, `FeatureTaskNoChecks(2)`, and `DuplicateSpecReference("widget/5")` returned; `build_model`/`render` yield `Err`, no markdown. |
| 11 | Rendered contract uses `## Task N: <name>` sections, `<!-- Specs: capability/N -->` comments, and `- [ ] [execution|output|inspection] <statement>` checks (existing-format fidelity) | PASS | Exact-string assertions confirm `# Contract` title, `## Task N: name`, `<!-- Specs: cap/N -->`, and `- [ ] [method] stmt` lines across execution/output/inspection. |
| 12 | A task whose `[inspection]` checks exceed 20% of its checks produces a non-fatal warning; a task at or under 20% produces none | PASS | 2/4 = 50% -> `"Task 9: 50% of checks are [inspection] (exceeds 20% cap)"`; 1/5 = 20% -> empty warnings; both renders succeeded. |

## Failed Checks (if any)
None.

## Recommendations (if FAIL)
N/A — all 12 contract checks PASS, verified by running the real
`parse_* -> build_model -> render` pipeline on independent fixtures.
