# Worker Brief: Task 8 - Cleanup (closing task: cleanup)

## Your Task

Clean up the crate before the module-shape re-check, and fix a known fail-loud debt in the deep core. Two parts:

### Part A (primary) — replace two silent fallbacks in `core::render` with fail-loud handling
The render module has two silent fallbacks for states that are provably unreachable today (the parser guarantees them), but a silent fallback violates the project's fail-loud rule. Convert both to loud, invariant-documenting handling.

1. **`fragment_check_method` in `src/core/render.rs`** currently ends with `VerificationMethod::parse(token).unwrap_or(VerificationMethod::Execution)`. The `unwrap_or(Execution)` silently defaults an unparseable method tag to `Execution` (which would *understate* the inspection-cap count). Since `parse_fragment` already validates every fragment check's `[<method>]` tag, an unparseable tag here is an invariant violation (a bug), not a runtime case. Replace the `.unwrap_or(VerificationMethod::Execution)` with `.expect("parse_fragment validated every fragment check's method tag")` (or equivalent loud handling). Do NOT default silently.
2. **`closing_section` in `src/core/render.rs`** currently looks up the fragment kind and ends with `.unwrap_or_default()`. Because `build_model` runs `validate_closing_kinds` first (which fails loud with `UnknownClosingKind` for any unknown kind), the kind is always present here. Replace the `.unwrap_or_default()` with `.expect("validate_closing_kinds guarantees the closing kind exists in the fragment")` (or equivalent), so a broken invariant panics loudly instead of silently producing an empty section.

These are localized 1-line changes each. They are on unreachable paths, so normal tests are unaffected; `make check` must stay green. (You do NOT need to add tests for the panics — they are documented invariant guards, not reachable behavior. If you can cheaply add a test that constructs an invalid model directly and asserts a panic via `#[should_panic]`, that's a nice-to-have, not required.)

If you judge the deeper fix is cleaner (store the parsed `VerificationMethod` on the fragment model so render never re-parses), you MAY do that instead — but keep it tightly scoped and keep all tests green. The `.expect` approach is the expected, minimal fix.

### Part B — standard cleanup
Remove any leftover scaffolding, dead code, temporary artifacts, and unused dependencies across `tools/darkfactory-render/`. The crate has been kept clean per-task, so this is mostly a verification pass — but check for and remove anything stray:
- Unused dependencies in `Cargo.toml` (the only dep should be `thiserror`; confirm it is actually used; remove anything unused).
- Dead code, unused `pub`/private items, commented-out code, `#[allow(dead_code)]` attributes, `todo!()`/`unimplemented!()` left behind.
- Temporary/scratch files accidentally committed under the crate.
- Do NOT remove the reserved `RenderError` variants or `pub` API that later code/consumers use — those are intentional public surface, not dead code (clippy does not flag them because the crate is a library).

## Contract checks this task must satisfy (you don't see the contract, but these are the cleanup bar)
- `cargo clippy --all-targets -- -D warnings` exits 0 (no dead-code or unused warnings).
- `grep -rn` over `tools/darkfactory-render/src` finds NO `todo!()`, `unimplemented!()`, or `#[allow(dead_code)]`.

## Project Context

(from proposal.md / mission closing-task definition) The mandatory cleanup closing task removes debris that per-task validation did not catch, so the module-shape re-check (Task 9) does not flag soon-to-be-deleted surface area. This task additionally discharges a fail-loud debt recorded against the deep core during Task 4 validation.

## What Previous Tasks Built (relevant)
- `src/core/render.rs` (Task 4, commit a12d37f1; unchanged since): contains `build_model`, `render`, and private helpers including `fragment_check_method` and `closing_section` (the two fix sites). `build_model` calls `core::parse::validate_closing_kinds` first.
- `core::parse` validates fragment method tags in `parse_fragment` and closing kinds in `validate_closing_kinds`.
- `make check` (fmt + clippy -D warnings + test) is green (76 tests); `make smoke` is green (10/10). The crate's only dependency is `thiserror = "2"`.

## Procedures

**Before changing code**, invoke the `/tdd` skill (Skill tool). For Part A the change is a fail-loud guard on an unreachable path (red-green doesn't strictly apply); ensure the full suite stays green after. For any dead code you remove, confirm tests still pass.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/make/git in the foreground.
3. Edit source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Verify
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` → green (fmt + clippy -D warnings + test). Paste exit code.
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make smoke` → still 10/10 exit 0 (the render path still works after the fail-loud change). Paste it.
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && grep -rn -e 'todo!()' -e 'unimplemented!()' -e '#\[allow(dead_code)\]' src` → no matches.
- Confirm no `unwrap_or(VerificationMethod::Execution)` / `unwrap_or_default()` silent fallback remains in render.rs.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-8] cleanup: fail-loud render guards; remove debris"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-8.md`
Format: Task; Commit SHA; Approach (the exact fail-loud change at each site; what cleanup, if any, was needed); Commands run + exit codes (make check, make smoke, the grep); Concerns.

## Final message
Reply with the commit SHA, the exact fail-loud change at each of the two render.rs sites, what (if anything) else was cleaned, confirmation `make check` and `make smoke` are green and the grep is empty.
