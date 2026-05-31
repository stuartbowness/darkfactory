I can’t write the file from this read-only session, so here is the markdown content.

```markdown
# Artifact Review: contract.md

## Summary

6 findings across coverage gaps, contradictions, ambiguity, and feasibility.

## Findings

### [HIGH] Type-aware detection is narrower than the spec
- **Dimension:** 3. Contradictions
- **Quote:** "Output JSON contains `"type_aware"` field indicating whether `--type-aware` is in the lint script"
- **Problem:** The spec says type-aware is enabled when the lint script contains `--type-aware` **or** the oxlint config enables type-aware mode. This contract only checks the script path, so an implementation could falsely report config-enabled projects as missing type-aware linting.
- **Suggestion:** Require `type_aware` to reflect both lint script flags and oxlint config settings.

### [HIGH] `erasableSyntaxOnly` is missing from the contract
- **Dimension:** 5. Coverage Gaps
- **Quote:** "For each tsconfig missing `strict`..." and "For each tsconfig missing `noUncheckedIndexedAccess`..."
- **Problem:** Prior specs require auditing and applying `erasableSyntaxOnly`, but the contract never checks that `audit.sh` reports it or that `SKILL.md` applies it. A worker could omit the setting entirely and still pass this contract.
- **Suggestion:** Add checks for detecting, reporting, and applying `compilerOptions.erasableSyntaxOnly`.

### [HIGH] JSONC and TypeScript oxlint config behavior is under-specified
- **Dimension:** 5. Coverage Gaps
- **Quote:** "When `.oxlintrc.json` exists, audit.sh reads it and reports rules from it"
- **Problem:** The specs require `.oxlintrc.jsonc` parsing and `oxlint.config.ts` detection, but the contract only tests `.oxlintrc.json` plus multiple-format ambiguity. Single JSONC or TS config projects could be broken without failing validation.
- **Suggestion:** Add checks for single `.oxlintrc.jsonc` parsing and single `oxlint.config.ts` detection/manual-apply behavior.

### [MEDIUM] No-linter package.json scenario is missing
- **Dimension:** 5. Coverage Gaps
- **Quote:** "Running `audit.sh` on a directory with no package.json outputs JSON with `"outcome": "skip"`"
- **Problem:** The project-detection spec separately requires handling a `package.json` with neither `oxlint` nor exact `eslint`, including showing `npm install -D oxlint` without running it. That path is not covered.
- **Suggestion:** Add a check for a Node project with no detected linter and the expected install-command recommendation.

### [MEDIUM] Human report format from ruleset-audit is not validated
- **Dimension:** 5. Coverage Gaps
- **Quote:** "SKILL.md instructs the agent to run `audit.sh` and parse its JSON output"
- **Problem:** The ruleset-audit spec requires a user-facing report grouped under "Oxlint Rules" and "TypeScript Compiler" with `[x]`, `[ ]`, and `[!]` markers plus rationales. The contract validates JSON production, but not the SKILL.md reporting instructions.
- **Suggestion:** Add SKILL.md checks for grouped report output, status markers, migration scope display, and rule rationales.

### [MEDIUM] The 200-scenario smoke test is hard to validate consistently
- **Dimension:** 1. Ambiguity
- **Quote:** "Test harness runs 200 unique edge case scenarios against audit.sh"
- **Problem:** "Unique edge case scenarios" is undefined. A worker and validator may disagree about whether generated variants count as unique, which can cause review churn.
- **Suggestion:** Define uniqueness as a named scenario matrix or require the summary to list 200 scenario IDs with the input dimensions varied.
```
