---
name: mission-resume
description: Resume an interrupted Darkfactory mission from file state. Finds active missions, reconstructs the orchestrator plan from disk artifacts, detects orphaned commits, and continues the build loop from the next unchecked task.
---

# Mission Resume

Resume a Darkfactory mission that was interrupted (laptop sleep, terminal close, crash, context compaction). All mission state lives on disk. This skill reads it, rebuilds the orchestrator plan, and picks up where the previous session left off.

## Phase 1: Find Active Missions

Search for OpenSpec changes that have an active mission (unchecked tasks remaining).

1. Glob for `openspec/changes/*/mission/config.yaml` to find all mission directories.
2. For each match, read the corresponding `openspec/changes/<change-name>/tasks.md`.
3. A mission is **active** if `tasks.md` contains at least one unchecked item (`- [ ]`).
4. Collect the list of active missions with their change name and progress count (checked vs total tasks).

### Handle results

- **No active missions found:** Report "No active missions found." and stop. Do not proceed to Phase 2.
- **Exactly one active mission:** Use it. Report "Resuming mission: <change-name>".
- **Multiple active missions:** List each one with its progress (e.g., "darkfactory-v2: 1/3 tasks complete"). Ask the user to pick which mission to resume. Wait for their answer before proceeding.

## Phase 2: Read Mission State

Read all artifacts for the selected mission. The change directory is `openspec/changes/<change-name>/` and the mission directory is `openspec/changes/<change-name>/mission/`.

Read these files:

| File | Path | Purpose |
|------|------|---------|
| Tasks | `openspec/changes/<change-name>/tasks.md` | Which tasks are done, which remain |
| Progress log | `mission/progress.md` | Chronological record of what happened |
| Mission config | `mission/config.yaml` | Engine assignments (worker, validators) |
| Contract | `contract.md` | Assertions grouped by task for validators |
| Proposal | `proposal.md` | What is being built and why |
| Design | `design.md` | Architecture decisions |

Also read all existing handoff files in `mission/handoffs/` and all validation reports in `mission/validations/`. These tell you the outcome of completed tasks.

## Phase 3: Detect Orphaned Commits

An orphaned commit is one where a worker committed code (matching the `[task-N]` pattern in the commit message) but no handoff file exists at `mission/handoffs/task-N.md`. This happens when the session died between the worker's commit and the orchestrator recording the result.

For each unchecked task in tasks.md:

1. Run: `git log --oneline --grep="\[task-N\]" -1`
2. If a commit is found but `mission/handoffs/task-N.md` does **not** exist, this is an orphaned commit.
3. Report the orphaned commit to the human: show the commit SHA, message, and the task description.
4. Ask: "This task has a commit but no handoff. Should I count it as done, or re-run the task?"
5. Wait for the human's answer before proceeding.

If the human says count as done:
- Mark the task as checked in tasks.md (`- [x]`).
- Append to progress.md: "Task N marked complete from orphaned commit <SHA> (human confirmed)."

If the human says re-run:
- Leave the task unchecked.
- Append to progress.md: "Task N orphaned commit <SHA> found but human requested re-run."

## Phase 4: Reconstruct the Orchestrator Plan

Enter Claude Code **plan mode**. Write a complete orchestrator operating manual that captures the full mission state and rules. This plan must survive context compaction -- it is the orchestrator's durable memory.

The plan must include:

### Mission identity
- Change name, proposal summary, design summary
- Total tasks and their descriptions
- Which tasks are complete (with commit SHAs from handoffs)
- Which task is next

### File paths
- Change directory: `openspec/changes/<change-name>/`
- Mission directory: `openspec/changes/<change-name>/mission/`
- Tasks file: `openspec/changes/<change-name>/tasks.md`
- Contract file: `openspec/changes/<change-name>/contract.md`
- Progress log: `openspec/changes/<change-name>/mission/progress.md`
- Config: `openspec/changes/<change-name>/mission/config.yaml`
- Briefs directory: `openspec/changes/<change-name>/mission/briefs/`
- Handoffs directory: `openspec/changes/<change-name>/mission/handoffs/`
- Validations directory: `openspec/changes/<change-name>/mission/validations/`
- Issues directory: `openspec/changes/<change-name>/mission/issues/`

### Engine configuration
- Read `config.yaml` and record which engine is assigned to each role (worker, scrutiny_validator, usertest_validator, demo_video).

### Orchestrator rules
- Serial execution: only one worker at a time
- Workers never see the contract (adversarial separation)
- Validators never see the proposal or design
- Each task maps to one worker invocation, one commit, one validation cycle
- Fix loop: up to 3 attempts per task before the circuit breaker asks the human
- Progress log must be updated after every significant event

## Phase 5: Log the Resume

Append to `mission/progress.md`:

```
## Mission Resumed

Mission resumed in new session. Picking up from task <N>.
Previously completed: tasks <list of completed task numbers>.
```

## Phase 6: Continue the Build Loop

For each remaining unchecked task, execute the following sequence. Process tasks in order -- serial execution, one at a time.

### Step 1: Assemble the brief

Check if a brief already exists at `mission/briefs/task-N.md`.
- If it exists, read it and use it.
- If it does not exist, assemble a new brief containing:
  - Task description from tasks.md
  - Project context (what the codebase does, relevant architecture)
  - Relevant excerpts from the design and proposal
  - Procedures: what to build, how to commit, how to write the handoff
  - Handoff report format (task name, commit SHA, approach, commands run, concerns)

Write the brief to `mission/briefs/task-N.md`.

**The brief must NOT include any content from the contract.** Workers never see the contract.

### Step 2: Dispatch the worker

Read `config.yaml` to determine the worker engine.

**If engine is `codex`:** Use the Monitor tool to dispatch a Codex worker and watch for the handoff file.

```bash
codex exec --approval-mode full-auto "You are a Worker in the Darkfactory mission system. Read your brief and follow its instructions exactly. Your brief is at: <absolute-path-to-brief>"
```

Monitor watches for: `mission/handoffs/task-N.md`

**If engine is `claude`:** Use a Claude subagent via the Agent tool. Pass the same instruction: read the brief, follow it exactly.

After the worker finishes, verify:
1. The handoff file exists at `mission/handoffs/task-N.md`
2. A commit with `[task-N]` in the message exists in git log

If either is missing, log the failure and ask the human for guidance.

### Step 3: Run scrutiny validation

Spawn a Claude subagent as the scrutiny validator. The validator receives:
- The contract assertions for this task (from contract.md)
- The commit diff (`git diff HEAD~1`)

The validator does NOT receive the proposal, design, or brief.

The scrutiny validator runs this check sequence:
1. Read the contract assertions for this task
2. Check each assertion against the committed code
3. Run any applicable automated checks (build, test, lint)
4. Run code review: does the code look correct, clean, and complete?
5. Run code simplify: is there unnecessary complexity?
6. Write the validation report to `mission/validations/task-N-scrutiny.md`

The report must state PASS or FAIL, list each assertion checked, and explain any failures.

### Step 4: Run user-test validation

Spawn a Claude subagent as the user-test validator. The validator receives:
- The contract assertions for this task
- The commit diff

The validator does NOT receive the proposal, design, or brief.

The user-test validator decides a test strategy:
- If the task involves an API: write and run HTTP tests
- If the task involves a UI: use agent-browser to test behavior
- If the task is purely structural (e.g., markdown generation): verify file existence, structure, and content

Write the validation report to `mission/validations/task-N-usertest.md`. The report must state PASS or FAIL with details.

### Step 5: Handle validation results

**If both validators pass:** Mark the task as checked in tasks.md (`- [x]`). Append to progress.md: "Task N passed scrutiny and user-test validation." Move to the next task.

**If either validator fails:** Enter the fix loop.

### Step 6: Fix loop

When validation fails:

1. Read the validation report(s) that failed.
2. Assemble a fix brief at `mission/briefs/task-N-fix-vM.md` (where M is the attempt number starting at 2). The fix brief contains:
   - The original task description
   - The specific validation failures (copy the relevant sections from the validation report)
   - The commit SHA of the failed attempt
   - Instructions: fix the failures, commit with message `[task-N] Fix: <description>`, write a handoff to `mission/handoffs/task-N-fix-vM.md`
3. Dispatch a fix worker using the same engine as the original worker.
4. After the fix worker completes, re-run both validators (scrutiny and user-test).
5. If both pass, mark the task as done and continue.
6. If either fails again, increment M and repeat from step 1.

**Circuit breaker:** After 3 failed fix attempts on the same task, stop the fix loop. Log the situation to progress.md. Report to the human: "Task N has failed validation 3 times. Here are the failures: [summary]. What would you like to do?" Wait for human guidance.

### Step 7: Update progress

After each task completes (or fails and hits the circuit breaker), append the outcome to `mission/progress.md` with:
- Task number and description
- Pass/fail status
- Commit SHA
- Number of fix attempts if any
- Timestamp

## Phase 7: Mission Completion

When all tasks are checked in tasks.md:

1. Append to progress.md: "All tasks complete. Mission finished."
2. Create a draft PR using the `gh` CLI. The PR body should summarize:
   - What the mission built
   - How many tasks, how many fix loops
   - Links to handoff files
3. Report to the human: "Mission complete. Draft PR created: <URL>"
