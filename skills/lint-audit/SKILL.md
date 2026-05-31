---
name: lint-audit
user-invokable: true
description: Audit a TypeScript project's oxlint config and tsconfig strictness against an agent-friendly ruleset. Reports gaps and offers to apply fixes. Use when setting up a new project, auditing an existing project's linting config, or before a mission to ensure mechanical enforcement is in place.
---

# Lint Audit

Audit a TypeScript project against the agent-friendly oxlint and TypeScript strictness ruleset, then offer to apply the missing mechanical safeguards.

## Inputs

- Use the project directory the user named.
- If the user did not name a project directory, use the current workspace directory.
- Resolve `<skill-dir>` as the directory containing this `SKILL.md`.

## Step 1: Run the Audit

Run:

```sh
bash <skill-dir>/audit.sh <project-dir>
```

Parse stdout as JSON. The audit script is the source of truth for detection and comparison. Do not reimplement config discovery or rule comparison in the skill workflow.

If the script exits non-zero but stdout contains valid JSON, still parse it and handle the `outcome`. If stdout is not valid JSON, show the command failure and stop.

## Step 2: Handle Outcomes

### `ok`

Present the combined report from the JSON fields, then continue to apply mode selection.

### `skip`

Explain why the audit was skipped, then stop.

- `linter: "eslint"` means the project uses ESLint but not oxlint.
- `linter: "none"` means no supported lint setup was found or no `package.json` exists.
- Any other value should be shown as reported by the script.

### `error`

Show `error` exactly enough for the user to act on it, then stop. Do not apply changes after an error outcome.

### `requires_selection`

Multiple oxlint config files exist. List the matching config files in the project root:

- `.oxlintrc.json`
- `.oxlintrc.jsonc`
- `oxlint.config.ts`

Ask the user which one to audit. Re-run:

```sh
bash <skill-dir>/audit.sh <project-dir> --oxlint-config <selected-config>
```

Parse the new JSON output and handle its outcome from the start of this step.

## Step 3: Present the Report

Use these status markers:

- `[x]` for `enabled`
- `[ ]` for `missing`
- `[!]` for `disabled` or `error`

Group the report exactly like this:

```text
Oxlint Rules
  <category>
    [x] <rule-name>
    [ ] <rule-name> - <rationale>
    [!] <rule-name> - disabled: <rationale>

TypeScript Compiler
  <tsconfig-path>
    [x] strict
    [ ] noUncheckedIndexedAccess (N errors to fix) - <rationale>
    [!] tsconfig - <rationale>

Type-Aware Linting
  [x] enabled
```

For oxlint findings, group by the `category` value from each finding. Preserve the rule names from the JSON.

For TypeScript findings, group by each `tsconfig_findings[].path`. For each setting, include `migration_scope` when present:

```text
[ ] strict (12 errors to fix) - <rationale>
```

For type-aware linting, use `type_aware`:

- `true`: `[x] enabled`
- `false`: `[ ] missing - add --type-aware to the package.json lint script`

After the report, state the active oxlint config path when `config_path` is present. If `config_path` is `null`, state that no oxlint config file was found and defaults were audited.

## Step 4: Choose Apply Mode

Offer exactly these modes:

- `apply all`: apply every missing recommendation without further prompts.
- `skip all`: make no changes and stop.
- `review per-item`: ask about each missing recommendation individually.

Only apply recommendations with status `missing`, plus type-aware linting when `type_aware` is `false`. Do not overwrite explicitly disabled rules in apply-all mode. Treat disabled rules as intentional until the user explicitly asks to change them.

## Step 5: Build the Accepted Change Set

For `apply all`, accept:

- Every oxlint finding with `status: "missing"`.
- Every tsconfig setting with `status: "missing"`.
- Type-aware linting when `type_aware` is `false`.

For `review per-item`, present each missing item and ask accept or deny.

For oxlint rules, show:

```text
<rule-name> (<category>): <rationale>
```

For tsconfig strictness settings, show the migration scope before asking:

```text
<tsconfig-path>: <setting-name> (N errors to fix)
<rationale>
```

If `migration_scope` is absent, say migration scope is unavailable.

For type-aware linting, show:

```text
package.json lint script: add --type-aware
```

## Step 6: Apply Oxlint Rule Changes

Accepted oxlint rules should be written as:

```json
{
  "rules": {
    "<rule-name>": "<severity>"
  }
}
```

Use each finding's `severity` when present. If severity is absent, use `"error"`.

### No Config Exists

If `config_path` is `null`, create `<project-dir>/.oxlintrc.json` with the accepted rules:

```json
{
  "rules": {
    "<rule-name>": "<severity>"
  }
}
```

Do not create the file when no oxlint rules were accepted.

### Existing `.oxlintrc.json`

If `config_path` is `.oxlintrc.json`, update only the `rules` object:

- Preserve all existing top-level keys.
- Preserve all existing rules.
- Add accepted rules that are not already present.
- Do not remove or change unrelated config.

### Existing `.oxlintrc.jsonc` or `oxlint.config.ts`

If `config_path` ends in `.jsonc` or `.ts`, do not edit the config file. Print the rule JSON for manual addition:

```json
{
  "<rule-name>": "<severity>"
}
```

Continue applying accepted tsconfig and package.json changes.

## Step 7: Apply TypeScript Strictness Changes

For each accepted tsconfig setting, update that specific file's `compilerOptions`.

- If `compilerOptions` exists, add the accepted setting with value `true`.
- If `compilerOptions` is missing, create it with the accepted setting.
- Preserve unrelated tsconfig keys.
- Do not edit tsconfig files for denied settings.

Supported settings are:

- `strict`
- `noUncheckedIndexedAccess`
- `erasableSyntaxOnly`

## Step 8: Apply Type-Aware Linting

When type-aware linting is accepted, update `<project-dir>/package.json`.

Find the lint script in `scripts.lint`. Append `--type-aware` to that command if it is not already present.

If there is no `scripts.lint`, report that the user must add one manually. Do not invent a lint command.

Preserve every other `package.json` field.

## Step 9: Verify and Report

After applying changes:

1. Re-run the audit command.
2. Present the updated report.
3. State which files changed.
4. State what verification command ran and whether it passed.

If manual oxlint additions were required for `.jsonc` or `.ts` configs, include them in the final report as manual follow-up items.
