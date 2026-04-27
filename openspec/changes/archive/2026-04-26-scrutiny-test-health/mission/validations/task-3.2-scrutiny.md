# Scrutiny Validation: Task 3.2

## Result: PASS

## Check Results
| Check | Result | Notes |
|-------|--------|-------|
| Test presence | PASS | `test-task-3.2.sh` added (66 lines) |
| Contract assertions | PASS | All 8 assertions verified by inspection |

## Contract Assertions
| # | Assertion | Result | Method | Evidence |
|---|-----------|--------|--------|----------|
| 1 | scrutiny-validator.md contains a check 13 titled "Diff coverage" | PASS | inspection | Line 221: `13. **Diff coverage:**` exists in the check sequence |
| 2 | Check 13 specifies the coverage tool per language: cargo tarpaulin for Rust, jest --coverage or c8 for TypeScript, pytest --cov for Python | PASS | inspection | Lines 231-233: Rust `cargo tarpaulin --out json` (with `cargo llvm-cov --json` fallback), TS `npx jest --coverage --json` or `npx c8 --reporter json`, Python `pytest --cov --cov-report json` |
| 3 | Check 13 instructs the validator to filter the coverage report to added lines from git diff --name-only | PASS | inspection | Line 237: "Run `git diff <commit-sha>^ <commit-sha> --name-only` to get changed files. Filter the coverage JSON to those changed files, then count only added lines" |
| 4 | Check 13 applies thresholds: 80%+ PASS, 70-79% WARNING, below 70% FAIL | PASS | inspection | Lines 239-242: "80% or higher: PASS", "70-79%: WARN", "Below 70%: FAIL and trigger the fix loop" |
| 5 | Check 13 handles missing coverage tool: skip with WARNING, then attempt auto-install as fallback | PASS | inspection | Line 235: "If the coverage tool is not installed, first report `diff-coverage \| SKIP \| <tool> not installed` as a WARNING, then attempt auto-install as a fallback" with language-specific install instructions and retry/keep-SKIP logic |
| 6 | Check 13 self-gates: reports SKIP when no production source files exist in the diff | PASS | inspection | Line 228: "If the diff contains no production source files, report `diff-coverage \| SKIP \| no production source files in diff`" |
| 7 | Check 13 states that for multi-language projects, coverage runs for the primary language only | PASS | inspection | Line 221: "Run coverage for the primary language only. In multi-language projects, use the language with the most production source files in the diff; do not run two coverage tools" |
| 8 | Check 13 output lists under-covered files with line ranges | PASS | inspection | Line 244: "Include details listing under-covered files with added-line ranges that lack coverage" |
