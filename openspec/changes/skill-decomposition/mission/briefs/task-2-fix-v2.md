# Fix Brief: Task 2 - Fork grill, language, prd skills (Attempt 2)

## What Failed

Three skill files are missing `user-invokable: true` in their frontmatter:
- `skills/grill/SKILL.md`
- `skills/prd/SKILL.md`
- `skills/language/SKILL.md`

## Diagnostic Triage

The source skills at `~/Code/dotfiles` don't have this field. The fork copied them faithfully but the contract requires this field to be present in the forked versions.

## Validator Output

`grep "user-invokable: true" skills/grill/SKILL.md skills/prd/SKILL.md skills/language/SKILL.md` → 0 matches (required: 3)

## Commit to Fix

66500d39aa954798f10ddca934360a170d0c4546

## What to Change

Add `user-invokable: true` to the frontmatter of all three files. The frontmatter of each currently looks like:

```
---
name: grill
description: ...
---
```

Change each to add the field after `name:`:

```
---
name: grill
user-invokable: true
description: ...
---
```

Apply the same change to `skills/prd/SKILL.md` and `skills/language/SKILL.md`.

No other changes. The body content of each file stays untouched.

## Verification

After the fix:
- `grep "user-invokable: true" skills/grill/SKILL.md skills/prd/SKILL.md skills/language/SKILL.md` → 3 matching lines
- `grep "^name:" skills/grill/SKILL.md` → `name: grill`
- `grep "^name:" skills/prd/SKILL.md` → `name: prd`
- `grep "^name:" skills/language/SKILL.md` → `name: language`

## Procedures

- Edit the frontmatter of all three files as described above — no other changes
- Commit as a single commit with message: `[task-2] fix: add user-invokable: true to grill, prd, language frontmatter`
- Write handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-2-fix-v2.md`

## Handoff Report Format

- Task: Task 2 - Fork grill, language, prd skills (Fix attempt 2)
- Commit SHA: <the full SHA>
- Approach: <what you fixed>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
