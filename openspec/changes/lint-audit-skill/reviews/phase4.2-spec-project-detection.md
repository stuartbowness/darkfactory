# Artifact Review: spec: project-detection

## Summary

5 findings across ambiguity, contradictions, missing edge cases, coverage gaps, and feasibility.

## Findings

### [HIGH] Monorepo oxlint configs are ignored
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "`.oxlintrc.json` exists at the project root"
- **Problem:** The spec handles subproject `tsconfig` files, but oxlint supports nested config files in package directories. The audit could miss package-specific rules in monorepos, so validation may pass while real lint behavior differs. Source: https://oxc.rs/docs/guide/usage/linter/nested-config
- **Suggestion:** Add scenarios for nested oxlint configs and define whether the skill audits all configs or only the root config.

### [HIGH] "All tsconfig files" conflicts with depth limit
- **Dimension:** 3. Contradictions
- **Quote:** "The skill SHALL find all TypeScript configuration files across the project." / "up to depth 3"
- **Problem:** These requirements disagree. One engineer may implement an unlimited search, another may stop at depth 3. A depth cap will miss valid monorepo layouts.
- **Suggestion:** Pick one rule: either remove the depth cap or state the cap as intentional and report skipped deeper files.

### [MEDIUM] Linter detection is underspecified for monorepos
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "by reading `devDependencies` and `dependencies` in `package.json`"
- **Problem:** The spec only mentions the root `package.json`, but the same artifact supports monorepo tsconfigs. A workspace may declare `oxlint` inside a package-level `package.json`, causing a false "No linter detected".
- **Suggestion:** Define whether dependency detection reads only the root package or all workspace package manifests.

### [MEDIUM] ESLint detection can be interpreted too broadly
- **Dimension:** 1. Ambiguity
- **Quote:** "`package.json` contains `eslint`"
- **Problem:** "Contains" could mean an exact dependency named `eslint`, or any dependency containing the string, such as `@eslint/js`, `eslint-plugin-react`, or `typescript-eslint`. That changes skip behavior.
- **Suggestion:** Specify exact package-name matching for `oxlint` and `eslint`.

### [MEDIUM] Installing oxlint is new scope without package-manager rules
- **Dimension:** 5. Coverage Gaps
- **Quote:** "offers to install oxlint"
- **Problem:** Prior artifacts describe an audit skill that reports gaps and applies config fixes. Installing a new dependency adds package-manager detection, lockfile changes, and user approval behavior that is not specified elsewhere.
- **Suggestion:** Either remove the install offer or add requirements for package manager detection, install command selection, and confirmation before changing dependencies.