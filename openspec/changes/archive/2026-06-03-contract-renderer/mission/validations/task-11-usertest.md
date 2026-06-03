# User-Test Validation: Task 11 (closing task: documentation)

## Result: PASS

## Test Strategy
Documentation task (tool README + repo README mention). Verified by reading the docs and, crucially, RE-RUNNING a documented command against the built binary to confirm the pasted output reproduces (the repo's "real output pasted" rule). Commit `06d9c2e5`.

## Tests Run
| # | Test | Method | Result |
|---|------|--------|--------|
| 1 | tool README exists with render + shadow + acceptance-syntax docs | read/grep | PASS |
| 2 | cutover-gate criteria + rollback procedure documented | read/grep | PASS (`## Cutover gate`, `## Rollback procedure`) |
| 3 | repo README mentions darkfactory-render | grep | PASS (3 mentions) |
| 4 | documented `render` command reproduces the pasted output | live run + verbatim line match | PASS (all 12 output lines present verbatim in the README) |
| 5 | make check + make smoke still green | make | PASS (76 tests; 10/10 smoke) |

## Contract Checks
| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | tool README documents render/shadow usage + acceptance syntax | PASS | sections present and accurate |
| 2 | cutover-gate criteria + rollback procedure documented | PASS | both sections present, match the shadow-comparison spec requirement |
| 3 | repo README mentions the tool | PASS | 3 mentions, accurate (read-only Stage 1) |
| 4 | a documented command run against the binary produces the pasted output | PASS | render output reproduced verbatim line-for-line |

## Failed Checks
None.

## Recommendations
N/A.
