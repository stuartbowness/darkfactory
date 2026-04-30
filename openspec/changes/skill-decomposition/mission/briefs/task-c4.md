# Worker Brief: Task C4 - Documentation

## Your Task

Verify that all in-repo documentation accurately reflects the post-refactor state. Fix anything that doesn't.

## Repo root: /Users/stuart/.darkfactory

## Checks to run

### 1. README.md accuracy

Run these verification greps:
- `grep -c "ln -sf" README.md` → should be 0
- `grep -iE "darkfactory-init|grill-me|write-a-prd|ubiquitous-language|test-quality-review|security-review" README.md` → should be no output
- `grep -c "install.sh" README.md` → should be ≥1
- Verify the skills table lists all 12 skill names: mission-init, grill, prd, language, tdd, test-quality, security, adversarial, dispatch (plus mission, mission-resume, mission-status)

### 2. UBIQUITOUS_LANGUAGE.md accuracy

- `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` → should be 0

### 3. Any other docs that reference old skill names

Scan all Markdown files in the repo (excluding `openspec/changes/skill-decomposition/` which is mission state, not docs):

```bash
grep -rE "grill-me|write-a-prd|ubiquitous-language|darkfactory-init|test-quality-review|security-review" \
  --include="*.md" \
  --exclude-path="*/openspec/changes/skill-decomposition/*" \
  .
```

If any matches found outside the expected mission state directory, fix them.

### 4. CLAUDE.md (if any)

Check if `CLAUDE.md` at the repo root mentions any old skill names. If it does, update them.

## If fixes are needed

Make the minimal changes necessary to correct any stale references. Do not rewrite or reorganize docs beyond what's needed.

## If no fixes are needed

Write the handoff saying what you checked and that everything was clean.

## Commit behavior

- If changes were made: commit with message `[task-c4] fix stale doc references`
- If no changes needed: no commit (just write the handoff)

## Write handoff to

`openspec/changes/skill-decomposition/mission/handoffs/task-c4.md`

## Handoff Report Format

- Task: Task C4 - Documentation
- Result: clean (no changes) | fixed (commit SHA: <sha>)
- What was checked: <list>
- What was changed: <list or "nothing">
- Concerns: <anything uncertain>
