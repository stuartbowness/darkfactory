# Adversarial Review B

Reviewer: Claude Opus 4.6 (adversarial pass)
Scope: 11 commits from 5a6e618..3256b83

## Findings

### MEDIUM: Remaining awkward "check whether the check" phrasing in SKILL.md

**File:** `skills/mission/SKILL.md`, line 397

The task-4 fix (398775c) fixed two of the doubled-check phrasings flagged by the smoke test:
- "check every check" on line 351 became "verify every check"
- "Check each contract check" in usertest-validator.md became "Verify each contract check"

A third instance was not fixed:

```
3. **Inspection checks** (`[inspection]`) -- check whether the check could be rewritten
```

The verb "check" immediately precedes the noun "check". This is the same awkward doubled pattern the task-4 fix was meant to eliminate. The smoke test (smoke-test.md section 4b, line 56) classified lines 395-397 as "acceptable" because the verb on line 395 is "confirm", not "check" -- but that reasoning only covers line 395. Line 397 uses "check whether the check" which is the same verb+noun collision as the two that were fixed.

Suggested fix: "determine whether the check could be rewritten" or "evaluate whether the check could be rewritten".

Severity: MEDIUM. The phrasing is technically correct but undermines the clarity goal of the rename mission.

---

### LOW: README uses "check" ambiguously for both validator steps and contract items

**File:** `README.md`, lines 96-105

The README describes the scrutiny validator's steps using informal "check" terminology:

```
1. **Scrutiny validator** -- a thorough quality check that runs a sequence of checks:
   ...
   - Contract check -- does the code do what the contract says it should?
   - Execution check -- actually runs the code to confirm it works
   - Test health checks -- silent skip detection, diff coverage...
```

Per the resolved ambiguity in UBIQUITOUS_LANGUAGE.md (line 183), a "gate" is a numbered step in the scrutiny validator's sequence, and a "check" is a single verifiable statement in a contract. The README uses "check" for both: "Contract check" (line 103), "Execution check" (line 104), "Complexity check" (line 101), and "Code standards check" (line 99) all describe validator steps (gates), not contract items.

This is borderline. The README is written for a general audience and deliberately avoids formal internal terminology. The word "check" here functions as generic English ("a thing we verify"), not as the domain term "Check" (a contract item). But the three-way split was specifically designed to prevent this exact ambiguity, and someone reading the glossary alongside the README would notice the conflict.

Severity: LOW. The README is user-facing prose where casual language is expected. No action needed unless the team wants strict glossary compliance in the README.

---

### INFO: Task-1 changed gate count from 11 to 14 -- correctness, not just a rename

**File:** `skills/mission/SKILL.md`, line 594

The task-1 diff (5a6e618) changed:

```
-The prompt runs 11 checks: test presence, build, tests, lint, code review,
-module depth, code standards, simplify, security (if applicable), contract
-assertions, behavioral conformance.
+The prompt runs 14 gates: test presence, build, tests, lint, code review,
+module depth, code standards, simplify, security (if applicable), contract
+checks, behavioral conformance, silent skip detection, diff coverage,
+test quality review.
```

This updated the count from 11 to 14 and added three gates that were missing from the original enumeration (silent skip detection, diff coverage, test quality review). The actual scrutiny-validator.md has had 14 gates since before this mission. This is a correctness fix bundled into a rename commit. The change is correct, but it goes beyond the task-1 scope ("Rename scrutiny validator steps from check to gate"). It was not caught by the per-task validator because the contract items only check for "gate N" naming and count, not for scope creep.

Severity: INFO. The fix is correct. Noting it because scope creep in rename missions can mask unintended changes.

---

## Cross-file consistency

### Gate/Check/Assertion three-way split

Verified across all in-scope files:

| Context | Expected term | Files checked | Result |
|---------|--------------|---------------|--------|
| Numbered scrutiny validator steps | gate | scrutiny-validator.md, SKILL.md | PASS -- "Gate N:" format throughout, zero "check [0-9]" hits |
| Contract items | check | SKILL.md, scrutiny-validator.md, usertest-validator.md, adversarial-reviewer.md, contract.md, plan.md, pr-body.md, worker-fix-brief.md, mission-resume/SKILL.md, docs/, README.md | PASS -- "assertion" only in code-test context |
| Code test assertions | assertion | test-quality/SKILL.md, tdd/tests.md, scrutiny-validator.md (gates 1, 14) | PASS -- unchanged from before the mission |
| UBIQUITOUS_LANGUAGE.md definitions | all three defined | UBIQUITOUS_LANGUAGE.md | PASS -- resolved ambiguity section documents the split |

### Contract item check

Every contract check in contract.md was verified against the final file state:

- Task 1: "gate N" naming throughout scrutiny-validator.md and SKILL.md. Zero "check [0-9]" hits. PASS.
- Task 2: "Group checks by task", "minimum set of checks", "total contract checks", "Inspection checks", "check count feels right" all present. PASS.
- Task 3: "Contract Checks for This Task" heading in both validator prompts. "Check" column headers. "Failed Checks" sections. "checks" in adversarial-reviewer.md, contract.md, plan.md, pr-body.md, worker-fix-brief.md, mission-resume/SKILL.md, docs/, README.md. PASS.
- Task 4: Smoke test verified. Preserved files unchanged. Awkward phrasings partially fixed. PASS with caveat on line 397.
- Task 5: No inconsistent capitalization found. No orphaned cross-references. PASS.
- Task 6: Module shape audit exists. Type Catalog entries match UBIQUITOUS_LANGUAGE.md. PASS.

### Over-eager renames

No code-test-context "assertion" was incorrectly renamed to "check". The three remaining "assertion" hits in scrutiny-validator.md (lines 113, 251, 257) are all about test assertions. Skills outside the rename scope (test-quality, tdd, prd) are untouched.

### Missed renames

No "assertion" in contract-item context remains in any in-scope file. grep confirmed zero hits.

## Summary

One MEDIUM finding (missed awkward phrasing on SKILL.md line 397), one LOW finding (README using "check" loosely for validator steps), and one INFO note (scope creep in task-1 that happens to be correct). The core rename is complete and consistent across all files. No over-eager renames. No missed renames. The three-way gate/check/assertion split is applied correctly in all formal contexts.
