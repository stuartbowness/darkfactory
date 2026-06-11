---
name: mission-resume
description: Resume an interrupted Darkfactory mission from file state. Finds active missions, restores the orchestrator plan, detects the orchestrator phase from progress.md markers, cross-checks the plan file, detects branch-scoped orphaned commits, and continues from the correct resume target. Stops and defers to /mission when the interruption happened during a planning phase.
---

# Mission Resume

Resume a Darkfactory mission that was interrupted (laptop sleep, terminal close, crash, context compaction). All mission state lives on disk. This skill reads it, rebuilds the orchestrator plan, and picks up where the previous session left off.

## Phase 1: Find Active Missions

Search for OpenSpec changes that have a resumable mission (unchecked tasks remaining, or completion not finished).

1. Glob for `openspec/changes/*/mission/config.yaml` to find all mission directories.
2. For each match, read the corresponding `openspec/changes/<change-name>/tasks.md`.
3. Determine whether each mission has **finished** (completed Phase 9). First locate its plan: read `plan_file` from `config.yaml`; if that field is missing (a mission started before it existed), fall back to the same content search Phase 2 uses: `find "$HOME/.claude/plans" -type f -name '*.md' -exec grep -l "Mission: <change-name>" {} + 2>/dev/null | head -1`.
   - If a plan is found and has a `## Status` field, that field is authoritative: the mission is finished only if the status is `COMPLETE`.
   - If no plan is found or it has no `## Status:` line (a true legacy mission), fall back to `mission/progress.md`: the mission is finished only if progress.md contains the completion log line, which `/mission` writes verbatim as `Draft PR created: <URL>. Adversarial reviews complete.` (match it case-sensitively, capital A). Do **not** consult this fallback when a plan status exists -- a `Draft PR created` line with a non-`COMPLETE` status means the mission crashed during completion, not that it finished.
4. A mission is **active** (resumable) if `tasks.md` contains at least one unchecked item (`- [ ]`; `- [~]` skipped tasks do not count), OR the mission has not finished per step 3.
5. Collect the list of active missions with their change name and progress count (checked vs total tasks), plus `completion pending` when all tasks are resolved but the mission has not finished.

### Handle results

- **No active missions found:** Report "No active missions found." and stop.
- **Exactly one active mission:** Use it. Report "Resuming mission: <change-name>".
- **Multiple active missions:** List each one with its progress or `completion pending` status. Ask the user to pick one.

## Phase 2: Read Mission State

Read all artifacts for the selected mission:

| File | Path | Purpose |
|------|------|---------|
| Tasks | `openspec/changes/<change-name>/tasks.md` | Which tasks are done, which remain |
| Progress log | `mission/progress.md` | Chronological record of what happened |
| Mission config | `mission/config.yaml` | Engine assignments, `base_sha`, `plan_file` |
| Plan file | path from `config.yaml` `plan_file` | Durable state: orchestrator phase hint, current task/step, engine config, codebase deltas |
| Contract | `contract.md` | Checks grouped by task for validators |
| Proposal | `proposal.md` | What is being built and why |
| Design | `design.md` | Architecture decisions |

Read `plan_file` from `config.yaml`. If the field is present and the file exists, read that plan. Otherwise -- the field is missing (a mission started before it existed), or it is set but the file is gone (moved or deleted) -- fall back to locating the plan by content: `find "$HOME/.claude/plans" -type f -name '*.md' -exec grep -l "Mission: <change-name>" {} + 2>/dev/null | head -1`. If no plan file is found by either path, note it -- Phase 4 will write one from disk state.

Also read existing handoff files in `mission/handoffs/` and validation reports in `mission/validations/` (including `task-N-scrutiny.md`, `task-N-usertest.md`). These reveal mid-cycle interruptions.

## Phase 3: Detect Mid-Cycle State and Orphaned Commits

First detect the whole-mission resume target. Then, only for build-phase resumes with unchecked tasks, check each unchecked task for partial completion. A task can be interrupted at various points. The resume skill detects where it stopped and resumes from that point.

### 3a: Detect the orchestrator phase

Run this **once for the whole mission**, not per task (unlike 3b and 3c). Phases 3b and 3c only cover missions where a worker has run. A mission interrupted during planning (Phase 7 or earlier) has no handoffs and no task commits, so without this step Phase 4 would wrongly assume the build loop is ready to start. Determine which orchestrator phase (0-9, 4a) was active when the session ended, and the **resume target** (the phase to re-enter from).

**Disk truth -- progress.md markers.** Always derive the phase from progress.md and tasks.md:

- **No `mission/progress.md` file:** the mission never reached Phase 6 (progress.md is created there). It stopped during planning, Phase 5 or earlier. Resume target: report and ask (see Phase 4).
- **progress.md exists:** check the markers in table order (top = most advanced phase); the **first row whose marker is present wins**. Every progress.md contains at least `Mission directory created.` (its first log entry), so the bottom row always matches as a floor:

| progress.md marker | Phase reached | Resume target |
|---------------------|----------------|-------------|
| `Draft PR created` | Phase 9 (PR created) | this skill's Phase 7 (mission completion) |
| `Task N complete.` (use the highest N present) | Phase 8, Task N done | If any unchecked `- [ ]` task remains in tasks.md: Phase 8, the first unchecked task. If none remains: this skill's Phase 7 (mission completion). |
| `Plan approved.` | planning done, build ready | Phase 8, the first unchecked task |
| `Mission directory created.` | Phase 6 done, build not started | /mission Phase 7 (launch confirmation) -- defer to /mission |

This table reports the detected phase. **Phase 4 makes the actual routing decision on concrete facts** (see "Branch on the resume target"), so it does not depend on disambiguating the "Phase 7" label here.

**Plan hint -- `## Orchestrator Phase`.** If the plan read in Phase 2 has a populated `## Orchestrator Phase` field, cross-check it against the disk-derived phase. "Populated" means the field is present, non-empty, and does not still contain the `<...>` placeholder text from the template. If the plan hint disagrees with the disk-derived phase, trust the disk-derived phase and note the correction for Phase 4.

Report the detected phase and resume target to the user before continuing. Carry both into Phase 4.

### 3b: Check for handoff with missing validations

Only run this step for a build-loop resume -- `Plan approved.` was logged AND `tasks.md` has at least one unchecked `- [ ]` task (Phase 4 branch 2's condition). Skip it otherwise: planning-phase resumes (`Plan approved.` never logged) defer to `/mission`, and completion resumes (no unchecked tasks) have no next task to reconcile.

For each unchecked task (skip `- [~]` skipped tasks), if `mission/handoffs/task-N.md` exists (worker finished), check which validation reports exist:

| Handoff | Scrutiny Report | User-test Report | Resume From |
|---------|----------------|-----------------|-------------|
| exists | missing | missing | Step 4 (scrutiny validation) |
| exists | exists (FAIL) | missing | Step 5a (fix loop with scrutiny failure) |
| exists | exists (PASS) | missing | Step 6 (user-test validation) |
| exists | exists (PASS) | exists (FAIL) | Step 5a (fix loop with user-test failure) |
| exists | exists (PASS) | exists (PASS) | Step 6a (demo video) then mark complete |

Log to progress.md: `<timestamp> -- Task N: resuming from <step name>. Previous session interrupted mid-cycle.`

### 3c: Detect orphaned commits

An orphaned commit is one where a worker committed code but no handoff file exists. This happens when the session died between the worker's commit and the orchestrator recording the result.

Only run this step for a build-loop resume -- `Plan approved.` was logged AND `tasks.md` has at least one unchecked `- [ ]` task (Phase 4 branch 2's condition). Skip it otherwise: planning-phase resumes (`Plan approved.` never logged) have no task commits yet and defer to `/mission`, and completion resumes (no unchecked tasks) have no next task to reconcile.

First, scope the search to this mission's commits. Read the pinned `base_sha` from `config.yaml`:

```bash
BASE_SHA=$(awk -F: '/^[[:space:]]*base_sha[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' openspec/changes/<change-name>/mission/config.yaml)
```

If `base_sha` is missing or not a valid commit (`git cat-file -e "${BASE_SHA}^{commit}" 2>/dev/null` fails), **stop and ask the user** which commit this mission branched from. Do not fall back to a repo-wide search -- that returns false positives from older missions on other branches.

**Stale `base_sha` check.** A valid `base_sha` can still be stale: if the plan or `mission/` bookkeeping was committed *after* `base_sha` was pinned, those commits sit above it and pollute the `base_sha..HEAD` scope (the symptom behind missions that needed a manual "pin base_sha" fix). Detect it:

```bash
FIRST_TASK=$(git log "$BASE_SHA"..HEAD --grep='\[task-' --reverse --format='%H' | head -1)
if [ -n "$FIRST_TASK" ]; then
  # Task commits exist: stale if anything sits between base_sha and the first one.
  if [ "$(git rev-list "$BASE_SHA".."$FIRST_TASK" | wc -l | tr -d ' ')" -gt 1 ]; then
    echo "MAYBE STALE -- candidate first task commit:"; git log --oneline -1 "$FIRST_TASK"
    echo "Commits between base_sha and it:"; git log --oneline "$BASE_SHA".."${FIRST_TASK}^"
  fi
else
  # No task commits yet: stale if bookkeeping was already committed above base_sha.
  git log --oneline "$BASE_SHA"..HEAD -- "openspec/changes/<change-name>/"
fi
```

Interpret the result:

- **Task commits exist and the gap is non-empty.** The commits between `base_sha` and the first `[task-N]` commit *may* be bookkeeping that belongs below the base. First confirm the candidate commit shown is actually this mission's first task -- not a `[task-N]` commit merged in from another branch or left by a polluted/hand-repaired history. If it is, **stop and ask the user** to confirm advancing `base_sha` to its parent (`${FIRST_TASK}^`), then write that SHA into `config.yaml`. Do **not** advance automatically, and do not trust `${FIRST_TASK}^` if the candidate is not this mission's first task.
- **No task commits yet but bookkeeping commits sit above `base_sha`.** The mission was interrupted before the first worker ran, but the plan or `mission/` files were already committed above the pinned base. **Stop and ask the user** to confirm advancing `base_sha` to `HEAD` before the build loop starts, then write it to `config.yaml`.
- **Otherwise** (no gap, or nothing above `base_sha`): `base_sha` is clean; continue.

For each unchecked task that did NOT have a handoff in step 3b:

1. Run: `git log "$BASE_SHA"..HEAD --oneline --grep="\[task-N\]" -1` (commits on this mission branch only).
2. If a commit is found but no handoff exists, report it to the human.
3. Ask: "This task has a commit but no handoff. Should I validate it (run full validation against this commit), or re-run the task from scratch?"

If validate: write a synthetic handoff file with the commit SHA, then run the full validation cycle (scrutiny + user-test). The human is confirming "the code exists," not "the code is correct."

If re-run: leave the task unchecked and start fresh.

## Phase 4: Restore the Orchestrator Plan

Enter Claude Code **plan mode**. The goal is an accurate operating manual that survives context compaction. Whether you restore or rebuild depends on what Phase 2 found.

### If a plan file exists (restore + reconcile)

Restore the plan from the existing file rather than re-deriving it. It already holds the orchestrator phase, current task/step, engine config, artifact paths, and codebase deltas.

Before trusting it, reconcile its state against disk reality:

1. **Task completion.** Compare the plan's `## Current Task` against the count of `- [x]` tasks in `tasks.md` and the highest `Task N complete.` in `progress.md`. If they diverge, the plan is stale -- update Current Task to the true next unchecked task and note the correction.
2. **Orchestrator phase.** Cross-check the plan's `## Orchestrator Phase` hint against the disk-derived phase detected in Phase 3a. If they disagree, trust the Phase 3a detection and update the field.

### If no plan file exists (rebuild)

Write a complete orchestrator operating manual from file state. Include: mission identity, all file paths, engine configuration, the detected `## Orchestrator Phase` (from Phase 3a), the mission flow (matching the /mission skill's Phase 8 build loop and Phase 9 completion), and the rules.

### Branch on the resume target

Route on concrete disk facts, tested in this order. Exactly one branch matches every state. This decision -- not the marker table's target label -- governs where the resume goes.

1. **Planning never finished** -- `Plan approved.` was never logged in progress.md. Do **not** enter the build loop. Report the detected phase to the user, for example: "This mission stopped in Phase 5 (contract writing), before the build loop started. Resuming planning needs `/mission`, which holds the planning-phase logic. Run `/mission` to continue from Phase 5, or tell me how you want to proceed." Then stop and wait. mission-resume does not contain planning-phase logic.
2. **Build loop has unchecked tasks** -- `Plan approved.` was logged and `tasks.md` still has at least one unchecked `- [ ]` task. Continue to Phase 5, then Phase 6 (the build loop). This also covers an inconsistent state where `Draft PR created` was logged but tasks remain unchecked: finish the tasks first, then complete.
3. **Completion unfinished** -- `Plan approved.` was logged, no unchecked `- [ ]` tasks remain, and the mission is not finished (per Phase 1's test). Continue to Phase 5, then go straight to **this skill's Phase 7** (mission completion). Skip Phase 6.

## Phase 5: Log the Resume

Append to `mission/progress.md`:

```
<timestamp> -- Mission resumed in new session. Picking up from task <N>.
Previously completed: tasks <list of completed task numbers>.
```

If the resume target is completion, write `Picking up from mission completion.` instead of a task number.

## Phase 6: Continue the Build Loop

Reached only when Phase 4 detected a build phase with unchecked tasks. Planning-phase resumes and completion-phase resumes already branched away in Phase 4.

Follow the `/mission` skill's Phase 8 build loop exactly -- every step, every sub-step, every rule. Do not re-implement the loop here; read Phase 8 in `.claude/skills/mission/SKILL.md` and execute it as written.

Start from the entry point established earlier: if Phase 3b detected a mid-cycle task, resume from the appropriate step in `/mission` Phase 8; otherwise start the **first unchecked `- [ ]` task in tasks.md** (skipping `- [~]` tasks) at Step 1. Use the first unchecked task, not arithmetic on the last completed task number -- tasks may be skipped or inserted out of sequence.

## Phase 7: Mission Completion

Reached when all tasks are resolved but the mission has not finished (Phase 4 branch 3).

Run **`/mission` Phase 9 in full** -- do not re-implement it here. Read Phase 9 in `.claude/skills/mission/SKILL.md` and execute it as written. Phase 9 runs, in order: Final Gates (whole-repo build, test, lint, format) -> Spec Delta Check -> Create Draft PR -> Dual Adversarial Review -> Present to human -> mark plan status `COMPLETE`. A completion resume **must** run these gates; it must not jump straight to the PR, or it can mark a mission `COMPLETE` on a codebase that does not build.

Because the previous session may have finished part of Phase 9, **resume from the first sub-step whose completion marker is absent from progress.md**, skipping the ones already logged:

| Last Phase 9 marker in progress.md | Resume from |
|---|---|
| none of the below | Final Gates |
| `Final gates PASSED` | Spec Delta Check |
| `Spec delta check CLEAN` | Create Draft PR, then the Dual Adversarial Review |
| `Draft PR created: <URL>. Adversarial reviews complete.` | Present to human, then mark `COMPLETE` |

`/mission` logs the draft PR and the adversarial review together on one line, and only after the review finishes (mission/SKILL.md Present-to-Human step) -- there is no separate "PR created, review pending" marker. So a crash any time between the spec-delta check and that combined line resumes at Create Draft PR: the idempotent PR step (below) reuses the existing PR, then the Dual Adversarial Review re-runs. The review is a human-facing report, so re-running it is safe.

Two adjustments when running Phase 9 from here:

- **Idempotent PR.** At the Create Draft PR sub-step, first check for **any** existing open PR on the branch -- draft or already promoted -- and reuse it instead of creating a duplicate:
  ```bash
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
  PR_URL=$(gh pr list --head "$BRANCH" --state open --json url --jq '.[].url' | head -1)
  ```
  If `PR_URL` is non-empty, reuse it (do not try to demote a promoted PR back to draft); otherwise create the draft PR as Phase 9 describes. Matching only drafts would miss a PR a human already promoted, and `gh pr create` would then fail against the existing PR.
- **Legacy PRD issues.** A mission planned before the proposal consolidation may reference a PRD GitHub issue in its plan or coverage-audit.md. Phase 9 no longer comments on, links, or closes any issue -- if the restored mission has one, surface it at Present-to-Human so the human can close the issue manually after merge.
- **base_sha guard.** Phase 9's adversarial review reads the pinned `base_sha` from config.yaml. If `base_sha` is missing or not a valid commit (`git cat-file -e "${BASE_SHA}^{commit}" 2>/dev/null` fails), do **not** abort -- stop and ask the user which commit this mission branched from (as in step 3c), then write it to config.yaml before running the review. Also run the **stale `base_sha` check** from step 3c: if bookkeeping commits sit between `base_sha` and the first `[task-N]` commit, the review walks them as if they were mission work. Stop, confirm the corrected `base_sha` with the user, and write it to config.yaml before running the review.
