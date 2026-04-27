# Task 6.1 Scrutiny Validation

**Commit:** 4bd9b7aaa80df29434b8997a4f2df4c5a200707c
**Date:** 2026-04-25
**Result:** PASS

## Assertions

- [x] **[inspection] darkfactory-init SKILL.md Step 4 includes language detection from build files (Cargo.toml, package.json, pyproject.toml/requirements.txt)**
  PASS. Lines 120-124 of `.claude/skills/darkfactory-init/SKILL.md` contain "Detect the project language from build files at the project root:" followed by `Cargo.toml` -> Rust, `package.json` -> TypeScript/Node, `pyproject.toml` or `requirements.txt` -> Python.

- [x] **[inspection] For Rust projects, init checks cargo tarpaulin --version and suggests cargo install cargo-tarpaulin if missing**
  PASS. Line 128: `Rust: Run cargo tarpaulin --version 2>/dev/null || echo "NOT_INSTALLED". If missing, suggest cargo install cargo-tarpaulin.`

- [x] **[inspection] For TypeScript projects, init checks for jest/c8 coverage support in package.json deps/devDeps and suggests npm install -D c8 if missing**
  PASS. Line 129: Checks `package.json` for `c8`, `@jest/coverage`, `istanbul`, or `jest` in `dependencies` or `devDependencies`. Notes that `jest` has built-in coverage. Suggests `npm install -D c8` if no coverage capability is found.

- [x] **[inspection] For Python projects, init checks pip show pytest-cov and suggests pip install pytest-cov if missing**
  PASS. Line 130: `Python: Run pip show pytest-cov 2>/dev/null || echo "NOT_INSTALLED". If missing, suggest pip install pytest-cov.`

- [x] **[inspection] Init skips coverage check with a note when no recognized language is detected**
  PASS. Line 132: Reports "No recognized language detected -- coverage tool check skipped." when no build files are found at the project root.

- [x] **[inspection] /test-quality-review is added to the supporting skills check in Step 2**
  PASS. Line 48 adds `~/.claude/skills/test-quality-review/SKILL.md` to the supporting skills list. Line 61 adds the corresponding symlink install command.

- [x] **[inspection] Step 5 status report includes a "Coverage tools" section**
  PASS. Lines 169-172 contain a "Coverage tools:" section with example entries for cargo-tarpaulin (Rust detected), pytest-cov (Python detected), and the "No recognized language detected -- skipped" fallback.

## Test Script

`openspec/changes/scrutiny-test-health/mission/tests/test-task-6.1.sh` executed successfully with output: `PASS: darkfactory-init coverage tooling and test-quality-review checks are present`

## Files Reviewed

- `.claude/skills/darkfactory-init/SKILL.md` (modified in commit)
- `openspec/changes/scrutiny-test-health/mission/tests/test-task-6.1.sh` (added in commit)
