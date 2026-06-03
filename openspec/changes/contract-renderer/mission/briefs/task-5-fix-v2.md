# Fix Brief: Task 5 - Shadow compare mode and CLI (Attempt 2)

## What Failed
Your Task 5 commit (24fe71ec) is strong — the LCS diff is correct, the CLI IO is careful, shadow is read-only, no new deps, all 5 contract checks verified live. One defect to fix, plus two test-coverage gaps.

1. **Empty "different" diff on a trailing-newline / line-ending mismatch.** `core/shadow.rs` `compare` does a raw byte-equality check first, then (when bytes differ) builds a line diff with `str::lines()`. `lines()` strips the trailing newline and `\r`. So two contracts that are line-content-identical but differ only by a trailing newline (or LF vs CRLF) return `Diff::Different("")` — an empty diff. The CLI then prints a blank line and exits 1, telling the operator "they differ" while showing nothing. This is reachable when the existing hand-authored contract lacks a final newline.

## Diagnostic Triage
(Precise root cause — no separate triage agent.)
- `core/shadow.rs:35-40` raw `rendered == existing` byte check returns `Identical` only on exact byte equality.
- `core/shadow.rs:51-52` (the `Different` path) computes the line diff via `lines()`, which discards the trailing newline / `\r`. When the only byte difference is the trailing newline or line endings, the line lists are equal, so the diff text is empty → `Diff::Different("")`.
- The CLI prints whatever the diff text is; an empty string prints a blank line and still exits `EXIT_DIFFERENT`.

## Fix
- Make `compare` NEVER return an empty `Different` diff. When `rendered != existing` byte-wise but the line content is identical (i.e. the computed line diff would be empty), report the difference EXPLICITLY with a clear, human-readable message instead of an empty string — e.g. `Diff::Different("contents are identical; the files differ only in trailing newline or line endings".into())`, or a dedicated message the CLI prints. Choose a sensible exit behavior and state it in the handoff (either: treat newline-only as effectively "no meaningful differences" with a note and exit 0; OR keep exit 1 but with the explicit message — your call, but the operator must NEVER see a blank/empty diff). Keep the normal line-diff behavior unchanged for real content differences.
- Add a unit test in shadow.rs for the trailing-newline case: two inputs identical except the existing one lacks a final newline → `compare` returns a non-empty, explicit result (assert the message text), not `Different("")`. Add a CRLF-vs-LF variant if practical.
- Add a `render` integration test (in tests/cli.rs) with a `--specs` dir containing TWO capability subdirectories (e.g. `alpha/spec.md` and `zulu/spec.md`, each with a scenario whose acceptance links to a feature task) and assert the rendered output orders the capabilities deterministically (sorted: alpha before zulu), locking in the `capabilities.sort()` behavior. Add the needed fixtures under `tools/darkfactory-render/tests/fixtures/`.

## Validator Output
- Gate 5 (code review): FAIL — `core/shadow.rs:51-52` empty `Different` diff on trailing-newline/CRLF-only differences.
- Coverage gaps: trailing-newline edge untested; multi-capability ordering untested.
- All other gates passed (build, tests 70/70, clippy/fmt clean, module depth, simplify, read-only shadow, no new deps). Do not change anything unrelated.

## Commit to Fix
24fe71ec3ebcbd1d8194fa10ef236e9ad32f9dbe

## Procedures

**Before writing any code**, invoke the `/tdd` skill (Skill tool) and follow red→green→refactor. Mandatory. Write the failing tests first (the trailing-newline `compare` test and the 2-capability ordering test), watch them fail, then fix.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git in the foreground.
3. Edit/create source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Constraints
- Targeted fix only; do not refactor unrelated code. Add NO new dependency (stdlib only).
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` must pass clean. Paste commands + exit codes into the handoff.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-5] fix: explicit shadow diff for newline-only differences + multi-capability ordering test"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-5-fix-v2.md`
Format: Task; Commit SHA; Approach (how compare now handles newline-only diffs + the exit behavior you chose); Commands run + exit codes; Concerns.

## Final message
Reply with the new commit SHA, how `compare` now reports a newline-only difference (and the exit code), confirmation the 2-capability ordering test was added, and the `make check` result.
