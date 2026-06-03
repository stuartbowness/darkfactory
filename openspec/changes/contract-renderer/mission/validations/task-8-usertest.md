# User-Test Validation: Task 8

## Result: PASS

## Test Strategy
Task 8 is the closing cleanup task: an infrastructure change that hardens
`tools/darkfactory-render/src/core/render.rs` to fail loud (replacing silent
`unwrap_or*` fallbacks with `.expect(...)` invariant guards) and removes debris.
There is no new user-facing behavior, so I did not drive HTTP or a browser.
Instead I verified the change the way it is meant to be verified:

- Ran the two executable contract checks (clippy clean, no debris grep).
- Confirmed the fail-loud cleanup actually landed by reading the commit diff and
  grepping `render.rs` for the two banned silent fallbacks.
- Re-ran `make check` and `make smoke` to prove the render path still works after
  the cleanup (regression guard).

All commands ran inside the worktree crate at
`/Users/home/.darkfactory-2-build/tools/darkfactory-render`. HEAD is the commit
under validation (`5350696`).

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo clippy --all-targets -- -D warnings` exits 0 | Manual (cargo) | PASS |
| 2 | `grep -rn` over `src` for `todo!()`/`unimplemented!()`/`#[allow(dead_code)]` finds nothing | Manual (grep) | PASS |
| 3 | `render.rs` no longer contains `unwrap_or(VerificationMethod::Execution)` or `.unwrap_or_default()` silent fallback | Manual (grep + diff) | PASS |
| 4 | `make check` (fmt + clippy + 57 tests) is green | Manual (make) | PASS |
| 5 | `make smoke` (10 render/shadow smoke checks) is green | Manual (make) | PASS |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | [execution] `cargo clippy --all-targets -- -D warnings` exits 0 (no dead-code or unused warnings) | PASS | `cargo clippy: No issues found`, CLIPPY_EXIT=0 |
| 2 | [execution] `grep -rn` over `tools/darkfactory-render/src` finds no `todo!()`, `unimplemented!()`, or `#[allow(dead_code)]` | PASS | grep over full `src` tree returned exit 1 (zero matches) |

## Additional Verification (per task instructions)
- Fail-loud cleanup landed. Commit `5350696` diff replaces the `.unwrap_or_default()`
  on the closing-task fragment lookup with
  `.expect("validate_closing_kinds guarantees the closing kind exists in the fragment")`,
  and replaces `VerificationMethod::parse(token).unwrap_or(VerificationMethod::Execution)`
  with `VerificationMethod::parse(token).expect("parse_fragment validated every fragment check's method tag")`.
  A grep of `render.rs` for both banned patterns returns nothing.
- `make check` exit 0: `cargo fmt --check` clean, clippy clean, 57 tests pass.
- `make smoke` exit 0: 10/10 smoke checks pass (render emits a contract, shadow
  leaves an identical contract byte-unchanged, shadow flags a differing contract).
  The render path still works.
- Worktree crate is clean: `git status --short tools/darkfactory-render` is empty.

## Failed Checks (if any)
None.

## Recommendations (if FAIL)
N/A -- all checks pass.
