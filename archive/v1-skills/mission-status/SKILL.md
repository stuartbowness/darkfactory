---
name: mission-status
description: Check the progress of an active Darkfactory mission. Read-only -- does not modify any files.
---

# Mission Status

Show the progress of an active mission. This skill is read-only -- it does not modify any mission files.

---

## Step 1: Find active missions

Search for OpenSpec changes that have a `mission/` directory:

```bash
ls openspec/changes/*/mission/config.yaml 2>/dev/null
```

- **If none found:** Report "No active missions found." and stop.
- **If exactly one:** Use it.
- **If multiple:** List them and ask the user which one to check.

---

## Step 2: Read state

Read these files:
1. `tasks.md` -- count checked vs unchecked tasks
2. `mission/progress.md` -- the event log
3. `mission/config.yaml` -- engine settings

---

## Step 3: Gather validation results

For each completed task, check for:
- `mission/validations/task-N-scrutiny.md` -- read the Overall line
- `mission/validations/task-N-usertest.md` -- read the Overall line
- `mission/validations/task-N-demo.mp4` -- check if exists
- `mission/issues/task-N-*.md` -- count fix attempts

---

## Step 4: Present summary

Format:

```
Mission: [change name]
Engine: worker=[codex/claude], validators=[claude]
Started: [timestamp from first progress.md entry]

Progress: [completed]/[total] tasks
[=========>          ] 45%

Tasks:
  [x] Task 1: [name] - Scrutiny: PASS, User-test: PASS, Demo: yes
  [x] Task 2: [name] - Scrutiny: PASS, User-test: PASS (2 attempts)
  [ ] Task 3: [name] - (next)
  [ ] Task 4: [name]

Recent activity:
  [last 5 entries from progress.md]
```

If a task is currently in progress (unchecked but has a brief file and no handoff), note it:
```
  [ ] Task 3: [name] - IN PROGRESS (worker dispatched, awaiting handoff)
```
