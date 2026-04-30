# Scrutiny Validation: Task 8

## Result: PASS

## Named Facts
- Detected language: Unknown (no Cargo.toml, package.json, pyproject.toml, or requirements.txt at repo root; the task diff touches only install.sh and README.md)
- File sizes: install.sh: 18 lines (committed version); 24 lines (after simplify fix applied to working tree)
- Error handling smells: 0 matches in 18 added production lines (0 per KLOC). No language-specific patterns apply (shell script, not Rust/TS/Python).
- Test runner: N/A — no test framework for shell scripts; no test files in diff. Behavioral conformance executed manually via execution scripts.

## Check Results
| Check | Result | Notes |
|-------|--------|-------|
| Test presence | SKIP | Task produces a shell installer with no testable logic beyond what the behavioral conformance check covers. Exempt per "shell scripts" exception. |
| Build | N/A | No build system at repo root. |
| Tests | N/A | No test framework at repo root. Behavioral conformance covers this. |
| Lint | N/A | No linter configured for shell scripts. `sh -n` syntax check passed. |
| Code review | PASS | Script is correct for its primary use case. One bug identified (unguarded POSIX glob — see notes). |
| Module depth | PASS | New top-level install.sh is appropriate. No module depth change. |
| Code standards | PASS | No GNU-only flags. POSIX sh throughout. |
| Code simplify | PASS with fix | Unguarded glob bug found and fixed in working tree by simplify review. Committed code had the bug; fix applied separately. |
| Security review | N/A | No auth, secrets, user input, or network boundaries touched. Symlinks created under user's own HOME only. |
| Contract assertions | PASS | All 6 assertions verified by execution. See table below. |
| Behavioral conformance | PASS | All assertions verified by actually running the committed script. |
| Silent skip detection | SKIP | No test files in diff. |
| Diff coverage | SKIP | No production source files (Rust/TS/Python) in diff. |
| Test quality review | SKIP | No test files in diff. |

## Contract Assertions
| # | Assertion | Result | Method | Evidence |
|---|-----------|--------|--------|----------|
| 1 | `test -f install.sh && test -x install.sh` exits 0 | PASS | execution | `test -f /tmp/install_commit.sh && test -x /tmp/install_commit.sh` exited 0. File extracted from commit via `git show`. |
| 2 | First line of install.sh is `#!/bin/sh` | PASS | inspection | `head -1` of committed file returns `#!/bin/sh`. |
| 3 | All 12 symlinks exist and resolve correctly after running | PASS | execution | Ran committed script with `HOME=/Users/stuart`. All 12 symlinks verified: correct target (`~/.darkfactory/skills/<name>/`), exist as symlinks (`-L`), and resolve to live directories (`-d`). Note: target paths include a trailing slash (e.g. `…/skills/mission/`) because the glob `"$SKILLS_DIR"/*/` produces directory paths with trailing slashes. The symlinks resolve correctly — this is not a functional defect. |
| 4 | Running a second time exits 0 with all 12 symlinks still resolving (idempotent) | PASS | execution | Second run of committed script exited 0. All 12 symlinks re-verified correct after second run. `rm -f "$target"` before `ln -sf` makes re-runs safe. |
| 5 | No GNU-only flags | PASS | inspection + execution | `grep -E "sed -i|date -d|stat -c|mktemp --suffix|readlink -f|grep -P" install.sh` returned no output (exit 1). Script uses only POSIX-portable constructs: `mkdir -p`, `basename`, `rm -f`, `ln -sf`, `echo`, POSIX arithmetic. |
| 6 | Output includes one line per skill linked and a final summary line with total count | PASS | execution | Each of the 12 skills produced exactly one line: `linked: <target> -> <source>`. Final line: `Done. 12 skills installed.` |

## Failed Assertions (if any)

None. All 6 contract assertions passed.

## Code Review Notes

One bug was found during the code review and simplify phases. It does not affect any contract assertion (all assertions test the normal use case where `~/.darkfactory/skills/` exists and contains 12 directories), but it is a real defect:

**Unguarded POSIX glob (not a contract failure)**

In POSIX `sh`, when the glob `"$SKILLS_DIR"/*/` matches nothing (empty directory or missing directory), the shell does not expand it — the literal string is used as-is. The loop body runs once with `…/skills/*/` as `skill_dir`, producing a bogus symlink named `*`. This was confirmed by running the script against a temp HOME with an empty skills directory:

```
linked: /tmp/home-test3.5apHiQ/.claude/skills/* -> /tmp/home-test3.5apHiQ/.darkfactory/skills/*/
Done. 1 skills installed.
```

The simplify review identified this and applied a fix to the working tree (two guards added: a pre-loop `[ ! -d "$SKILLS_DIR" ]` check that exits 1, and a `[ -d "$skill_dir" ] || continue` guard inside the loop). The fix is not yet committed. Since all contract assertions test the normal-path behavior, this does not affect the PASS result — but the fix should be committed.

## Recommendations

The unguarded glob fix from the simplify review should be committed before the task is closed. The fix is already in the working tree at `/Users/stuart/.darkfactory/install.sh`.
