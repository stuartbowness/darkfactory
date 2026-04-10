# Worker Brief: Task 1 - Main Mission Skill

## Your Task

Generate `darkfactoryV2/.claude/skills/mission/SKILL.md` -- the main orchestrator skill for the Darkfactory mission system. This skill must cover all 8 phases of a mission, from initial planning through to completion. It should be a complete, self-contained SKILL.md file with YAML frontmatter.

## Project Context

Darkfactory is a lightweight multi-agent mission system built from Claude Code primitives (skills, subagents, plan mode, Monitor) and Codex CLI. It orchestrates autonomous code generation using three roles:
- **Orchestrator** -- the main Claude Code session in plan mode
- **Workers** -- subagents (Codex by default, configurable) that implement tasks
- **Validators** -- Claude subagents that check work against a validation contract

The mission flow is: `idea -> spec -> build -> verify -> ship -> merge`

## Architecture

The skill must implement these 8 phases:

### Phase 0: Enter Plan Mode
- Enter Claude Code plan mode immediately as the first action
- Write a durable orchestrator operating manual that survives context compaction
- The plan must contain: state directory paths, OpenSpec artifact paths, engine config, skills to use (list below), the full mission flow steps, and rules
- Skills to reference: /grill-me, /ubiquitous-language, /opsx:propose, /review, /code-simplify, /security-review, /test-app, /demo-video, /pr, /opsx:archive

### Phase 1: Grill for Alignment
- Run /grill-me to interview the user relentlessly until shared understanding
- Allow user to skip (with warning about contract quality)

### Phase 2: Establish Shared Language
- Run /ubiquitous-language to create/update UBIQUITOUS_LANGUAGE.md
- Note: if UBIQUITOUS_LANGUAGE.md already exists, read it at the start of the skill. If not, read it after this phase.

### Phase 3: Create OpenSpec Artifacts
- Run /opsx:propose to create proposal.md, design.md, tasks.md
- Each task must be a vertical slice (thin end-to-end path through all layers, demoable, verifiable)
- Follow the /prd-to-plan tracer bullet pattern
- One task = one worker = one commit = one validation cycle
- Update the plan with artifact paths after creation

### Phase 4: Write Validation Contract
- Create contract.md in the change directory with assertions grouped by task
- Assertions must describe observable behavior verifiable from outside the code
- Workers never see the contract (adversarial separation)
- Cover happy path AND error cases
- Be specific and unambiguous

### Phase 5: Create Mission Directory
- Create mission/ directory at openspec/changes/<change-name>/mission/
- Create config.yaml with engine defaults: worker: codex, scrutiny_validator: claude, usertest_validator: claude, demo_video: claude
- Create progress.md with mission name, start timestamp, and "Plan approved" entry
- Create subdirectories: briefs/, handoffs/, validations/, issues/

### Phase 6: Present for Approval
- Show task breakdown, contract assertion counts, engine config
- Allow user to request changes or approve
- Record approval in progress.md

### Phase 7: Build Loop (for each unchecked task in tasks.md)

**Step 1 - Assemble brief:** Write to mission/briefs/task-N.md. Include: task description, relevant excerpts from proposal.md and design.md, procedures (single commit with message "[task-N] description", handoff report format). Brief NEVER contains contract.md content.

**Step 2 - Dispatch worker:** Check engine in config.yaml.
- If codex: Use Monitor tool to kick off `codex exec --full-auto --sandbox workspace-write -m gpt-5.4 -c model_reasoning_effort="high" -o <handoff-path> "<prompt>" 2>/dev/null &` in background. Monitor watches for handoff file (poll every 2s). Include the full bash script in the skill. Timeout: 600000ms (10 minutes).
- If claude: Use Agent tool to spawn subagent with brief content. Subagent writes handoff to same path.

**Step 3 - Handle result:** TASK_COMPLETE -> read handoff, extract commit SHA. TASK_FAILED -> log to progress.md, ask human. Timeout -> log, ask human.

**Step 4 - Scrutiny validation:** Spawn Claude subagent with: commit SHA, contract assertions for this task, and a detailed check sequence (git show, build, test, lint, /review, /code-simplify, /security-review if sensitive, check each assertion PASS/FAIL). Include the full validator prompt in the skill. Validator writes to mission/validations/task-N-scrutiny.md. Validator does NOT receive proposal, design, or worker reasoning.

**Step 5 - Handle validation:** If PASS -> Step 6. If FAIL -> Step 5a (fix loop).

**Step 5a - Fix loop:** Track attempt number. Write failure details to mission/issues/task-N-v[attempt].md. If attempt >= 3: circuit breaker (ask human: retry/skip/abort). If attempt < 3: write fix brief to mission/briefs/task-N-fix-v[attempt+1].md (include what failed, validator output, commit SHA, procedures -- NO contract). Dispatch fix worker. After fix: go back to Step 4 (re-run BOTH validators).

**Step 6 - User-test validation:** Spawn Claude subagent with: commit SHA, contract assertions, instructions to read diff and decide test strategy (backend-only -> HTTP tests, frontend-only -> browser tests via /test-app, both -> both). Include full validator prompt. Validator writes to mission/validations/task-N-usertest.md. Can report SKIP for untestable assertions. On FAIL -> fix loop (Step 5a). On PASS -> Step 6a.

**Step 6a - Demo video (frontend only):** Orchestrator decides if task has frontend component. If yes: spawn subagent to run /demo-video, save to mission/validations/task-N-demo.mp4. Demo failure is non-blocking (log and continue).

**Step 7 - Track progress:** Mark task complete in tasks.md (- [ ] -> - [x]). Append to progress.md with timestamp, task name, commit SHA, validation results. Update plan status. If more tasks -> Step 1. If no more tasks -> Phase 8.

### Phase 8: Mission Completion
- Create draft PR via /pr skill (always draft, never ready-for-review)
- PR body: task list with commit SHAs, validation results table, issues encountered, mission stats
- Log draft PR URL to progress.md
- Update plan status to COMPLETE

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
- Mission, Orchestrator, Worker, Validator, Contract, Assertion, Brief, Handoff, Monitor, Engine, Plan mode, Fix loop, Circuit breaker, Serial execution, Vertical slice, Progress log, Mission directory, Mission config

## Procedures
- Create the directory `darkfactoryV2/.claude/skills/mission/` if it doesn't exist
- Write the complete SKILL.md file with YAML frontmatter (name: mission, description explaining what the skill does)
- Commit all changes in a single commit with message: "[task-1] Generate main mission skill for darkfactoryV2"
- Write your handoff report to openspec/changes/darkfactory-v2/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Main Mission Skill
- Commit SHA: [the full SHA]
- Approach: [brief explanation of what you did and why]
- Commands run: [any commands and their exit codes]
- Concerns: [anything you're uncertain about]
