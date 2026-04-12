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
| Mission config | `mission/config.yaml` | Engine assignments |
| Contract | `contract.md` | Assertions grouped by task for validators |
| Proposal | `proposal.md` | What is being built and why |
| Design | `design.md` | Architecture decisions |

Also read existing handoff files in `mission/handoffs/` and validation reports in `mission/validations/` (including `task-N-scrutiny.md`, `task-N-usertest.md`). These reveal mid-cycle interruptions.

## Phase 3: Detect Mid-Cycle State and Orphaned Commits

Check each unchecked task for partial completion. A task can be interrupted at various points. The resume skill detects where it stopped and resumes from that point.

For each unchecked task (skip `- [~]` skipped tasks):

### 3a: Check for handoff with missing validations

If `mission/handoffs/task-N.md` exists (worker finished), check which validation reports exist:

| Handoff | Scrutiny Report | User-test Report | Resume From |
|---------|----------------|-----------------|-------------|
| exists | missing | missing | Step 4 (scrutiny validation) |
| exists | exists (PASS) | missing | Step 6 (user-test validation) |
| exists | exists (PASS) | exists (PASS) | Step 6a (demo video) then mark complete |

If a validation report exists but shows FAIL, resume from the fix loop (Step 5a) with that failure.

Log to progress.md: `<timestamp> -- Task N: resuming from <step name>. Previous session interrupted mid-cycle.`

### 3b: Detect orphaned commits

An orphaned commit is one where a worker committed code but no handoff file exists. This happens when the session died between the worker's commit and the orchestrator recording the result.

For each unchecked task that did NOT have a handoff in step 3a:

1. Run: `git log --oneline --grep="\[task-N\]" -1`
2. If a commit is found but no handoff exists, report it to the human.
3. Ask: "This task has a commit but no handoff. Should I validate it (run full validation against this commit), or re-run the task from scratch?"

If validate: write a synthetic handoff file with the commit SHA, then run the full validation cycle (scrutiny + user-test). The human is confirming "the code exists," not "the code is correct."

If re-run: leave the task unchecked and start fresh.

## Phase 4: Reconstruct the Orchestrator Plan

Enter Claude Code **plan mode**. Write a complete orchestrator operating manual from file state. This plan must survive context compaction.

The plan must include: mission identity, all file paths, engine configuration, the mission flow (matching the /mission skill's Phase 8 build loop and Phase 9 completion), and the rules.

## Phase 5: Log the Resume

Append to `mission/progress.md`:

```
Mission resumed in new session. Picking up from task <N>.
Previously completed: tasks <list of completed task numbers>.
```

## Phase 6: Continue the Build Loop

Follow the /mission skill's Phase 8 build loop exactly. For each remaining unchecked task, run Steps 1-7 as defined in the /mission skill:

1. **Assemble brief** -- check if brief exists, reuse or create. Brief never contains contract content.
2. **Dispatch worker** -- use the full Monitor-based dispatch for Codex:

```bash
BRIEF_PATH="<absolute path to mission/briefs/task-N.md>"
HANDOFF_PATH="<absolute path to mission/handoffs/task-N.md>"
BRIEF_CONTENT=$(cat "$BRIEF_PATH")

# Clean stale temp files and directories from previous workers to avoid filling /tmp
find /tmp -maxdepth 1 -user "$(whoami)" -mmin +30 -not -name "claude-*" \( -type f -delete -o -type d -exec rm -rf {} + \) 2>/dev/null

codex exec \
  --full-auto \
  --sandbox workspace-write \
  -m gpt-5.4 \
  -c model_reasoning_effort="high" \
  -o "$HANDOFF_PATH" \
  "$BRIEF_CONTENT" \
  2>/dev/null &

CODEX_PID=$!

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

For Claude workers, use the Agent tool with the brief content.

3. **Handle result** -- TASK_COMPLETE, TASK_FAILED, or timeout.
4. **Scrutiny validation** -- spawn Claude subagent with the full 9-step check sequence:
   1. Test presence gate (auto-FAIL if no new tests)
   2. Build
   3. Tests
   4. Lint
   5. Code review (/review) -- check for bugs, security, logic, breaking changes
   6. Code standards -- 5 anti-pattern checks
   7. Code simplify (/simplify)
   8. Security review (/security-review) -- conditional on sensitive code
   9. Contract assertions
5. **Handle scrutiny result** -- PASS proceeds to Step 6. FAIL enters fix loop.
5a. **Fix loop** -- write symptom-based fix brief (no contract text), dispatch fix worker, re-run both per-task validators. Circuit breaker at 3.
6. **User-test validation** -- spawn Claude subagent that decides API tests, browser tests, or both.
6a. **Demo video** -- frontend tasks only, non-blocking on failure.
7. **Track progress** -- mark task complete, update progress.md.

If Phase 3a detected a mid-cycle task, resume from the appropriate step rather than starting from Step 1.

## Phase 7: Mission Completion

When all tasks are checked in tasks.md:

1. Create a draft PR using `gh pr create --draft` with a mission summary (tasks, validation results, issues, stats).
2. Run dual adversarial review against the full diff (`git diff <base-branch>...HEAD`):
   - Reviewer A (engine from config.yaml, default codex) via Monitor
   - Reviewer B (engine from config.yaml, default claude) via Agent tool
   - Both receive the full diff and the full contract
   - Reports written to `mission/validations/adversarial-a.md` and `adversarial-b.md`
3. Present both reports to the human alongside the draft PR link.
4. Log to progress.md and update plan status to COMPLETE.

The adversarial review is a human-facing report, not an automated gate. The human decides what to address.
