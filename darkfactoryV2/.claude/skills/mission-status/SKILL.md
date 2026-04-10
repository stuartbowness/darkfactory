---
name: mission-status
description: Check the progress of an active Darkfactory mission. Read-only -- does not modify any files.
---

# Mission Status

Show a formatted progress summary for an active Darkfactory mission. This skill is strictly **read-only** -- it MUST NOT create, modify, or delete any files.

## When to use

- The user runs `/mission-status` or asks "how is the mission going?"
- The user wants to check on a running mission from a separate Claude Code session without interrupting the orchestrator

## Procedure

### Step 1: Find active missions

Search for OpenSpec changes that have a mission directory:

```
openspec/changes/*/mission/config.yaml
```

Use the Glob tool to find all matching paths.

**Handle zero, one, or multiple results:**

- **No missions found:** Report "No active missions found." and stop.
- **One mission found:** Use it automatically.
- **Multiple missions found:** List them by change name and ask the user which one to inspect. Wait for their answer before continuing.

Extract the **change name** from the path (the `*` segment in `openspec/changes/*/mission/`).

### Step 2: Read mission state

Read these files. All reads are non-destructive -- do not write anything.

1. **Mission config** (`openspec/changes/<change>/mission/config.yaml`)
   - Extract engine settings: `worker`, `scrutiny_validator`, `usertest_validator`, `demo_video`

2. **Tasks list** (`openspec/changes/<change>/tasks.md`)
   - Parse each checkbox line: `- [x]` = completed, `- [ ]` = not completed
   - Count total tasks and completed tasks
   - Extract each task's name (the text after the checkbox)

3. **Progress log** (`openspec/changes/<change>/mission/progress.md`)
   - Read the full file
   - Extract the timestamp from the first entry (this is the mission start time)
   - Extract the last 5 entries for the "Recent activity" section

### Step 3: Gather validation results per completed task

For each task (numbered by its position in the tasks list), check for validation artifacts:

1. **Scrutiny validation:** Read `mission/validations/task-N-scrutiny.md`
   - If it exists, find the line starting with "Overall" and extract the result (PASS or FAIL)
   - If it does not exist, report as "pending"

2. **User-test validation:** Read `mission/validations/task-N-usertest.md`
   - If it exists, find the line starting with "Overall" and extract the result (PASS or FAIL)
   - If it does not exist, report as "pending"

3. **Demo video:** Check if `mission/validations/task-N-demo.mp4` exists
   - Report "yes" if it exists, omit if it does not

4. **Fix attempts:** Search for `mission/issues/task-N-*.md` files
   - Count the number of matching files
   - If any exist, append "(N attempts)" to the task's status line

### Step 4: Detect in-progress tasks

For each unchecked task, check whether work has started:

- If `mission/briefs/task-N.md` exists but `mission/handoffs/task-N.md` does NOT exist, mark the task as:
  `IN PROGRESS (worker dispatched, awaiting handoff)`
- If neither brief nor handoff exists, show no extra annotation (the task has not started yet)
- Mark the first unstarted unchecked task with `(next)` to indicate it is next in the queue

### Step 5: Display formatted progress

Print the summary in this exact format:

```
Mission: <change name>
Engine: worker=<engine>, validators=<engine>
Started: <timestamp from first progress.md entry>

Progress: <completed>/<total> tasks
[===========>                    ] <percent>%

Tasks:
  [x] Task 1: <name> - Scrutiny: PASS, User-test: PASS, Demo: yes
  [x] Task 2: <name> - Scrutiny: PASS, User-test: PASS (2 attempts)
  [ ] Task 3: <name> - IN PROGRESS (worker dispatched, awaiting handoff)
  [ ] Task 4: <name> - (next)
  [ ] Task 5: <name>

Recent activity:
  <last 5 entries from progress.md>
```

**Progress bar rules:**
- The bar is 30 characters wide (between the square brackets)
- Fill with `=` up to the completion percentage, ending with `>` at the leading edge
- Fill the remainder with spaces
- Show the percentage as a whole number

**Task line rules:**
- Use `[x]` for completed tasks and `[ ]` for unchecked tasks
- After the task name, show validation results separated by commas
- Only show Scrutiny/User-test/Demo if the corresponding artifact exists or the task is completed
- Append fix attempt count in parentheses if any issues were found
- For in-progress tasks, show the `IN PROGRESS` annotation instead of validation results
- For the next unstarted task, show `(next)`
- For remaining unstarted tasks, show just the task name with no annotation

## Constraints

- **Read-only:** This skill MUST NOT create, modify, or delete any files. Every operation is a read or a glob search.
- **Non-intrusive:** This skill does not interact with the running mission. It only reads files on disk.
- **Graceful degradation:** If any file is missing or malformed, skip it and note what was unavailable rather than failing entirely.
