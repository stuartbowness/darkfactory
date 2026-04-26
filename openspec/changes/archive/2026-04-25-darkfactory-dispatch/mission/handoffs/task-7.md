Task: Task 7 - Mission skill integration  
Commit SHA: not created

I could read the target file at `/Users/stuart/.darkfactory/.claude/skills/mission/SKILL.md`, but the sandbox only permits writes under `/Users/stuart/.darkfactory/tools/darkfactory-dispatch` and `.git`. Both the skill file and requested handoff path are outside that writable set, so the patch tool rejected the edit.

Commands run:
- `rg ... .claude/skills/mission/SKILL.md` from tool dir: exit 2, file not found there
- `git status --short`: exit 0
- `rg --files -g 'SKILL.md' ../..`: exit 0, found `../../.claude/skills/mission/SKILL.md`
- `sed` reads around both target blocks: exit 0
- patch attempts: rejected by sandbox as writing outside project

No tests or commit were run because no file changes could be applied. Rerun this worker with `/Users/stuart/.darkfactory` as the writable project root, or add `/Users/stuart/.darkfactory` to writable roots.