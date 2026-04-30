# Fix Brief: Task 1a - Update mission SKILL.md skill references (Attempt 2)

## What Failed

`skills/mission/SKILL.md` has no reference to `/mission-init`. The file needs a call to `/mission-init` added at the start of Phase 0 so the skill verifies Darkfactory setup before the mission begins.

## Diagnostic Triage

The `/darkfactory-init` name was never present in the file, so the substitution task produced no `/mission-init` reference. The human has decided: add a `/mission-init` call to Phase 0 as a setup verification step.

## Validator Output

`grep -c "/mission-init" skills/mission/SKILL.md` → 0 (required ≥1)

## Commit to Fix

1b951021c61265b1642e77d0b8fe3a066fc07f61

## What to Add

In `skills/mission/SKILL.md`, at the very start of Phase 0 (immediately after the `> **Rules:** 5, 8` block, before "Enter plan mode immediately"), add a setup verification step:

```
Before entering plan mode, run `/mission-init` to verify that all Darkfactory skills are installed and required CLI tools are available. If `/mission-init` reports any issues, resolve them before proceeding.
```

Also add a gate item to the Phase 0 Gate block:
```
- [ ] `/mission-init` passed (all skills symlinked, CLI tools present)
```

The Phase 0 section currently looks like this (lines 38-61):

```markdown
## Phase 0: Enter Plan Mode

> **Rules:** 5, 8
> **Goal:** Write the durable operating plan that survives context compaction.
> **Inputs:** User's idea (from conversation), UBIQUITOUS_LANGUAGE.md (if exists)
> **Outputs:** Plan (in plan mode), UBIQUITOUS_LANGUAGE.md confirmed read or noted absent

Enter plan mode immediately. Write a durable operating manual as the plan. This plan survives context compaction and is the orchestrator's source of truth.
```

Change it to:

```markdown
## Phase 0: Enter Plan Mode

> **Rules:** 5, 8
> **Goal:** Write the durable operating plan that survives context compaction.
> **Inputs:** User's idea (from conversation), UBIQUITOUS_LANGUAGE.md (if exists)
> **Outputs:** Plan (in plan mode), UBIQUITOUS_LANGUAGE.md confirmed read or noted absent

Run `/mission-init` to verify that all Darkfactory skills are installed and required CLI tools are available. If `/mission-init` reports any issues, resolve them before proceeding.

Enter plan mode immediately. Write a durable operating manual as the plan. This plan survives context compaction and is the orchestrator's source of truth.
```

And add the gate item so the Phase 0 gate becomes:

```markdown
**Gate -- proceed when:**
- [ ] `/mission-init` passed (all skills symlinked, CLI tools present)
- [ ] Plan is written and active in plan mode
- [ ] Darkfactory SHA captured in plan
- [ ] UBIQUITOUS_LANGUAGE.md checked (read if exists, noted if absent)
```

**That is the complete change.** No other modifications.

## Verification

After the change, these must hold:
- `grep -c "/mission-init" skills/mission/SKILL.md` → ≥1
- `grep -c "/grill-me" skills/mission/SKILL.md` → 0
- `grep -c "/write-a-prd" skills/mission/SKILL.md` → 0
- `grep -c "/ubiquitous-language" skills/mission/SKILL.md` → 0
- `grep -c "/darkfactory-init" skills/mission/SKILL.md` → 0

## Procedures

- Make only the two additions described above (one paragraph before "Enter plan mode", one gate item)
- Do NOT use /tdd — this is a .md file edit, not code
- Commit as a single commit with message: `[task-1a] fix: add /mission-init call to Phase 0`
- Write handoff report to `openspec/changes/skill-decomposition/mission/handoffs/task-1a-fix-v2.md`

## Handoff Report Format

- Task: Task 1a - Update mission SKILL.md skill references (Fix attempt 2)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
