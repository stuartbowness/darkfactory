# Fix Brief: All v2 Skills (Post-Adversarial Review)

## What Failed

An adversarial Codex review found 7 issues across the v2 skills, plus one new feature needs to be added.

## Issues to Fix

### Issue 1: Circuit breaker is fake (mission skill)
The circuit breaker offers "Retry: reset attempt counter and try again" which means it can be bypassed forever. Fix: retry should NOT reset the counter. Retry means "try one more time" (attempt 4), not "start over." After the human says retry, increment the counter and allow one more attempt. If it fails again, circuit breaker triggers again with the higher count.

### Issue 2: "Skip" is an impossible state (mission skill)
The fix loop offers "skip" but tasks.md only has `- [ ]` and `- [x]`. There's no skipped state, and the build loop picks up the next unchecked task, so a skipped task would be re-attempted. Fix: add a skipped state. When the human says skip, change the task to `- [~]` in tasks.md (or `- [x] (SKIPPED)`) and log it to progress.md. The build loop should ignore tasks marked as skipped when looking for the next unchecked task.

### Issue 3: Config schema inconsistency (mission skill)
v2 uses `engine:` (singular) in config.yaml but the design decisions doc and v1 use `engines:` (plural). Fix: standardize on `engines:` (plural) throughout. Also add the new adversarial reviewer engines to the default config.

### Issue 4: Status can't parse mission reports (status skill)
v2 status looks for lines starting with "Overall" to extract validation results, but v2 mission's validator prompts write "## Result: PASS | FAIL". Fix: make status look for BOTH patterns -- "## Overall:" AND "## Result:" -- to be robust against format variations. Better yet, look for the first line containing "PASS" or "FAIL" after the heading.

### Issue 5: Resume can't resume mid-cycle (resume skill)
If the session dies between "worker done" and "validation done," both versions restart the task from scratch. Fix: the resume skill should check not just for unchecked tasks, but also for tasks that have a handoff file but no validation reports. If a handoff exists (worker finished) but no scrutiny report exists, resume from scrutiny validation instead of re-dispatching a worker. If scrutiny exists but no user-test, resume from user-test.

### Issue 6: Monitor not used throughout (mission skill)
The design says "one pattern: Monitor throughout" but v2 only uses Monitor for worker dispatch. Validators use direct Agent tool calls. Fix: this is actually correct behavior. The Agent tool returns synchronously within the Claude session. Monitor is for external processes (like Codex) where Claude needs to go idle. Claude subagent validators don't need Monitor because they run within Claude's own context. Update the design note in the skill to clarify: "Monitor is used for external process dispatch (Codex workers, Codex adversarial reviews). Claude subagent validators use the Agent tool directly since they run within the session."

### Issue 7: Orphaned commit loophole (resume skill)
Both versions allow counting an orphaned commit as "done" without validation, which violates the design. Fix: when the human says "count as done" for an orphaned commit, don't just mark it complete. Instead, run the full validation cycle (scrutiny + user-test + adversarial review) against that commit SHA. The human is confirming "the code exists," not "the code is correct." Validation still needs to happen.

### New Feature: Dual adversarial review step (mission skill + resume skill + config)
Add a new validation step after user-test and before demo video. Two adversarial reviewers run in parallel:
- adversarial_reviewer_a (default: codex) -- dispatched via Monitor since it's an external process
- adversarial_reviewer_b (default: claude) -- dispatched via Agent tool

Both receive the commit SHA and the full contract (all assertions, not just this task's). They independently review the code for flaws, inconsistencies, cross-file compatibility issues, and anything the previous validators missed.

The orchestrator reads both reports, synthesizes them into a single findings list: deduplicate shared findings, flag disagreements (one says PASS, other says FAIL), and prioritize. If the synthesis has any FAIL findings, enter the fix loop. The fix brief includes the synthesized findings.

Config addition:
```yaml
engines:
  worker: codex
  scrutiny_validator: claude
  usertest_validator: claude
  adversarial_reviewer_a: codex
  adversarial_reviewer_b: claude
  demo_video: claude
```

Updated flow per task:
```
worker -> scrutiny -> user-test -> adversarial review (parallel: codex + claude) -> orchestrator synthesizes -> demo video -> mark complete
```

On adversarial review failure, re-run ALL validators after the fix (scrutiny, user-test, both adversarial reviewers).

Reports written to:
- mission/validations/task-N-adversarial-a.md
- mission/validations/task-N-adversarial-b.md
- mission/validations/task-N-adversarial-synthesis.md

## Files to Modify

1. darkfactoryV2/.claude/skills/mission/SKILL.md -- issues 1, 2, 3, 6, new feature
2. darkfactoryV2/.claude/skills/mission-resume/SKILL.md -- issues 5, 7, new feature
3. darkfactoryV2/.claude/skills/mission-status/SKILL.md -- issue 4

## Procedures
- Fix all issues listed above across all three files
- Commit all changes in a single commit with message: "[task-1] fix: address adversarial review findings and add dual adversarial review step"
- Write your handoff report to openspec/changes/darkfactory-v2/mission/handoffs/task-1-fix-v2.md

## Handoff Report Format
- Task: Fix all v2 skills post-adversarial review
- Commit SHA: [the full SHA]
- Approach: [what you changed and why]
- Commands run: [any commands and exit codes]
- Concerns: [anything uncertain]
