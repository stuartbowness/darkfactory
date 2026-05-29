---
name: mission-resume
description: Resume an interrupted Darkfactory mission from file state. Finds active missions, restores the orchestrator plan, detects the orchestrator phase (from the plan file or progress.md markers), detects branch-scoped orphaned commits, and continues the build loop from the next unchecked task. Stops and defers to /mission when the interruption happened during a planning phase.
---

# Mission Resume

Resume a Darkfactory mission that was interrupted (laptop sleep, terminal close, crash, context compaction). All mission state lives on disk. This skill reads it, rebuilds the orchestrator plan, and picks up where the previous session left off.

## Phase 1: Find Active Missions

Search for OpenSpec changes that have an active mission (unchecked tasks remaining).

1. Glob for `openspec/changes/*/mission/config.yaml` to find all mission directories.
2. For each match, read the corresponding `openspec/changes/<change-name>/tasks.md`.
3. A mission is **active** if `tasks.md` contains at least one unchecked item (`- [ ]`). Tasks marked `- [~]` (skipped) are not considered unchecked.
4. Collect the list of active missions with their change name and progress count (checked vs total tasks).

### Handle results

- **No active missions found:** Report "No active missions found." and stop.
- **Exactly one active mission:** Use it. Report "Resuming mission: <change-name>".
- **Multiple active missions:** List each one with its progress. Ask the user to pick one.

## Phase 2: Read Mission State

Read all artifacts for the selected mission:

| File | Path | Purpose |
|------|------|---------|
| Tasks | `openspec/changes/<change-name>/tasks.md` | Which tasks are done, which remain |
| Progress log | `mission/progress.md` | Chronological record of what happened |
| Mission config | `mission/config.yaml` | Engine assignments, `base_sha`, `plan_file` |
| Plan file | path from `config.yaml` `plan_file` | Authoritative state: orchestrator phase, current task/step, engine config, codebase deltas |
| Contract | `contract.md` | Checks grouped by task for validators |
| Proposal | `proposal.md` | What is being built and why |
| Design | `design.md` | Architecture decisions |

Read `plan_file` from `config.yaml`. If the field is present and the file exists, read that plan -- it is the authoritative record of where the mission stopped. If `plan_file` is missing (a mission started before this field existed), fall back to locating the plan by content: `ls ~/.claude/plans/*.md 2>/dev/null | xargs grep -l "Mission: <change-name>" 2>/dev/null | head -1`. If no plan file is found by either path, note it -- Phase 4 will write one from disk state.

Also read existing handoff files in `mission/handoffs/` and validation reports in `mission/validations/` (including `task-N-scrutiny.md`, `task-N-usertest.md`). These reveal mid-cycle interruptions.

## Phase 3: Detect Mid-Cycle State and Orphaned Commits

Check each unchecked task for partial completion. A task can be interrupted at various points. The resume skill detects where it stopped and resumes from that point.

For each unchecked task (skip `- [~]` skipped tasks):

### 3a: Check for handoff with missing validations

If `mission/handoffs/task-N.md` exists (worker finished), check which validation reports exist:

| Handoff | Scrutiny Report | User-test Report | Resume From |
|---------|----------------|-----------------|-------------|
| exists | missing | missing | Step 4 (scrutiny validation) |
| exists | exists (FAIL) | missing | Step 5a (fix loop with scrutiny failure) |
| exists | exists (PASS) | missing | Step 6 (user-test validation) |
| exists | exists (PASS) | exists (FAIL) | Step 5a (fix loop with user-test failure) |
| exists | exists (PASS) | exists (PASS) | Step 6a (demo video) then mark complete |

Log to progress.md: `<timestamp> -- Task N: resuming from <step name>. Previous session interrupted mid-cycle.`

### 3b: Detect orphaned commits

An orphaned commit is one where a worker committed code but no handoff file exists. This happens when the session died between the worker's commit and the orchestrator recording the result.

First, scope the search to this mission's commits. Read the pinned `base_sha` from `config.yaml`:

```bash
BASE_SHA=$(awk -F: '/^[[:space:]]*base_sha[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' openspec/changes/<change-name>/mission/config.yaml)
```

If `base_sha` is missing or not a valid commit (`git cat-file -e "${BASE_SHA}^{commit}"` fails), **stop and ask the user** which commit this mission branched from. Do not fall back to a repo-wide search -- that returns false positives from older missions on other branches.

For each unchecked task that did NOT have a handoff in step 3a:

1. Run: `git log "$BASE_SHA"..HEAD --oneline --grep="\[task-N\]" -1` (commits on this mission branch only).
2. If a commit is found but no handoff exists, report it to the human.
3. Ask: "This task has a commit but no handoff. Should I validate it (run full validation against this commit), or re-run the task from scratch?"

If validate: write a synthetic handoff file with the commit SHA, then run the full validation cycle (scrutiny + user-test). The human is confirming "the code exists," not "the code is correct."

If re-run: leave the task unchecked and start fresh.

### 3c: Detect the orchestrator phase

Run this **once for the whole mission**, not per task (unlike 3a/3b). Phases 3a and 3b only cover missions where a worker has run. A mission interrupted during planning (Phase 7 or earlier) has no handoffs and no task commits, so without this step Phase 4 would wrongly assume the build loop is ready to start. Determine which orchestrator phase (0-9, 4a) was active when the session ended, and the **resume target** (the phase to re-enter from).

**Primary source -- the plan file.** If the plan read in Phase 2 has a populated `## Orchestrator Phase` field, use it. That field names the phase, step, and last-checkpoint timestamp. This is authoritative.

**Fallback -- progress.md markers.** If no plan file was found, or its `## Orchestrator Phase` field is empty (a mission started before this field existed):

- **No `mission/progress.md` file:** the mission never reached Phase 6 (progress.md is created there). It stopped during planning, Phase 5 or earlier. Resume target: report and ask (see Phase 4).
- **progress.md exists:** scan it for these markers, taking the most recent (last) match. Every progress.md contains at least `Mission directory created.` (its first line), so one row always matches:

| progress.md marker | Phase reached | Resume target |
|---------------------|----------------|-------------|
| `Draft PR created` | Phase 9 (PR done) | Phase 7 (completion) of this skill |
| highest `Task N complete.` | Phase 8, Task N done | Phase 8, Task N+1 |
| `Plan approved.` | planning done, build ready | Phase 8, Task 1 |
| `Mission directory created.` | Phase 6 done, build not started | Phase 7 (launch confirmation) |

Report the detected phase and resume target to the user before continuing. Carry both into Phase 4.

## Phase 4: Restore the Orchestrator Plan

Enter Claude Code **plan mode**. The goal is an accurate operating manual that survives context compaction. Whether you restore or rebuild depends on what Phase 2 found.

### If a plan file exists (restore + reconcile)

Restore the plan from the existing file rather than re-deriving it. It already holds the orchestrator phase, current task/step, engine config, artifact paths, PRD issue number, and codebase deltas.

Before trusting it, reconcile its state against disk reality:

1. **Task completion.** Compare the plan's `## Current Task` against the count of `- [x]` tasks in `tasks.md` and the highest `Task N complete.` in `progress.md`. If they diverge, the plan is stale -- update Current Task to the true next unchecked task and note the correction.
2. **Orchestrator phase.** Cross-check the plan's `## Orchestrator Phase` against the phase detected in Phase 3c. If they disagree, trust the Phase 3c detection (it is computed from current disk state) and update the field.

### If no plan file exists (rebuild)

Write a complete orchestrator operating manual from file state. Include: mission identity, all file paths, engine configuration, the detected `## Orchestrator Phase` (from Phase 3c), the mission flow (matching the /mission skill's Phase 8 build loop and Phase 9 completion), and the rules.

### Branch on the resume target

Use the **resume target** from Phase 3c, not the raw phase label. The boundary is whether the build loop is ready to run -- which is true once `Plan approved.` was logged (Phase 7 finished), even though that is still "Phase 7" by name.

- **Build loop is ready** -- resume target is Phase 8 (Task 1 or later) or this skill's Phase 7 completion. This covers any mission where `Plan approved.` was logged or tasks are already complete. Continue to Phase 5, then the build loop.
- **Planning never finished** -- resume target is launch confirmation (Phase 7) or earlier, i.e. `Plan approved.` was never logged. Do **not** enter the build loop. Report the detected phase to the user, for example: "This mission stopped in Phase 5 (contract writing), before the build loop started. Resuming planning needs `/mission`, which holds the planning-phase logic. Run `/mission` to continue from Phase 5, or tell me how you want to proceed." Then stop and wait. mission-resume does not contain planning-phase logic.

## Phase 5: Log the Resume

Append to `mission/progress.md`:

```
<timestamp> -- Mission resumed in new session. Picking up from task <N>.
Previously completed: tasks <list of completed task numbers>.
```

## Phase 6: Continue the Build Loop

Reached only when Phase 4 detected a build phase (Phase 8 or later). Planning-phase resumes already stopped in Phase 4.

Follow the `/mission` skill's Phase 8 build loop exactly -- every step, every sub-step, every rule. Do not re-implement the loop here; read Phase 8 in `.claude/skills/mission/SKILL.md` and execute it as written.

Start from the entry point established earlier: if Phase 3a detected a mid-cycle task, resume from the appropriate step in `/mission` Phase 8; otherwise start the next unchecked task (Task N+1 per Phase 3c) at Step 1.

## Phase 7: Mission Completion

When all tasks are checked in tasks.md:

1. Create a draft PR using `gh pr create --draft` with a mission summary (tasks, validation results, issues, stats).
2. Run dual adversarial review against the full diff (`git diff <base-branch>...HEAD`) following `/mission` Phase 9's Dual Adversarial Review section:
   - Reviewer A (engine from config.yaml, default codex) dispatched first via Bash with `run_in_background: true` (returns a shell task ID)
   - Reviewer B (engine from config.yaml, default claude) dispatched via Agent tool, which blocks until it returns
   - When B returns, call BashOutput on A's shell task ID to retrieve its terminal line
   - Both receive the full diff and the full contract
   - Reports written to `mission/validations/adversarial-a.md` and `adversarial-b.md`
3. Present both reports to the human alongside the draft PR link.
4. Log to progress.md and update plan status to COMPLETE.

The adversarial review is a human-facing report, not an automated gate. The human decides what to address.
