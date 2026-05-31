# Task 2 Scrutiny Validation

Commit: `3057c01d02d80fa9228693938ddc32cb02535c4c`
Date: 2026-05-30
Validator: scrutiny (blind -- no proposal/design/worker reasoning seen)

## Gate Results

| # | Gate | Result | Notes |
|---|------|--------|-------|
| 1 | Test presence | PASS | `skills/lint-audit/tests/audit-sh.sh` (202 lines, 5 test functions) |
| 2 | Build | N/A | Shell script |
| 3 | Tests | PASS | `sh tests/audit-sh.sh` exits 0, all assertions pass |
| 4 | Lint | PASS | `sh -n audit.sh` exits 0 (no syntax errors) |
| 5 | Code review | PASS | See findings below |
| 6 | Module depth | N/A | |
| 7 | Frontend depth | N/A | |
| 8 | Code standards | PASS | No GNU-only flags, POSIX sh shebang, `set -eu`, proper quoting |
| 9 | Code simplify | PASS | Complexity is warranted by the number of config formats handled |
| 10 | Security | PASS | No network calls, no eval, no user input passed to eval |
| 11 | Contract checks | PASS | 25/25 checks pass (see below) |
| 12 | Behavioral conformance | PASS | All checks executed with real temp directories |
| 13 | Silent skip | PASS | No contract checks were silently skipped |
| 14 | Coverage | PASS | Tests cover: no-package, eslint-only, oxlint-no-config, json-config, tsconfig-strict |
| 15 | Test quality | PASS | Tests use mock tsc, assert specific JSON fields, cover positive and negative cases |

## Contract Check Results (25/25 PASS)

| # | Check | Result |
|---|-------|--------|
| 1 | `audit.sh` exists and is executable (chmod +x) | PASS -- mode 100755 in git |
| 2 | oxlint in package.json => `"outcome": "ok"` | PASS |
| 3 | eslint only in package.json => `"outcome": "skip"` | PASS |
| 4 | No package.json => `"outcome": "skip"` | PASS |
| 5 | Both oxlint and eslint => `"outcome": "ok"` | PASS |
| 6 | `.oxlintrc.json` exists => reads it and reports rules | PASS |
| 7 | No config => default-enabled as `enabled`, others as `missing` | PASS |
| 8 | Multiple config formats => `"outcome": "requires_selection"` | PASS |
| 9 | `--oxlint-config <path>` resolves ambiguity => `"outcome": "ok"` | PASS |
| 10 | Output contains `tsconfig_findings` with per-file entries | PASS |
| 11 | Missing `strict` => `migration_scope` integer (3 errors) | PASS |
| 12 | Missing `noUncheckedIndexedAccess` => `migration_scope` integer (1 error) | PASS |
| 13 | Missing `erasableSyntaxOnly` => `"missing"` | PASS |
| 14 | `.oxlintrc.jsonc` parsed (comments stripped) => rules detected | PASS |
| 15 | `oxlint.config.ts` detected, notes auto-apply limited | PASS |
| 16 | oxlint, no eslint, no config => `"outcome": "ok"` with findings | PASS |
| 17 | `type_aware` field from `--type-aware` in lint script | PASS |
| 18 | `type_aware` false when not in lint script | PASS |
| 19 | `type_aware` true from config `typeAware` field | PASS |
| 20 | jq not available => exit 1 with JSON error (code inspection) | PASS |
| 21-25 | Additional sub-assertions (config_path, linter field, etc.) | PASS |

## Code Review Findings

### Portability

- Shebang: `#!/bin/sh` with `set -eu` -- correct POSIX sh.
- No GNU-only flags detected (`sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, `grep -P` all absent).
- No bash-isms (`[[`, process substitution, arrays, `declare`).
- `mktemp -d "${TMPDIR:-/tmp}/..."` -- POSIX-portable form.
- `find` uses `-path './*/*/*/*' -prune` for depth limiting instead of `-maxdepth` (GNU extension) -- correct.
- `wc -l | tr -d ' '` handles BSD wc's leading whitespace -- correct.

### Dependency: perl

- `strip_json_comments()` uses `perl -0777 -pe` for JSONC comment stripping.
- perl is pre-installed on macOS and virtually all Linux distros. This is acceptable.
- The regex correctly handles `://` in URLs (the `(^|[^:])` lookbehind prevents stripping `//` after a colon).

### Error handling

- `set -eu` catches all unset variables and failed commands.
- jq parse failures are caught and reported as structured JSON errors.
- Missing config files produce structured error output, not crashes.
- tsc failures fall back to raw tsconfig parsing.
- Trap cleans up temp directory on all exit paths (EXIT, HUP, INT, TERM).

### Potential issues (non-blocking)

1. **perl dependency is undocumented**: The script depends on perl for JSONC parsing but the jq-missing check at the top does not check for perl. If perl is absent and a `.jsonc` file is encountered, the script will fail with an unhelpful error. Low risk since perl is ubiquitous, but worth noting.

2. **`sed -n '1p'`**: Used at line 164 to read the first line from config candidates. This is POSIX-compliant but `head -1` would be more idiomatic. Non-blocking.

3. **Recursive descent in jq (`..`)**: Line 262 uses `.. | objects` to search for typeAware in the config. This works but could be slow on very large configs. Non-blocking since oxlint configs are small.

## Verdict

**PASS** -- All 25 contract checks pass. Code is portable, well-structured, and handles edge cases correctly. The test suite covers the critical paths. No blocking issues found.
