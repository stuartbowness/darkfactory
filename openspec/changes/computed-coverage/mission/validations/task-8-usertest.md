# User-Test Validation: Task 8

## Result: PASS

## Test Strategy

This is a documentation-only commit (`9f0bf79`): it touches `ARCH.md`, repo
`README.md`, and `tools/darkfactory-render/README.md`, with no source changed.
So the test strategy is two-part:

1. Confirm each of the three docs describes the `coverage` subcommand, its flags
   (`--specs`/`--tasks`), and its exit codes (`0`/`3`/`2`).
2. Build the `darkfactory-render` binary and RUN every example command the tool
   README pastes, byte-comparing the pasted output (and exit code) against the
   real output to prove nothing is fabricated. I also ran the documented error
   paths (bad path, duplicate task number) to confirm the exit-`2` claim, and
   confirmed `coverage` rejects `--fragment`/`--existing` as the README implies.

All commands were run from `tools/darkfactory-render/`. No `.log` files were
written into the repo; no files were created anywhere in the working tree.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo build` succeeds; binary present at `target/debug/darkfactory-render` | Manual | PASS |
| 2 | README "No gaps (exit 0)" example: output matches verbatim, exit 0 | CLI | PASS |
| 3 | README "Gaps found (exit 3)" example: output matches verbatim, exit 3 | CLI | PASS |
| 4 | Bad path -> `error: ...` on stderr, exit 2 | CLI | PASS |
| 5 | Duplicate task number -> `error: duplicate task number 2`, exit 2 | CLI | PASS |
| 6 | `coverage --fragment ...` rejected with `error: unexpected flag` , exit 2 | CLI | PASS |
| 7 | `--help` lists the `coverage` subcommand | CLI | PASS |
| 8 | Referenced fixtures (`covered_specs`, `covered_tasks.md`, `migrated/specs`, `migrated/tasks.md`) exist | Manual | PASS |
| 9 | `core::coverage` module exists with `analyze`, `render_report`, `check_duplicate_tasks` (matches ARCH.md) | Manual | PASS |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | `tools/darkfactory-render/README.md` documents the coverage subcommand (`--specs`/`--tasks`, exit `0`/`3`/`2`) with an example whose pasted output matches the tool's real output | PASS | README adds a `## coverage` section (line 156+) documenting `--specs`/`--tasks`, the report shape, and exit codes 0/3/2. Both pasted examples were run and reproduced byte-for-byte: the `covered_specs` example printed `1 scenarios, 1 covered, 0 gaps` + `None.` (exit 0); the `migrated` example printed `11 scenarios, 8 covered, 3 gaps` with the three `domain-type-boundary` "no acceptance" gap blocks (exit 3). The exit-2 claim holds: bad path and duplicate task number both print `error: <message>` to stderr and exit 2. No fabricated output. |
| 2 | repo `README.md` mentions the coverage subcommand / Stage 2 | PASS | Line 141 documents `darkfactory-render coverage --specs <dir> --tasks <md>`, calls it "Stage 2", describes the tolerant computed coverage check, and states exit `3` on gaps / `0` when fully covered. |
| 3 | `ARCH.md` describes the `core::coverage` module and `coverage` subcommand | PASS | Line 88 describes `core::coverage` (with `analyze`, `render_report`, `check_duplicate_tasks`); line 89 lists `coverage` as a CLI subcommand; line 97 adds a "Computed coverage (Stage 2)" paragraph covering the Gap report shape and exit codes 0/3/2. The described module/functions exist in `src/core/coverage.rs` and are wired into `src/cli.rs`. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A — all three contract checks pass. Documentation is accurate and the pasted
example output is reproducible against the real binary (no fabrication).
