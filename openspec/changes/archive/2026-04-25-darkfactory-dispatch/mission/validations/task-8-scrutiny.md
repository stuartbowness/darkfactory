# Task 8.1 Scrutiny Validation

**Commit:** 37d5c79
**Date:** 2026-04-23
**Validator:** Claude Opus 4.6

## Exempt Gates

- **Test presence:** Exempt (markdown skill file, not testable code)
- **Build/test/lint:** Exempt (no compiled code changed)

## Contract Assertions

### 1. `SKILL.md` includes `darkfactory-dispatch` in the required CLI tools check
**PASS**

Line 93 of SKILL.md now reads:
```
darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"
```
This sits inside the Step 4 CLI dependency check block alongside `codex`, `gh`, `openspec`, and `jq`. The pattern matches the existing convention (command, `--version`, stderr redirect, fallback echo).

### 2. Install instructions reference `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
**PASS**

Line 109 of SKILL.md now reads:
```
- darkfactory-dispatch: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
```
This is listed in the "Install commands if missing" block under Step 4, alongside install commands for `codex`, `gh`, `openspec`, and `jq`.

## Additional Checks

### Status report template updated
**PASS** -- The Step 5 status report template at line 142 includes `[x] darkfactory-dispatch` in the CLI tools section, between `codex` and `gh`. Users will see dispatch status in the setup report.

### `jq` also added
**NOTE** -- The commit adds `jq` as a new CLI dependency check (line 97) and install instruction (line 113: `brew install jq`). This is correct -- Task 7 introduced `jq` parsing of `darkfactory-dispatch` JSON output, so the init skill should verify `jq` is present. The `jq` entry also appears in the status report template at line 145.

### Alphabetical/logical ordering
**PASS** -- The CLI tools in all three locations (checks, install commands, status template) follow the same order: codex, darkfactory-dispatch, gh, openspec, jq. Consistent ordering across sections.

### No unrelated changes
**PASS** -- The diff shows exactly 11 lines added across three sections (Step 4 checks, Step 4 install commands, Step 5 report template). No lines removed. No changes to Steps 1-3. The commit is tightly scoped.

### macOS/Linux portability
**PASS** -- The `darkfactory-dispatch --version` check uses only POSIX-portable constructs. The install path `~/.darkfactory/tools/darkfactory-dispatch && make install` uses `make`, which is available on both platforms. The `brew install jq` instruction is macOS-specific, but this is consistent with the existing `brew install gh` instruction -- both tools are available via other package managers on Linux, and the init skill reports what to install rather than running the install itself.

## Code Review

The diff is clean. 11 lines added, 0 removed. All additions follow the existing patterns in the file exactly. The three locations that needed updating (CLI check block, install instructions, status template) are all covered.

## Verdict

**PASS** -- Both contract assertions satisfied. Status template updated. No portability issues. Commit is minimal and correctly scoped.
