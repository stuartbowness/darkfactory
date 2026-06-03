# Handoff: Task 8 - Cleanup (closing task)

## Task

Closing-task cleanup. Two parts:
- Part A (primary): replace two silent fallbacks in `src/core/render.rs` with fail-loud handling.
- Part B: remove leftover dead code / scaffolding / unused deps (verification pass).

## Commit SHA

`5350696619e9a52fda4b7c98b92a13d21eba6311`

Only `tools/darkfactory-render/src/core/render.rs` is staged (1 file changed, 12 insertions, 14 deletions). No `openspec/changes/` files staged.

## Approach

### Part A — fail-loud render guards (the only code change)

1. `fragment_check_method` (was render.rs:308-314). The tail
   `VerificationMethod::parse(token).unwrap_or(VerificationMethod::Execution)`
   became
   `VerificationMethod::parse(token).expect("parse_fragment validated every fragment check's method tag")`.
   rustfmt wrapped the `.expect(...)` onto its own line. Doc comment updated:
   the old comment described the silent fallback; it now states an unparseable
   tag is an invariant violation that panics loudly. `parse_fragment` validates
   every fragment check's `[<method>]` tag, so the token here is always known.

2. `closing_section` (was render.rs:242-265). The lookup chain ended with
   `.map(|known| { ... }).unwrap_or_default()`, which silently produced an empty
   `Vec<CheckLine>` for a missing kind. Restructured to fail loud: the chain now
   does `.find(...).expect("validate_closing_kinds guarantees the closing kind exists in the fragment").checks.iter().map(...).collect()`.
   `build_model` runs `validate_closing_kinds` first (fails loud with
   `UnknownClosingKind`), so the kind is always present here.

Both sites are on provably-unreachable invariant paths, so no existing test
exercises them and no new test was added (per brief: nice-to-have only). The
full suite stays green.

### Part B — cleanup verification (no changes needed)

The crate was already clean. Verified and found nothing to remove:
- `Cargo.toml`: only dependency is `thiserror = "2"`, and it is used
  (`#[derive(thiserror::Error, ...)]` in `src/core/types.rs:192`). No unused dep.
- `grep -rn` over `src` for `todo!()`, `unimplemented!()`, `#[allow(dead_code)]`:
  no matches.
- No `FIXME`/`XXX`/`HACK`/`dbg!`/`println!`/`eprintln!` debris in `src`.
- No commented-out code.
- No stray scratch/tmp files tracked under the crate
  (no `.bak`/`.tmp`/`.orig`/`.log`/scratch). `target/` is gitignored, not tracked.
- Reserved `RenderError` variants and intentional `pub` API left untouched.

## Commands run + exit codes

- `make check` (fmt --check + clippy --all-targets -D warnings + test): exit 0.
  (First run failed at the fmt step because the long `.expect` line wasn't
  wrapped; applied rustfmt's wrapping, then green.)
- `cargo test`: exit 0 (all tests pass; suite unchanged at 76).
- `make smoke`: exit 0, 10/10 passed.
- `grep -rn -e 'todo!()' -e 'unimplemented!()' -e '#[allow(dead_code)]' src`:
  no matches (exit 1).
- `grep -n -e 'unwrap_or(VerificationMethod::Execution)' -e 'unwrap_or_default()' src/core/render.rs`:
  no matches (exit 1). Both silent fallbacks gone.

## Concerns

None. The two changes are localized fail-loud guards on unreachable paths;
behavior on all reachable paths is unchanged (smoke + full suite confirm).
