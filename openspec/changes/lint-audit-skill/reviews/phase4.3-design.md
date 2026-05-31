# Artifact Review: design.md

## Summary

5 findings across ambiguity, contradictions, coverage gaps, and feasibility.

## Findings

### [HIGH] Multiple oxlint configs require user choice but `audit.sh` has no way to receive it
- **Dimension:** 3. Contradictions
- **Quote:** "`audit.sh` | Narrow: one entry point (`audit.sh <project-dir>`), outputs JSON findings to stdout"
- **Problem:** The prior spec says multiple oxlint config formats must ask the user which one to use. This design puts config discovery inside a non-interactive script with only one argument, so the implementer has no defined path for continuing after the user chooses.
- **Suggestion:** Define a script contract for ambiguity, such as emitting `requires_selection`, then re-running with `audit.sh <project-dir> --oxlint-config <path>`.

### [HIGH] Type-aware lint apply is required but package.json mutation is not designed
- **Dimension:** 5. Coverage Gaps
- **Quote:** "`SKILL.md` (agent) | `package.json` | Reads lint script to check for `--type-aware` | Inbound"
- **Problem:** The specs require apply-all and per-item apply to include type-aware linting enablement, which means changing the lint script. The design only says `package.json` is read, not updated. A worker could implement reporting correctly but fail the apply requirement.
- **Suggestion:** Add `package.json` as an outbound apply target and specify how the lint script is selected and modified.

### [MEDIUM] `jq` is both optional and required
- **Dimension:** 3. Contradictions
- **Quote:** "If `oxlint`, `tsc`, or `jq` are not installed, audit.sh reports them as findings (not as crashes)."
- **Problem:** The test strategy also says "`jq` (required, available everywhere)", and the design relies on `jq` for JSON parsing. If `jq` is missing, the script cannot reliably parse configs or produce normal findings. Treating it as a recoverable finding will make error handling messy and brittle.
- **Suggestion:** Make `jq` a hard prerequisite with a JSON error object, or remove the dependency and define shell-only parsing/output rules.

### [MEDIUM] RULES.md is machine-parsed but its grammar is undefined
- **Dimension:** 1. Ambiguity
- **Quote:** "`audit.sh` | `RULES.md` | Reads file to get recommended ruleset (parsed as structured data)"
- **Problem:** Markdown is not a stable data format unless the exact parseable structure is defined. Two engineers could write different parsers for bullets, headings, tables, or fenced blocks. This affects tests and rule comparison.
- **Suggestion:** Put a fenced JSON or YAML block in `RULES.md`, or add a separate `rules.json` and keep `RULES.md` as human documentation.

### [MEDIUM] AuditFinding status is too narrow for required outcomes
- **Dimension:** 1. Ambiguity
- **Quote:** "`AuditFinding` | A single finding from the audit: rule name, status (enabled/missing/disabled), category, rationale"
- **Problem:** The design also needs to represent parse errors, missing tools, ESLint-only skip, no package.json, multiple config ambiguity, and partial/unknown TypeScript config handling. `enabled/missing/disabled` does not cover those states, so workers and validators may disagree on JSON shape.
- **Suggestion:** Extend the JSON contract with explicit top-level outcomes and finding statuses such as `error`, `warning`, `skipped`, `unknown`, and `requires_selection`.