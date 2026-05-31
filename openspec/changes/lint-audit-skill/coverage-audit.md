# Coverage Audit: lint-audit-skill

## PRD Requirements → Task Mapping

| # | PRD Requirement | Spec Capability | Task(s) | Status | Deviation | Prerequisites met? |
|---|----------------|-----------------|---------|--------|-----------|---------------------|
| S1 | Detect oxlint from package.json, skip ESLint-only | project-detection/1-4 | Task 2 (audit.sh) | COVERED | -- | -- |
| S2 | Read oxlint config in any format (.json, .jsonc, .ts) | project-detection/5-8 | Task 2 (audit.sh) | COVERED | -- | -- |
| S3 | Compare against agent-friendly ruleset in RULES.md | ruleset-audit/1-4, ruleset-audit/5-6 | Task 1 (rules.json) + Task 2 (audit.sh) | COVERED | Interpretation: PRD says "RULES.md", design split into rules.json (machine data) + RULES.md (human docs). audit.sh reads rules.json. Same data, better separation. | -- |
| S4 | Produce combined report with status markers | ruleset-audit/9-11 | Task 2 (audit.sh) + Task 3 (SKILL.md) | COVERED | -- | -- |
| S5 | Run tsc per tsconfig for migration scope | ruleset-audit/6-8 | Task 2 (audit.sh) | COVERED | -- | -- |
| S6 | Per-item accept/deny + apply-all/skip-all | config-apply/1-3 | Task 3 (SKILL.md) | COVERED | -- | -- |
| U1 | New project: apply all in one pass | config-apply/1 | Task 3 (SKILL.md) | COVERED | -- | -- |
| U2 | Existing project: see what's missing | ruleset-audit/1-11 | Task 2 + Task 3 | COVERED | -- | -- |
| U3 | See migration scope before enabling strict | ruleset-audit/6, config-apply/7 | Task 2 + Task 3 | COVERED | -- | -- |
| U4 | Accept/deny individually | config-apply/3 | Task 3 (SKILL.md) | COVERED | -- | -- |
| U5 | Apply all at once | config-apply/1 | Task 3 (SKILL.md) | COVERED | -- | -- |
| U6 | Skip ESLint-only projects | project-detection/2 | Task 2 (audit.sh) | COVERED | -- | -- |
| U7 | Detect all tsconfigs in monorepo | project-detection/9-12 | Task 2 (audit.sh) | COVERED | -- | -- |
| U8 | Create .oxlintrc.json if none exists | config-apply/4 | Task 3 (SKILL.md) | COVERED | -- | -- |
| U9 | Show oxlint canonical rule names | ruleset-audit/1-4 | Task 1 (rules.json) | COVERED | -- | -- |
| U10 | Show which rules are already enabled | ruleset-audit/1-4 | Task 2 (audit.sh) | COVERED | -- | -- |
| U11 | Handle monorepo structures | project-detection/9-12, ruleset-audit/5-8 | Task 2 (audit.sh) | COVERED | -- | -- |
| U12 | Gate 4 auto-enforces added rules | N/A (no code change needed) | N/A | COVERED | Interpretation: Gate 4 already runs the project's linter; no task needed | -- |

## Summary

- **18 PRD requirements mapped**
- **18 COVERED, 0 gaps, 0 deferred, 0 skipped**
- **1 Interpretation:** U12 (Gate 4 enforcement) requires no implementation -- Gate 4 already runs the project's linter. Rules added to oxlint config are automatically enforced.
- **0 Substitutions, 0 Removals**
