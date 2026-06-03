# Issue: Task 5 - Shadow compare mode and CLI (Attempt 1)

## Failed Gates
- Gate 5 (code review): FAIL — trailing-newline/CRLF empty diff in shadow::compare

## Observed Symptoms
- `core/shadow.rs` `compare` returns an empty "different" diff when two contracts differ ONLY by a trailing newline (or LF vs CRLF). The function first does a raw byte-equality check (so newline-different inputs are NOT treated as identical), then falls to a line-based diff built on `str::lines()`, which strips the trailing newline and `\r`. Result: no lines differ, so it returns `Diff::Different("")` — an empty diff string. The CLI then prints a blank line to stdout and exits 1 (DIFFERENT), telling the operator the contracts differ while showing nothing actionable. This is reachable whenever the existing hand-authored contract lacks a final newline (or uses CRLF), which is common.
- Coverage gaps surfaced by the same review: there is no test for the trailing-newline edge, and no `render` test with more than one capability — so the deterministic capability ordering (`capabilities.sort()` in cli.rs) is unverified by the suite.

## Validator Recommendations
- `shadow::compare` must NEVER produce an empty `Different` diff. Detect the line-content-identical-but-bytes-differ case and report it explicitly (e.g. a message like "contents identical; differs only in trailing newline / line endings") with a sensible exit code. Add a unit test for the trailing-newline case (and CRLF if practical).
- Add a `render` integration test with TWO capability subdirectories asserting the output orders capabilities deterministically (sorted), locking in the `capabilities.sort()` behavior.
- Do not add any new dependency. Targeted fix only; do not change unrelated code.
