## 1. Scrutiny Validator Prompt: Deterministic Pre-passes

- [x] 1.1 Add language detection and deterministic pre-pass section to scrutiny-validator.md. Detect project language from build files (Cargo.toml, package.json, pyproject.toml). Run three deterministic pre-passes before the check sequence: (a) file size count on touched source files, (b) error handling smell grep on changed production lines with per-language patterns and safe-pattern exclusions, (c) test runner summary line parsing. Store results as named facts the check sequence references. Include tests that verify the pre-pass instructions produce correct grep patterns and line counts for each language.

## 2. Scrutiny Validator Prompt: Enhance Existing Checks

- [ ] 2.1 Enhance check 3 (tests) to incorporate test runner output parsing from the pre-pass. Update the check to report structured pass/fail/skip/ignored counts, group failures by common error message, and add a WARNING condition when exit code is 0 but fewer than 90% of tests actually ran. Update the output table row format. Include tests that verify the enhanced output format for Rust, TypeScript, and Python test runner summary lines.

- [ ] 2.2 Enhance check 6 (module depth) to incorporate the file size pre-pass. When any touched source file exceeds 900 lines, inject the file name, line count, and design.md concern boundary cross-reference into the module depth review instructions. Exclude standalone test files from the check. Include tests that verify the file size injection for files above and below the threshold.

- [ ] 2.3 Enhance check 7 (code standards) to incorporate the error handling smell density from the pre-pass. Inject the smell count per KLOC and specific locations into the code standards review instructions. Apply graduated thresholds: under 5/KLOC PASS, 5-10 WARNING, over 10 FAIL. Exclude test files. Include tests that verify the threshold logic and pattern matching for Rust (.unwrap() vs .unwrap_or_default()), TypeScript (catch {} vs catch with handler), and Python (except: pass vs except SpecificError).

## 3. Scrutiny Validator Prompt: New Checks

- [ ] 3.1 Add check 12 (silent skip detection) to scrutiny-validator.md. Grep test files in the diff for language-specific silent-skip patterns (Rust: else { return } in #[test] fns; TypeScript: if (!process.env.X) return in test() blocks; Python: conditional return in test_ fns). Do NOT flag explicit skip markers (#[ignore], it.skip(), @pytest.mark.skip). Any silent skip is a FAIL. Self-gate when no test files in diff. Include tests with example diffs for each language showing both flagged and unflagged patterns.

- [ ] 3.2 Add check 13 (diff coverage) to scrutiny-validator.md. Run the language-appropriate coverage tool (cargo tarpaulin for Rust, jest --coverage or c8 for TypeScript, pytest --cov for Python). Filter coverage report to added lines from git diff --name-only. Apply thresholds: 80%+ PASS, 70-79% WARNING, below 70% FAIL. Skip with WARNING if coverage tool not installed (then attempt auto-install as fallback). Self-gate when no production source files in diff. For multi-language projects, run coverage for the primary language only. Include tests that verify threshold logic and self-gating behavior.

## 4. Test Quality Review Skill

- [ ] 4.1 Create the test-quality-review skill in the layercodedev/skills repo. Write SKILL.md with the 5 evaluation dimensions: (a) assertion quality -- flag tautological assertions per language, (b) implementation coupling -- flag mock-count assertions on internal functions, (c) branch coverage by inspection -- flag new if/else/match/try-catch with only one branch tested, (d) behavioral changes without tests -- flag new public functions or changed return types with no corresponding test, (e) edge case awareness -- flag public input-accepting functions with only happy-path tests. Output structured markdown findings with severity P0-P3, file:line, evidence, and remediation. Follow the SKILL.md format used by existing layercode skills (frontmatter with name and description, then numbered procedural steps).

## 5. Scrutiny Validator: Wire Test Quality Review

- [ ] 5.1 Add check 14 (test quality review) to scrutiny-validator.md. Invoke the /test-quality-review skill as a subagent (same pattern as /review and /simplify). Read the skill's markdown findings. Apply fail behavior: any P0 finding is FAIL, 3+ P1 findings is FAIL, P2/P3 findings are WARNING only. Self-gate when no test files in diff. Include tests that verify the severity-to-result mapping.

## 6. darkfactory-init Update

- [ ] 6.1 Update darkfactory-init SKILL.md to add coverage tool checks. In Step 4 (CLI dependencies), detect project language from build files and check for the corresponding coverage tool: cargo-tarpaulin for Rust, jest/c8 coverage for TypeScript, pytest-cov for Python. Suggest install commands if missing. Do not auto-install. Add /test-quality-review to the supporting skills check in Step 2. Update Step 5 status report with a "Coverage tools" section. Skip coverage check with a note if no recognized language detected. Include tests that verify the language detection and tool check logic.

## 7. Closing Tasks

- [ ] 7.1 Smoke test: Run the updated scrutiny validator prompt against a synthetic diff that exercises all 6 new capabilities (silent skip, test runner parsing, coverage, file size, error handling smells, test quality review). Verify each check produces the expected output format and result. Test with at least Rust and TypeScript examples.

- [ ] 7.2 Cleanup: Remove any temporary artifacts, debug output, or scaffolding introduced during development. Verify no backwards-compatibility shims remain.

- [ ] 7.3 Module shape re-check: Re-read design.md module descriptions. Compare what the scrutiny-validator.md prompt actually contains against the design's stated structure (deterministic pre-pass section, new checks section, concern separation). Verify the test-quality-review skill structure matches the design. Write findings to mission/validations/module-shape-audit.md.

- [ ] 7.4 Documentation: Update README.md to reflect the new scrutiny checks (3 enhancements, 3 new checks), the new /test-quality-review skill, and the darkfactory-init coverage tool check. Use terms from UBIQUITOUS_LANGUAGE.md consistently. Execute any commands shown in docs and paste actual output.
