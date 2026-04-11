---
name: mission
description: Start an autonomous Darkfactory mission. Grills the user for alignment, creates OpenSpec artifacts, writes a validation contract, writes the mission directory, dispatches workers serially, validates each task against the contract, runs fix loops with circuit breakers, and creates a draft PR on completion.
---

# /mission

Start an autonomous multi-agent mission for a single OpenSpec change. The orchestrator (this session) plans the work, dispatches workers one at a time, and validates each result against a contract the workers never see.

**Flow:** `idea -> spec -> build -> verify -> ship -> merge`

---

## Phase 0: Enter Plan Mode

Enter plan mode immediately. Write a durable operating manual as the plan. This plan survives context compaction and is the orchestrator's source of truth.

### Plan Template

Write this plan, filling in values as they become known:

```markdown
# Mission: <change-name>

## Status: PLANNING | APPROVED | BUILDING | COMPLETE | PAUSED

## State Directory
openspec/changes/<change-name>/mission/

## OpenSpec Artifacts
- Proposal: openspec/changes/<change-name>/proposal.md
- Design: openspec/changes/<change-name>/design.md
- Tasks: openspec/changes/<change-name>/tasks.md
- Contract: openspec/changes/<change-name>/contract.md

## Engine Config
- Workers: <engine from config.yaml, default codex>
- Scrutiny validator: <engine, default claude>
- User-test validator: <engine, default claude>
- Adversarial reviewer A: <engine, default codex>
- Adversarial reviewer B: <engine, default claude>
- Demo video: <engine, default claude>

## Skills to Use
- /grill-me -- alignment interview (planning)
- /ubiquitous-language -- shared language (planning)
- /write-a-prd -- structured PRD from grill conversation (planning)
- /opsx:propose -- create proposal, design, tasks from PRD (planning)
- /review -- code review (scrutiny validation)
- /code-simplify -- simplify pass (scrutiny validation)
- /security-review -- security check (scrutiny validation, sensitive tasks only)
- /tdd -- test-driven development for workers (build phase)
- /test-app -- browser tests (user-test validation)
- /demo-video -- frontend demo recording (post-validation)
- /pr -- draft PR (completion)
- /opsx:archive -- archive change (post-merge)

## Mission Flow
For each unchecked task in tasks.md (skip `- [~]` tasks):
1. Read mission state from progress.md
2. Write brief to mission/briefs/task-N.md
3. Dispatch worker (engine from config.yaml)
4. Read handoff from mission/handoffs/task-N.md
5. Spawn scrutiny validator (commit SHA + contract assertions)
6. If fail -> write to issues/, spawn fix worker, re-validate all. Circuit breaker at 3.
7. Spawn user-test validator (commit SHA + contract assertions)
8. If fail -> same fix/re-validate loop
9. Run dual adversarial review (reviewer A + reviewer B in parallel), synthesize, fix loop if FAIL
10. If frontend task -> spawn demo video
11. Mark task done in tasks.md, update progress.md

## Rules
- One worker at a time (serial execution)
- Workers never see contract.md
- Validators never see proposal.md or design.md
- On any failure, re-run ALL validators after fix (scrutiny, user-test, adversarial review)
- Write all decisions to progress.md

## Current Task: <N or "none">
## Current Step: <step name or "none">
```

### First Action

Check if `UBIQUITOUS_LANGUAGE.md` exists in the repo root. If it does, read it now so all phases use consistent terminology.

---

## Phase 1: Grill for Alignment

Run `/grill-me` to interview the user about their idea. The grill session should drive toward shared understanding of what is being built, why, and what "done" looks like.

If the user says "skip":
- Warn: "Skipping the grill means the contract will be based on whatever I can infer. Contract quality depends on alignment quality. Are you sure?"
- If they confirm, proceed to Phase 2.

After the grill completes, summarize the key decisions in the plan.

---

## Phase 2: Establish Shared Language

Run `/ubiquitous-language` to create or update `UBIQUITOUS_LANGUAGE.md`.

- If the file already existed and was read in Phase 0, this phase updates it with any new terms from the grill session.
- If the file did not exist, this phase creates it. Read the file after creation so all subsequent phases use the terms.

---

## Phase 3: Write a PRD

Run `/write-a-prd` to turn the grill conversation into a structured product requirements document. The PRD captures: problem statement, solution, user stories, implementation decisions, testing decisions, and out-of-scope items.

The PRD is filed as a GitHub issue. This issue becomes the source of truth that `/opsx:propose` reads when creating the OpenSpec artifacts. The PRD ensures nothing from the grill conversation is lost or misinterpreted during the transition to technical artifacts.

---

## Phase 4: Create OpenSpec Artifacts

Run `/opsx:propose` to create the OpenSpec change with:
- `proposal.md` -- what and why
- `design.md` -- how (architecture decisions)
- `tasks.md` -- ordered checklist of tasks

### Task Quality Standard

Each task in tasks.md must be a **vertical slice**: a thin end-to-end path through all layers (schema, API, UI, tests) that delivers narrow but complete functionality. Follow the `/prd-to-plan` tracer bullet pattern.

**One task = one worker = one commit = one validation cycle.**

A task is good if you can demo it and verify it independently after it lands.

After creation, update the plan with the actual artifact paths.

---

## Phase 5: Write Validation Contract

Create `contract.md` in the change directory (`openspec/changes/<change-name>/contract.md`).

### Contract Rules

1. Group assertions by task, matching the task list in tasks.md.
2. Every assertion must describe **observable behavior** verifiable from outside the code (HTTP response, CLI output, file on disk, UI state, build passing, etc.).
3. Do NOT reference implementation details (function names, variable names, internal data structures).
4. Cover the **happy path AND error cases** for each task.
5. Be specific and unambiguous. A validator reading only the contract and the diff should be able to check every assertion without guessing.
6. Workers never see this file. This is the adversarial separation that makes validation meaningful.

### Contract Template

```markdown
# Validation Contract

## Task 1: <name>
- [ ] <assertion describing observable behavior>
- [ ] <assertion describing observable behavior>
- [ ] <error case assertion>

## Task 2: <name>
- [ ] <assertion>
- [ ] <assertion>
```

Update the plan with the contract path after creation.

---

## Phase 6: Create Mission Directory

Create the mission directory at `openspec/changes/<change-name>/mission/` with this structure:

```
mission/
  config.yaml
  progress.md
  briefs/
  handoffs/
  validations/
  issues/
```

### config.yaml

```yaml
engines:
  worker: codex
  scrutiny_validator: claude
  usertest_validator: claude
  adversarial_reviewer_a: codex
  adversarial_reviewer_b: claude
  demo_video: claude
```

### progress.md

```markdown
# Mission: <change-name>

Started: <ISO 8601 timestamp>

## Log

- <timestamp> -- Plan approved. <N> tasks, <M> assertions in contract.
```

Create the four subdirectories: `briefs/`, `handoffs/`, `validations/`, `issues/`.

---

## Phase 7: Present for Approval

Show the user:
1. **Task breakdown** -- the full task list from tasks.md with descriptions
2. **Contract assertion counts** -- how many assertions per task, total count
3. **Engine config** -- which engine runs each role

Ask: "Ready to start the build? You can request changes or approve."

- If changes requested: loop back to whichever phase needs adjustment.
- If approved: record approval in progress.md with timestamp. Update plan status to BUILDING.

---

## Phase 8: Build Loop

For each unchecked task (`- [ ]`) in tasks.md, run Steps 1-8. Process tasks in order, one at a time (serial execution). Skip tasks marked `- [~]` (skipped) and `- [x]` (complete).

### Step 1: Assemble Brief

Write a brief to `mission/briefs/task-N.md`.

#### Brief Template

```markdown
# Worker Brief: Task N - <name>

## Your Task
<task description from tasks.md, expanded with acceptance criteria>

## Project Context
<relevant excerpt from proposal.md -- what is being built and why>

## Architecture
<relevant excerpt from design.md -- how this task fits into the system>

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
<any domain terms the worker needs to understand>

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-N] <description>"
- Write your handoff report to openspec/changes/<change-name>/mission/handoffs/task-N.md

## Handoff Report Format
- Task: Task N - <name>
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
```

**The brief NEVER contains contract.md content.** This is the adversarial separation.

### Step 2: Dispatch Worker

Read the engine from `mission/config.yaml`.

#### If engine is `codex`:

Use the Monitor tool to run the worker in the background. Use this exact bash script:

```bash
BRIEF_PATH="<absolute path to mission/briefs/task-N.md>"
HANDOFF_PATH="<absolute path to mission/handoffs/task-N.md>"
BRIEF_CONTENT=$(cat "$BRIEF_PATH")

# Start Codex in background
codex exec \
  --full-auto \
  --sandbox workspace-write \
  -m gpt-5.4 \
  -c model_reasoning_effort="high" \
  -o "$HANDOFF_PATH" \
  "$BRIEF_CONTENT" \
  2>/dev/null &

CODEX_PID=$!

# Poll for handoff file or process exit
while true; do
  if [ -f "$HANDOFF_PATH" ]; then
    echo "TASK_COMPLETE: Handoff written to $HANDOFF_PATH"
    break
  fi
  if ! kill -0 $CODEX_PID 2>/dev/null; then
    echo "TASK_FAILED: Codex process exited without writing handoff"
    break
  fi
  sleep 2
done
```

Use Monitor with a timeout of 600000ms (10 minutes). When Monitor reports:
- `TASK_COMPLETE` -> proceed to Step 3
- `TASK_FAILED` -> proceed to Step 3 (failure path)
- Timeout -> proceed to Step 3 (timeout path)

#### If engine is `claude`:

Use the Agent tool to spawn a subagent. Pass the full brief content as the prompt. Instruct the subagent to write its handoff to the same path (`mission/handoffs/task-N.md`).

### Step 3: Handle Result

Read the handoff file at `mission/handoffs/task-N.md`.

- **TASK_COMPLETE** (handoff exists and contains a commit SHA): Extract the commit SHA. Log to progress.md: `<timestamp> -- Task N: worker completed. Commit: <SHA>`. Proceed to Step 4.
- **TASK_FAILED** (handoff missing or no commit SHA): Log to progress.md: `<timestamp> -- Task N: worker FAILED. <reason>`. Ask the human: retry, skip, or abort. If skip: mark the task as `- [~]` in tasks.md and log `<timestamp> -- Task N: SKIPPED by human.` to progress.md. The build loop ignores `- [~]` tasks.
- **Timeout** (Monitor timed out): Log to progress.md: `<timestamp> -- Task N: worker TIMED OUT after 10 minutes.`. Ask the human: retry, skip, or abort. If skip: mark the task as `- [~]` in tasks.md and log `<timestamp> -- Task N: SKIPPED by human.` to progress.md. The build loop ignores `- [~]` tasks.

### Step 4: Scrutiny Validation

Spawn a Claude subagent (via the Agent tool) with this prompt:

#### Scrutiny Validator Prompt

```
You are a scrutiny validator for the Darkfactory mission system. Your job is to verify that a worker's commit meets the validation contract. You have NOT seen the proposal, design, or worker reasoning. You judge only by the code and the contract.

## Commit to Validate
SHA: <commit-sha>

## Contract Assertions for This Task
<paste the assertions from contract.md for task N>

## Check Sequence

Run these checks in order. For each, report PASS or FAIL with evidence.

1. **Diff review:** Run `git show <commit-sha>` and read the full diff. Does the code look correct and complete?

2. **Build:** Run the project's build command. Does it pass?

3. **Tests:** Run the project's test command. Do all tests pass? Are there new tests covering the task's functionality?

4. **Lint:** Run the project's lint command. Are there any violations?

5. **Code review:** Run /review against the diff. Report any bugs, security issues, or logic errors found.

6. **Code simplify:** Run /code-simplify against the changed files. Report any unnecessary complexity.

7. **Security review (if applicable):** If this task touches authentication, authorization, secrets, user input handling, or network boundaries, run /security-review. Otherwise skip and note "N/A -- no sensitive paths touched."

8. **Contract assertions:** For each assertion in the contract, check it against the diff and test results. Mark each assertion PASS or FAIL with a one-line explanation.

## Output Format

Write your report to: <absolute path to mission/validations/task-N-scrutiny.md>

Use this format:

```markdown
# Scrutiny Validation: Task N

## Result: PASS | FAIL

## Check Results
| Check | Result | Notes |
|-------|--------|-------|
| Diff review | PASS/FAIL | ... |
| Build | PASS/FAIL | ... |
| Tests | PASS/FAIL | ... |
| Lint | PASS/FAIL | ... |
| Code review | PASS/FAIL | ... |
| Code simplify | PASS/FAIL | ... |
| Security review | PASS/FAIL/N/A | ... |

## Contract Assertions
| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | <assertion text> | PASS/FAIL | <one-line evidence> |
| 2 | ... | ... | ... |

## Failed Assertions (if any)
<detailed explanation of each failure>

## Recommendations (if FAIL)
<specific guidance for fixing the failures>
```

IMPORTANT: The overall result is FAIL if ANY check fails or ANY contract assertion fails. Be strict. A near-miss is still a FAIL.
```

Read the validator's report from `mission/validations/task-N-scrutiny.md`.

### Step 5: Handle Scrutiny Validation Result

- **PASS**: Log to progress.md: `<timestamp> -- Task N: scrutiny validation PASSED.` Proceed to Step 6.
- **FAIL**: Proceed to Step 5a (fix loop).

### Step 5a: Fix Loop

Track the attempt number (starts at 1 for the initial worker, increments on each fix).

1. **Write failure details** to `mission/issues/task-N-v<attempt>.md`:

```markdown
# Issue: Task N - <name> (Attempt <attempt>)

## Failed Checks
<list of failed checks from validator report>

## Failed Assertions
<list of failed assertions with evidence>

## Validator Recommendations
<recommendations from validator>
```

2. **Circuit breaker check:** If attempt >= 3, stop the loop. Log to progress.md: `<timestamp> -- Task N: circuit breaker triggered after <attempt> attempts.` Update plan status to PAUSED. Ask the human: "Task N has failed validation 3 times. Options: retry (allows one more attempt, counter continues from current value), skip (mark as skipped with `- [~]`, continue to next task), abort (stop the mission)."

   - **Retry:** Increment the attempt counter by 1 (e.g., attempt 3 becomes attempt 4). Allow one more fix attempt. If it fails again, the circuit breaker triggers again at the new count. Retry does NOT reset the counter.
   - **Skip:** Mark the task as `- [~]` in tasks.md. Log to progress.md: `<timestamp> -- Task N: SKIPPED by human after <attempt> failed attempts.` Continue to the next unchecked task. The build loop ignores `- [~]` tasks (they are neither unchecked nor complete).
   - **Abort:** Update plan status to PAUSED. Stop the build loop. Report to the human.

3. **Write fix brief** to `mission/briefs/task-N-fix-v<attempt+1>.md`:

#### Fix Brief Template

```markdown
# Fix Brief: Task N - <name> (Attempt <attempt+1>)

## What Failed
<specific failed assertions and checks from the validator report>

## Validator Output
<relevant error messages, test failures, review findings>

## Commit to Fix
<SHA of the commit that failed validation>

## Procedures
- Use `/tdd` to fix the issues described above
- Do not refactor or change anything unrelated
- Commit as a single commit with message: "[task-N] fix: <description of what was fixed>"
- Write handoff report to openspec/changes/<change-name>/mission/handoffs/task-N-fix-v<attempt+1>.md

## Handoff Report Format
- Task: Task N - <name> (Fix attempt <attempt+1>)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
```

**The fix brief NEVER contains contract.md content.** Same adversarial separation as the original brief.

4. **Dispatch fix worker** using the same engine and method as Step 2, but with the fix brief.

5. **After fix completes:** Go back to **Step 4** (re-run scrutiny validation with the new commit SHA). All validators must re-run after every fix (scrutiny, user-test, both adversarial reviewers), not just the one that failed.

### Step 6: User-Test Validation

Spawn a Claude subagent (via the Agent tool) with this prompt:

#### User-Test Validator Prompt

```
You are a user-test validator for the Darkfactory mission system. Your job is to verify that a worker's commit delivers the correct user-facing behavior. You have NOT seen the proposal, design, or worker reasoning.

## Commit to Validate
SHA: <commit-sha>

## Contract Assertions for This Task
<paste the assertions from contract.md for task N>

## Instructions

1. **Read the diff:** Run `git show <commit-sha>` to understand what changed.

2. **Decide your test strategy** based on what the diff touches:
   - **Backend only** (API routes, database, server logic, CLI): Write and run HTTP tests against the API endpoints. Use curl or a test script.
   - **Frontend only** (UI components, pages, styles): Run /test-app to drive a browser and verify UI behavior.
   - **Both backend and frontend**: Run HTTP tests first, then /test-app for the UI.
   - **Infrastructure only** (config, build, CI): Verify the build passes and config is correct. You may mark behavioral assertions as SKIP with reason "infrastructure-only change, not testable via HTTP or browser."

3. **Check each contract assertion:** For each assertion, run the appropriate test. Mark PASS, FAIL, or SKIP.
   - PASS: The assertion is demonstrably true.
   - FAIL: The assertion is demonstrably false, or the test reveals a bug.
   - SKIP: The assertion cannot be tested from outside (e.g., internal refactor). Provide a reason.

## Output Format

Write your report to: <absolute path to mission/validations/task-N-usertest.md>

Use this format:

```markdown
# User-Test Validation: Task N

## Result: PASS | FAIL

## Test Strategy
<what you decided and why>

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | <what was tested> | HTTP/Browser/Manual | PASS/FAIL |
| 2 | ... | ... | ... |

## Contract Assertions
| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | <assertion text> | PASS/FAIL/SKIP | <evidence or skip reason> |
| 2 | ... | ... | ... |

## Failed Assertions (if any)
<detailed explanation of each failure>

## Recommendations (if FAIL)
<specific guidance for fixing the failures>
```

IMPORTANT: The overall result is FAIL if ANY contract assertion fails (SKIP does not count as failure). Be thorough -- actually run the tests, don't just read the code and guess.
```

Read the validator's report from `mission/validations/task-N-usertest.md`.

- **PASS**: Log to progress.md: `<timestamp> -- Task N: user-test validation PASSED.` Proceed to Step 6a.
- **FAIL**: Enter the fix loop (Step 5a) with the user-test failure details. After the fix, re-run **all** validators (Step 4, then Step 6, then Step 6a).
- **SKIP** (all assertions skipped): Log to progress.md: `<timestamp> -- Task N: user-test validation SKIPPED (all assertions infrastructure-only).` Proceed to Step 6a.

### Step 6a: Dual Adversarial Review

Run two independent adversarial reviewers in parallel. They receive the commit SHA and the **full contract** (all assertions across all tasks, not just the current task's). They look for flaws, inconsistencies, cross-file compatibility issues, and anything the previous validators missed.

Read the engine assignments from `config.yaml`:
- `adversarial_reviewer_a` (default: codex) -- dispatched via Monitor since it is an external process
- `adversarial_reviewer_b` (default: claude) -- dispatched via Agent tool since it runs within the session

#### Adversarial Reviewer Prompt (used for both A and B)

```
You are an adversarial reviewer for the Darkfactory mission system. Your job is to find flaws, inconsistencies, cross-file compatibility issues, and anything the previous validators (scrutiny + user-test) may have missed.

## Commit to Review
SHA: <commit-sha>

## Full Validation Contract (all tasks)
<paste the ENTIRE contract.md>

## Instructions

1. Run `git show <commit-sha>` to read the full diff.
2. Look for:
   - Logic errors or edge cases not covered by tests
   - Cross-file inconsistencies (types, interfaces, naming)
   - Contract assertions that technically pass but have subtle flaws
   - Security issues, race conditions, or data integrity problems
   - Missing error handling or graceful degradation
   - Anything that would break when integrated with other tasks' code
3. For each finding, classify as PASS (no issue) or FAIL (real problem that must be fixed).
4. Be adversarial: assume the code is wrong until proven right.

## Output Format

Write your report to: <absolute path to mission/validations/task-N-adversarial-{a|b}.md>

```markdown
# Adversarial Review {A|B}: Task N

## Result: PASS | FAIL

## Findings
| # | Category | Severity | Finding | Verdict |
|---|----------|----------|---------|---------|
| 1 | <category> | HIGH/MEDIUM/LOW | <description> | PASS/FAIL |
| 2 | ... | ... | ... | ... |

## Failed Findings (if any)
<detailed explanation of each FAIL finding with evidence and fix guidance>

## Summary
<overall assessment>
```

IMPORTANT: The overall result is FAIL if ANY finding has a FAIL verdict. Be strict and adversarial.
```

#### Dispatch Both Reviewers in Parallel

**Reviewer A** (if engine is `codex`): Use Monitor with the same pattern as worker dispatch, but passing the adversarial reviewer prompt. Write report to `mission/validations/task-N-adversarial-a.md`.

**Reviewer B** (if engine is `claude`): Use Agent tool with the adversarial reviewer prompt. Write report to `mission/validations/task-N-adversarial-b.md`.

Both run at the same time. Wait for both to complete before proceeding.

#### Synthesize Results

After both reports are written, the orchestrator reads both and creates a synthesis report at `mission/validations/task-N-adversarial-synthesis.md`:

```markdown
# Adversarial Review Synthesis: Task N

## Result: PASS | FAIL

## Shared Findings (both reviewers agree)
| # | Finding | Reviewer A | Reviewer B | Verdict |
|---|---------|-----------|-----------|---------|
| 1 | <finding> | PASS/FAIL | PASS/FAIL | PASS/FAIL |

## Disagreements (reviewers differ)
| # | Finding | Reviewer A | Reviewer B | Orchestrator Verdict | Rationale |
|---|---------|-----------|-----------|---------------------|-----------|
| 1 | <finding> | PASS | FAIL | <orchestrator decides> | <why> |

## Unique Findings
| # | Source | Finding | Verdict |
|---|--------|---------|---------|
| 1 | Reviewer A only | <finding> | PASS/FAIL |
| 2 | Reviewer B only | <finding> | PASS/FAIL |

## Action Items (if FAIL)
<prioritized list of issues that must be fixed>
```

**Synthesis rules:**
- Deduplicate shared findings (same issue reported by both).
- Flag disagreements and let the orchestrator make the final call based on evidence.
- Include unique findings from each reviewer.
- Overall result is FAIL if ANY finding in the synthesis has a FAIL verdict.

#### Handle Adversarial Review Result

- **PASS**: Log to progress.md: `<timestamp> -- Task N: adversarial review PASSED (both reviewers).` Proceed to Step 6b.
- **FAIL**: Enter the fix loop (Step 5a) with the synthesis findings. After the fix, re-run **all** validators (Step 4, then Step 6, then Step 6a).

### Step 6b: Demo Video (Frontend Only)

The orchestrator decides whether this task has a frontend component by checking the diff. If the task touches UI files (components, pages, styles, templates), it has a frontend component.

**If frontend component exists:**

Spawn a Claude subagent to run `/demo-video`. The subagent should save the recording to `mission/validations/task-N-demo.mp4`.

**Demo failure is non-blocking.** If the demo fails to record:
- Log to progress.md: `<timestamp> -- Task N: demo video FAILED to record. <reason>. Continuing.`
- Proceed to Step 7.

**If no frontend component:** Skip this step. Proceed to Step 7.

### Step 7: Track Progress (updated numbering: was Step 7, steps above were renumbered)

1. Mark the task complete in `tasks.md`: change `- [ ]` to `- [x]` for this task.

2. Append to `progress.md`:

```markdown
- <timestamp> -- Task N complete.
  - Commit: <SHA>
  - Scrutiny: PASS
  - User-test: PASS | SKIP
  - Adversarial review: PASS
  - Demo: RECORDED | SKIPPED | FAILED
  - Fix attempts: <number, 0 if first try passed>
```

3. Update the plan: set Current Task to the next unchecked task, or "none" if all done.

4. **If more tasks remain:** Go back to Step 1 for the next task.

5. **If all tasks are done:** Proceed to Phase 8.

---

## Phase 9: Mission Completion

### Create Draft PR

Run `/pr` to create a **draft** pull request. Never create a ready-for-review PR.

#### PR Body Template

```markdown
## Mission: <change-name>

### Tasks
| # | Task | Commit | Status |
|---|------|--------|--------|
| 1 | <name> | <short SHA> | Done |
| 2 | <name> | <short SHA> | Done |

### Validation Results
| Task | Scrutiny | User-Test | Adversarial | Demo | Fix Attempts |
|------|----------|-----------|-------------|------|-------------|
| 1 | PASS | PASS | PASS | N/A | 0 |
| 2 | PASS | PASS | PASS | RECORDED | 1 |

### Issues Encountered
<list any issues from mission/issues/, or "None">

### Mission Stats
- Total tasks: <N>
- Total assertions: <M>
- Total fix attempts: <sum>
- Circuit breakers triggered: <count>
- Duration: <start to end>
```

### Finalize

1. Log the draft PR URL to progress.md: `<timestamp> -- Draft PR created: <URL>`
2. Update plan status to COMPLETE.
3. Tell the user: "Mission complete. Draft PR is at <URL>. Review it and promote to ready-for-review when satisfied. Run /opsx:archive after merge."

---

## Resume Support

If `/mission-resume` is invoked (via the mission-resume skill), the orchestrator should:

1. Enter plan mode.
2. Find the mission directory by scanning `openspec/changes/*/mission/config.yaml`.
3. Read `progress.md` to determine the last completed task.
4. Read `tasks.md` to find the next unchecked task.
5. Rebuild the plan from file state.
6. Continue from the appropriate step in Phase 7.

This skill does not implement resume directly -- that is handled by the `/mission-resume` skill. But the file-backed state design ensures resume is always possible.

---

## Status Checking

Running `/mission-status` (via the mission-status skill) in a separate session should show the current state by reading `progress.md`, `tasks.md`, and `config.yaml`. This skill does not implement status -- that is handled by the `/mission-status` skill.

---

## Rules (Always Enforced)

1. **Serial execution.** One worker at a time. Never dispatch two workers in parallel. (Exception: adversarial reviewers A and B run in parallel since they are independent read-only reviewers.)
2. **Adversarial separation.** Workers never see `contract.md`. Validators never see `proposal.md` or `design.md`.
3. **All validators re-run after every fix.** Even if only one failed, all validators (scrutiny, user-test, both adversarial reviewers) re-run to catch regressions.
4. **Circuit breaker at 3.** After 3 failed attempts on the same task, stop and ask the human. Retry allows one more attempt but does NOT reset the counter.
5. **File-backed state.** All decisions and status go to disk. The mission must be resumable from files alone.
6. **Single commit per task.** One task, one worker, one commit, one validation cycle.
7. **Draft PRs only.** Never create a ready-for-review PR. The human promotes it.
8. **Plan is the operating manual.** Keep the plan updated after every phase and step. It must be accurate if context compacts.
9. **Skipped tasks use `- [~]`.** The build loop treats `- [~]` as neither unchecked nor complete. Skipped tasks are not re-attempted.
10. **Monitor is for external processes.** Monitor is used for external process dispatch (Codex workers, Codex adversarial reviews). Claude subagent validators use the Agent tool directly since they run within the session and return synchronously.
