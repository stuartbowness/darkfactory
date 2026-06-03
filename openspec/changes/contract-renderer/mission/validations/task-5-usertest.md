# User-Test Validation: Task 5

## Result: PASS

## Test Strategy
Task 5 ships a CLI binary (`darkfactory-render`) with two subcommands, `render`
and `shadow`. This is a backend/CLI change, so I drove the REAL compiled binary
end to end rather than reading code or running only unit tests. I built with
`cargo build` and invoked `./target/debug/darkfactory-render` against the
committed fixtures under `tests/fixtures/cli/`. For each contract check I ran the
binary, captured stdout, stderr, and the exit code, and (for shadow) hashed the
existing contract before and after to prove byte-stability. I captured a baseline
`git status --porcelain` before any run and diffed it afterward to prove the
working tree is untouched. I also ran the bonus newline-only re-validation the
task flagged, and ran the full `cargo test` suite as corroboration.

The CLI exit codes (confirmed in `src/cli.rs`) are: 0 success, 1 shadow-differs
(`EXIT_DIFFERENT`), 2 RenderError or usage error (`EXIT_RENDER_ERROR` /
`EXIT_USAGE`). The fill-in's "exit non-zero" for RenderError is satisfied by 2.

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `render` over `specs/`+`tasks.md`+`fragment.md` writes contract, exit 0 | Manual (binary) | PASS |
| 2 | `shadow` vs `contract_different.md` prints `-`/`+` line diff, exit 1 | Manual (binary) | PASS |
| 3 | existing `contract_different.md` byte-unchanged after shadow (sha256 pre/post) | Manual (shasum) | PASS |
| 4 | `shadow` vs `contract_identical.md` prints "no differences", exit 0 | Manual (binary) | PASS |
| 5 | existing `contract_identical.md` byte-unchanged after shadow (sha256 pre/post) | Manual (shasum) | PASS |
| 6 | RenderError: `bad_specs/` (task 99 dangling) -> exit 2, stderr message, empty stdout | Manual (binary) | PASS |
| 7 | RenderError: missing specs dir -> exit 2, stderr message, empty stdout | Manual (binary) | PASS |
| 8 | inspection-cap breach: `inspection_specs/` -> stderr `warning:`, contract on stdout, exit 0 | Manual (binary) | PASS |
| 9 | working tree unchanged after all shadow runs (status diff + HEAD + fixtures + crate tree) | Manual (git) | PASS |
| 10 | bonus: newline-only difference -> explicit message on stdout, exit 1, never blank | Manual (binary) | PASS |
| 11 | full crate test suite (73 tests) | cargo test | PASS |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | `darkfactory-render render` over a fixture writes a contract in the expected format and exits 0. | PASS | `render --specs tests/fixtures/cli/specs --tasks .../tasks.md --fragment .../fragment.md` exited 0, empty stderr, stdout was the full contract (`# Contract` + `## Task 2/3/7` with `<!-- Specs: widget/1 -->` and `- [ ] [execution]/[output]` lines) matching `contract_identical.md` byte for byte. |
| 2 | `darkfactory-render shadow <existing-contract>` outputs a diff and leaves the existing `contract.md` byte-unchanged. | PASS | shadow vs `contract_different.md` exited 1 and printed a two-line diff (`- - [ ] ... DIFFERENT greeting ...` / `+ - [ ] ... a greeting ...`). sha256 of the existing file was identical before and after (`6bc97e4b...3ea6e7`). |
| 3 | When the rendered and existing contracts are identical, `shadow` reports no differences. | PASS | shadow vs `contract_identical.md` exited 0, stdout was exactly `no differences`, empty stderr; file sha256 unchanged (`6681a993...3b4d24`). |
| 4 | A `RenderError` makes the CLI exit non-zero with a message on stderr; an inspection-cap breach prints a warning to stderr and exits 0. | PASS | RenderError path: `bad_specs/` (acceptance links to task 99) exited 2 with stderr `error: acceptance entry links to task 99, which does not exist` and empty stdout; a missing specs dir exited 2 with `error: cannot read specs dir ...`. Inspection-cap path: `inspection_specs/` (50% inspection) exited 0, rendered the contract to stdout, and printed `warning: Task 2: 50% of checks are [inspection] (exceeds 20% cap)` to stderr. |
| 5 | A `shadow` run modifies no mission-skill or pipeline files (working tree unchanged except any explicit output path). | PASS | Baseline `git status --porcelain` captured before runs; after all shadow runs the status entries were byte-identical (the only diff was a trailing-newline artifact in the snapshot files themselves). HEAD unchanged (`d5e5a4f...`); no tracked source/skill/tool file modified; all fixtures `git diff`-clean; crate tree fully clean (no untracked scratch). shadow has no `--output` flag and writes only to stdout/stderr, so it cannot touch any file. |

## Failed Checks (if any)
None.

## Recommendations (if FAIL)
N/A -- all five contract checks pass.

## Notes
- Bonus re-validation (the stated reason for this re-run): rendering, stripping
  the trailing newline from the result, then `shadow`-ing against that file
  exited 1 and printed the explicit message
  `contents are identical; the files differ only in trailing newline or line endings`
  on stdout. The diff is never blank for differing inputs. The fix is confirmed
  through the real binary, not just the new unit tests.
- The full crate test suite passed: 73 tests across 7 suites.
- All scratch files were created under `mktemp -d` in `/tmp` and removed; the
  worktree (including the crate) is clean.
