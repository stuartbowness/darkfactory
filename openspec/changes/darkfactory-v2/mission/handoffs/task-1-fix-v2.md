# Handoff: Fix all v2 skills post-adversarial review

- Task: Fix all v2 skills post-adversarial review (7 issues + 1 new feature)
- Commit SHA: c125276c12679fa64262db187e759d54c0b4781b
- Approach:
  - Issue 1 (fake circuit breaker): Retry no longer resets the counter. It increments by 1 and allows one more attempt. Circuit breaker re-triggers at the new count if the attempt fails.
  - Issue 2 (skip impossible state): Added `- [~]` as the skipped task marker. Build loop explicitly skips `- [~]` tasks. Skip option added to all human-facing decision points (circuit breaker, worker failure, timeout).
  - Issue 3 (config schema): Changed `engine:` to `engines:` (plural) in config.yaml template. Added `adversarial_reviewer_a: codex` and `adversarial_reviewer_b: claude` to the default config.
  - Issue 4 (status parsing): Status skill now looks for both `## Overall:` and `## Result:` patterns, falling back to the first line containing "PASS" or "FAIL". Also added adversarial review synthesis report parsing and `[~]` skipped task display.
  - Issue 5 (resume mid-cycle): Added Phase 3a to resume skill that checks for handoff files with missing validation reports. Uses a table to map which validations exist to the correct resume point (scrutiny, user-test, adversarial review, or demo video).
  - Issue 6 (Monitor clarification): Added Rule 10 explaining Monitor is for external process dispatch only. Claude subagent validators use Agent tool directly since they return synchronously. Updated Rule 1 to note the adversarial reviewer parallel exception.
  - Issue 7 (orphaned commit loophole): "Count as done" replaced with "validate it" -- writes a synthetic handoff and runs the full validation cycle (scrutiny, user-test, adversarial review). Human confirms code exists; validation still happens.
  - New feature (dual adversarial review): Added Step 6a between user-test and demo video. Two reviewers (codex via Monitor, claude via Agent) run in parallel with full contract. Orchestrator synthesizes findings, deduplicates, resolves disagreements. On FAIL, enters fix loop with all validators re-running. Reports written to task-N-adversarial-a.md, task-N-adversarial-b.md, task-N-adversarial-synthesis.md.
- Commands run: git add + git commit (exit 0)
- Concerns: None. All 7 issues and the new feature are addressed across all three files.
