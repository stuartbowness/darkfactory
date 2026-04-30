# Task 6 User-Test Report — Write Adversarial Skill

**Validator:** user-test  
**Commit:** 1ac6f1c791b331fcb85f44e3bd9fdb0104f06a6d  
**Date:** 2026-04-26  
**Verdict: PASS**

---

## Assertions

### Assertion 1: File exists
**Command:** `test -f skills/adversarial/SKILL.md && echo PASS || echo FAIL`  
**Result:** PASS  
**Status: PASS**

### Assertion 2: name field is correct
**Command:** `grep "^name:" skills/adversarial/SKILL.md`  
**Result:** `name: adversarial`  
**Status: PASS**

### Assertion 3: Contains `gh pr diff`
**Command:** `grep -c "gh pr diff" skills/adversarial/SKILL.md`  
**Result:** 2  
**Required:** >= 1  
**Status: PASS**

### Assertion 4: Contains `darkfactory-dispatch`
**Command:** `grep -c "darkfactory-dispatch" skills/adversarial/SKILL.md`  
**Result:** 2  
**Required:** >= 1  
**Status: PASS**

### Assertion 5: Contains `adversarial-codex.md`
**Command:** `grep -c "adversarial-codex.md" skills/adversarial/SKILL.md`  
**Result:** 1  
**Required:** >= 1  
**Status: PASS**

### Assertion 6: Contains severity labels (critical/warning/nit)
**Command:** `grep -icE "critical|warning|nit" skills/adversarial/SKILL.md`  
**Result:** 7  
**Required:** >= 1  
**Status: PASS**

### Assertion 7 (inspection): Dispatch/parallel/disagreement logic

Reviewed full file content. Key structural checks:

**Codex dispatched before Claude:** Step 2 instructs dispatching Codex with `darkfactory-dispatch --async` and `run_in_background: true`. Step 3 begins "While Codex is running in the background" — confirming the intended parallel execution order.

**Both receive identical input:** The skill saves the diff once (line 46) and uses the same content for both reviewers. The Codex prompt template and the Claude Agent prompt are identical except for the report output path.

**Disagreements surfaced after both complete:** Step 4 waits for Codex to finish, then reads both reports. It defines three types of disagreement: findings unique to Codex, findings unique to Claude, and findings both report but with different severity. These are all presented in a structured block to the user.

**Report paths are well-defined:** The skill specifies both `adversarial-codex.md` and `adversarial-claude.md` paths with mission-aware and fallback (`/tmp/`) variants.

**Status: PASS**

---

## Summary

All 7 assertions pass. The adversarial skill file is present, correctly named, references `gh pr diff`, uses `darkfactory-dispatch`, uses `adversarial-codex.md`, uses severity labels, and structurally implements the parallel dispatch + disagreement-surfacing pattern the contract requires.
