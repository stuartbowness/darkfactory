# Task 1 User-Test Validation

**Commit:** e2ce827b1eebb85fa9cf085c24c94f1ae17761be
**File:** `skills/lint-audit/rules.json`
**Verdict:** PASS

## Contract Checks

- [x] `skills/lint-audit/rules.json` exists and is valid JSON
- [x] Top-level `"oxlint_version"` field contains semver string (`1.67.0`)
- [x] `"rules"` array contains at least 8 entries (14 found)
- [x] Each rule entry has all required fields: `name`, `category`, `severity`, `default_enabled`, `rationale`
- [x] Rule names use oxlint canonical format (no `@typescript-eslint/` prefix)
- [x] All 5 required rules present:
  - `typescript/no-explicit-any`
  - `typescript/no-non-null-assertion`
  - `typescript/no-floating-promises`
  - `typescript/no-misused-promises`
  - `typescript/switch-exhaustiveness-check`

## Details

The file contains 14 rules across 4 categories:
- **type-safety** (5): no-explicit-any, no-non-null-assertion, no-unsafe-assignment, no-unsafe-call, no-unsafe-member-access
- **error-handling** (3): no-floating-promises, no-misused-promises, no-empty
- **code-quality** (4): switch-exhaustiveness-check, no-unused-vars, no-unused-expressions, no-constant-condition
- **imports** (2): no-cycle, named

All rules have severity `"error"`. Four rules have `default_enabled: true` (no-floating-promises, no-unused-vars, no-unused-expressions, no-constant-condition); the rest are `false`.
