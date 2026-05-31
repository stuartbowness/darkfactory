# Task 3 Scrutiny Validation

Commit: `6e3929b1f749b8e0ceac2f90a325fca0ce9a233d`
Files: `skills/lint-audit/SKILL.md`, `skills/lint-audit/RULES.md`

## Contract Checks

### 1. `skills/lint-audit/SKILL.md` exists with frontmatter containing `name: lint-audit` and `user-invokable: true`

**PASS**

```yaml
---
name: lint-audit
user-invokable: true
description: Audit a TypeScript project's oxlint config and tsconfig strictness against an agent-friendly ruleset...
---
```

Lines 1-5 of SKILL.md.

### 2. SKILL.md instructs the agent to run `audit.sh` and parse its JSON output

**PASS**

> "Run: `bash <skill-dir>/audit.sh <project-dir>`"
> "Parse stdout as JSON. The audit script is the source of truth for detection and comparison. Do not reimplement config discovery or rule comparison in the skill workflow."

Lines 19-25.

### 3. SKILL.md includes apply-all, skip-all, and per-item review modes

**PASS**

> "Offer exactly these modes:
> - `apply all`: apply every missing recommendation without further prompts.
> - `skip all`: make no changes and stop.
> - `review per-item`: ask about each missing recommendation individually."

Lines 108-112 (Step 4).

### 4. SKILL.md instructs creating `.oxlintrc.json` when no oxlint config exists and user accepts rules

**PASS**

> "If `config_path` is `null`, create `<project-dir>/.oxlintrc.json` with the accepted rules"
> "Do not create the file when no oxlint rules were accepted."

Lines 162-172 (Step 6, "No Config Exists" subsection).

### 5. SKILL.md instructs updating existing `.oxlintrc.json` by adding rules without removing existing ones

**PASS**

> "If `config_path` is `.oxlintrc.json`, update only the `rules` object:
> - Preserve all existing top-level keys.
> - Preserve all existing rules.
> - Add accepted rules that are not already present.
> - Do not remove or change unrelated config."

Lines 176-182 (Step 6, "Existing `.oxlintrc.json`" subsection).

### 6. SKILL.md instructs printing manual instructions (not auto-editing) for `.oxlintrc.jsonc` and `oxlint.config.ts` formats

**PASS**

> "If `config_path` ends in `.jsonc` or `.ts`, do not edit the config file. Print the rule JSON for manual addition"

Lines 185-193 (Step 6, "Existing `.oxlintrc.jsonc` or `oxlint.config.ts`" subsection).

### 7. SKILL.md instructs updating tsconfig.json files to add strictness settings when accepted

**PASS**

> "For each accepted tsconfig setting, update that specific file's `compilerOptions`.
> - If `compilerOptions` exists, add the accepted setting with value `true`.
> - If `compilerOptions` is missing, create it with the accepted setting.
> - Preserve unrelated tsconfig keys.
> - Do not edit tsconfig files for denied settings."

Lines 197-203 (Step 7).

### 8. SKILL.md instructs showing migration scope before each strictness setting (strict, noUncheckedIndexedAccess, erasableSyntaxOnly) in per-item mode

**PASS**

> "For tsconfig strictness settings, show the migration scope before asking:
> ```text
> <tsconfig-path>: <setting-name> (N errors to fix)
> <rationale>
> ```
> If `migration_scope` is absent, say migration scope is unavailable."

Lines 131-139 (Step 5). The report format in Step 3 also includes migration scope inline:

> "For TypeScript findings, group by each `tsconfig_findings[].path`. For each setting, include `migration_scope` when present:
> `[ ] strict (12 errors to fix) - <rationale>`"

Lines 91-95 (Step 3).

The three supported settings are explicitly listed in Step 7 (line 205-208): `strict`, `noUncheckedIndexedAccess`, `erasableSyntaxOnly`.

### 9. SKILL.md instructs modifying the package.json lint script to add `--type-aware` when accepted

**PASS**

> "When type-aware linting is accepted, update `<project-dir>/package.json`.
> Find the lint script in `scripts.lint`. Append `--type-aware` to that command if it is not already present.
> If there is no `scripts.lint`, report that the user must add one manually. Do not invent a lint command.
> Preserve every other `package.json` field."

Lines 212-218 (Step 8).

### 10. `skills/lint-audit/RULES.md` exists with human-readable documentation of the agent-friendly ruleset

**PASS**

RULES.md exists (97 lines). It documents each category (Type-Safety, Error-Handling, Code-Quality, Imports) with explanations of why those rules matter for agents, lists all rules per category, and includes guidance on adding new rules.

### 11. RULES.md references the same pinned oxlint version as rules.json

**PASS**

RULES.md line 8:
> "The oxlint defaults are pinned to oxlint `1.67.0`. Keep this version in sync with `rules.json`."

rules.json line 2:
> `"oxlint_version": "1.67.0"`

Both reference `1.67.0`.

## Additional Review

### Consistency: RULES.md vs rules.json

All 14 rules in rules.json are listed in RULES.md under their correct categories:

| Category | rules.json | RULES.md |
|---|---|---|
| type-safety | 5 rules | 5 rules listed |
| error-handling | 3 rules (no-floating-promises, no-misused-promises, no-empty) | 3 rules listed |
| code-quality | 4 rules | 4 rules listed |
| imports | 2 rules | 2 rules listed |

No rules are missing or extra in either direction.

RULES.md also documents the 3 tsconfig compiler settings (`strict`, `noUncheckedIndexedAccess`, `erasableSyntaxOnly`) which matches the SKILL.md Step 7 list.

### Code Standards / Portability

SKILL.md contains two shell code blocks:

1. `bash <skill-dir>/audit.sh <project-dir>` (line 23)
2. `bash <skill-dir>/audit.sh <project-dir> --oxlint-config <selected-config>` (line 58)

Both use `bash` explicitly (not `/bin/bash` or `sh`). No GNU-only flags. No portability issues.

### Quality: Agent Clarity

The instructions are unambiguous and follow a linear step sequence (Steps 1-9). Key design decisions that prevent agent drift:

- "The audit script is the source of truth... Do not reimplement config discovery or rule comparison in the skill workflow." (prevents duplication)
- "Offer exactly these modes" (prevents invention)
- "Do not overwrite explicitly disabled rules in apply-all mode" (prevents destructive behavior)
- "Do not edit the config file. Print the rule JSON for manual addition" (clear boundary for JSONC/TS)
- "Do not create the file when no oxlint rules were accepted" (prevents empty file creation)
- Edge cases handled: missing `scripts.lint`, missing `compilerOptions`, absent `migration_scope`, non-JSON stdout on non-zero exit.

One minor observation: Step 9 says "Re-run the audit command" but does not specify which form of the command when `--oxlint-config` was used in the `requires_selection` path. An agent following the full flow would need to remember the selected config. This is implicit but could be made explicit. Not a contract failure.

## Result

**11/11 checks PASS. No contract violations.**
