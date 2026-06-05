# User-Test Validation: Task 5

## Result: PASS

## Test Strategy
The diff touches only `tools/darkfactory-render/tests/smoketest.sh` (the
end-to-end smoke test script). This is a CLI/infrastructure change, so the
correct test is to run `make smoke` from the crate root and confirm it builds
the real binary and exercises the `coverage` subcommand against real on-disk
fixtures with the asserted exit codes and output. I also ran the `coverage`
subcommand directly against both fixtures, independently of the script, to
confirm the assertions are not fabricated and that the binary genuinely
produces the asserted exit codes and summary lines.

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `make smoke` (from `tools/darkfactory-render/`) builds the binary and runs all 5 tests | Manual (shell) | PASS |
| 2 | Direct `coverage --specs migrated/specs --tasks migrated/tasks.md` (gap fixture) | Manual (CLI) | PASS |
| 3 | Direct `coverage --specs cli/covered_specs --tasks cli/covered_tasks.md` (no-gap fixture) | Manual (CLI) | PASS |
| 4 | Confirm no `.log` files written into the repo and no stray crate changes | Manual (find/git) | PASS |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | `make smoke` passes and its script invokes `darkfactory-render coverage` against a real on-disk fixture, asserting the produced report and exit code (gaps case exit 3 + summary `11 scenarios, 8 covered, 3 gaps`; no-gaps case exit 0 + `0 gaps`/`None.`) | PASS | `make smoke` -> `16/16 passed` / `ALL PASSED`, exit 0. Script Test 4 invokes `"$BIN" coverage --specs tests/fixtures/migrated/specs --tasks tests/fixtures/migrated/tasks.md`; direct run gives exit 3 + stdout `# Coverage Gap Report` and `11 scenarios, 8 covered, 3 gaps`. Test 5 invokes `"$BIN" coverage --specs tests/fixtures/cli/covered_specs --tasks tests/fixtures/cli/covered_tasks.md`; direct run gives exit 0 + `1 scenarios, 1 covered, 0 gaps` and `None.`. Both fixtures exist on disk (`tests/fixtures/migrated/specs` + `tasks.md`; `tests/fixtures/cli/covered_specs/widget/spec.md` + `covered_tasks.md`) with real spec/task content, not empty placeholders. |

## Notes
- The script's assertions use `grep -qF -e <substring>` against the binary's
  captured stdout (real output, no mocks), and `[ "$CODE" -eq N ]` against the
  captured exit code, so the checks genuinely gate on behavior.
- All temp output is written to a `mktemp -d /tmp/render-smoke.XXXXXX` dir and
  removed on exit; no `.log` or other files were written into the repo
  (`find tools/darkfactory-render -name '*.log'` -> none; crate tree clean).
- The gap fixture's summary numbers (11 scenarios, 8 covered, 3 gaps) were
  confirmed by direct execution, matching the asserted summary line exactly.

## Failed Checks (if any)
None.

## Recommendations (if FAIL)
N/A — all checks pass.
