---
name: mission
description: Start an autonomous Darkfactory mission. Grills the user for alignment, creates OpenSpec artifacts, writes a contract, writes the mission directory, dispatches workers serially, validates each task against the contract, runs fix loops with circuit breakers, and creates a draft PR on completion.
---

# /mission

Start an autonomous multi-agent mission for a single OpenSpec change. The orchestrator (this session) plans the work, dispatches workers one at a time, and validates each result against a contract the workers never see.

**Flow:** `idea -> spec -> build -> verify -> ship -> merge`

---

## Phase 0: Enter Plan Mode

Enter plan mode immediately. Write a durable operating manual as the plan. This plan survives context compaction and is the orchestrator's source of truth.

### Plan Template

Write this plan, filling in values as they become known:

```markdown
# Mission: <change-name>

## Status: PLANNING | APPROVED | BUILDING | COMPLETE | PAUSED

## State Directory
openspec/changes/<change-name>/mission/

## OpenSpec Artifacts
- Proposal: openspec/changes/<change-name>/proposal.md
- Specs: openspec/changes/<change-name>/specs/<capability>/spec.md (one per capability)
- Design: openspec/changes/<change-name>/design.md
- Tasks: openspec/changes/<change-name>/tasks.md (includes mandatory closing tasks)
- Coverage Audit: openspec/changes/<change-name>/coverage-audit.md
- Contract: openspec/changes/<change-name>/contract.md

## Spec Coverage Audit
- PRD issue: <GitHub issue URL from Phase 3>
- Coverage table: <pending | complete>
- Gaps found: <count or "none">
- Gaps resolved: <added N tasks | deferred N | skipped N>

## Engine Config
- Workers: <engine from config.yaml, default codex>
- Scrutiny validator: <engine, default claude>
- User-test validator: <engine, default claude>
- Adversarial reviewer A: <engine, default codex>
- Adversarial reviewer B: <engine, default claude>
- Demo video: <engine, default claude>

## Skills to Use
- /grill-me -- alignment interview (planning)
- /ubiquitous-language -- shared language (planning)
- /write-a-prd -- structured PRD from grill conversation (planning)
- openspec CLI -- create artifacts sequentially: proposal, specs, design, tasks (planning)
- /improve-codebase-architecture -- architecture review for existing codebases (planning, optional)
- /review -- code review (scrutiny validation)
- /simplify -- simplify pass (scrutiny validation)
- /security-review -- security check (scrutiny validation, sensitive tasks only)
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
5. Spawn scrutiny validator (commit SHA + contract assertions)
6. If fail -> write to issues/, spawn fix worker, re-validate. Circuit breaker at 3.
7. Spawn user-test validator (commit SHA + contract assertions)
8. If fail -> same fix/re-validate loop
9. If frontend task -> spawn demo video
10. Mark task done in tasks.md, update progress.md

After all tasks complete:
11. Create draft PR
12. Run dual adversarial review (Codex + Claude) against full diff
13. Present both reports to human alongside PR link

## Rules
- One worker at a time (serial execution)
- Workers never see contract.md
- Validators never see proposal.md or design.md
- On any failure, re-run both per-task validators after fix (scrutiny, user-test)
- Write all decisions to progress.md

## Current Task: <N or "none">
## Current Step: <step name or "none">
```

### First Action

Check if `UBIQUITOUS_LANGUAGE.md` exists in the repo root. If it does, read it now so all phases use consistent terminology.

---

## Phase 1: Grill for Alignment

Run `/grill-me` to interview the user about their idea. The grill session should drive toward shared understanding of what is being built, why, and what "done" looks like.

If the user says "skip":
- Warn: "Skipping the grill means the contract will be based on whatever I can infer. Contract quality depends on alignment quality. Are you sure?"
- If they confirm, proceed to Phase 2.

After the grill completes, summarize the key decisions in the plan.

---

## Phase 2: Establish Shared Language

Run `/ubiquitous-language` to create or update `UBIQUITOUS_LANGUAGE.md`.

- If the file already existed and was read in Phase 0, this phase updates it with any new terms from the grill session.
- If the file did not exist, this phase creates it. Read the file after creation so all subsequent phases use the terms.

---

## Phase 3: Write a PRD

Run `/write-a-prd` to turn the grill conversation into a structured product requirements document. The PRD captures: problem statement, solution, user stories, implementation decisions, testing decisions, and out-of-scope items.

The PRD is filed as a GitHub issue. This issue becomes the source of truth for creating OpenSpec artifacts in Phase 4. The PRD ensures nothing from the grill conversation is lost or misinterpreted during the transition to technical artifacts.

---

## Phase 4: Create and Review OpenSpec Artifacts

The `spec-driven` schema generates four artifacts in dependency order: `proposal -> specs -> design -> tasks`. Generate each artifact one at a time, present it to the human for review, and get approval before generating the next.

**Do NOT run `/opsx:propose` as a single pass.** Instead, orchestrate artifact creation using the openspec CLI commands directly:

1. `openspec new change "<name>"` -- create the change directory
2. `openspec status --change "<name>" --json` -- get artifact order and dependencies
3. For each artifact in order:
   a. `openspec instructions <artifact-id> --change "<name>" --json` -- get instructions for this artifact
   b. Create the artifact file following the instructions and template
   c. `openspec status --change "<name>" --json` -- re-run status to refresh the artifact list (newly unblocked artifacts and per-capability spec IDs appear only after their dependencies are written)

### Review Pattern (used at every review point in planning)

Every artifact review follows this pattern:
1. **AI presents** -- show the artifact content to the human in the chat
2. **AI recommends** -- state whether it looks good, flag any concerns, suggest specific changes if needed
3. **Human decides** -- approve, request changes, or defer to the AI recommendation

The human can always say "looks good" or "go with your recommendation" to keep momentum. If changes are requested, revise and re-present.

### Phase 4.1: Generate and Review Proposal

Generate `proposal.md` (what and why). This establishes the capabilities that will become spec files.

Present it to the human with a recommendation. If approved, proceed to Phase 4.2.

### Phase 4.2: Generate and Review Specs

Generate `specs/<capability>/spec.md` files (one per capability listed in the proposal). Each spec contains formal WHEN/THEN requirements using SHALL/MUST language.

Present all spec files to the human with a recommendation. Specs are the formal requirements that flow into tasks and the contract. Missing or vague specs are the primary cause of missed requirements in the final build.

If approved, proceed to Phase 4.3.

### Phase 4.3: Generate and Review Design

Generate `design.md` (how -- architecture decisions, alternatives considered, risks). Reference the proposal for motivation and specs for requirements.

Design is required for every mission. Even simple changes benefit from a short design document that records what was considered and why the chosen approach was picked. The build loop and worker briefs depend on design.md existing -- skipping it would break downstream phases.

#### Module Depth

For each major component or module in the design, consider its depth. A deep module has a small interface hiding significant implementation. A shallow module has a large interface with thin logic behind it. Prefer deep modules -- they are more testable at the boundary, easier for workers to implement in a single task, and easier for subsequent workers to use correctly.

For each module, describe:
- **Interface** -- what callers see (public API, CLI flags, function signatures)
- **What it hides** -- the complexity behind the interface that callers don't need to know about
- **Why this boundary** -- what makes this the right place to draw the line

If a module has a large interface relative to its implementation, ask whether it should be split differently or whether some of its interface belongs in a higher-level module. A good test: can a worker implement this module in a single task and a validator test it at the boundary? If not, the module may be too shallow or too broad.

Present it to the human with a recommendation. If approved, proceed to Phase 4.3a (existing codebases) or Phase 4.4 (greenfield).

### Phase 4.3a: Architecture Review (Optional)

**Only for missions that modify an existing codebase. Skip for greenfield projects.**

Run `/improve-codebase-architecture` to review the existing codebase in light of the approved design. This identifies:
- Existing modules that need refactoring before the new work can land cleanly
- Shallow abstractions that should be deepened
- Tightly-coupled components that should be separated
- Architectural patterns that conflict with the planned design

The orchestrator recommends whether to run this step based on the mission scope. Present findings to the human. Any refactoring work becomes tasks that go before feature tasks in Phase 4.4.

If skipped or approved, proceed to Phase 4.4.

### Phase 4.4: Generate and Review Tasks

Generate `tasks.md` (ordered checklist of implementation tasks).

#### Task Quality Standard

Each task must be a **vertical slice**: a thin end-to-end path through all layers (schema, API, UI, tests) that delivers narrow but complete functionality.

**One task = one worker = one commit = one validation cycle.**

A task is good if you can demo it and verify it independently after it lands.

#### Mandatory Closing Tasks

Every mission's tasks.md MUST include these closing tasks after all feature tasks:

1. **Cleanup task (mandatory).** Remove unused dependencies, dead code, leftover scaffolding, temporary artifacts, and any backwards-compatibility shims. Workers sometimes leave debris that per-task validation does not catch. This task cleans up the codebase before documentation.

2. **Documentation task (mandatory).** Write or update documentation for what was built. The orchestrator recommends the documentation scope based on what the feature tasks produce -- this varies by mission type. Not every mission needs install instructions, architecture diagrams, or API docs. Use terms from `UBIQUITOUS_LANGUAGE.md` consistently. If unsure what documentation is needed, ask the human during this review.

3. **CI configuration task (optional).** Add or update CI/CD pipeline configuration. The orchestrator recommends whether to include this based on the change scope. The human decides during this review.

If the architecture review (Phase 4.3a) surfaced refactoring work, those tasks go before the feature tasks in tasks.md.

Present the full task list (including closing tasks) to the human with a recommendation. If approved, proceed to Phase 4a.

After all artifacts are reviewed and approved, update the plan with the actual artifact paths.

---

## Phase 4a: Spec Coverage Audit

Before writing the contract, verify that every PRD requirement is covered by at least one task. This phase catches requirements that were present in the PRD but lost during the transition to specs and tasks.

### Steps

1. **Read the PRD** from the GitHub issue created in Phase 3.
2. **Read tasks.md** and all spec files from `specs/`.
3. **Build a coverage table** mapping every PRD requirement to the task(s) that implement it:

```markdown
# Spec Coverage Audit

## Coverage Table
| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | <requirement from PRD> | <WHEN/THEN from spec, or "no spec"> | Task N | COVERED |
| 2 | <requirement from PRD> | <WHEN/THEN from spec, or "no spec"> | -- | GAP |
| 3 | <requirement from PRD> | <WHEN/THEN from spec, or "no spec"> | Task M | COVERED |

## Gaps
| # | PRD Requirement | AI Recommendation | Human Decision |
|---|----------------|-------------------|----------------|
| 2 | <requirement> | Add task / Defer / Skip | <pending> |
```

4. **Flag gaps.** For each uncovered requirement, the AI recommends one of:
   - **Add** -- create a new task (and spec requirement if missing) to cover it
   - **Defer** -- out of scope for this mission, track for later
   - **Skip** -- intentionally not implementing (document why)

5. **Present the coverage table and gap recommendations** to the human. Follow the standard review pattern: AI presents, AI recommends, human decides.

6. **If specs change** (new spec requirements added to cover gaps), re-present the affected spec files for human review before proceeding. The human must approve any new or modified specs.

7. **If tasks change** (new tasks added to cover gaps), re-present the updated tasks.md for review. The human must approve the final task list before proceeding.

8. **Persist the coverage audit to disk.** Write the coverage table to `openspec/changes/<change-name>/coverage-audit.md`. This file is needed for mission resume (so a resumed session can see what was already mapped without re-running the audit) and for contract writing (Phase 5 cross-references it).

9. **Record the coverage audit** in the plan. Update the Spec Coverage Audit section with the PRD issue URL, gap count, and resolutions.

---

## Phase 5: Write Contract

Create `contract.md` in the change directory (`openspec/changes/<change-name>/contract.md`).

### Contract Rules

1. Group assertions by task, matching the task list in tasks.md.
2. Every assertion must describe **observable behavior** verifiable from outside the code (HTTP response, CLI output, file on disk, UI state, build passing, etc.).
3. Do NOT reference implementation details (function names, variable names, internal data structures).
4. Be specific and unambiguous. A validator reading only the contract and the diff should be able to check every assertion without guessing.
5. Workers never see this file. This is the adversarial separation that makes validation meaningful.
6. **Write the minimum set of assertions needed to verify the task is complete and correct.** Each assertion should trace to a specific PRD requirement, including error behaviors specified in the PRD or specs. Do not add assertions for edge cases, nice-to-haves, or defensive behaviors unless the PRD explicitly requires them. If an assertion would not cause you to reject the task when it fails, do not include it.

### Cross-Referencing the PRD

For each task, use the coverage audit from Phase 4a to find which PRD requirements map to it. Write assertions that capture the specific details from those requirements -- not generic behavioral checks, but assertions that would fail if the specific requirement were not implemented.

**Process for each task:**
1. Look up the PRD requirements mapped to this task in the coverage table.
2. Read the corresponding spec requirements (WHEN/THEN scenarios) for precise behavioral expectations.
3. Write assertions that verify those specific behaviors with specific values, endpoints, messages, or states from the PRD and specs.
4. If a PRD requirement mentions specific constraints (limits, formats, error messages, status codes), those constraints must appear in the assertions verbatim.

### Contract Template

```markdown
# Contract

## Task 1: <name>
<!-- PRD Requirements: #1, #3 -->
- [ ] <assertion capturing specific PRD/spec requirement detail>
- [ ] <assertion capturing specific PRD/spec requirement detail>

## Task 2: <name>
<!-- PRD Requirements: #2 -->
- [ ] <assertion capturing specific PRD/spec requirement detail>
- [ ] <assertion capturing specific PRD/spec requirement detail>
```

### Contract Review

After writing the contract, present it to the human following the standard review pattern: AI presents, AI recommends, human decides.

For each task section, highlight:
- Which PRD requirements the assertions trace to
- Whether the assertion count feels right (too many suggests over-specification; too few suggests gaps)
- Any PRD requirements that were hard to express as observable assertions

If the human requests changes, revise and re-present.

Update the plan with the contract path after approval.

---

## Phase 6: Create Mission Directory

Create the mission directory at `openspec/changes/<change-name>/mission/` with this structure:

```
mission/
  config.yaml
  progress.md
  briefs/
  handoffs/
  validations/
  issues/
```

### config.yaml

```yaml
engines:
  worker: codex
  scrutiny_validator: claude
  usertest_validator: claude
  adversarial_reviewer_a: codex
  adversarial_reviewer_b: claude
  demo_video: claude
```

### progress.md

```markdown
# Mission: <change-name>

Started: <ISO 8601 timestamp>

## Log

- <timestamp> -- Mission directory created. Awaiting approval.
```

Create the four subdirectories: `briefs/`, `handoffs/`, `validations/`, `issues/`.

---

## Phase 7: Launch Confirmation

Since every artifact has been individually reviewed and approved during Phases 4, 4a, and 5, this phase is a lightweight launch confirmation -- not a full review.

Show the user a summary dashboard:
1. **Mission overview** -- change name, total tasks (feature + closing), total contract assertions
2. **Coverage audit result** -- number of PRD requirements covered, any deferred/skipped items
3. **Engine config** -- which engine runs each role (from config.yaml)

Ask: "Everything has been reviewed. Ready to start the build?"

- If yes: append `<timestamp> -- Plan approved. <N> tasks, <M> assertions in contract.` to progress.md. Update plan status to BUILDING. Proceed to Phase 8.
- If the human wants to revisit something: loop back to the specific phase they name. Do not re-review everything.

---

## Phase 8: Build Loop

For each unchecked task (`- [ ]`) in tasks.md, run Steps 1-7. Process tasks in order, one at a time (serial execution). Skip tasks marked `- [~]` (skipped) and `- [x]` (complete).

### Step 1: Assemble Brief

Write a brief to `mission/briefs/task-N.md`.

#### Brief Template

```markdown
# Worker Brief: Task N - <name>

## Your Task
<task description from tasks.md, expanded with acceptance criteria>

## Project Context
<relevant excerpt from proposal.md -- what is being built and why>

## Architecture
<relevant excerpt from design.md -- how this task fits into the system>

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
<any domain terms the worker needs to understand>

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-N] <description>"
- Write your handoff report to openspec/changes/<change-name>/mission/handoffs/task-N.md

## Handoff Report Format
- Task: Task N - <name>
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
```

**The brief NEVER contains contract.md content.** This is the adversarial separation.

### Step 2: Dispatch Worker

Read the engine from `mission/config.yaml`.

#### If engine is `codex`:

Use the Monitor tool to run the worker in the background. Use this exact bash script:

```bash
BRIEF_PATH="<absolute path to mission/briefs/task-N.md>"
HANDOFF_PATH="<absolute path to mission/handoffs/task-N.md>"
BRIEF_CONTENT=$(cat "$BRIEF_PATH")

# Start Codex in background
codex exec \
  --full-auto \
  --sandbox workspace-write \
  -m gpt-5.4 \
  -c model_reasoning_effort="high" \
  -o "$HANDOFF_PATH" \
  "$BRIEF_CONTENT" \
  2>/dev/null &

CODEX_PID=$!

# Poll for handoff file or process exit
while true; do
  if [ -f "$HANDOFF_PATH" ]; then
    echo "TASK_COMPLETE: Handoff written to $HANDOFF_PATH"
    break
  fi
  if ! kill -0 $CODEX_PID 2>/dev/null; then
    echo "TASK_FAILED: Codex process exited without writing handoff"
    break
  fi
  sleep 2
done
```

Use Monitor with a timeout of 600000ms (10 minutes). When Monitor reports:
- `TASK_COMPLETE` -> proceed to Step 3
- `TASK_FAILED` -> proceed to Step 3 (failure path)
- Timeout -> proceed to Step 3 (timeout path)

#### If engine is `claude`:

Use the Agent tool to spawn a subagent. Pass the full brief content as the prompt. Instruct the subagent to write its handoff to the same path (`mission/handoffs/task-N.md`).

### Step 3: Handle Result

Read the handoff file at `mission/handoffs/task-N.md`.

- **TASK_COMPLETE** (handoff exists and contains a commit SHA): Extract the commit SHA. Log to progress.md: `<timestamp> -- Task N: worker completed. Commit: <SHA>`. Proceed to Step 4.
- **TASK_FAILED** (handoff missing or no commit SHA): Log to progress.md: `<timestamp> -- Task N: worker FAILED. <reason>`. Ask the human: retry, skip, or abort. If skip: mark the task as `- [~]` in tasks.md and log `<timestamp> -- Task N: SKIPPED by human.` to progress.md. The build loop ignores `- [~]` tasks.
- **Timeout** (Monitor timed out): Log to progress.md: `<timestamp> -- Task N: worker TIMED OUT after 10 minutes.`. Ask the human: retry, skip, or abort. If skip: mark the task as `- [~]` in tasks.md and log `<timestamp> -- Task N: SKIPPED by human.` to progress.md. The build loop ignores `- [~]` tasks.

### Step 4: Scrutiny Validation

Spawn a Claude subagent (via the Agent tool) with this prompt:

#### Scrutiny Validator Prompt

```
You are a scrutiny validator for the Darkfactory mission system. Your job is to verify that a worker's commit meets the validation contract. You have NOT seen the proposal, design, or worker reasoning. You judge only by the code and the contract.

## Commit to Validate
SHA: <commit-sha>

## Contract Assertions for This Task
<paste the assertions from contract.md for task N>

## Check Sequence

Run these checks in order. For each, report PASS or FAIL with evidence.

1. **Test presence gate:** Run `git show <commit-sha> --stat` and check if any test files were added or modified. If the diff contains zero test files and zero new test assertions, this is an automatic FAIL. Workers are required to use /tdd -- no new tests means TDD was not followed.

2. **Build:** Run the project's build command. Does it pass?

3. **Tests:** Run the project's test command. Do all tests pass?

4. **Lint:** Run the project's lint command. Are there any violations?

5. **Code review:** Run /review against the diff (this includes reading the full diff). Check specifically for:
   - Bugs: off-by-one errors, null/undefined access, race conditions, missing error handling at boundaries
   - Security: injection, auth issues, secrets in code, unsafe input handling
   - Logic: wrong conditions, missing edge cases, incorrect assumptions
   - Breaking changes: API contract changes, removed exports, changed behavior
   Report findings by severity (critical / warning / nit) with file:line references.

6. **Code standards check:** Review the diff for these specific anti-patterns. Flag any found:
   - Backwards-compatibility code: old code paths kept "just in case" (renamed `_` variables, re-exports of removed types, `// removed` comments, deprecated shims). If unused, it should be deleted.
   - Fallback lookups: code that tries one path and falls back to another. There should be exactly one lookup path. If it fails, throw an error.
   - Belt-and-suspenders: redundant validation that duplicates what the framework, DB, or upstream already guarantees (re-checking auth after middleware, null-checking non-null TypeScript values, re-validating input that a validator already checked).
   - Feature flags for dead code: if the old way is being replaced, delete it. Don't gate behind a flag.
   - Defensive handling of impossible states: catch blocks or null checks for conditions that cannot occur given current code paths.

7. **Code simplify:** Run /simplify against the changed files. Report any additional unnecessary complexity beyond the standards check above.

8. **Security review (if applicable):** If the diff touches authentication, authorization, secrets, user input handling, or network boundaries, run /security-review against the diff. Report findings by severity (Critical, High, Medium). Otherwise note "N/A -- no sensitive paths touched."

9. **Contract assertions:** For each assertion in the contract, check it against the diff and test results. Mark each assertion PASS or FAIL with a one-line explanation.

## Output Format

Write your report to: <absolute path to mission/validations/task-N-scrutiny.md>

Use this format:

```markdown
# Scrutiny Validation: Task N

## Result: PASS | FAIL

## Check Results
| Check | Result | Notes |
|-------|--------|-------|
| Test presence | PASS/FAIL | ... |
| Build | PASS/FAIL | ... |
| Tests | PASS/FAIL | ... |
| Lint | PASS/FAIL | ... |
| Code review | PASS/FAIL | ... |
| Code standards | PASS/FAIL | ... |
| Code simplify | PASS/FAIL | ... |
| Security review | PASS/FAIL/N/A | ... |

## Contract Assertions
| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | <assertion text> | PASS/FAIL | <one-line evidence> |
| 2 | ... | ... | ... |

## Failed Assertions (if any)
<detailed explanation of each failure>

## Recommendations (if FAIL)
<specific guidance for fixing the failures>
```

IMPORTANT: The overall result is FAIL if ANY check fails or ANY contract assertion fails. Be strict. A near-miss is still a FAIL.
```

Read the validator's report from `mission/validations/task-N-scrutiny.md`.

### Step 5: Handle Scrutiny Validation Result

- **PASS**: Log to progress.md: `<timestamp> -- Task N: scrutiny validation PASSED.` Proceed to Step 6.
- **FAIL**: Proceed to Step 5a (fix loop).

### Step 5a: Fix Loop

Track the attempt number (starts at 1 for the initial worker, increments on each fix).

1. **Write failure details** to `mission/issues/task-N-v<attempt>.md`:

```markdown
# Issue: Task N - <name> (Attempt <attempt>)

## Failed Checks
<list of failed checks from validator report>

## Observed Symptoms
<describe what went wrong in terms of behavior -- error messages, test failures, unexpected output. Do NOT copy contract assertion text here.>

## Validator Recommendations
<recommendations from validator>
```

2. **Circuit breaker check:** If attempt >= 3, stop the loop. Log to progress.md: `<timestamp> -- Task N: circuit breaker triggered after <attempt> attempts.` Update plan status to PAUSED. Ask the human: "Task N has failed validation 3 times. Options: retry (allows one more attempt, counter continues from current value), skip (mark as skipped with `- [~]`, continue to next task), abort (stop the mission)."

   - **Retry:** Increment the attempt counter by 1 (e.g., attempt 3 becomes attempt 4). Allow one more fix attempt. If it fails again, the circuit breaker triggers again at the new count. Retry does NOT reset the counter.
   - **Skip:** Mark the task as `- [~]` in tasks.md. Log to progress.md: `<timestamp> -- Task N: SKIPPED by human after <attempt> failed attempts.` Continue to the next unchecked task. The build loop ignores `- [~]` tasks (they are neither unchecked nor complete).
   - **Abort:** Update plan status to PAUSED. Stop the build loop. Report to the human.

3. **Write fix brief** to `mission/briefs/task-N-fix-v<attempt+1>.md`:

#### Fix Brief Template

```markdown
# Fix Brief: Task N - <name> (Attempt <attempt+1>)

## What Failed
<describe the observed symptoms -- error messages, unexpected behavior, test failures. Do NOT copy assertion text from the contract. Describe what went wrong, not what the contract expected.>

## Validator Output
<relevant error messages, build/test output, review findings -- symptoms only>

## Commit to Fix
<SHA of the commit that failed validation>

## Procedures
- Use `/tdd` to fix the issues described above
- Do not refactor or change anything unrelated
- Commit as a single commit with message: "[task-N] fix: <description of what was fixed>"
- Write handoff report to openspec/changes/<change-name>/mission/handoffs/task-N-fix-v<attempt+1>.md

## Handoff Report Format
- Task: Task N - <name> (Fix attempt <attempt+1>)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
```

**The fix brief NEVER contains contract.md content.** Same adversarial separation as the original brief.

4. **Dispatch fix worker** using the same engine and method as Step 2, but with the fix brief.

5. **After fix completes:** Go back to **Step 4** (re-run scrutiny validation with the new commit SHA). Both per-task validators must re-run after every fix (scrutiny and user-test), not just the one that failed.

### Step 6: User-Test Validation

Spawn a Claude subagent (via the Agent tool) with this prompt:

#### User-Test Validator Prompt

```
You are a user-test validator for the Darkfactory mission system. Your job is to verify that a worker's commit delivers the correct user-facing behavior. You have NOT seen the proposal, design, or worker reasoning.

## Commit to Validate
SHA: <commit-sha>

## Contract Assertions for This Task
<paste the assertions from contract.md for task N>

## Instructions

1. **Read the diff:** Run `git show <commit-sha>` to understand what changed.

2. **Decide your test strategy** based on what the diff touches:
   - **Backend only** (API routes, database, server logic, CLI): Write and run HTTP tests against the API endpoints. Use curl or a test script.
   - **Frontend only** (UI components, pages, styles): Run /test-app to drive a browser and verify UI behavior.
   - **Both backend and frontend**: Run HTTP tests first, then /test-app for the UI.
   - **Infrastructure only** (config, build, CI): Verify the build passes and config is correct. You may mark behavioral assertions as SKIP with reason "infrastructure-only change, not testable via HTTP or browser."

3. **Check each contract assertion:** For each assertion, run the appropriate test. Mark PASS, FAIL, or SKIP.
   - PASS: The assertion is demonstrably true.
   - FAIL: The assertion is demonstrably false, or the test reveals a bug.
   - SKIP: The assertion cannot be tested from outside (e.g., internal refactor). Provide a reason.

## Output Format

Write your report to: <absolute path to mission/validations/task-N-usertest.md>

Use this format:

```markdown
# User-Test Validation: Task N

## Result: PASS | FAIL

## Test Strategy
<what you decided and why>

## Tests Run
| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | <what was tested> | HTTP/Browser/Manual | PASS/FAIL |
| 2 | ... | ... | ... |

## Contract Assertions
| # | Assertion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | <assertion text> | PASS/FAIL/SKIP | <evidence or skip reason> |
| 2 | ... | ... | ... |

## Failed Assertions (if any)
<detailed explanation of each failure>

## Recommendations (if FAIL)
<specific guidance for fixing the failures>
```

IMPORTANT: The overall result is FAIL if ANY contract assertion fails (SKIP does not count as failure). Be thorough -- actually run the tests, don't just read the code and guess.
```

Read the validator's report from `mission/validations/task-N-usertest.md`.

- **PASS**: Log to progress.md: `<timestamp> -- Task N: user-test validation PASSED.` Proceed to Step 6a (demo video).
- **FAIL**: Enter the fix loop (Step 5a) with the user-test failure details. After the fix, re-run **both** validators (Step 4, then Step 6).
- **SKIP** (all assertions skipped): Log to progress.md: `<timestamp> -- Task N: user-test validation SKIPPED (all assertions infrastructure-only).` Proceed to Step 6a (demo video).

### Step 6a: Demo Video (Frontend Only)

The orchestrator decides whether this task has a frontend component by checking the diff. If the task touches UI files (components, pages, styles, templates), it has a frontend component.

**If frontend component exists:**

Spawn a Claude subagent to run `/demo-video`. The subagent should save the recording to `mission/validations/task-N-demo.mp4`.

**Demo failure is non-blocking.** If the demo fails to record:
- Log to progress.md: `<timestamp> -- Task N: demo video FAILED to record. <reason>. Continuing.`
- Proceed to Step 7.

**If no frontend component:** Skip this step. Proceed to Step 7.

### Step 7: Track Progress

1. Mark the task complete in `tasks.md`: change `- [ ]` to `- [x]` for this task.

2. Append to `progress.md`:

```markdown
- <timestamp> -- Task N complete.
  - Commit: <SHA>
  - Scrutiny: PASS
  - User-test: PASS | SKIP
  - Demo: RECORDED | SKIPPED | FAILED
  - Fix attempts: <number, 0 if first try passed>
```

3. Update the plan: set Current Task to the next unchecked task, or "none" if all done.

4. **If more tasks remain:** Go back to Step 1 for the next task.

5. **If all tasks are done:** Proceed to Phase 9.

---

## Phase 9: Mission Completion

### Create Draft PR

Create a **draft** pull request using `gh pr create --draft`. Never create a ready-for-review PR. Use the PR body template below.

#### PR Body Template

```markdown
## Mission: <change-name>

### Tasks
| # | Task | Commit | Status |
|---|------|--------|--------|
| 1 | <name> | <short SHA> | Done |
| 2 | <name> | <short SHA> | Done |

### Validation Results
| Task | Scrutiny | User-Test | Demo | Fix Attempts |
|------|----------|-----------|------|-------------|
| 1 | PASS | PASS | N/A | 0 |
| 2 | PASS | PASS | RECORDED | 1 |

### Issues Encountered
<list any issues from mission/issues/, or "None">

### Mission Stats
- Total tasks: <N>
- Total assertions: <M>
- Total fix attempts: <sum>
- Circuit breakers triggered: <count>
- Duration: <start to end>
```

### Dual Adversarial Review

After the draft PR is created, run a final adversarial review against the full diff of all mission commits. This is a human-facing report, not an automated gate.

Run two independent reviewers in parallel:
- **Reviewer A** (engine from `adversarial_reviewer_a` in config.yaml, default: codex) -- dispatched via Monitor
- **Reviewer B** (engine from `adversarial_reviewer_b` in config.yaml, default: claude) -- dispatched via Agent tool

Both receive the full diff (`git diff <base-branch>...HEAD`) and the full contract.md.

#### Adversarial Reviewer Prompt (used for both A and B)

```
You are an adversarial reviewer. Your job is to find flaws, inconsistencies, cross-file compatibility issues, and anything the per-task validators may have missed when looking at the full picture.

## Full Diff
Run `git diff <base-branch>...HEAD` to see all changes from this mission.

## Full Validation Contract
<paste the ENTIRE contract.md>

## Instructions

1. Read the full diff.
2. Look for:
   - Logic errors or edge cases not covered by tests
   - Cross-file inconsistencies (types, interfaces, naming)
   - Contract assertions that technically pass but have subtle flaws
   - Security issues, race conditions, or data integrity problems
   - Missing error handling or graceful degradation
   - Integration issues between different tasks' code
3. For each finding, classify severity as HIGH, MEDIUM, or LOW.
4. Be adversarial: assume the code is wrong until proven right.

## Output Format

Write your report to: <absolute path to mission/validations/adversarial-{a|b}.md>

```markdown
# Adversarial Review {A|B}

## Findings
| # | Category | Severity | Finding |
|---|----------|----------|---------|
| 1 | <category> | HIGH/MEDIUM/LOW | <description> |
| 2 | ... | ... | ... |

## Details
<detailed explanation of each finding with evidence and suggested fix>

## Summary
<overall assessment>
```
```

Dispatch both in parallel. Wait for both to complete.

### Present to Human

Log to progress.md: `<timestamp> -- Draft PR created: <URL>. Adversarial reviews complete.`

Present the results to the user:

```
Mission complete.

Draft PR: <URL>

Adversarial Review A (Codex):
<summary of findings -- count by severity>

Adversarial Review B (Claude):
<summary of findings -- count by severity>

Full reports at:
- mission/validations/adversarial-a.md
- mission/validations/adversarial-b.md

Review the PR and the adversarial findings. You decide what to address.
After merge, run /opsx:archive to archive the change.
```

Update plan status to COMPLETE.

---

## Resume Support

If `/mission-resume` is invoked (via the mission-resume skill), the orchestrator should:

1. Enter plan mode.
2. Find the mission directory by scanning `openspec/changes/*/mission/config.yaml`.
3. Read `progress.md` to determine the last completed task.
4. Read `tasks.md` to find the next unchecked task.
5. Rebuild the plan from file state.
6. Continue from the appropriate step in Phase 8.

This skill does not implement resume directly -- that is handled by the `/mission-resume` skill. But the file-backed state design ensures resume is always possible.

---

## Status Checking

Running `/mission-status` (via the mission-status skill) in a separate session should show the current state by reading `progress.md`, `tasks.md`, and `config.yaml`. This skill does not implement status -- that is handled by the `/mission-status` skill.

---

## Rules (Always Enforced)

1. **Serial execution.** One worker at a time. Never dispatch two workers in parallel. (Exception: the two adversarial reviewers in Phase 9 run in parallel since they are independent read-only reviewers.)
2. **Adversarial separation.** Workers never see `contract.md`. Validators never see `proposal.md` or `design.md`.
3. **Both per-task validators re-run after every fix.** Even if only one failed, both scrutiny and user-test re-run to catch regressions. Adversarial review runs once at the end, not per task.
4. **Circuit breaker at 3.** After 3 failed attempts on the same task, stop and ask the human. Retry allows one more attempt but does NOT reset the counter.
5. **File-backed state.** All decisions and status go to disk. The mission must be resumable from files alone.
6. **Single commit per task.** One task, one worker, one commit, one validation cycle.
7. **Draft PRs only.** Never create a ready-for-review PR. The human promotes it.
8. **Plan is the operating manual.** Keep the plan updated after every phase and step. It must be accurate if context compacts.
9. **Skipped tasks use `- [~]`.** The build loop treats `- [~]` as neither unchecked nor complete. Skipped tasks are not re-attempted.
10. **Monitor is for external processes.** Monitor is used for external process dispatch (Codex workers, Codex adversarial reviews). Claude subagent validators use the Agent tool directly since they run within the session and return synchronously.
