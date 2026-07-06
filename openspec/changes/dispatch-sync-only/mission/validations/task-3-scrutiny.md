# Scrutiny Validation: Task 3

## Result: PASS

Commit: `ac41660` (HEAD). Diff range: `eece9ea..ac41660`.
Change: converts the `/adversarial` skill from the removed `--async` +
`darkfactory-dispatch wait "$DISPATCH_ID"` pattern to a synchronous `--json`
dispatch run in one backgrounded Bash call and collected via `BashOutput`.

## Deterministic Pre-passes

- `Detected language: Unknown` (touched file is `skills/adversarial/SKILL.md`, a
  skill markdown file; repo root has no `Cargo.toml`/`package.json`/`pyproject.toml`
  that this diff touches). No code diff exists.
- `File sizes: none` (no production source files in the detected languages touched;
  the only file is markdown, 166 lines).
- `Error handling smells: 0 matches in 0 changed production lines (0 per KLOC).
  Locations: none.` (Markdown-only diff; no Rust/TS/Python production lines.)
- `Test runner: unavailable` (no test command applies to a markdown-only skill change).

Because there is no Rust/code diff, the build, test, lint, coverage, silent-skip,
test-quality, module-depth, and frontend-depth gates are N/A. Validation focuses on
the contract-checks gate and the code-review gate per the task framing.

## Gate Results

| Gate | Result | Notes |
|-------|--------|-------|
| Gate 1: Test presence | N/A | Documentation/skill-content change; produces no testable compiled code. Exempt per gate 1 exception. |
| Gate 2: Build | N/A | No build system touched; markdown-only diff. |
| Gate 3: Tests | N/A | `Test runner: unavailable`; no test command applies. |
| Gate 4: Lint | N/A | No linter applies to a skill markdown file. |
| Gate 5: Code review | PASS | Converted Step 2/Step 4 shell logic verified correct; see below. |
| Gate 6: Module depth | N/A | No source module touched. |
| Gate 7: Frontend depth review | SKIP | No `.tsx`/`.jsx` files in diff. |
| Gate 8: Code standards | PASS | No error-handling smells; no back-compat shims, fallback lookups, or dead flags. Removed the old `--async` path entirely rather than gating it. |
| Gate 9: Code simplify | PASS | Single dispatch path; no redundant branches. |
| Gate 10: Security review | N/A | No auth/secrets/network/filesystem trust boundary touched. |
| Gate 11: Contract checks | PASS | All three contract checks pass; see table. |
| Gate 12: Behavioral conformance | PASS | Status-branch logic executed in isolation across 5 cases; markers correct. |
| Gate 13: Silent skip detection | SKIP | No test files in diff. |
| Gate 14: Diff coverage | SKIP | No production source files in diff. |
| Gate 15: Test quality review | SKIP | No test files in diff. |

## Contract Checks

| # | Check | Result | Method | Evidence |
|---|-------|--------|--------|----------|
| 1 | `grep -E "\-\-async\|darkfactory-dispatch (status\|wait\|result)\|async_started" skills/adversarial/SKILL.md` returns no matches | PASS | execution | Grep returned zero matches. |
| 2 | Skill dispatches reviewer with `--json` run via `run_in_background`, collected with `BashOutput` | PASS | inspection | Line 62 and line 96 use `--json` in a single `run_in_background` Bash call; line 122 collects via `BashOutput`. |
| 3 | Skill reads dispatch `.status` from captured output; emits `REVIEW_COMPLETE` on `completed` and `REVIEW_FAILED` on `failed`/`timed_out` | PASS | execution | Lines 103-110 parse `.status`; isolated run of the branch produced `REVIEW_COMPLETE` only for `completed`+report-present, and `REVIEW_FAILED` for `failed`, `timed_out`, missing report, and dispatcher crash. |

## Code Review Detail (Gate 5)

**(a) Marker produced within the single backgrounded call.** The original bug was
that `DISPATCH_ID` was captured in one Bash call and consumed by a separate
`darkfactory-dispatch wait "$DISPATCH_ID"` in a later Bash call, so the shell
variable did not survive. The new Step 2 (lines 77-111) runs the synchronous
`--json` dispatch, captures stdout into `DISPATCH_RESULT`, parses `.status`, removes
the prompt file, and echoes the `REVIEW_COMPLETE`/`REVIEW_FAILED` marker, all inside
one script. Nothing downstream depends on a shell variable crossing a Bash-call
boundary. The bug class is eliminated.

**(b) Step 4 collects via BashOutput and reads the report only on success.** Step 4
(line 122) retrieves the backgrounded call's stdout with `BashOutput`, and if the
task has not finished, re-collects after the completion notification. Line 124 reads
the Codex report file only on a `REVIEW_COMPLETE` line; a `REVIEW_FAILED` line, or a
dispatcher `.status` of `failed`/`timed_out`, is treated as a Codex review failure
and reported (with the log path) instead of claiming a completed report.

**(c) Parallelism preserved.** Step 2 dispatches Codex first, non-blocking
(`run_in_background: true`); Step 3 dispatches the Claude reviewer via the Agent
tool while Codex runs; Step 4 collects Codex afterward. This matches the ordering
in the mission Phase 9 "Reviewer A Dispatch Script".

**Cross-check against mission Phase 9.** The Step 2 dispatch script
(`skills/adversarial/SKILL.md` lines 77-111) is structurally identical to the proven
"Reviewer A Dispatch Script (codex)" in `skills/mission/SKILL.md` lines 975-1011:
same flag set (`--model/--effort/--timeout/--sandbox/--prompt-file/--output/--cwd/
--log/--json 2>>"$LOG_PATH"`), same empty-output guard, same `.status` parse, same
`completed && [ -f "$REPORT_PATH" ]` success condition, and the same
`REVIEW_COMPLETE`/`REVIEW_FAILED` markers. The `.status` values checked
(`completed`, `failed`, `timed_out`) match the dispatcher's documented sync-mode JSON
output (`darkfactory-dispatch --help`: `"status":"completed|timed_out|failed"`).

**Behavioral test.** The parse-and-branch block was extracted and run against five
inputs:
- `completed` + report present -> `REVIEW_COMPLETE: report written`
- `completed` + report missing -> `REVIEW_FAILED: ... report missing`
- `failed` (with error) -> `REVIEW_FAILED: dispatch status failed; error: boom ...`
- `timed_out` -> `REVIEW_FAILED: dispatch status timed_out ...`
- empty output (crash) -> `REVIEW_FAILED: dispatcher crashed with no output ...`

All five produced the correct marker.

## Failed Checks

None.

## Overall Verdict: PASS
