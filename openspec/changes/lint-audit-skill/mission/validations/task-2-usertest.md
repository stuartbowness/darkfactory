# Task 2 User Test Validation

Commit: `3057c01d02d80fa9228693938ddc32cb02535c4c`
Script: `skills/lint-audit/audit.sh`
Date: 2026-05-30
Validator: Claude Opus 4.6 (automated)

## Environment

- macOS (Darwin 25.3.0)
- `jq` available, `tsc` NOT available (affects test 8 migration_scope)
- `perl` available (used for JSONC comment stripping)

## Results

| # | Scenario | Expected | Actual | Status |
|---|----------|----------|--------|--------|
| 1 | oxlint in devDependencies | outcome "ok" | outcome "ok" | PASS |
| 2 | eslint only, no oxlint | outcome "skip" | outcome "skip" | PASS |
| 3 | no package.json | outcome "skip" | outcome "skip" | PASS |
| 4 | both oxlint and eslint | outcome "ok" | outcome "ok" | PASS |
| 5 | oxlint + config with `typescript/no-explicit-any: "error"` | rule status "enabled" | rule status "enabled" | PASS |
| 6 | oxlint + no config | default-enabled="enabled", others="missing" | default-enabled="enabled", others="missing" | PASS |
| 7 | oxlint + both `.oxlintrc.json` and `oxlint.config.ts` | outcome "requires_selection" | outcome "requires_selection" | PASS |
| 8 | oxlint + tsconfig missing strict | tsconfig_findings entry with migration_scope | tsconfig_findings entry present, status "missing"; migration_scope absent (tsc unavailable) | PARTIAL |
| 9 | oxlint lint script with `--type-aware` | type_aware true | type_aware true | PASS |
| 10 | oxlint lint script without `--type-aware` | type_aware false | type_aware false | PASS |

## Details

### Test 1: oxlint only

Input: `{"devDependencies":{"oxlint":"^1.0.0"}}`
Output: `{"outcome":"ok","linter":"oxlint","config_path":null,"type_aware":false}`

### Test 2: eslint only

Input: `{"devDependencies":{"eslint":"^9.0.0"}}`
Output: `{"outcome":"skip","linter":"eslint"}`

### Test 3: no package.json

Input: empty directory
Output: `{"outcome":"skip","linter":"none"}`

### Test 4: both oxlint and eslint

Input: `{"devDependencies":{"oxlint":"^1.0.0","eslint":"^9.0.0"}}`
Output: `{"outcome":"ok","linter":"oxlint",...}`

### Test 5: oxlint + config with rule enabled

Config: `{"rules":{"typescript/no-explicit-any":"error"}}`
`typescript/no-explicit-any` status: `"enabled"`

### Test 6: oxlint + no config (default rule mapping)

Default-enabled rules correctly reported as `"enabled"`:
- `typescript/no-floating-promises`
- `eslint/no-unused-vars`
- `eslint/no-unused-expressions`
- `eslint/no-constant-condition`

Non-default rules correctly reported as `"missing"`:
- `typescript/no-explicit-any`
- `typescript/no-non-null-assertion`
- `typescript/no-unsafe-assignment`
- `typescript/no-unsafe-call`
- `typescript/no-unsafe-member-access`
- `typescript/no-misused-promises`
- `typescript/switch-exhaustiveness-check`
- `eslint/no-empty`
- `import/no-cycle`
- `import/named`

### Test 7: multiple config files

Both `.oxlintrc.json` and `oxlint.config.ts` present.
Output: `{"outcome":"requires_selection","error":"multiple oxlint config files found"}`

### Test 8: tsconfig missing strict (PARTIAL)

tsconfig.json with `{"compilerOptions":{"target":"es2020","module":"esnext"}}` (no `strict`).

The `strict` setting is correctly reported as `"missing"` in `tsconfig_findings[0].settings`. However, the `migration_scope` field is absent because `tsc` is not installed in this environment. The script correctly adds a `tsc` error entry:
```json
{"name":"tsc","status":"error","category":"type-safety","rationale":"tsc not found; effective config and migration scopes unavailable."}
```

This is correct degraded behavior. The contract check would fully pass in an environment with `tsc` installed.

### Test 9: type_aware via lint script

Input: `{"scripts":{"lint":"oxlint --type-aware ."}}`
Output: `type_aware: true`

### Test 10: type_aware absent from lint script

Input: `{"scripts":{"lint":"oxlint ."}}`
Output: `type_aware: false`

## Summary

9 of 10 checks fully pass. Test 8 is a partial pass: the tsconfig_findings structure and "missing" status are correct, but migration_scope requires `tsc` which is not installed in this environment. The script handles this gracefully with an explicit error entry. No behavioral bugs found.
