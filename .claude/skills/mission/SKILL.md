---
name: mission
description: Start an autonomous Darkfactory mission. Grills the user for alignment, creates OpenSpec artifacts, writes a contract, writes the mission directory, dispatches workers serially, validates each task against the contract, runs fix loops with circuit breakers, and creates a draft PR on completion.
---

# /mission

Start an autonomous multi-agent mission for a single OpenSpec change. The orchestrator (this session) plans the work, dispatches workers one at a time, and validates each result against a contract the workers never see.

**Flow:** `align -> spec -> contract -> build -> validate -> ship`

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

---

## Phase Index

0. **Enter Plan Mode** -- write the durable operating plan
1. **Grill for Alignment** -- align on what is being built and why
2. **Establish Shared Language** -- create/update UBIQUITOUS_LANGUAGE.md
3. **Write a PRD** -- file requirements as a GitHub issue
4. **Create and Review OpenSpec Artifacts** -- proposal, specs, design, tasks (human-approved)
4a. **Spec Coverage Audit** -- verify every PRD requirement maps to a task
5. **Write Contract** -- validation assertions workers never see
6. **Create Mission Directory** -- set up mission/ state directory
7. **Launch Confirmation** -- human confirms, build starts
8. **Build Loop** -- dispatch workers, validate per task, fix loop with circuit breaker
9. **Mission Completion** -- final gates, spec delta check, draft PR, dual adversarial review

---

## Phase 0: Enter Plan Mode

> **Rules:** 5, 8
> **Goal:** Write the durable operating plan that survives context compaction.
> **Inputs:** User's idea (from conversation), UBIQUITOUS_LANGUAGE.md (if exists)
> **Outputs:** Plan (in plan mode), initial UBIQUITOUS_LANGUAGE.md check

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
- PRD issue number: <N>
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

## Codebase Deltas
<Accumulated after each task passes validation. Used to populate "What Previous Tasks Built" in subsequent briefs. Generated from git diff, never from handoff reports.>

## Current Task: <N or "none">
## Current Step: <step name or "none">
```

### First Action

Check if `UBIQUITOUS_LANGUAGE.md` exists in the repo root. If it does, read it now so all phases use consistent terminology.

**Gate -- proceed when:**
- [ ] Plan is written and active in plan mode
- [ ] UBIQUITOUS_LANGUAGE.md checked (read if exists, noted if absent)

---

## Phase 1: Grill for Alignment

> **Rules:** --
> **Goal:** Align on what is being built, why, and what "done" looks like.
> **Inputs:** User's idea
> **Outputs:** Key decisions summarized in plan

Run `/grill-me` to interview the user about their idea. The grill session should drive toward shared understanding of what is being built, why, and what "done" looks like.

If the user says "skip":
- Warn: "Skipping the grill means the contract will be based on whatever I can infer. Contract quality depends on alignment quality. Are you sure?"
- If they confirm, proceed to Phase 2.

After the grill completes, summarize the key decisions in the plan.

**Gate -- proceed when:**
- [ ] Grill complete or explicitly skipped (with warning acknowledged)
- [ ] Key decisions summarized in plan

---

## Phase 2: Establish Shared Language

> **Rules:** 5
> **Goal:** Establish shared terminology for all subsequent phases.
> **Inputs:** Grill conversation, existing UBIQUITOUS_LANGUAGE.md (if any)
> **Outputs:** UBIQUITOUS_LANGUAGE.md (created or updated)

Run `/ubiquitous-language` to create or update `UBIQUITOUS_LANGUAGE.md`.

- If the file already existed and was read in Phase 0, this phase updates it with any new terms from the grill session.
- If the file did not exist, this phase creates it. Read the file after creation so all subsequent phases use the terms.

**Gate -- proceed when:**
- [ ] UBIQUITOUS_LANGUAGE.md exists and is loaded into context

---

## Phase 3: Write a PRD

> **Rules:** 5
> **Goal:** Capture requirements as a structured PRD filed as a GitHub issue.
> **Inputs:** Grill conversation, UBIQUITOUS_LANGUAGE.md
> **Outputs:** GitHub issue (PRD), issue URL + number recorded in plan

Run `/write-a-prd` to turn the grill conversation into a structured product requirements document. The PRD captures: problem statement, solution, user stories, implementation decisions, testing decisions, and out-of-scope items.

The PRD is filed as a GitHub issue. This issue becomes the source of truth for creating OpenSpec artifacts in Phase 4. The PRD ensures nothing from the grill conversation is lost or misinterpreted during the transition to technical artifacts.

After the issue is created, record both the issue URL and the issue number in the plan (under Spec Coverage Audit). The issue number is needed in Phase 9 to link the draft PR back to the issue.

**Gate -- proceed when:**
- [ ] PRD filed as GitHub issue
- [ ] Issue URL and issue number recorded in plan

---

## Phase 4: Create and Review OpenSpec Artifacts

> **Rules:** 5
> **Goal:** Generate and get human approval on all four OpenSpec artifacts.
> **Inputs:** PRD (GitHub issue), UBIQUITOUS_LANGUAGE.md
> **Outputs:** proposal.md, specs/, design.md, tasks.md -- all human-approved

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

#### Existing Codebases

When the mission modifies an existing codebase (not greenfield), the design must evaluate the current code structure against the planned architecture. For each module the design touches, read the existing code and assess whether it aligns with the target design. If existing modules are shallow, tightly coupled, or conflict with the planned architecture, the design should call this out explicitly and recommend refactoring as part of the implementation. These refactoring decisions flow naturally into task ordering in Phase 4.4 -- refactoring tasks go before the feature tasks that depend on them.

Present it to the human with a recommendation. If approved, proceed to Phase 4.4.

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

If the design (Phase 4.3) identified refactoring needed for existing code, those tasks go before the feature tasks in tasks.md.

Present the full task list (including closing tasks) to the human with a recommendation. If approved, proceed to Phase 4a.

After all artifacts are reviewed and approved, update the plan with the actual artifact paths.

**Gate -- proceed when:**
- [ ] proposal.md human-approved
- [ ] All specs/<capability>/spec.md files human-approved
- [ ] design.md human-approved
- [ ] tasks.md (with closing tasks) human-approved
- [ ] Plan updated with actual artifact paths

---

## Phase 4a: Spec Coverage Audit

> **Rules:** 5
> **Goal:** Verify every PRD requirement maps to at least one task before writing the contract.
> **Inputs:** PRD (GitHub issue), specs/, tasks.md
> **Outputs:** coverage-audit.md, updated specs/tasks if gaps found

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
| 4 | <requirement from PRD> | <WHEN/THEN from spec, or "no spec"> | Task K | APPROVED-DEVIATION |

## Gaps
| # | PRD Requirement | AI Recommendation | Human Decision |
|---|----------------|-------------------|----------------|
| 2 | <requirement> | Add task / Defer / Skip | <pending> |

## Deviations Requiring Approval
| # | PRD Requirement | Classification | What spec says | What we're doing instead | Prerequisites met? | Impact | Human Decision |
|---|----------------|---------------|----------------|-------------------------|-------------------|--------|----------------|
| 4 | <requirement> | Substitution | <original> | <deviation> | All covered / GAP: <missing> | <consequence> | <pending> |
```

The "Prerequisites met?" column applies to Substitutions only. For Interpretations and Removals, write "--".

4. **Flag gaps and classify deviations.** For each requirement that is not directly covered:

   **If GAP (not implemented, no plan to implement differently):**
   - **Add** -- create a new task (and spec requirement if missing) to cover it
   - **Defer** -- out of scope for this mission, track for later
   - **Skip** -- intentionally not implementing (document why)

   **If DEVIATION (implementing differently than the spec says):**
   Classify the deviation into one of three categories:
   - **Interpretation** -- the spec is ambiguous and this is a reasonable reading (e.g., "cap at B" could mean cap-down or cap-up). Requires AI explanation. Approved in batch with other COVERED items.
   - **Substitution** -- replacing the specified approach with a different one (e.g., Podman instead of bwrap). Requires explicit human approval with the specific trade-off stated: "The spec says X. We are doing Y instead because Z. This means [concrete consequence]."
   - **Removal** -- not implementing a specified feature (e.g., no thin-tier fallback). Requires explicit human approval with the impact stated: "Users who need X will not be able to use the tool for that purpose."

   #### Prerequisite check for substitutions

   For every Substitution deviation, ask: **"Does the deviated approach require implementation work that the mapped task will not cover?"**

   Break down the deviated approach into its prerequisites. If any prerequisite is not covered by the mapped task or another existing task, add it to the `## Gaps` table and route it through the normal gap remediation flow (Add task / Defer / Skip). The deviation cannot be approved until all its prerequisites are resolved. Resolution options:
   - **Add a task** to cover the missing prerequisite (goes through the gap flow, triggers spec/task re-review)
   - **Expand the mapped task** to include the prerequisite (update tasks.md, re-present for review)
   - **Reclassify as Removal** if the prerequisite is too large or out of scope (this forces the human to acknowledge the requirement is being dropped, not substituted)

   Example of this check catching a gap: PRD says "detect SIGINT during promotion and prompt for resume or cleanup." Deviation says "auto-cleanup instead of interactive prompt" (Substitution, mapped to the cleanup task). The deviated approach (auto-cleanup on next run) requires signal handling to detect interruption and mark state as dirty. The cleanup task only handles stale state detection -- it does not implement signal handling. **This is a gap.** Either add a signal handling task, expand the cleanup task to include signal handling, or reclassify as Removal ("we are not implementing interrupted promotion detection").

   Present the prerequisite analysis for each Substitution alongside the deviation in the review step:

   ```
   Deviation #54: Interrupted promotion recovery
   Classification: Substitution
   Original: SIGINT detection + resume/cleanup prompt
   Deviated: Auto-cleanup on next run (no interactive prompt)
   Prerequisites of deviated approach:
     ✓ Stale state detection (covered by Task 3.3)
     ✗ Signal handling to mark state as dirty on interruption (NOT covered by any task)
   → GAP: Add signal handling to Task 3.3, or add a new task, or reclassify as Removal
   ```

   Mark approved deviations as **APPROVED-DEVIATION** in the coverage table (not COVERED). This status persists through the contract and is re-surfaced during the spec delta check (Phase 9).

   **Present substitutions and removals in a dedicated review step.** After presenting the main coverage table, present all substitution and removal deviations together in a single batch. The human approves or rejects each individually within that step. Do not mix them with the COVERED items review. Interpretations may be batch-approved with the main table.

5. **Present the coverage table and gap recommendations** to the human. Follow the standard review pattern: AI presents, AI recommends, human decides.

6. **If specs change** (new spec requirements added to cover gaps), re-present the affected spec files for human review before proceeding. The human must approve any new or modified specs.

7. **If tasks change** (new tasks added to cover gaps), re-present the updated tasks.md for review. The human must approve the final task list before proceeding.

8. **Persist the coverage audit to disk.** Write the coverage table to `openspec/changes/<change-name>/coverage-audit.md`. This file is needed for mission resume (so a resumed session can see what was already mapped without re-running the audit) and for contract writing (Phase 5 cross-references it). For each Substitution deviation, include the prerequisite analysis (which prerequisites were checked, which were covered, which were gaps, and how they were resolved). This ensures a resumed session knows why a substitution was approved safe.

9. **Record the coverage audit** in the plan. Update the Spec Coverage Audit section with the PRD issue URL, gap count, and resolutions.

**Gate -- proceed when:**
- [ ] Coverage audit complete -- every PRD requirement mapped
- [ ] All gaps resolved (added, deferred, or skipped)
- [ ] All deviations classified and approved by human
- [ ] Changed specs/tasks re-approved by human (if any changed)
- [ ] coverage-audit.md persisted to disk
- [ ] Plan updated with audit results

---

## Phase 5: Write Contract

> **Rules:** 2, 5
> **Goal:** Write the validation contract that workers never see.
> **Inputs:** coverage-audit.md, specs/, tasks.md, PRD (GitHub issue)
> **Outputs:** contract.md (human-approved, testability pre-flight passed)

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

Each assertion must include a **verification method** tag indicating how a validator should check it:
- `[execution]` -- verifiable by running a command, HTTP request, or test script (strongest)
- `[output]` -- verifiable by checking command output, file contents, or API response
- `[inspection]` -- verifiable only by reading code (weakest)

Cap `[inspection]` assertions at **20% of total per task**. If more than 20% require inspection, rewrite them as `[execution]` or `[output]` where possible. If an assertion genuinely cannot be made executable, it is acceptable -- but flag it during contract review.

```markdown
# Contract

## Task 1: <name>
<!-- PRD Requirements: #1, #3 -->
- [ ] [execution] <assertion capturing specific PRD/spec requirement detail>
- [ ] [output] <assertion capturing specific PRD/spec requirement detail>

## Task 2: <name>
<!-- PRD Requirements: #2, #4 (DEVIATED) -->
- [ ] [execution] <assertion capturing specific PRD/spec requirement detail>
- [ ] [inspection] <assertion capturing the deviated behavior as actually implemented>
```

When a PRD requirement traces to an APPROVED-DEVIATION item in the coverage audit, mark it with `(DEVIATED)` in the PRD Requirements comment. The assertion itself describes the actual implemented behavior (the deviation), not the original spec requirement. This gives the human visibility during contract review into which assertions exist because of an approved deviation.

### Contract Review

After writing the contract, present it to the human following the standard review pattern: AI presents, AI recommends, human decides.

For each task section, highlight:
- Which PRD requirements the assertions trace to
- Whether the assertion count feels right (too many suggests over-specification; too few suggests gaps)
- Any PRD requirements that were hard to express as observable assertions

If the human requests changes, revise and re-present.

### Contract Testability Pre-flight

After the human approves the contract, run a quick testability audit before proceeding. For each assertion, verify:

1. **Executable assertions** (`[execution]`) -- confirm the assertion describes a concrete action and expected result that a validator could script (e.g., "POST /api/users with {email: 'a@b.com'} returns 201"). If the assertion is too vague to script, flag it.
2. **Output assertions** (`[output]`) -- confirm the assertion names a specific file, value, or output to check. If it says "appropriate error message" instead of a specific message, flag it.
3. **Inspection assertions** (`[inspection]`) -- check whether the assertion could be rewritten as `[execution]` or `[output]`. If yes, rewrite it. If genuinely not possible (e.g., "no direct database access outside the repository layer"), keep it.

If any assertions are flagged as ambiguous:
- Rewrite them to be specific and re-present only the changed assertions to the human for quick approval
- Do not re-review the entire contract

Log the testability breakdown to progress.md: `<timestamp> -- Contract testability pre-flight: <N> execution, <M> output, <P> inspection (<Q>% inspection).`

Update the plan with the contract path after approval.

**Gate -- proceed when:**
- [ ] contract.md human-approved
- [ ] Testability pre-flight passed (inspection assertions <=20%)
- [ ] Testability breakdown logged to progress.md
- [ ] Plan updated with contract path

---

## Phase 6: Create Mission Directory

> **Rules:** 5
> **Goal:** Create the mission directory with config and progress tracking files.
> **Inputs:** Change name, engine preferences
> **Outputs:** mission/ directory with config.yaml, progress.md, and subdirectories

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

**Gate -- proceed when:**
- [ ] mission/ directory exists with config.yaml and progress.md
- [ ] briefs/, handoffs/, validations/, issues/ subdirectories created

---

## Phase 7: Launch Confirmation

> **Rules:** 5
> **Goal:** Get human confirmation before starting the build.
> **Inputs:** All approved artifacts, config.yaml
> **Outputs:** Human approval, progress.md updated, plan status = BUILDING

Since every artifact has been individually reviewed and approved during Phases 4, 4a, and 5, this phase is a lightweight launch confirmation -- not a full review.

Show the user a summary dashboard:
1. **Mission overview** -- change name, total tasks (feature + closing), total contract assertions
2. **Coverage audit result** -- number of PRD requirements covered, any deferred/skipped items
3. **Engine config** -- which engine runs each role (from config.yaml)

Ask: "Everything has been reviewed. Ready to start the build?"

- If yes: append `<timestamp> -- Plan approved. <N> tasks, <M> assertions in contract.` to progress.md. Update plan status to BUILDING. Proceed to Phase 8.
- If the human wants to revisit something: loop back to the specific phase they name. Do not re-review everything.

**Gate -- proceed when:**
- [ ] Human confirms "ready to start the build"
- [ ] progress.md updated with approval timestamp and task/assertion counts
- [ ] Plan status = BUILDING

---

## Phase 8: Build Loop

> **Rules:** 1, 2, 3, 4, 5, 6, 8, 9, 10
> **Goal:** Build all tasks with per-task validation.
> **Inputs:** tasks.md, contract.md, design.md, proposal.md, config.yaml
> **Outputs:** Committed code per task, validation reports, codebase deltas in plan

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

## What Previous Tasks Built
<For task 1, this section is empty: "This is the first task. No prior code has been committed for this mission."

For task 2+, include a codebase delta summary generated from `git diff` after each previous task passed validation. List ONLY observable facts about the code -- never include worker reasoning, handoff content, or concerns.

For each completed task, include:
- Files created or modified (paths only)
- Public function/endpoint signatures added (name, params, return type)
- Key type definitions or interfaces introduced
- Patterns used (e.g., "repository pattern via src/repositories/userRepo.ts")

Example:
### Task 1: User service
- Created: src/services/users.ts, src/models/user.ts, src/repositories/userRepo.ts
- Endpoints: POST /api/users (body: {email: string, name: string}) → 201 {id, email, name}
- Types: User {id: string, email: string, name: string, createdAt: Date}
- Pattern: repository pattern via src/repositories/userRepo.ts

### Task 2: Auth middleware
- Created: src/middleware/requireAuth.ts, src/services/auth.ts
- Exports: requireAuth (Express middleware, attaches req.user: User)
- Endpoints: POST /api/auth/login (body: {email, password}) → 200 {token: string}
>

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

# Clean stale temp files and directories from previous workers to avoid filling /tmp
find /tmp -mindepth 1 -maxdepth 1 -user "$(whoami)" -mmin +30 -not -name "claude-*" \( -type f -delete -o -type d -exec rm -rf {} + \) 2>/dev/null

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

10. **Behavioral conformance:** For each assertion that describes observable behavior (CLI output, exit codes, file content, API response, shell behavior), write and execute a minimal test script that verifies the behavior by actually running the code. Do not rely solely on reading the code -- run it. Examples:
    - CLI flag assertion: run the binary with the flag, check output
    - Exit code assertion: run the command, check `$?`
    - Shell alias assertion: source the alias, run `type <name>`, check the definition
    - API response assertion: curl the endpoint, check the response body
    
    If an assertion cannot be tested via execution (e.g., internal architecture, code organization, build configuration), mark it as "verified by inspection" with an explanation of why execution was not possible.
    
    If a behavioral test is inconclusive (e.g., the test script errors out, the environment lacks a dependency), mark it as "verified by inspection (execution inconclusive: <reason>)" and fall back to code reading. Do not let an inconclusive execution check block the validation.
    
    Report each assertion's verification method (execution or inspection) in the contract assertions table.

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
| Behavioral conformance | PASS/FAIL/N/A | ... |

## Contract Assertions
| # | Assertion | Result | Method | Evidence |
|---|-----------|--------|--------|----------|
| 1 | <assertion text> | PASS/FAIL | execution/inspection | <one-line evidence> |
| 2 | ... | ... | ... | ... |

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

3. **Diagnostic triage.** Spawn a lightweight Claude subagent to diagnose the failure. The subagent receives:
   - The validator report (from `mission/validations/task-N-scrutiny.md` or `task-N-usertest.md` -- whichever triggered this fix loop entry)
   - The worker's diff (`git show <commit-sha>`)

   The subagent does NOT receive `contract.md`, `proposal.md`, or `design.md`. It describes what the code does wrong, not what the contract expected.

   Subagent prompt: "You are diagnosing a validation failure. Read the validator report and the code diff. For each failure, identify the probable root cause: which file and line, what the code does, and why that causes the observed symptom. Be specific -- name files, lines, variables, and values. Describe what the code does wrong, not what it should do. Write your triage to: `<path to mission/issues/task-N-v<attempt>-triage.md>`"

   The triage output is included in the fix brief (Step 4 below) as an additional "Diagnostic Triage" section. If the triage subagent fails or times out, skip it and proceed with the fix brief without it -- triage is helpful but not blocking.

4. **Write fix brief** to `mission/briefs/task-N-fix-v<attempt+1>.md`:

#### Fix Brief Template

```markdown
# Fix Brief: Task N - <name> (Attempt <attempt+1>)

## What Failed
<describe the observed symptoms -- error messages, unexpected behavior, test failures. Do NOT copy assertion text from the contract. Describe what went wrong, not what the contract expected.>

## Diagnostic Triage
<paste the triage subagent's output from mission/issues/task-N-v<attempt>-triage.md. If triage was skipped or failed, write "Triage unavailable -- diagnose from the validator output below.">

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

5. **Dispatch fix worker** using the same engine and method as Step 2, but with the fix brief.

6. **After fix completes:** Go back to **Step 4** (re-run scrutiny validation with the new commit SHA). Both per-task validators must re-run after every fix (scrutiny and user-test), not just the one that failed.

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

2. **Generate codebase delta for subsequent briefs.** Run `git diff <pre-task-sha>..HEAD` (where pre-task-sha is the commit before this task's first commit, recorded when the worker was dispatched) to extract what this task added. Summarize into a codebase delta entry containing: files created/modified (paths), public function/endpoint signatures, key types, and patterns used. Store this delta in the plan under a `## Codebase Deltas` section. The next task's brief (Step 1) reads accumulated deltas from the plan and writes them into the "What Previous Tasks Built" section.

   **Guard rail:** Generate the delta from `git diff` output only. Never read handoff reports to build this summary. The delta contains facts about what exists in the code, not why or how the worker built it.

3. Append to `progress.md`:

```markdown
- <timestamp> -- Task N complete.
  - Commit: <SHA>
  - Scrutiny: PASS
  - User-test: PASS | SKIP
  - Demo: RECORDED | SKIPPED | FAILED
  - Fix attempts: <number, 0 if first try passed>
```

4. Update the plan: set Current Task to the next unchecked task, or "none" if all done.

5. **If more tasks remain:** Go back to Step 1 for the next task.

6. **If all tasks are done:** Proceed to Phase 9.

**Gate -- proceed when:**
- [ ] All tasks in tasks.md are `- [x]` (complete) or `- [~]` (skipped)
- [ ] No unchecked `- [ ]` tasks remain
- [ ] Each completed task has passing scrutiny and user-test reports
- [ ] Plan codebase deltas are current

---

## Phase 9: Mission Completion

> **Rules:** 1, 2, 5, 7, 10
> **Goal:** Final gates, draft PR, and dual adversarial review.
> **Inputs:** All committed code, contract.md, coverage-audit.md
> **Outputs:** Draft PR, adversarial review reports, plan status = COMPLETE

### Final Gates

Before creating the PR, re-run whole-repo quality checks to catch regressions introduced by tasks that ran after the cleanup task (e.g., documentation or CI tasks can introduce lint warnings or formatting issues).

Run the project's build, test, lint, and format commands against the full repository:
- Build: does it compile/build cleanly?
- Tests: do all tests pass?
- Lint: zero warnings (e.g., `cargo clippy -- -D warnings`, `eslint .`)
- Format: zero diffs (e.g., `cargo fmt --check`, `prettier --check .`)

**If all gates pass:** Log to progress.md: `<timestamp> -- Final gates PASSED.` Proceed to the spec delta check.

**If any gate fails:** Spawn a fix task targeting only the gate failures. This fix task:
- Gets a brief describing the specific failures (error messages, warnings, format diffs)
- Goes through scrutiny validation only (skip user-test -- this is a hygiene fix)
- Follows the standard circuit breaker (3 attempts). If the circuit breaker triggers, ask the human.
- After the fix passes, re-run all four gates to confirm no new regressions

### Spec Delta Check

After final gates pass, verify that the built codebase actually contains what the coverage audit promised. This catches items marked COVERED during planning that were lost or incompletely implemented during the build.

Spawn a Claude subagent with this prompt:

```
You are a spec delta checker for the Darkfactory mission system. Your job is to verify that the built codebase matches the coverage audit from planning.

## Coverage Audit
<paste the full coverage-audit.md content>

## Instructions

For each item in the coverage table:

**If COVERED:** Verify the feature is actually present in the codebase. Read the relevant code, check that the described behavior exists. You are not running the code -- just confirming the implementation is present and plausible.

**If APPROVED-DEVIATION:** List the deviation and its classification (Interpretation/Substitution/Removal). Do not re-litigate -- just confirm the deviated behavior (not the original spec) is implemented.

**If GAP (deferred/skipped):** Note it as intentionally excluded. No check needed.

## Output Format

Write your report to: <absolute path to mission/validations/spec-delta.md>

```markdown
# Spec Delta Check

## Result: CLEAN | DISCREPANCIES FOUND

## Coverage Verification
| # | PRD Requirement | Audit Status | Verified | Notes |
|---|----------------|-------------|----------|-------|
| 1 | <requirement> | COVERED | YES/NO | <evidence or discrepancy> |
| 2 | <requirement> | APPROVED-DEVIATION (Substitution) | YES | <confirms deviated behavior present> |
| 3 | <requirement> | GAP (deferred) | SKIPPED | intentionally excluded |

## Discrepancies (if any)
| # | PRD Requirement | Expected | Found | Severity |
|---|----------------|----------|-------|----------|
| N | <requirement> | <what audit promised> | <what's actually in code> | HIGH/MEDIUM/LOW |

## APPROVED-DEVIATION Summary
| # | PRD Requirement | Classification | Deviated Behavior |
|---|----------------|---------------|-------------------|
| 2 | <requirement> | Substitution | <what was implemented instead> |
```
```

Read the delta check report from `mission/validations/spec-delta.md`.

**If CLEAN (no discrepancies):** Log to progress.md: `<timestamp> -- Spec delta check CLEAN. <N> items verified, <M> approved deviations listed.` Proceed to create the draft PR.

**If DISCREPANCIES FOUND:** Present each discrepancy to the human with three options:
- **Add fix task** -- create a new task to implement the missing item. The task goes through the normal build loop (brief, worker, scrutiny validation, user-test validation). After completion, re-run final gates and the spec delta check.
- **Reclassify as APPROVED-DEVIATION** -- the item was covered in planning but the implementation intentionally diverged. The human provides a reason and classification (Interpretation/Substitution/Removal). Update the coverage audit on disk.
- **Accept gap** -- acknowledge the gap and proceed. Log it in progress.md as an accepted gap.

Log the human's decision for each discrepancy to progress.md.

### Create Draft PR

Create a **draft** pull request using `gh pr create --draft`. Never create a ready-for-review PR. Use the PR body template below.

#### PR Body Template

```markdown
Closes #<PRD issue number from plan>

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

The `Closes #<N>` directive links the PR to the PRD issue. GitHub will auto-close the issue when the PR is merged. Since the PR is created as a draft, the issue stays open until the developer promotes and merges the PR.

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

**Gate -- mission is COMPLETE when:**
- [ ] Final gates passed (build, test, lint, format)
- [ ] Spec delta check clean (no unresolved discrepancies)
- [ ] Draft PR created
- [ ] Both adversarial reviews complete and presented to human
- [ ] Plan status = COMPLETE

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
