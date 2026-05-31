# Artifact Review: spec: config-apply

## Summary

5 findings across 4 dimensions.

## Findings

### [HIGH] Apply-all does not define handling for type-aware linting
- **Dimension:** 1. Ambiguity
- **Quote:** "the skill applies every missing recommendation (oxlint rules + tsconfig settings) without further prompts"
- **Problem:** The spec later adds a type-aware linting recommendation that changes the lint script, but apply-all only names oxlint rules and tsconfig settings. One engineer may apply `--type-aware`; another may only print a recommendation.
- **Suggestion:** State whether type-aware linting is report-only or included in apply-all/per-item apply.

### [HIGH] Active oxlint config selection is underspecified
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "WHEN `oxlint.config.ts` is the active config and the user accepts rules"
- **Problem:** The proposal requires detecting `.oxlintrc.json`, `.oxlintrc.jsonc`, and `oxlint.config.ts`, but this spec does not define precedence when more than one exists. Implementation needs one rule for deciding the active config before it can safely write.
- **Suggestion:** Add config precedence rules, including what to do when multiple oxlint configs exist.

### [MEDIUM] JSONC preservation requirement needs a concrete parser/format policy
- **Dimension:** 6. Feasibility
- **Quote:** "adds the accepted rules to the existing JSONC config, preserving comments"
- **Problem:** Preserving comments while editing JSONC is not supported by normal JSON parsing. Without a defined parser or formatting strategy, this likely becomes fragile string editing and review churn.
- **Suggestion:** Require a JSONC-aware edit path and define whether existing formatting/order must be preserved beyond comments.

### [MEDIUM] tsconfig inheritance is ignored
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "adds `\"strict\": true` to `compilerOptions` in that tsconfig"
- **Problem:** Many `tsconfig.json` files use `extends`. A setting may already be enabled in the inherited config, or a child config may intentionally override it. The spec says to update files when accepting recommendations, but does not say whether detection and apply operate on effective config or literal file contents.
- **Suggestion:** Define whether recommendations are based on effective TypeScript config resolution or only direct `compilerOptions` in each file.

### [MEDIUM] Migration scope command only covers strict
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "shows the error count from `tsc -p <tsconfig> --strict --noEmit --pretty false`"
- **Problem:** `noUncheckedIndexedAccess` can produce substantial new errors too, and `erasableSyntaxOnly` can also fail existing code. The spec only requires migration scope for `strict`, so users may apply the other two blind.
- **Suggestion:** Require a migration scope command for every strictness setting before accept/deny, or explicitly justify why only `strict` gets this treatment.