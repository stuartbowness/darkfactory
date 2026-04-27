# Adversarial Review B: Scrutiny Test Health Mission

Reviewer: Claude (adversarial-b)
Base SHA: 98007bfd60a806f62eafb21b5c204716a58983eb
Branch: mission/darkfactory-dispatch (9 mission commits)

## P0: Critical

### 1. tasks.md not marked done

Only task 1.1 is checked `[x]` in `openspec/changes/scrutiny-test-health/tasks.md`. The remaining 12 tasks (2.1 through 7.4) are still `[ ]` despite all corresponding code being committed and validated. Darkfactory requires marking tasks done in tasks.md after validation (see commit eceec5c "Require marking tasks done in tasks.md after validation"). This is a process violation that will confuse `/mission-status` and `/mission-resume` if the mission is interrupted.

**Fix:** Mark tasks 2.1, 2.2, 2.3, 3.1, 3.2, 5.1, 6.1, 7.1, 7.2, 7.3, 7.4 as `[x]`. Task 4.1 (create the test-quality-review skill in layercodedev/skills repo) was done externally; decide whether to mark it done or note it was out-of-scope for this repo.

### 2. README Required external skills table missing /test-quality-review

The README's "Required external skills" table (lines 196-208) does not include `/test-quality-review`. This skill is a required dependency: the scrutiny validator always invokes it as check 14, and `darkfactory-init` checks for it as a required supporting skill. A user following the README would not know to install it.

The `/darkfactory-init` section at line 55 does mention it in prose, and the SKILL.md correctly lists it in Step 2 and Step 5. But the canonical table is the authoritative list of external skills, and `/test-quality-review` is absent from it.

**Fix:** Add a row to the Required external skills table:

```
| `/test-quality-review` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/test-quality-review) | Scrutiny validator: LLM-based test quality analysis (check 14) |
```

Also update the summary paragraph after the table to mention it alongside `/security-review`.

## P1: Important

### 3. Check 14 severity mapping has a gap for 1-2 P1 findings

The severity-based result behavior for check 14 (test quality review) defines four cases:
- Any P0 = FAIL
- 3+ P1 = FAIL
- P2 and P3 **only** = WARN
- No findings = PASS

The combination of 1-2 P1 findings (with no P0 and possibly some P2/P3) falls through all four rules. It is not "P2 and P3 only" because P1 is present. It is not "3+ P1" because the count is below 3. It is not "no findings." The validator would have to guess.

**Fix:** Add: "1-2 P1 findings are a WARNING; include them in the report but do not fail the task."

### 4. Silent skip check 12 remediation guidance is Rust-centric

The remediation guidance at the end of check 12 says: "Either fail explicitly with `panic!("requires X")` or mark as `#[ignore]` / `@skip` so the test runner reports it as skipped, not passed."

- `panic!()` is Rust-only. TypeScript equivalent is `throw new Error()`. Python equivalent is `pytest.fail()` or `raise`.
- `#[ignore]` is Rust-only. TypeScript equivalent is `it.skip()` or `describe.skip()`. Python equivalent is `@pytest.mark.skip(reason="...")`.
- `@skip` is not a standard marker in any of the three languages.

The "do not flag" sections earlier in check 12 correctly list language-specific markers, but the remediation guidance collapses them all into Rust terminology.

**Fix:** Make the remediation language-aware: "Either fail explicitly (`panic!("requires X")` in Rust, `throw new Error("requires X")` in TypeScript, `pytest.fail("requires X")` in Python) or use the runner's explicit skip marker (`#[ignore]` in Rust, `it.skip()` in TypeScript, `@pytest.mark.skip(reason="...")` in Python) so the test runner reports it as skipped, not passed."

### 5. darkfactory-init does not mention cargo-llvm-cov as a Rust coverage fallback

The scrutiny validator (check 13) specifies: "Rust: run `cargo tarpaulin --out json` first; if that is unavailable or fails because the tool is missing, use `cargo llvm-cov --json` as fallback."

The darkfactory-init SKILL.md only checks for `cargo-tarpaulin` and only suggests installing it. If a user has `cargo-llvm-cov` installed but not tarpaulin, `darkfactory-init` will report a missing tool even though the scrutiny validator would use the fallback successfully.

**Fix:** Add a note in the coverage tool verification for Rust: "Also check for `cargo llvm-cov --version` as an alternative. If tarpaulin is missing but llvm-cov is present, report 'cargo-llvm-cov (fallback) installed.'"

### 6. README term count says "25 terms" but actual count is 42

The README line 289 says: `[Ubiquitous language](UBIQUITOUS_LANGUAGE.md) -- shared terminology used throughout the codebase (25 terms)`. The actual count was 35 before this mission and is now 42 after adding the 7 Test health terms. The mission's documentation task (7.4) should have caught this.

**Fix:** Update to `(42 terms)`.

## P2: Minor

### 7. Python "bare raise without context" is ambiguous

The error handling smell pre-pass flags "bare `raise` without context" for Python. This is ambiguous because:
- `raise` (bare re-raise inside an except block) is idiomatic Python for preserving the original traceback. It is not a smell.
- `raise SomeException()` without `from original` when chaining exceptions could lose context.
- The phrase "without context" could mean either case.

The Rust and TypeScript sections have explicit safe-pattern exclusions. Python's safe pattern only lists `except SpecificError: with a handler` but does not clarify when bare `raise` is safe vs. flagged.

**Fix:** Clarify: "Flag `raise` used without `from` when re-raising a different exception type (loses original traceback). Do not flag bare `raise` inside an `except` block (preserves original traceback)."

### 8. Error handling smell density: "Under 5 per KLOC" threshold boundary is ambiguous

Check 7 says:
- Under 5 per KLOC: PASS
- 5-10 per KLOC: WARNING
- Over 10 per KLOC: FAIL

Exactly 5.0 per KLOC falls in WARNING (covered by "5-10"). Exactly 10.0 per KLOC is ambiguous -- "5-10" includes it, but "Over 10" does not. The likely intent is `>=5 && <=10` for WARNING and `>10` for FAIL, but a validator could read "5-10" as exclusive of 10.

**Fix:** Use precise notation: `<5 per KLOC: PASS`, `5-10 per KLOC: WARNING`, `>10 per KLOC: FAIL`.

### 9. skill-decomposition proposal committed in this branch

Commit 8b20a3b ("WIP: skill-decomposition proposal and OpenSpec change directory") added `openspec/changes/skill-decomposition/proposal.md` and its specs directory. This is unrelated to the scrutiny-test-health mission and appears to be a WIP artifact that was committed to the same branch.

**Fix:** Either move it to a separate branch or explicitly note it in the PR description as intentional pre-work for a future mission.

### 10. Error handling smell pre-pass: .expect("") boundary not precise enough

The pre-pass flags `.expect("")` (empty string) but not `.expect("non-empty message")`. Edge cases like `.expect(" ")` (whitespace only) or `.expect("x")` (single character) are not addressed. An LLM validator will use judgment, but the lack of precision could lead to inconsistent flagging.

**Fix:** Clarify: "Flag `.expect()` with an empty or whitespace-only message string."

### 11. Pass rate formula does not handle zero-test edge case

The formula `passed / (passed + failed + skipped) * 100` produces a division-by-zero when the test suite reports 0 passed, 0 failed, 0 skipped. While unlikely, this could happen with an empty test suite or when all tests are filtered out. The "no summary line" fallback handles the case where the parser cannot parse, but a parsed result of `0, 0, 0` would still trigger the formula.

**Fix:** Add: "If passed + failed + skipped equals zero, report pass rate as `N/A`."

## Summary

| Severity | Count |
|----------|-------|
| P0       | 2     |
| P1       | 4     |
| P2       | 5     |

The main deliverable (scrutiny-validator.md) is substantively complete: all 14 checks are present, the deterministic pre-passes are well-structured, the output format table matches the check count, and the language-specific patterns cover Rust, TypeScript, and Python for all relevant checks. The UBIQUITOUS_LANGUAGE.md additions are thorough and internally consistent. The test-quality-review skill at layercodedev/skills is well-structured with correct severity levels that align with check 14's consumption.

However, two P0 issues need fixing before merge: the tasks.md checkboxes are not updated, and the README external skills table is missing the new required skill. The P1 items (severity gap, Rust-centric remediation, missing fallback tool check, stale term count) are real but not blocking -- they would cause confusion or inconsistency during actual validation runs.

## Verdict: REQUEST_CHANGES

Fix the two P0 items (tasks.md checkboxes, README external skills table) and the P1 severity gap in check 14. The remaining P1 and P2 items are recommended but not blocking.
