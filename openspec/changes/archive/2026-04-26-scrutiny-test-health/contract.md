# Contract

## Task 1.1: Language detection and deterministic pre-passes
<!-- PRD Requirements: #2, #4, #5, #9 -->
- [ ] [inspection] The scrutiny-validator.md prompt contains a "Deterministic Pre-passes" section that runs before the check sequence
- [ ] [inspection] The pre-pass section includes language detection logic that checks for Cargo.toml (Rust), package.json (TypeScript/Node), and pyproject.toml or requirements.txt (Python) at the repo root
- [ ] [inspection] The pre-pass section includes a file size count step that counts lines on every source file the task touched, storing file names and line counts
- [ ] [inspection] The pre-pass section includes an error handling smell grep step with per-language patterns: Rust (.unwrap(), panic!(), todo!(), unimplemented!(), empty .expect("")), TypeScript (catch {}, as any, // @ts-ignore, .catch(() => {})), Python (except: pass, except Exception: pass, type: ignore without code)
- [ ] [inspection] The error handling smell grep explicitly excludes safe patterns: Rust (.unwrap_or(), .unwrap_or_default(), .unwrap_or_else(), .expect("non-empty")), TypeScript (catch with handler body), Python (except SpecificError with handler)
- [ ] [inspection] The pre-pass section includes a test runner output parsing step with format patterns for Rust (test result: ok. N passed; M failed; P ignored), Jest (Tests: N passed, M failed, P skipped, Q total), and pytest (N passed, M failed, P skipped)
- [ ] [inspection] Pre-pass results are stored as named facts that the check sequence references (not inlined into individual checks)

## Task 2.1: Enhance check 3 (tests) with test runner output parsing
<!-- PRD Requirements: #2, #10, #12 -->
- [ ] [inspection] Check 3 in scrutiny-validator.md references the test runner output parsing results from the pre-pass
- [ ] [inspection] Check 3 reports structured pass/fail/skip/ignored counts in the output table row (not just PASS/FAIL)
- [ ] [inspection] Check 3 includes instructions to group failures by common error message
- [ ] [inspection] Check 3 adds a WARNING condition when exit code is 0 but fewer than 90% of tests actually ran

## Task 2.2: Enhance check 6 (module depth) with file size pre-pass
<!-- PRD Requirements: #4, #7, #10 -->
- [ ] [inspection] Check 6 in scrutiny-validator.md references the file size results from the pre-pass
- [ ] [inspection] When any touched source file exceeds 900 lines, the check instructions tell the LLM to inject the file name and line count into its module depth evaluation
- [ ] [inspection] The check instructions tell the LLM to cross-reference design.md for concern boundaries when a file exceeds 900 lines
- [ ] [inspection] Standalone test files (files in tests/ directories, *.test.ts, *.spec.ts, test_*.py) are explicitly excluded from the 900-line check
- [ ] [inspection] Rust #[cfg(test)] blocks inside production files count toward the 900-line threshold (explicitly stated)

## Task 2.3: Enhance check 7 (code standards) with error handling smell density
<!-- PRD Requirements: #5, #7, #10 -->
- [ ] [inspection] Check 7 in scrutiny-validator.md references the error handling smell results from the pre-pass
- [ ] [inspection] The check instructions tell the LLM to evaluate the smell count per KLOC with graduated thresholds: under 5/KLOC PASS, 5-10 WARNING, over 10 FAIL
- [ ] [inspection] The check instructions tell the LLM to review each flagged location and provide file:line references with specific remediation guidance
- [ ] [inspection] Test files are explicitly excluded from the smell count

## Task 3.1: Silent skip detection (check 12)
<!-- PRD Requirements: #1, #7, #10 -->
- [ ] [inspection] scrutiny-validator.md contains a check 12 titled "Silent skip detection"
- [ ] [inspection] Check 12 includes grep patterns for Rust silent skips: else { return } and conditional return in #[test] / #[tokio::test] functions
- [ ] [inspection] Check 12 includes grep patterns for TypeScript silent skips: if (!process.env.X) return in it() / test() blocks
- [ ] [inspection] Check 12 includes grep patterns for Python silent skips: conditional return in test_ functions based on env vars
- [ ] [inspection] Check 12 explicitly states that #[ignore], it.skip(), describe.skip(), and @pytest.mark.skip are NOT flagged
- [ ] [inspection] Check 12 states that any silent skip is a FAIL
- [ ] [inspection] Check 12 self-gates: reports SKIP when no test files exist in the diff
- [ ] [inspection] Check 12 output includes file path, line number, test function name, and remediation guidance for each finding

## Task 3.2: Diff coverage (check 13)
<!-- PRD Requirements: #3, #7, #9, #10, #12 -->
- [ ] [inspection] scrutiny-validator.md contains a check 13 titled "Diff coverage"
- [ ] [inspection] Check 13 specifies the coverage tool per language: cargo tarpaulin for Rust, jest --coverage or c8 for TypeScript, pytest --cov for Python
- [ ] [inspection] Check 13 instructs the validator to filter the coverage report to added lines from git diff --name-only
- [ ] [inspection] Check 13 applies thresholds: 80%+ PASS, 70-79% WARNING, below 70% FAIL
- [ ] [inspection] Check 13 handles missing coverage tool: skip with WARNING, then attempt auto-install as fallback
- [ ] [inspection] Check 13 self-gates: reports SKIP when no production source files exist in the diff
- [ ] [inspection] Check 13 states that for multi-language projects, coverage runs for the primary language only
- [ ] [inspection] Check 13 output lists under-covered files with line ranges

## Task 4.1: Test quality review skill
<!-- PRD Requirements: #6 -->
- [ ] [output] A file test-quality-review/SKILL.md exists in the layercodedev/skills repo
- [ ] [inspection] The SKILL.md has frontmatter with name and description fields matching the layercode skills format
- [ ] [inspection] The skill evaluates 5 dimensions: assertion quality, implementation coupling, branch coverage by inspection, behavioral changes without tests, edge case awareness
- [ ] [inspection] Assertion quality dimension includes tautological patterns for Rust (assert!(true), assert_eq!(x, x)), TypeScript (expect(x).toBeDefined()), and Python (assert True, assert result is not None without value check)
- [ ] [inspection] Implementation coupling dimension flags mock-count assertions on internal (non-boundary) functions
- [ ] [inspection] Branch coverage dimension instructs the LLM to compare production diff with test diff and flag untested branches
- [ ] [inspection] Behavioral changes dimension flags new public functions or changed return types with no corresponding test
- [ ] [inspection] Edge case dimension flags public input-accepting functions with only happy-path tests
- [ ] [inspection] Output format is structured markdown with severity (P0-P3), file:line, evidence, and remediation per finding

## Task 5.1: Wire test quality review into scrutiny (check 14)
<!-- PRD Requirements: #6, #7, #10 -->
- [ ] [inspection] scrutiny-validator.md contains a check 14 titled "Test quality review"
- [ ] [inspection] Check 14 invokes /test-quality-review as a subagent (same delegation pattern as /review and /simplify)
- [ ] [inspection] Check 14 applies severity-based fail behavior: any P0 is FAIL, 3+ P1s is FAIL, P2/P3 are WARNING only
- [ ] [inspection] Check 14 self-gates: reports SKIP when no test files exist in the diff

## Task 6.1: darkfactory-init coverage tool checks
<!-- PRD Requirements: #8, #11 -->
- [ ] [inspection] darkfactory-init SKILL.md Step 4 includes language detection from build files (Cargo.toml, package.json, pyproject.toml/requirements.txt)
- [ ] [inspection] For Rust projects, init checks cargo tarpaulin --version and suggests cargo install cargo-tarpaulin if missing
- [ ] [inspection] For TypeScript projects, init checks for jest/c8 coverage support in package.json deps/devDeps and suggests npm install -D c8 if missing
- [ ] [inspection] For Python projects, init checks pip show pytest-cov and suggests pip install pytest-cov if missing
- [ ] [inspection] Init skips coverage check with a note when no recognized language is detected
- [ ] [inspection] /test-quality-review is added to the supporting skills check in Step 2
- [ ] [inspection] Step 5 status report includes a "Coverage tools" section

## Task 7.1: Smoke test
<!-- PRD Requirements: #1, #2, #3, #4, #5, #6 -->
- [ ] [execution] Run the scrutiny validator prompt logic against a synthetic Rust diff containing a silent skip pattern and verify check 12 reports FAIL
- [ ] [execution] Run the scrutiny validator prompt logic against a synthetic TypeScript diff containing a tautological assertion and verify check 14 reports FAIL (via /test-quality-review)

## Task 7.2: Cleanup
<!-- No specific PRD requirements -->
- [ ] [inspection] No temporary artifacts, debug output, or backwards-compatibility shims remain in any modified files

## Task 7.3: Module shape re-check
<!-- No specific PRD requirements -->
- [ ] [output] mission/validations/module-shape-audit.md exists and contains findings for each module (scrutiny-validator.md, test-quality-review skill, darkfactory-init)

## Task 7.4: Documentation
<!-- No specific PRD requirements -->
- [ ] [output] README.md mentions the 3 enhanced checks (test runner parsing, file size pre-pass, error handling smells) and 3 new checks (silent skip detection, diff coverage, test quality review)
- [ ] [output] README.md mentions the /test-quality-review skill
- [ ] [output] README.md mentions darkfactory-init coverage tool checks
