---
name: mission-resume
description: Resume an interrupted Darkfactory mission from file state. Reconstructs the orchestrator plan and continues the build loop from the next unchecked task.
---

# Resume Mission

You are the **Orchestrator** resuming an interrupted mission. Read UBIQUITOUS_LANGUAGE.md if it exists and use those terms precisely.

---

## Step 1: Find the active mission

Search for OpenSpec changes that have a `mission/` directory with unchecked tasks:

```bash
# Find changes with mission directories
ls openspec/changes/*/mission/config.yaml 2>/dev/null
```

For each match, read the corresponding `tasks.md` and check for unchecked tasks (`- [ ]`).

- **If no active missions found:** Report "No active missions found." and stop.
- **If exactly one active mission:** Use it. Report: "Resuming mission: [change name]"
- **If multiple active missions:** List them with their progress (N/M tasks complete) and ask the user to pick one.

---

## Step 2: Read mission state

Read these files from the active change directory:

1. `tasks.md` -- which tasks are done, which is next
2. `mission/progress.md` -- what happened so far
3. `mission/config.yaml` -- engine settings
4. `contract.md` -- validation assertions (for validators)
5. `proposal.md` -- project context
6. `design.md` -- architecture decisions

---

## Step 3: Check for orphaned commits

An orphaned commit is one where the worker committed but didn't write a handoff (session died between commit and handoff).

For each unchecked task, check git log for commits matching the pattern `[task-N]`:

```bash
git log --oneline --grep="\[task-N\]" -1
```

- **If a matching commit exists but no handoff file:** Ask the human: "Found a commit for task N ([short SHA]) but no handoff report. Should I count this as complete, or re-run the task?"
  - If count as complete: mark the task done in tasks.md, log to progress.md, move on
  - If re-run: proceed as if the task hasn't started

---

## Step 4: Reconstruct the plan

Enter plan mode and write the orchestrator operating manual, reflecting the current state:

```
# Mission: [change name] (RESUMED)

## Status
Phase: build (resumed)
Current task: [next unchecked task number]
Completed: [list of completed task numbers]

## State Directory
openspec/changes/[change-name]/mission/

## OpenSpec Artifacts
- Proposal: openspec/changes/[change-name]/proposal.md
- Design: openspec/changes/[change-name]/design.md
- Tasks: openspec/changes/[change-name]/tasks.md
- Contract: openspec/changes/[change-name]/contract.md

## Engine Config
[read from mission/config.yaml]

## Skills to Use
- Alignment: /grill-me (during planning, before artifacts are created)
- Shared language: /ubiquitous-language (during planning, establish terms)
- Planning: /opsx:propose (create proposal, design, tasks)
- Task breakdown: follow /prd-to-plan tracer bullet pattern
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

## Step 5: Log the resume

Append to `mission/progress.md`:
```
- [timestamp] Mission resumed in new session. Picking up from task [N].
```

---

## Step 6: Continue the build loop

Follow Phase 7 from the /mission skill (the build phase worker loop), starting from the next unchecked task. The flow is identical to a normal mission -- brief, dispatch, validate, track, iterate.
