# Contract: lint-audit-skill

## Task 1: Create rules.json agent-friendly ruleset

<!-- Specs: ruleset-audit/5, ruleset-audit/6 -->
- [ ] `skills/lint-audit/rules.json` exists and is valid JSON [execution]
- [ ] rules.json contains a top-level `"oxlint_version"` field with a semver string [output]
- [ ] rules.json contains a `"rules"` array with at least 8 rule entries [output]
- [ ] Each rule entry has fields: `name`, `category`, `severity`, `default_enabled`, `rationale` [output]
- [ ] Rule names use oxlint canonical format (e.g., `typescript/no-explicit-any`, not `@typescript-eslint/no-explicit-any`) [output]
- [ ] Rules include at minimum: `typescript/no-explicit-any`, `typescript/no-non-null-assertion`, `typescript/no-floating-promises`, `typescript/no-misused-promises`, `typescript/switch-exhaustiveness-check` [output]

## Task 2: Build audit.sh detection and comparison script

<!-- Specs: project-detection/1, project-detection/2, project-detection/3, project-detection/4, project-detection/5, project-detection/6, project-detection/7, project-detection/8, project-detection/9, project-detection/10, project-detection/11, project-detection/12, project-detection/13, project-detection/14, ruleset-audit/1, ruleset-audit/2, ruleset-audit/3, ruleset-audit/4, ruleset-audit/7, ruleset-audit/8, ruleset-audit/9, ruleset-audit/10, ruleset-audit/11 -->
- [ ] `skills/lint-audit/audit.sh` exists and is executable (`chmod +x`) [execution]
- [ ] Running `audit.sh` on a directory with oxlint in package.json outputs valid JSON with `"outcome": "ok"` [execution]
- [ ] Running `audit.sh` on a directory with only `eslint` in package.json outputs JSON with `"outcome": "skip"` [execution]
- [ ] Running `audit.sh` on a directory with no package.json outputs JSON with `"outcome": "skip"` [execution]
- [ ] Running `audit.sh` on a directory with both `oxlint` and `eslint` outputs JSON with `"outcome": "ok"` [execution]
- [ ] When `.oxlintrc.json` exists, audit.sh reads it and reports rules from it [execution]
- [ ] When no oxlint config exists, audit.sh reports default-enabled rules as `enabled` and others as `missing` [execution]
- [ ] When multiple oxlint config formats exist, audit.sh outputs `"outcome": "requires_selection"` [execution]
- [ ] `--oxlint-config <path>` flag resolves the config ambiguity and produces `"outcome": "ok"` [execution]
- [ ] Output JSON contains `"tsconfig_findings"` with one entry per discovered tsconfig file [output]
- [ ] For each tsconfig missing `strict`, the finding includes `"migration_scope"` with an integer error count [output]
- [ ] For each tsconfig missing `noUncheckedIndexedAccess`, the finding includes `"migration_scope"` with an integer error count [output]
- [ ] For each tsconfig missing `erasableSyntaxOnly`, the finding reports it as `missing` [output]
- [ ] When `.oxlintrc.jsonc` is the only config, audit.sh reads and parses it (stripping comments) [execution]
- [ ] When `oxlint.config.ts` is the only config, audit.sh reports it as detected but notes auto-apply is limited [execution]
- [ ] Running `audit.sh` on a directory with oxlint but no eslint in package.json and no linter config outputs `"outcome": "ok"` with findings [execution]
- [ ] Output JSON contains `"type_aware"` field indicating whether `--type-aware` is in the lint script or type-aware mode is enabled in the oxlint config [output]
- [ ] If jq is not available, audit.sh exits 1 with a JSON error object [execution]
- [ ] audit.sh works on macOS (BSD coreutils) without GNU-only flags [inspection]

## Task 3: Write SKILL.md workflow and RULES.md documentation

<!-- Specs: config-apply/1, config-apply/2, config-apply/3, config-apply/4, config-apply/5, config-apply/6, config-apply/7, config-apply/8 -->
- [ ] `skills/lint-audit/SKILL.md` exists with frontmatter containing `name: lint-audit` and `user-invokable: true` [output]
- [ ] SKILL.md instructs the agent to run `audit.sh` and parse its JSON output [output]
- [ ] SKILL.md includes apply-all, skip-all, and per-item review modes [output]
- [ ] SKILL.md instructs creating `.oxlintrc.json` when no oxlint config exists and user accepts rules [output]
- [ ] SKILL.md instructs updating existing `.oxlintrc.json` by adding rules without removing existing ones [output]
- [ ] SKILL.md instructs printing manual instructions (not auto-editing) for `.oxlintrc.jsonc` and `oxlint.config.ts` formats [output]
- [ ] SKILL.md instructs updating tsconfig.json files to add strictness settings when accepted [output]
- [ ] SKILL.md instructs showing migration scope before each strictness setting (strict, noUncheckedIndexedAccess, erasableSyntaxOnly) in per-item mode [output]
- [ ] SKILL.md instructs modifying the package.json lint script to add `--type-aware` when accepted [output]
- [ ] `skills/lint-audit/RULES.md` exists with human-readable documentation of the agent-friendly ruleset [output]
- [ ] RULES.md references the same pinned oxlint version as rules.json [output]

## Task 4: Smoke test -- 200-iteration edge case loop

<!-- Specs: project-detection/1-14, ruleset-audit/1-11, config-apply/1-8 -->
- [ ] Test harness runs 200 unique edge case scenarios against audit.sh [execution]
- [ ] All 200 scenarios pass (0 failures in final run) [execution]
- [ ] Durable summary written to `mission/validations/smoke-test-200.md` with pass/fail counts and any fixes applied [output]

## Task 5: Cleanup

- [ ] No unused files, dead code, or temporary artifacts in `skills/lint-audit/` [inspection]
- [ ] All shell scripts pass `shellcheck` or are free of obvious portability issues [execution]

## Task 6: Update install.sh, README.md, ARCH.md

- [ ] `install.sh` symlinks `skills/lint-audit` to `~/.claude/skills/lint-audit` [execution]
- [ ] README.md lists `/lint-audit` in the skill list with a one-line description [output]
- [ ] ARCH.md describes the lint-audit skill's role in the mechanical enforcement stack [output]

## Task 7: Module shape re-check

- [ ] `mission/validations/module-shape-audit.md` exists with findings for each module from design.md [output]
- [ ] Each module (audit.sh, SKILL.md, rules.json, RULES.md) is listed with drift status [output]
