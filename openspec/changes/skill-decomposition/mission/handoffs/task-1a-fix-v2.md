Implemented and committed the fix.

Commit: `e14fa34e0d1b377c09ecbee64946d2827f09e20e`  
Message: `[task-1a] fix: add /mission-init call to Phase 0`

Changed [skills/mission/SKILL.md](/Users/stuart/.darkfactory/skills/mission/SKILL.md:42) with only the two requested additions. Wrote the handoff report at [task-1a-fix-v2.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/handoffs/task-1a-fix-v2.md:1).

Verification passed:

```text
/mission-init=2
/grill-me=0
/write-a-prd=0
/ubiquitous-language=0
/darkfactory-init=0
```

I staged only `skills/mission/SKILL.md` for the fix commit. The repo still has pre-existing unrelated uncommitted OpenSpec/tool changes, plus the handoff file written after the commit so it could include the final SHA.