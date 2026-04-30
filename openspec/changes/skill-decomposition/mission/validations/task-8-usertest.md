# User-Test Validation: Task 8

## Result: PASS

## Test Strategy

The commit adds `install.sh` — a shell script. Testing requires running it, not just reading it.

The real `~/.claude/skills/` directory is writable on this machine, so I tested against both:
1. A temp HOME (`mktemp -d`) with a fake `.darkfactory/skills/` tree — to isolate first-run behavior without side effects.
2. The real HOME — to verify the second-run (idempotency) and output format assertions, since the symlinks already existed from a prior install.

Both runs produced identical output (12 `linked:` lines + summary). Symlinks were checked with `readlink` after each run.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | File exists and is executable | `test -f install.sh && test -x install.sh` | PASS |
| 2 | First line is `#!/bin/sh` | `read -r firstline < install.sh` | PASS |
| 3 | All 12 symlinks created on first run | Temp HOME run + `readlink` loop | PASS |
| 4 | Second run exits 0, all 12 symlinks still correct | Real HOME second run + `readlink` loop | PASS |
| 5 | No GNU-only flags in script | `grep -E "sed -i|date -d|..."` on install.sh | PASS |
| 6 | Output: one line per skill + final count | Captured stdout from both runs | PASS |

## Contract Assertions

| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | `test -f install.sh && test -x install.sh` exits 0 | PASS | Command returned exit 0 |
| 2 | First line is `#!/bin/sh` | PASS | `read -r firstline` returned `#!/bin/sh` |
| 3 | All 12 symlinks exist and resolve correctly | PASS | All 12 skills reported `OK` via `readlink` loop in temp HOME |
| 4 | Idempotent: second run exits 0, symlinks still correct | PASS | Real HOME second run exited 0; all 12 symlinks still resolve |
| 5 | No GNU-only flags | PASS | `grep` returned no output (exit 1 = no matches) |
| 6 | Output includes one line per skill + final summary count | PASS | 12 `linked:` lines + `Done. 12 skills installed.` observed in both runs |

## Failed Assertions (if any)

None.

## Notes

The script hardcodes `DARKFACTORY="$HOME/.darkfactory"` and ignores any `DARKFACTORY` env override. This means the test instruction's suggestion to pass `DARKFACTORY=/Users/stuart/.darkfactory` as an env override does not work. The correct test method (used here) is to run with a HOME that contains `.darkfactory/skills/` — either the real HOME, or a temp HOME with the structure manually created.

This is not a contract failure: the contract only requires the script to work when run from `~/.darkfactory` as documented in README.md. The lack of an env override is a minor limitation but outside the scope of the six assertions.
