Implemented Task 1 and committed it.

Commit: `e2ce827b1eebb85fa9cf085c24c94f1ae17761be`

Added:
- [rules.json](/Users/stuart/.darkfactory/skills/lint-audit/rules.json) with oxlint `1.67.0` and 14 rules
- [validate-rules-json.sh](/Users/stuart/.darkfactory/skills/lint-audit/tests/validate-rules-json.sh) schema test
- [task-1.md](/Users/stuart/.darkfactory/openspec/changes/lint-audit-skill/mission/handoffs/task-1.md) handoff report, intentionally uncommitted so it can name the final SHA

Checks run:
- `skills/lint-audit/tests/validate-rules-json.sh` -> pass
- `jq '.rules | length' skills/lint-audit/rules.json` -> `14`
- `git diff --cached --check` -> pass before commit

One concern is captured in the handoff: `typescript/no-floating-promises` is default-enabled only when oxlint type-aware linting is enabled, so Task 2 should combine `default_enabled` with its `type_aware` detection.

Sources used: [Oxc latest release](https://github.com/oxc-project/oxc), [type-aware linting](https://oxc.rs/docs/guide/usage/linter/type-aware.html), [no-floating-promises](https://oxc.rs/docs/guide/usage/linter/rules/typescript/no-floating-promises.html), [no-unused-vars](https://oxc.rs/docs/guide/usage/linter/rules/eslint/no-unused-vars.html).