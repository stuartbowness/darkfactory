# Scrutiny Validation: Task 1 - Main Mission Skill

## Overall: PASS

## Build/Test/Lint
- Build: N/A (markdown skill files)
- Test: N/A
- Lint: N/A

## Code Review

### Minor: Step numbering in Phase 7 is slightly confusing

The build loop uses Steps 1-7, but Step 5 has a sub-step 5a (fix loop), and Step 6 has sub-step 6a (demo video). The numbering is functionally correct but could trip up a reader on first pass. For example, Step 5 (handle scrutiny result) says "Proceed to Step 6" on PASS, while Step 6 (user-test) says "Proceed to Step 6a" on PASS. The hierarchy is clear enough once you read it, but a flat numbering (Steps 1-9 without sub-steps) would be easier to follow.

Severity: Low. The orchestrator LLM should handle this fine since the instructions are explicit about what to do in each case.

### No other issues found

The file is well-structured, internally consistent, and comprehensive. The plan template is durable (survives context compaction). All cross-references between phases are correct. Both validator prompts are complete and self-contained. The fix loop correctly re-runs both validators after every fix. The circuit breaker is clearly defined.

## Contract Assertions

1. **File exists with valid YAML frontmatter (`name: mission` and description)**: PASS - Lines 1-4 contain standard YAML frontmatter with `name: mission` and a multi-sentence description.
2. **Documents Phase 0 (enter plan mode and write orchestrator operating manual)**: PASS - Lines 14-81 define Phase 0 with "Enter plan mode immediately. Write a durable operating manual as the plan." Includes full plan template.
3. **Documents Phase 1 (grill the user for alignment using /grill-me)**: PASS - Lines 84-93 define Phase 1: "Run `/grill-me` to interview the user about their idea." Includes skip-warning flow.
4. **Documents Phase 2 (establish shared language using /ubiquitous-language)**: PASS - Lines 96-101 define Phase 2: "Run `/ubiquitous-language` to create or update `UBIQUITOUS_LANGUAGE.md`." Handles both existing and new file cases.
5. **Documents Phase 3 (create OpenSpec artifacts using /opsx:propose)**: PASS - Lines 104-122 define Phase 3: "Run `/opsx:propose` to create the OpenSpec change" with proposal.md, design.md, tasks.md. Includes task quality standard (vertical slices).
6. **Documents Phase 4 (write validation contract with assertions grouped by task)**: PASS - Lines 125-153 define Phase 4 with 6 contract rules. Template on lines 139-150 shows assertions grouped under `## Task N` headings.
7. **Documents Phase 5 (create mission directory with config.yaml, progress.md, and subdirectories)**: PASS - Lines 156-192 define the directory structure including config.yaml, progress.md, and four subdirectories (briefs/, handoffs/, validations/, issues/).
8. **Documents Phase 6 (present plan for human approval)**: PASS - Lines 196-208 show the user task breakdown, assertion counts, and engine config, then ask for approval.
9. **Documents Phase 7 (build loop with all required components)**: PASS - Lines 210-547. Brief assembly (Step 1, lines 216-248), worker dispatch (Step 2, lines 250-296), scrutiny validation (Step 4, lines 306-377), user-test validation (Step 6, lines 443-513), demo video (Step 6a, lines 515-527), fix loop with circuit breaker at 3 (Step 5a, lines 388-441), progress tracking (Step 7, lines 529-547).
10. **Documents Phase 8 (mission completion with draft PR and mission summary)**: PASS - Lines 550-593. Runs `/pr` to create draft PR. PR body template includes tasks table, validation results table, issues list, and mission stats.
11. **Includes Monitor bash script for dispatching Codex workers**: PASS - Lines 258-287 contain a complete bash script using `codex exec` with `--full-auto`, polling for handoff file, and outputting TASK_COMPLETE or TASK_FAILED. Line 289 specifies Monitor with 600000ms timeout.
12. **Includes Claude subagent fallback for worker dispatch**: PASS - Lines 294-296: "Use the Agent tool to spawn a subagent. Pass the full brief content as the prompt."
13. **Includes full scrutiny validator prompt with check sequence**: PASS - Lines 310-377 contain the complete prompt with 8 checks in order: diff review, build, tests, lint, code review (/review), code simplify (/code-simplify), security review (/security-review), and contract assertions.
14. **Includes full user-test validator prompt with test strategy decision**: PASS - Lines 449-506 contain the complete prompt with a 4-way test strategy decision (backend-only, frontend-only, both, infrastructure-only) and instructions for PASS/FAIL/SKIP per assertion.
15. **Includes fix brief format with failure details and commit SHA**: PASS - Lines 407-436 show the fix brief template with "What Failed", "Validator Output", and "Commit to Fix" (with SHA placeholder). Lines 416-418 explicitly reference specific failed assertions and the commit SHA.
16. **States workers never see the contract (adversarial separation)**: PASS - Stated in 5 places: line 69 (plan rules), line 136 (contract rules), line 248 (brief note), line 437 (fix brief note), line 618 (enforced rules section).
17. **States validators never see the proposal or design**: PASS - Stated in 4 places: line 70 (plan rules), lines 312/451 (validator prompts say "You have NOT seen the proposal, design, or worker reasoning"), line 618 (enforced rules section).
18. **Uses terms from UBIQUITOUS_LANGUAGE.md correctly**: PASS - All 11 terms verified: Mission (title, throughout), Orchestrator (lines 8, 39, 519), Worker (throughout Steps 2-3, fix loop), Validator (Steps 4-6, prompts), Contract (Phase 4, Steps 4-6), Assertion (Phase 4, validator prompts, progress tracking), Brief (Step 1, fix brief), Handoff (Step 3, handoff report format), Monitor (Step 2, line 289), Engine (config.yaml, lines 37-39, 252), Plan mode (Phase 0, line 14).
19. **References all required skills**: PASS - All 9 verified: /grill-me (lines 43, 86), /ubiquitous-language (lines 44, 98), /opsx:propose (lines 45, 107), /review (lines 46, 335), /code-simplify (lines 47, 336), /security-review (lines 48, 337), /test-app (lines 49, 465), /demo-video (lines 50, 521), /pr (lines 51, 554).

## Summary

All 19 contract assertions pass. The skill file is a comprehensive, well-structured 624-line document that covers the complete mission lifecycle from idea through draft PR. The only quality note is minor step-numbering complexity in Phase 7, which does not affect correctness. No build, test, or lint checks apply to this markdown skill file.
