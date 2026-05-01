# Mission: <change-name>

## Status: PLANNING | APPROVED | BUILDING | COMPLETE | PAUSED

## State Directory
openspec/changes/<change-name>/mission/

## Darkfactory SHA
<full SHA of ~/.darkfactory at plan-creation time; append "-dirty" if working tree has uncommitted changes>
Captured so this mission can be reproduced against the exact skill version that ran it.

## OpenSpec Artifacts
- Proposal: openspec/changes/<change-name>/proposal.md
- Specs: openspec/changes/<change-name>/specs/<capability>/spec.md (one per capability)
- Design: openspec/changes/<change-name>/design.md
- Tasks: openspec/changes/<change-name>/tasks.md (includes mandatory closing tasks)
- Coverage Audit: openspec/changes/<change-name>/coverage-audit.md
- Contract: openspec/changes/<change-name>/contract.md

## Spec Coverage Audit
- PRD issue: <GitHub issue URL from Phase 3>
- PRD issue number: <N>
- Coverage table: <pending | complete>
- Gaps found: <count or "none">
- Gaps resolved: <added N tasks | deferred N | skipped N>

## Engine Config
Defaults defined in Phase 6 config.yaml. 

## Deterministic Anchors
Filled during Phase 4.3 (5 anchors) and Phase 4.3a (task strategy anchor). The orchestrator references these when populating worker briefs and making phase decisions.

- Workspace layout: <e.g., "two-crate workspace: core lib + cli binary" (see design.md Architecture Decision)>
- Task decomposition strategy: <e.g., "layered, 7 feature tasks + 5 closing" (see Phase 4.3a approval)>
- Primary domain types: <e.g., "Todo, TodoStatus, IronTodoError" (see design.md Type Catalog)>
- Error handling strategy: <e.g., "thiserror in core, never anyhow" (see design.md Error Strategy)>
- Test strategy: <e.g., "unit + integration, serial with DB" (see design.md)>
- Chosen architecture: <e.g., "workspace-with-lib-and-bin, scored 10/12" (see design.md Architecture Decision)>

## Skills to Use
- /grill -- alignment interview (planning)
- /language -- shared language (planning)
- /prd -- structured PRD from grill conversation (planning)
- openspec CLI -- create artifacts sequentially: proposal, specs, design, tasks (planning)
- /review -- code review (scrutiny validation)
- /simplify -- simplify pass (scrutiny validation)
- /security -- security check (scrutiny validation, sensitive tasks only)
- /tdd -- test-driven development for workers (build phase)
- /test-app -- browser tests (user-test validation)
- /demo-video -- frontend demo recording (post-validation)
- gh pr create --draft (completion, no skill needed)
- /opsx:archive -- archive change (post-merge)

## Mission Flow
For each unchecked task in tasks.md (skip `- [~]` tasks):
1. Read mission state from progress.md
2. Write brief to mission/briefs/task-N.md
3. Dispatch worker (engine from config.yaml)
4. Read handoff from mission/handoffs/task-N.md
5. Spawn scrutiny validator (commit SHA + contract checks)
6. If fail -> write to issues/, spawn fix worker, re-validate. Circuit breaker at 3.
7. Spawn user-test validator (commit SHA + contract checks)
8. If fail -> same fix/re-validate loop
9. If frontend task -> spawn demo video
10. Mark task done in tasks.md, update progress.md

After all tasks complete:
11. Run final gates (build, test, lint, format)
12. Run spec delta check against coverage audit
13. Create draft PR
14. Run dual adversarial review (Codex + Claude): each walks mission commits one at a time, groups findings by task, then does a cross-task pass
15. Present both reports to human alongside PR link

## Rules
- One worker at a time (serial execution)
- Workers never see contract.md
- Validators never see proposal.md or design.md
- On any failure, re-run both per-task validators after fix (scrutiny, user-test)
- Write all decisions to progress.md

## Codebase Deltas
<Accumulated after each task passes validation. Used to populate "What Previous Tasks Built" in subsequent briefs. Generated from git diff, never from handoff reports.>

## Current Task: <N or "none">
## Current Step: <step name or "none">
