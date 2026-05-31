# Artifact Review: spec: ruleset-audit

## Summary

5 findings across ambiguity, terminology consistency, missing edge cases, coverage gaps, and feasibility.

## Findings

### [HIGH] Tsconfig inheritance will be misreported
- **Dimension:** 4. Missing Edge Cases
- **Quote:** "**WHEN** `compilerOptions.strict` is absent or `false` in the tsconfig"
- **Problem:** A tsconfig can inherit `strict`, `noUncheckedIndexedAccess`, or `erasableSyntaxOnly` through `extends`. Treating absent as missing will create false findings and wrong migration counts.
- **Suggestion:** Require the audit to inspect the effective config, for example via `tsc --showConfig -p <tsconfig>`.

### [HIGH] RULES.md rationale format is undefined
- **Dimension:** 5. Coverage Gaps
- **Quote:** "**WHEN** a rule is reported as missing\n- **THEN** the report includes a one-line rationale from RULES.md explaining why the rule matters for agents"
- **Problem:** The prior proposal only says RULES.md defines canonical rule names. It does not define where rationale lives or how audit.sh should extract it.
- **Suggestion:** Define a simple RULES.md schema with rule name, desired severity, default coverage, and one-line rationale.

### [MEDIUM] Oxlint default detection has no source of truth
- **Dimension:** 6. Feasibility
- **Quote:** "**THEN** the skill checks whether the rule is enabled by default in oxlint"
- **Problem:** Oxlint defaults can vary by installed version and config mode. Without a required lookup method, one implementation may hard-code stale defaults while another shells out to local tooling.
- **Suggestion:** Specify the exact default source: local oxlint metadata when available, or a pinned default map in RULES.md.

### [MEDIUM] Partial disable status is undefined
- **Dimension:** 1. Ambiguity
- **Quote:** "the report uses status markers (`[x]` enabled, `[ ]` missing, `[!]` partially/explicitly disabled)"
- **Problem:** The scenarios define explicit disable only. They do not say what counts as partially disabled, such as file overrides, category disables, or mixed severities.
- **Suggestion:** Add scenarios for overrides and mixed config, or remove “partially” from the status definition.

### [LOW] Migration scope command conflicts with glossary definition
- **Dimension:** 2. Terminology Consistency
- **Quote:** "runs `tsc -p <tsconfig> --strict --noEmit --pretty false 2>&1 | grep -c 'error TS'` to show migration scope"
- **Problem:** The glossary defines Migration scope as measured with `wc -l`, while this spec counts lines containing `error TS`. Reviewers may expect different numbers.
- **Suggestion:** Update the glossary or the spec so Migration scope has one canonical measurement.