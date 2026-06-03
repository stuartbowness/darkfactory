# Fix Brief: Task 2 - Spec and acceptance parsing (Attempt 2)

## What Failed
Your Task 2 commit (fcb6500f) is functionally strong, but validation found two defects to fix. Both are about fail-loud honesty, not parsing logic.

1. **Silent-skip in the openspec-validate test.** `tests/openspec_validate.rs:38-44` — `acceptance_carrying_spec_passes_openspec_validate` returns early (only `eprintln!`, no assertion) when `openspec` is not on PATH. A returned-early Rust `#[test]` reports as `ok`/passed, so this test passes WITHOUT verifying anything whenever `openspec` is missing. The "skip loudly via the test name" comment is wrong — there is no skip status; it is indistinguishable from a real pass.

2. **The empty/whitespace-statement error names no offender.** When an acceptance entry's statement is empty or whitespace-only, the parser returns `RenderError::EmptyStatement`, a unit variant carrying no identifier — so the error cannot tell the author WHICH scenario or entry was at fault. Compare `EmptyAcceptanceList(String)`, which carries `capability/N`. There is also no test for a whitespace-only statement (only a genuinely-empty one).

## Diagnostic Triage
(Precise root cause; no separate triage agent needed.)

1. `tests/openspec_validate.rs:38` — the guard `if !openspec_available() { eprintln!(...); return; }` is the silent skip. `openspec` IS present in this repo (openspec 1.3.1, used by the mission pipeline), so the correct behavior is to REQUIRE it: when `openspec` is absent, the test should fail loud (`panic!`/assert with a clear message) rather than return. Keep the actual validate assertion (build temp project, run `openspec validate widget --type spec`, assert exit 0) exactly as-is for the present-openspec path.

2. `src/core/types.rs:177-178` — `RenderError::EmptyStatement` is a unit variant. Change it to carry an identifier (e.g. `EmptyStatement(String)`) with an `#[error(...)]` message that includes the offender. Then update the call site in `src/core/parse.rs` (the empty-statement path, around `parse.rs:201-214`) to pass a useful identifier — the scenario's `capability/N` and/or the entry — using the existing `scenario_identifier` helper where practical. Caution: the same code path currently also catches a structurally-malformed entry line (e.g. a non-numeric task link or missing method brackets) and maps it to `EmptyStatement`. With the new payload, do NOT let a non-empty malformed line be reported as "empty statement: <line>" — that is misleading. Distinguish a genuinely empty/whitespace statement (→ `EmptyStatement(id)`) from a structurally malformed entry, and give the malformed case an accurate message using an existing variant (do not add a new variant).

## Validator Output
- Gate 13 (silent skip detection): FAIL — `openspec_validate.rs:38-44` conditional return with no assertion.
- Gate 11 (contract checks): FAIL — the empty/whitespace-statement error does not name the offending scenario/entry; whitespace-only statement untested.
- All other gates passed (build, tests 24/24, clippy/fmt clean, module depth, code review, simplify, test quality). Do not change anything else.

## Commit to Fix
fcb6500f519c71f816ad7a5e14d5b601c26411e5

## Procedures

**Before writing any code**, invoke the `/tdd` skill (Skill tool) and follow red→green→refactor. Mandatory. Write the failing tests first: (a) a whitespace-only statement returns an error whose message/payload names the scenario/entry; (b) an empty statement likewise; (c) if you adjust the malformed-entry handling, a test that a structurally-malformed entry produces an accurate (non-"empty") error. For the openspec test, the assertion already exists — the change is removing the silent return so the test fails loud when openspec is missing.

### ENVIRONMENT — non-negotiable
You work in a git worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git/openspec in the foreground.
3. Edit source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Constraints
- Do not refactor or change anything unrelated. This is a targeted two-part fix.
- Keep `make check` green: `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` (zero fmt diffs, zero clippy -D warnings, all tests pass). Paste commands + exit codes into the handoff.
- Note: changing `RenderError::EmptyStatement`'s shape may touch any existing test that constructs/matches it — update those too so the suite stays green.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-2] fix: fail-loud openspec test + name offender in empty-statement error"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-2-fix-v2.md`

Format:
- Task: Task 2 - Spec and acceptance parsing (Fix attempt 2)
- Commit SHA: <full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes, incl. make check>
- Concerns: <anything uncertain>

## Final message
Reply with the commit SHA, a one-line summary of each of the two fixes, and the `make check` result.
