# Adversarial Review B

Reviewer: Claude Opus 4.6
Base: cdba772..384089e (5 commits across tasks 1-4, 6)

## Task 2: audit.sh

### HIGH: JSONC comment stripper corrupts strings containing `//`

**File:** `skills/lint-audit/audit.sh`, lines 123-128

The `strip_json_comments` function uses a perl regex to remove `//` line comments:

```perl
s{(^|[^:])//[^\n\r]*}{$1}gm;
```

The `[^:]` guard only protects `//` preceded by `:` (to preserve URLs like `https://...`). Any `//` inside a JSON string value that is NOT preceded by `:` will be stripped, corrupting the JSON.

Reproduction:

```jsonc
{
  "message": "foo // bar",
  "path": "a//b"
}
```

After stripping, both string values are truncated at `//`, producing invalid JSON that causes `jq` to fail.

The fix requires a proper string-aware comment stripper. Either parse with a state machine that tracks whether the current position is inside a quoted string, or use a tool like `jsonc-strip` or `node -e` to parse JSONC properly. A minimal perl approach would need to alternate between matching quoted strings (preserving them) and matching comments (removing them).

This is HIGH because `.oxlintrc.jsonc` is a supported config format and JSONC files commonly contain string values with `//` (URLs, file paths, documentation strings).

### HIGH: tsc --showConfig returning non-JSON crashes the script

**File:** `skills/lint-audit/audit.sh`, line 308

When `tsc --showConfig` exits 0 but produces non-JSON output (which some tsc versions do on certain error conditions), the file `$effective_config` contains garbage. The `jq` call on line 355 then fails with a parse error and `set -eu` causes the entire script to exit with no structured JSON output.

The `if !` guard on line 308 only catches non-zero exit codes. It does not validate that the output is valid JSON before proceeding.

Fix: after line 308 succeeds, validate `$effective_config` with `jq '.' "$effective_config" >/dev/null 2>&1` and fall back to `strip_json_comments` if validation fails.

### MEDIUM: Config not found returns `outcome: "ok"` instead of `"error"`

**File:** `skills/lint-audit/audit.sh`, lines 202-211 and 419

When `--oxlint-config` points to a nonexistent file, `config_parse_error` is set, and an error finding is added to `oxlint_findings`. But `emit_report` is still called with `outcome: "ok"` on line 419 because the error path only populates the findings array -- it does not change the outcome.

The SKILL.md Step 2 says `error` outcomes should stop without applying changes. With `outcome: "ok"`, the skill workflow will proceed to offer apply mode with an error finding mixed into the results.

Fix: either set a flag to emit `outcome: "error"` when `config_parse_error` is non-empty, or treat any finding with `status: "error"` in the SKILL.md workflow as blocking.

### MEDIUM: Severity field missing from audit output, SKILL.md references it

**File:** `skills/lint-audit/audit.sh`, lines 229-256; `skills/lint-audit/SKILL.md`, Step 6

SKILL.md Step 6 says: "Use each finding's `severity` when present. If severity is absent, use `"error"`." But `audit.sh` never includes `severity` in the oxlint_findings output. The finding objects contain only `name`, `status`, `category`, and `rationale`.

Currently all rules in `rules.json` have `severity: "error"`, so the fallback produces the correct result. But if a `"warn"` rule is added to `rules.json`, the SKILL.md instructions will cause the agent to write `"error"` instead of `"warn"`.

Fix: either add `severity` to the findings output in `audit.sh`, or remove the severity reference from SKILL.md and hardcode `"error"`.

### MEDIUM: `type_aware_config` detection uses recursive descent

**File:** `skills/lint-audit/audit.sh`, lines 261-271

The `.. | objects` recursive descent operator matches `typeAware` keys at any nesting depth. If a future oxlint config has a `typeAware` key nested inside a rule's options object, this would produce a false positive.

Low urgency since current oxlint configs do not have this nesting, but the detection is broader than it needs to be.

### LOW: `tr '/.' '__'` filename collision risk

**File:** `skills/lint-audit/audit.sh`, line 305

The expression `printf '%s' "$tsconfig_path" | tr '/.' '__'` maps both `/` and `.` to `_`. Two different tsconfig paths like `a.b/c.json` and `a/b.c.json` would both map to `a_b_c_json`, causing one's settings file to overwrite the other's.

This is unlikely in real projects but is a latent correctness issue. A safer approach would be to use a counter or hash.

### LOW: `find` depth limit is 3 levels, not documented

**File:** `skills/lint-audit/audit.sh`, line 283

```sh
find . \( -path './*/*/*/*' -prune \) -o -type f -name 'tsconfig*.json' -print
```

This prunes at depth 4+, so tsconfigs at `./a/b/c/tsconfig.json` are found but `./a/b/c/d/tsconfig.json` is not. The depth limit is not documented anywhere. For monorepos with deep package nesting, some tsconfigs could be silently skipped.

## Task 1: rules.json

### LOW: Validation test requires jq 1.6+ for `IN()` function

**File:** `skills/lint-audit/tests/validate-rules-json.sh`, line 14

The `IN()` function was added in jq 1.6. Debian 11 (bullseye) ships jq 1.6, so this is fine for supported platforms, but Ubuntu 20.04 ships jq 1.5 which does not have `IN()`.

Given the CLAUDE.md requirement for Debian/Ubuntu compatibility, this could fail on older LTS releases. Using `any(. == $value; .)` instead of `IN()` would be more portable.

## Task 3: SKILL.md and RULES.md

### LOW: SKILL.md Step 2 `requires_selection` lists hardcoded config filenames

**File:** `skills/lint-audit/SKILL.md`, lines 51-54

SKILL.md lists `.oxlintrc.json`, `.oxlintrc.jsonc`, and `oxlint.config.ts` as the files to show for `requires_selection`. But `audit.sh` (line 132) scans for the same three filenames independently. If the candidates list changes in `audit.sh`, the SKILL.md instructions would be stale.

This is a documentation/implementation drift risk, not a current bug.

## Task 6: README and ARCH.md

### LOW: ARCH.md skill count says 24, actual count is 23

**File:** `ARCH.md`

ARCH.md says "24 skills total" and lists 23 skill names plus "(with basic sub-skill)" for mission. There are 23 `SKILL.md` files on disk. The count is inflated by 1 if "basic" is counted as a separate skill, but basic is a sub-mode of `/mission`, not a standalone skill directory. This is a minor doc inaccuracy.

## Cross-Task Issues

### MEDIUM: No test for JSONC config parsing

The test suite (`tests/audit-sh.sh`) has no test case for `.oxlintrc.jsonc` parsing. The JSONC comment stripping is the most fragile part of `audit.sh` (see HIGH finding above), yet it has zero test coverage. There are tests for JSON configs, no-config, and eslint-only projects, but the JSONC path is untested.

### LOW: No test for `requires_selection` outcome

The `requires_selection` path (multiple config files) is not tested. This path has a subtle difference from the main path: it creates its own `$tsconfig_findings` local file and calls `emit_report` with `$type_aware_script` instead of `$type_aware`. A test would catch regressions here.

### LOW: No test for `--oxlint-config` override

The `--oxlint-config` flag is not exercised by any test. The nonexistent-file case (which produces the wrong outcome, see MEDIUM above) and the valid-override case are both uncovered.

## Summary

| Severity | Count | Key Issues |
|----------|-------|------------|
| HIGH     | 2     | JSONC stripper corrupts strings with `//`; non-JSON tsc output crashes script |
| MEDIUM   | 3     | Config-not-found returns `ok`; severity missing from output; no JSONC test coverage |
| LOW      | 6     | `IN()` portability; filename collision; find depth; doc drift; skill count; missing tests |
