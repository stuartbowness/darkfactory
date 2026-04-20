# Darkfactory Design Decisions

Captured from the grill-me session on 2026-04-10.

## Vision

A lightweight multi-agent mission system built entirely from Claude Code primitives (skills, subagents, plan mode, hooks) and Codex CLI. No custom IDE or app. Inspired by Factory's Missions architecture.

## Phase Flow

```
align -> spec -> contract -> build -> validate -> ship
```

Inner loop per task: `spec -> build -> verify -> ship` with repetition on validation failure.

## Architecture

### Orchestrator
- IS the main Claude Code session (Opus 4.6, not configurable -- it's the session)
- Enters plan mode at start -- the plan is the durable operating manual that survives compaction
- Interactive during planning (grills the user, proposes plan, gets approval)
- Autonomous during execution (runs the worker/validator loop)
- Manages all state via files on disk
- Spawns workers via Codex, validators via Claude subagents

### Workers
- One worker per task (vertical slice pattern)
- Receive a brief file -- self-contained, includes task + proposal excerpt + design excerpt + procedures
- Do NOT receive the contract (adversarial separation)
- Do NOT receive previous workers' handoff reports (clean context)
- Commit all work in a single commit
- Write a structured handoff report to disk
- Default engine: Codex (configurable in mission/config.yaml)

### Per-task validators (run sequentially)
1. **Scrutiny validator** -- test presence gate, build/test/lint, /review, code standards, /simplify, /security-review (conditional), contract assertions. Always runs.
2. **User-test validator** -- decides API tests (HTTP) and/or browser tests (agent-browser) based on what changed. Always runs.
3. **Demo video** -- recorded after both validators pass, for frontend tasks. Orchestrator decides when.

### End-of-mission review
4. **Dual adversarial review** -- runs once after the draft PR is created, against the full diff. Codex and Claude independently review the code. Both reports are presented to the human. No automated fix loop -- the human decides what to address.

Validators receive: commit SHA + contract assertions. They do NOT receive proposal, design, or worker reasoning.

### Fix Loop
- On validation failure: orchestrator writes a symptom-based fix brief (no contract text) to mission/issues/, spawns a fix worker
- Fix worker uses `/tdd` to fix
- Fix worker commits a single fix commit
- Both per-task validators re-run after every fix (scrutiny + user-test)
- Circuit breaker: 3 attempts on the same task -> pause and ask human
- Skipped tasks marked with `- [~]` in tasks.md -- the build loop ignores them

## Key Design Decisions

### Bash run_in_background for external processes
- Long-running external processes (Codex workers, Codex adversarial reviews) run via Bash with `run_in_background: true`, teed to a log file under `mission/logs/`
- Claude subagent validators use the Agent tool directly since they run within the session
- The log file lets the human watch progress in another terminal (`tail -f`) without the agent paying context cost for mid-run events

### File-backed state
- All mission state lives on disk in the mission directory
- The orchestrator reads fresh state before each cycle (survives compaction)
- Enables session resume if Claude Code dies
- tmux as first line of defense against session death

### Serial execution
- One worker at a time (matches Factory's approach)
- Parallelization only for read-only operations within validators
- Correctness compounds over time

### Vertical slices
- Each task in tasks.md should be a thin vertical slice cutting through all layers
- A completed task is demoable and verifiable on its own
- One task = one worker = one commit = one validation cycle

### Configurable engines
- mission/config.yaml specifies which engine per role (codex/claude)
- Default: Codex for workers and adversarial reviewer A, Claude for validators, adversarial reviewer B, and demo video
- Swap engines when new models drop without changing anything else

### OpenSpec integration
- `/mission` is the single workflow for all work (small fixes to large epics). Scope adapts naturally.
- contract.md is a new OpenSpec artifact (validation assertions, written during planning)
- mission/ directory nests inside openspec/changes/<change-name>/
- Archiving the change archives the mission artifacts too

### Entry point and completion
- `/mission` skill starts a mission
- `/mission-resume` resumes from file state
- `/mission-status` shows progress
- On completion: draft PR created via `gh pr create --draft`
- Human reviews and promotes to ready

## Orchestrator Plan Structure

```markdown
# Mission: [change-name]

## State Directory
openspec/changes/<change-name>/mission/

## OpenSpec Artifacts
- Proposal: openspec/changes/<change-name>/proposal.md
- Design: openspec/changes/<change-name>/design.md
- Tasks: openspec/changes/<change-name>/tasks.md
- Contract: openspec/changes/<change-name>/contract.md

## Engine Config
- Workers: codex
- Scrutiny validator: claude
- User-test validator: claude
- Adversarial reviewer A: codex
- Adversarial reviewer B: claude
- Demo video: claude

## Skills to Use
- Alignment: /grill-me (during planning, before artifacts are created)
- Shared language: /ubiquitous-language (during planning, establish terms)
- PRD: /write-a-prd (turn grill conversation into structured PRD)
- Planning: /opsx:propose (create proposal, design, tasks from PRD)
- Task breakdown: each task is a vertical slice through all layers, demoable and verifiable on its own
- Contract: written by orchestrator after planning, before workers start
- Worker implementation: /tdd (test-driven development for workers)
- Worker briefs: assembled from proposal.md + design.md + task description
- Scrutiny validation: build/test/lint + /review + /simplify against contract
- Security validation: /security-review (for tasks touching auth/sensitive paths)
- User-test validation: /test-app (agent-browser) and/or HTTP tests against API
- Adversarial review: dual Codex + Claude review in parallel, orchestrator synthesizes
- Demo video: /demo-video (for frontend tasks)
- Completion: gh pr create --draft (draft PR with mission summary)
- Archival: /opsx:archive (after merge)

## Mission Flow
For each unchecked task in tasks.md:
1. Read mission state from mission/progress.md
2. Write brief to mission/briefs/task-N.md
3. Kick off worker via Bash `run_in_background` (engine per config.yaml)
4. On completion, read mission/handoffs/task-N.md
5. Spawn scrutiny validator (commit SHA + contract assertions)
6. If fail -> write symptom-based fix brief, spawn fix worker, re-validate both. Circuit breaker: 3 attempts -> ask human.
7. Spawn user-test validator (commit SHA + contract assertions)
8. If fail -> same fix/re-validate loop
9. If frontend task -> spawn demo video
10. Mark task complete in tasks.md, update progress.md

After all tasks complete:
11. Create draft PR
12. Run dual adversarial review (Codex + Claude) against full diff
13. Present both reports to human alongside PR link

## Rules
- One worker at a time (serial execution)
- Workers never see contract.md
- Validators never see proposal.md or design.md
- On any failure, re-run BOTH validators after fix
- Write all decisions to progress.md
```

## Mission Directory Structure

```
openspec/changes/<change-name>/
  .openspec.yaml
  proposal.md
  design.md
  tasks.md
  contract.md
  mission/
    config.yaml
    progress.md
    briefs/
      task-1.md
      task-2.md
    handoffs/
      task-1.md
      task-2.md
    validations/
      task-1-scrutiny.md
      task-1-usertest.md
      task-1-demo.mp4
    issues/
      task-1-v1.md
```

## Worker Brief Format

```markdown
# Worker Brief: Task N - [name]

## Your Task
[Task description and acceptance criteria]

## Project Context
[Relevant excerpt from proposal.md]

## Architecture
[Relevant excerpt from design.md]

## Procedures
- Implement the task as described
- Commit all changes in a single commit with message: "[task-N] description"
- Write your handoff report to mission/handoffs/task-N.md using the format below

## Handoff Report Format
- Task: [which task]
- Commit SHA: [the SHA]
- Approach: [brief explanation of what you did and why]
- Commands run: [build/test/lint commands and exit codes]
- Concerns: [anything you're uncertain about]
```

## Contract Format

```markdown
# Contract

## Task 1: [name]
- [Assertion verifiable from outside without knowing the code]
- [Another assertion]

## Task 2: [name]
- [Assertion]
```

## Fix Brief Format

```markdown
# Fix Brief: Task N - [name] (Attempt 2)

## What Failed
[Specific failed assertions from the validator report]

## Validator Output
[Relevant error messages, test failures, review findings]

## Commit to Fix
[SHA of the broken commit]

## Procedures
- Fix only the issues described above
- Do not refactor or change anything unrelated
- Commit as a single commit with message: "[task-N] fix: description"
- Write handoff report to mission/handoffs/task-N-fix-v2.md
```

## Build Order (Darkfactory itself)

1. **Orchestrator skeleton** -- /mission skill, plan mode, grill, /opsx:propose, contract.md, mission directory
2. **Worker loop** -- brief writing, Codex invocation via Bash `run_in_background`, handoff reading
3. **Scrutiny validation** -- build/test/lint + /review + fix loop with circuit breaker
4. **User-test validation** -- agent-browser + HTTP tests
5. **Demo video + draft PR** -- completion phase
6. **Resume + status** -- /mission-resume, /mission-status skill
