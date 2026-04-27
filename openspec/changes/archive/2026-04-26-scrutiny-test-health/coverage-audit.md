# Spec Coverage Audit

## Coverage Table

| # | PRD Requirement | Spec | Task(s) | Status | Prerequisites met? |
|---|----------------|------|---------|--------|-------------------|
| 1 | Silent test skips detected during scrutiny | silent-skip-detection | 1.1, 3.1 | COVERED | -- |
| 2 | Test runner output parsed into structured counts | scrutiny-validation (check 3 enhancement) | 1.1, 2.1 | COVERED | -- |
| 3 | Diff coverage measured on each task's added lines | diff-coverage | 1.1, 3.2 | COVERED | -- |
| 4 | Files over 900 lines flagged during module depth review | scrutiny-validation (check 6 enhancement) | 1.1, 2.2 | COVERED | -- |
| 5 | Error handling smell density counted deterministically | scrutiny-validation (check 7 enhancement) | 1.1, 2.3 | COVERED | -- |
| 6 | LLM-based test quality review | test-quality-review | 4.1, 5.1 | COVERED | -- |
| 7 | All new checks self-gate based on diff contents | silent-skip-detection, diff-coverage, test-quality-review, scrutiny-validation | 3.1, 3.2, 5.1, 2.1, 2.2, 2.3 | COVERED | -- |
| 8 | Coverage tools verified during darkfactory-init | darkfactory-init | 6.1 | COVERED | -- |
| 9 | Language detection from build files | diff-coverage, scrutiny-validation | 1.1 | COVERED | -- |
| 10 | Fix worker gets file paths, line numbers, remediation guidance | scrutiny-validation, silent-skip-detection, diff-coverage, test-quality-review | 2.1, 2.2, 2.3, 3.1, 3.2, 5.1 | COVERED | -- |
| 11 | Clear guidance on which coverage tool to install per language | darkfactory-init | 6.1 | COVERED | -- |
| 12 | Coverage percentages visible alongside pass/fail results | diff-coverage, scrutiny-validation (check 3 enhancement) | 2.1, 3.2 | COVERED | -- |

## Implementation Decisions (from PRD)

| # | Decision | Task(s) | Status |
|---|----------|---------|--------|
| D1 | All checks in scrutiny prompt, not external scripts | 1.1, 2.1-2.3, 3.1-3.2, 5.1 | COVERED |
| D2 | Language detection from build files (Cargo.toml, package.json, pyproject.toml) | 1.1 | COVERED |
| D3 | Coverage for primary language only in multi-language projects | 3.2 | COVERED |
| D4 | Coverage thresholds: 80%+ PASS, 70-79% WARNING, <70% FAIL | 3.2 | COVERED |
| D5 | Error handling patterns: flag .unwrap() but not .unwrap_or_default(); flag catch {} but not catch with handler; flag except: pass but not except SpecificError | 2.3 | COVERED |
| D6 | Test files excluded from error handling smell count and file size check | 2.2, 2.3 | COVERED |
| D7 | Missing coverage tool: skip with WARNING, fallback auto-install | 3.2 | COVERED |
| D8 | New /test-quality-review skill in layercodedev/skills repo | 4.1 | COVERED |
| D9 | Test quality review outputs simple markdown, not JSON | 4.1 | COVERED |
| D10 | Severity-based fail: P0=FAIL, 3xP1=FAIL, P2/P3=WARN | 5.1 | COVERED |

## Summary

- **12 PRD requirements:** 12 COVERED, 0 gaps, 0 deviations
- **10 implementation decisions:** 10 COVERED
- **All requirements trace to at least one task**
- **No deviations from spec**
