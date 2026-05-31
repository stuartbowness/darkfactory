# Module Shape Audit

## Modules

- **audit.sh** -- no drift. Interface matches design (single entry point, JSON stdout). 419 lines, executable.
- **SKILL.md** -- no drift. Frontmatter matches design (name: lint-audit, user-invokable: true). Workflow covers all three apply modes.
- **rules.json** -- no drift. Schema matches design (oxlint_version, rules array). 14 rules, pinned to 1.67.0.
- **RULES.md** -- no drift. Human documentation, references same version as rules.json.

## Type Catalog

- **AuditReport** -- matches design. Fields: outcome, linter, config_path, oxlint_findings, tsconfig_findings, type_aware. Outcome values: ok, skip, error, requires_selection.
- **AuditFinding** -- matches design. Fields: name, status, category, rationale. Status values: enabled, missing, disabled, error. Note: `severity` field from rules.json is not carried into findings (design says optional), and `migration_scope` appears only on tsconfig settings findings. Both are acceptable deviations from the flat AuditFinding type -- the JSON output uses the same shape with optional fields.

## Summary

No meaningful drift between design.md and committed code.
