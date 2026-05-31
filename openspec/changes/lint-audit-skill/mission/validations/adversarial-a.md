Wrote the adversarial review report to:

[adversarial-a.md](/Users/stuart/.darkfactory/openspec/changes/lint-audit-skill/mission/validations/adversarial-a.md)

Key findings included:
- 2 HIGH findings: missing `migration_scope` when `tsc` is unavailable, and the 200-test validation summary not proving 200/200 passing.
- 5 MEDIUM findings: tsconfig discovery depth bug, JSONC string corruption, disabled-rule rationale loss, import plugin enforcement gap, unsafe `--type-aware` append instruction.
- 2 LOW findings around `oxlint.config.ts` actionability and module-shape validation depth.

Checks run:
- `git log` and commit diff review for all 5 mission commits.
- `skills/lint-audit/tests/validate-rules-json.sh`
- `skills/lint-audit/tests/audit-sh.sh`
- `sh -n` on all shell scripts.
- Manual repros for the main edge cases.

Skipped: `shellcheck`, because it is not installed in this environment.

Sources used for Oxlint behavior: official Oxlint config reference and `import/no-cycle` rule docs:
- https://oxc.rs/docs/guide/usage/linter/config-file-reference
- https://oxc.rs/docs/guide/usage/linter/rules/import/no-cycle