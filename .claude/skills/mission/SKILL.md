---
name: mission
description: Start an autonomous Darkfactory mission. Grills the user for alignment, creates OpenSpec artifacts, writes a validation contract, and sets up the mission directory. Use when the user wants to start a multi-task autonomous build cycle.
---

# Darkfactory Mission

You are the **Orchestrator**. You guide the user from a goal description through the full spec phase, producing all artifacts needed for autonomous implementation. If UBIQUITOUS_LANGUAGE.md exists, read it now and use those terms precisely. If it doesn't exist yet, read it after Phase 2 of this document and use those terms for all remaining phases.

**Input**: The argument after `/mission` is the goal description. If none provided, ask what they want to build.

---

## Phase 0: Enter Plan Mode

**Do this immediately, before anything else.**

Enter Claude Code plan mode and write the orchestrator operating manual. This plan survives context compaction -- it is your durable reference for the entire mission. You will update it as artifacts are created.

Write this initial plan:

```
# Mission: [goal summary]

## Status
Phase: spec (idea -> SPEC -> build -> verify -> ship -> merge)
Step: grill

## State Directory
(will be set after OpenSpec change is created)

## OpenSpec Artifacts
(will be populated after /opsx:propose)

## Engine Config
(will be populated after mission directory is created)

## Skills to Use
- Alignment: /grill-me (during planning, before artifacts are created)
- Shared language: /ubiquitous-language (during planning, establish terms)
- Planning: /opsx:propose (create proposal, design, tasks)
- Task breakdown: follow /prd-to-plan tracer bullet pattern -- each task is a thin vertical slice through all layers, demoable and verifiable on its own
- Contract: written by orchestrator after planning, before workers start
- Worker briefs: assembled from proposal.md + design.md + task description
- Scrutiny validation: build/test/lint + /review + /code-simplify against contract
- Security validation: /security-review (for tasks touching auth/sensitive paths)
- User-test validation: /test-app (agent-browser) and/or HTTP tests against API
- Demo video: /demo-video (for frontend tasks)
- Completion: /pr (create draft PR with mission summary)
- Archival: /opsx:archive (after merge)

## Mission Flow
For each unchecked task in tasks.md:
1. Read mission state from mission/progress.md
2. Write brief to mission/briefs/task-N.md
3. Kick off worker via Monitor (engine per config.yaml)
4. On completion, read mission/handoffs/task-N.md
5. Spawn scrutiny validator (commit SHA + contract assertions)
6. If fail -> write to mission/issues/, spawn fix worker, re-validate. Circuit breaker: 3 attempts -> ask human.
7. Spawn user-test validator (commit SHA + contract assertions)
8. If fail -> same fix/re-validate loop
9. If frontend task -> spawn demo video
10. Mark task complete in tasks.md, update progress.md

## Rules
- One worker at a time (serial execution)
- Workers never see contract.md (adversarial separation)
- Validators never see proposal.md or design.md
- On any failure, re-run BOTH validators after fix
- Write all decisions to progress.md
```

---

## Phase 1: Grill for Alignment

Run `/grill-me` on the user's goal. Interview them relentlessly until you reach shared understanding. This is the most important step -- the quality of everything downstream depends on it.

Focus on:
- What exactly they want built
- What "done" looks like from a user's perspective
- Edge cases and error scenarios
- What is explicitly NOT in scope
- Integration points with existing code
- Technical constraints or preferences

**If the user asks to skip:** Warn that skipping the grill reduces the quality of the validation contract, which reduces the quality of autonomous validation. Allow it if they insist.

When alignment is reached, proceed to Phase 2.

---

## Phase 2: Establish Shared Language

Run `/ubiquitous-language` to extract domain terms from the grill conversation. This creates or updates UBIQUITOUS_LANGUAGE.md.

The shared language improves:
- How you write the proposal and contract (precise terms)
- How workers understand their briefs
- How validators interpret assertions

Proceed to Phase 3.

---

## Phase 3: Create OpenSpec Artifacts

Run `/opsx:propose` with a kebab-case name derived from the goal. This creates:
- `proposal.md` -- what and why
- `design.md` -- how (architecture decisions)
- `tasks.md` -- implementation checklist

**Critical guidance for tasks.md:** Each task MUST be a vertical slice following the tracer bullet pattern from /prd-to-plan:
- Each task delivers a narrow but COMPLETE path through every layer (schema, API, UI, tests)
- A completed task is demoable or verifiable on its own
- Prefer many thin tasks over few thick ones
- Do NOT include specific file names or function names that are likely to change
- DO include durable decisions: route paths, schema shapes, data model names
- One task = one worker = one commit = one validation cycle

After `/opsx:propose` completes, update the plan with the state directory and artifact paths:

```
## State Directory
openspec/changes/<change-name>/mission/

## OpenSpec Artifacts
- Proposal: openspec/changes/<change-name>/proposal.md
- Design: openspec/changes/<change-name>/design.md
- Tasks: openspec/changes/<change-name>/tasks.md
- Contract: openspec/changes/<change-name>/contract.md
```

Proceed to Phase 4.

---

## Phase 4: Write the Validation Contract

Create `contract.md` in the change directory (alongside proposal.md, design.md, tasks.md).

The contract defines "done" independently of implementation. Validators use it as their sole definition of correctness. Workers never see it.

**Format:**

```markdown
# Validation Contract: [change name]

## Task 1: [task name from tasks.md]
- [Plain language assertion verifiable from outside the code]
- [Another assertion]
- [Each assertion should be checkable by running tests, hitting APIs, or driving a browser]

## Task 2: [task name]
- [Assertion]
- [Assertion]

(one section per task in tasks.md)
```

**Rules for writing assertions:**
- Describe observable behavior, not implementation details
- A validator with no knowledge of the code should be able to check each assertion
- Cover the happy path AND error cases from the grill conversation
- Be specific enough to be unambiguous ("user sees an error message" is bad, "submitting an empty name shows 'Name is required'" is good)
- Each task must have at least one assertion
- The sum of all assertions across all tasks defines the complete "done" criteria

Proceed to Phase 5.

---

## Phase 5: Create Mission Directory

Create the mission directory structure at `openspec/changes/<change-name>/mission/`:

```
mission/
  config.yaml
  progress.md
  briefs/
  handoffs/
  validations/
  issues/
```

**config.yaml contents:**

```yaml
engines:
  worker: codex
  scrutiny_validator: claude
  usertest_validator: claude
  demo_video: claude
```

**progress.md contents:**

```markdown
# Mission: [change name]
Started: [timestamp]

## Log
- [timestamp] Plan approved. Spec phase complete. Ready for build phase.
```

Create the subdirectories: `briefs/`, `handoffs/`, `validations/`, `issues/`.

Update the plan with the engine config:

```
## Engine Config
- Workers: codex (gpt-5.4, high effort)
- Validators: claude (subagents)
- Demo video: claude (subagent)
```

Proceed to Phase 6.

---

## Phase 6: Present for Approval

Show the user a summary of everything created:

1. **Tasks** -- list each task from tasks.md with a one-line description
2. **Contract** -- list the assertion count per task
3. **Engine config** -- which engine handles each role
4. **Mission directory** -- confirm it's created and ready

Ask the user:
- Does the task breakdown look right? Too coarse? Too fine?
- Are the contract assertions comprehensive enough?
- Any changes to the engine config?

**If the user requests changes:** Update the relevant artifacts and re-present.

**If the user approves:** Record approval in progress.md and update the plan status:

```
## Status
Phase: build
Current task: 1
```

Proceed to Phase 7.

---

## Phase 7: Build Phase -- Worker Loop

This is the autonomous execution loop. For each unchecked task in tasks.md, the orchestrator assembles a brief, dispatches a worker, reads the handoff, and tracks progress.

**Before starting, read `mission/config.yaml` to determine the worker engine.**

### For each unchecked task in tasks.md:

**Step 1: Assemble the brief.**

Read the task description from tasks.md. Read proposal.md and design.md. Extract the sections relevant to this specific task -- don't dump entire files.

Write the brief to `mission/briefs/task-N.md` using this format:

```markdown
# Worker Brief: Task N - [task name]

## Your Task
[Task description from tasks.md with enough context to be self-contained]

## Project Context
[Relevant excerpt from proposal.md -- the what and why]

## Architecture
[Relevant excerpt from design.md -- technical decisions relevant to this task]

## Procedures
- Implement the task as described
- Commit all changes in a single commit with message: "[task-N] description"
- Write your handoff report to mission/handoffs/task-N.md using the format below

## Handoff Report Format
- Task: [which task]
- Commit SHA: [the full SHA]
- Approach: [brief explanation of what you did and why]
- Commands run: [build/test/lint commands and their exit codes]
- Concerns: [anything you're uncertain about]
```

**CRITICAL: The brief must NEVER contain content from contract.md.** Workers and validators are adversarially separated.

**Step 2: Dispatch the worker.**

Check the engine in `mission/config.yaml`.

**If `worker: codex`:** Use the Monitor tool to kick off Codex in the background and watch for the handoff file.

Monitor command:
```bash
BRIEF_PATH="[path to mission/briefs/task-N.md]"
HANDOFF_PATH="[path to mission/handoffs/task-N.md]"

# Kick off Codex in background
codex exec --full-auto --sandbox workspace-write \
  -m gpt-5.4 -c model_reasoning_effort="high" \
  -o "$HANDOFF_PATH" \
  "Read your brief at $BRIEF_PATH and follow its instructions exactly. Implement the task, commit in a single commit with the message format specified, and write your handoff report." \
  2>/dev/null &
CODEX_PID=$!

# Watch for completion or failure
while kill -0 $CODEX_PID 2>/dev/null; do
  if [ -f "$HANDOFF_PATH" ]; then
    echo "TASK_COMPLETE: handoff written at $HANDOFF_PATH"
    break
  fi
  sleep 2
done

# Check if process exited
wait $CODEX_PID 2>/dev/null
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ] && [ ! -f "$HANDOFF_PATH" ]; then
  echo "TASK_FAILED: codex exited with code $EXIT_CODE"
elif [ ! -f "$HANDOFF_PATH" ]; then
  echo "TASK_FAILED: codex exited but no handoff file written"
fi
```

Monitor description: `"Worker: task N - [task name]"`
Monitor timeout: 600000 (10 minutes)

**If `worker: claude`:** Use the Agent tool to spawn a Claude subagent. Pass the full brief content as the prompt. Instruct the subagent to write the handoff report to the same file path. Read the handoff file after the subagent returns.

**Step 3: Handle the worker result.**

When the Monitor fires (or the Agent returns):

- **If TASK_COMPLETE:** Read `mission/handoffs/task-N.md`. Extract the commit SHA. Proceed to Step 4.
- **If TASK_FAILED:** Log the failure to `mission/progress.md` with the error details. Ask the human for guidance (retry, skip, or abort).
- **If timeout:** Log the timeout to `mission/progress.md`. Ask the human for guidance.

**Step 4: Run scrutiny validation.**

Spawn a Claude subagent via the Agent tool. The subagent is the **Scrutiny Validator**. Its prompt must contain:

1. The commit SHA from the worker handoff
2. The contract assertions for this task (read from contract.md, the section matching this task)
3. The following instructions:

```
You are the Scrutiny Validator. You are checking whether a worker's implementation
satisfies the validation contract. You have NOT seen this code before. You do not
know how or why it was built -- only what it should do.

## Your inputs
- Commit SHA: [SHA]
- Contract assertions for this task:
  [paste the assertions from contract.md for this task]

## Checks to run (in order)

1. Run `git show [SHA]` to see the full diff
2. Run the project's build command. Record pass/fail and exit code.
3. Run the project's test command. Record pass/fail and exit code.
4. Run the project's lint command. Record pass/fail and exit code.
5. Run /review against the diff. Record findings by severity.
6. Run /code-simplify against the diff. Record any simplification findings.
7. If the diff touches auth, sessions, credentials, or security-sensitive code:
   run /security-review against the diff. Record findings.
8. For each contract assertion: determine PASS or FAIL.
   - PASS: explain briefly how the code satisfies the assertion
   - FAIL: explain specifically what is missing or broken

## Write your report

Write your report to: mission/validations/task-N-scrutiny.md

Format:
  # Scrutiny Validation: Task N - [name]
  
  ## Overall: PASS or FAIL
  
  ## Build/Test/Lint
  - Build: [PASS/FAIL] (exit code: [N])
  - Test: [PASS/FAIL] (exit code: [N])
  - Lint: [PASS/FAIL] (exit code: [N])
  
  ## Code Review
  [/review findings by severity]
  
  ## Simplicity
  [/code-simplify findings]
  
  ## Security (if applicable)
  [/security-review findings]
  
  ## Contract Assertions
  - [Assertion 1]: PASS/FAIL - [evidence]
  - [Assertion 2]: PASS/FAIL - [evidence]
  
  ## Summary
  [1-2 sentence summary of the overall result]
```

**CRITICAL:** The validator prompt must NOT contain the proposal, design, worker brief, or worker's reasoning. Only the commit SHA, contract assertions, and check instructions.

**Step 5: Handle the validation result.**

Read `mission/validations/task-N-scrutiny.md`.

- **If overall PASS:** Proceed to Step 6 (user-test validation, when available -- for now, skip to Step 7).
- **If overall FAIL:** Enter the fix loop (Step 5a).

**Step 5a: Fix loop.**

Track the attempt number. This is attempt 1 if it's the first failure, 2 after the first fix, etc.

1. Write failure details to `mission/issues/task-N-v[attempt].md` containing:
   - Which assertions failed and why
   - Build/test/lint failures
   - Review findings rated Critical or Important

2. If attempt >= 3: **Circuit breaker.** Log all failures to progress.md and ask the human:
   - **Retry:** Reset attempt counter and try again
   - **Skip:** Mark task as skipped in progress.md, move to next task
   - **Abort:** Stop the build phase, log "Mission paused by human"

3. If attempt < 3: Assemble a fix brief and dispatch a fix worker.

   Write the fix brief to `mission/briefs/task-N-fix-v[attempt+1].md`:

   ```markdown
   # Fix Brief: Task N - [name] (Attempt [attempt+1])

   ## What Failed
   [Specific failed assertions from the validator report]

   ## Validator Output
   [Relevant error messages, test failures, review findings]

   ## Commit to Fix
   [SHA of the broken commit]

   ## Procedures
   - Fix only the issues described above
   - Do not refactor or change anything unrelated to the failures
   - Commit as a single commit with message: "[task-N] fix: description"
   - Write your handoff report to mission/handoffs/task-N-fix-v[attempt+1].md
   ```

   Dispatch the fix worker using the same mechanism as Step 2 (Codex or Claude per config). Wait for completion via Monitor.

   After the fix worker completes: go back to **Step 4** (re-run scrutiny validation with the new commit SHA). Both validators must re-run after every fix.

**Step 6: Run user-test validation.** (placeholder -- implemented in change: usertest-validation)

When the user-test validator is available, spawn it here with the commit SHA and contract assertions. On failure, enter the fix loop (Step 5a) with the user-test failures. For now, skip to Step 7.

**Step 7: Track progress.**

1. Mark the task complete in tasks.md: change `- [ ]` to `- [x]` for this task
2. Append to `mission/progress.md`:
   ```
   - [timestamp] Task N complete: [task name]. Commit: [short SHA]. Scrutiny: PASS. Worker: [codex/claude].
   ```
3. Update the plan:
   ```
   ## Status
   Phase: build
   Current task: [next task number]
   Completed: [list of completed task numbers]
   ```
4. Check if unchecked tasks remain in tasks.md:
   - **If yes:** Go back to Step 1 for the next unchecked task.
   - **If no:** All tasks complete. Log to progress.md:
     ```
     - [timestamp] Build phase complete. All tasks implemented and validated.
     ```
     Update the plan status:
     ```
     ## Status
     Phase: build COMPLETE
     All tasks implemented and validated.
     ```

The build phase is complete.
