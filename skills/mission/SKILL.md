---
name: mission
description: Start an autonomous Darkfactory mission. Full mode grills the user for alignment, creates OpenSpec artifacts, writes a contract, dispatches workers serially, validates each task against the contract, runs fix loops with circuit breakers, and creates a draft PR on completion. Pass --basic for a lightweight single-task build-validate-fix loop with no planning, no workers, and no PR, configurable with --no-tdd and --instruction <path> or an inline task description.
flow: align -> spec -> contract -> build -> validate -> ship
---

## Mode Selection

If the args string contains `--basic`, read and follow [basic/SKILL.md](basic/SKILL.md) instead. **Stop reading this file.** The remaining content below is for full mission mode only.

Supported basic mode flags (parsed by basic/SKILL.md):
- `--basic` -- activate basic mode (required)
- `--no-tdd` -- skip TDD, implement directly then write tests
- `--instruction <path>` -- read task description from file
- Unquoted/quoted text after flags -- inline task description

If `--basic` is not present, continue with full mission mode below.

---

## Rules (Always Enforced)

1. **Serial execution.** One worker at a time. Never dispatch two workers in parallel. (Exception: the two adversarial reviewers in Phase 9 run in parallel since they are independent read-only reviewers.)
2. **Adversarial separation.** Workers never see `contract.md`. Validators never see `proposal.md` or `design.md`.
3. **Both per-task validators re-run after every fix.** Even if only one failed, both scrutiny and user-test re-run to catch regressions. Exception: Phase 9 gate fixes run scrutiny only. Adversarial review runs once at the end, not per task.
4. **Circuit breaker at 3.** After 3 failed attempts on the same task, stop and ask the human. Retry allows one more attempt but does NOT reset the counter.
5. **File-backed state.** All decisions and status go to disk. The mission must be resumable from files alone.
6. **Single commit per task.** One task, one worker, one commit, one validation cycle.
7. **Draft PRs only.** Never create a ready-for-review PR. The human promotes it.
8. **Plan is the operating manual.** Keep the plan updated after every phase and step. It must be accurate if context compacts. On entering any major phase (0-9, 4.5) and at every phase gate, set the plan's `## Orchestrator Phase` field to the current phase name, step, and an ISO 8601 `Last checkpoint` timestamp. `mission-resume` cross-checks this field against disk state when choosing a re-entry point.
9. **Skipped tasks use `- [~]`.** The build loop treats `- [~]` as neither unchecked nor complete. Skipped tasks are not re-attempted.
10. **Mark tasks done in tasks.md immediately.** After a task passes validation, change `- [ ]` to `- [x]` in tasks.md before moving to the next task. This is required for resume to work correctly.
11. **Artifact review before human Q&A.** Every planning artifact (Phases 2-5) is dispatched to Codex for review via `/artifact-review` before presenting to the human. Material findings surface in the gate digest; the full review stays in the linked checkpoint, never pasted into chat (see the Review Pattern). If `/artifact-review` fails, note the failure and continue — review is non-blocking. The review result (or failure notice) must be persisted as a checkpoint before presenting the artifact to the human. For Phase 2, record the checkpoint in the plan's Review Checkpoints section (the change directory does not exist yet). From Phase 3 onward, write checkpoint files to `reviews/` in the change directory.

---

## Phase Index

0. **Enter Plan Mode** -- write the durable operating plan
1. **Grill for Alignment** -- align on what is being built and why
2. **Establish Shared Language** -- create/update UBIQUITOUS_LANGUAGE.md
3. **Write the Proposal** -- author proposal.md, the single intent document
4. **Create and Review OpenSpec Artifacts** -- specs, design, tasks (human-approved)
4.1. **Generate and Review Specs** -- formal WHEN/THEN requirements, one per capability
4.2. **Generate and Review Design** -- architecture, module inventory, type catalog
4.3. **Task Strategy Approval** -- approve decomposition approach before generating tasks
4.4. **Generate and Review Tasks** -- ordered checklist with topology metadata
4.5. **Spec Coverage Audit** -- verify every proposal requirement maps to a task
5. **Write Contract** -- validation checks workers never see
6. **Create Mission Directory** -- set up mission/ state directory
7. **Launch Confirmation** -- human confirms, build starts
8. **Build Loop** -- dispatch workers, validate per task, fix loop with circuit breaker
9. **Mission Completion** -- final gates, spec delta check, draft PR, dual adversarial review

The shared **Review Pattern** (artifact review -> checkpoint -> present digest -> decide) is defined between Phases 2 and 3 and used at every planning review point.

---

## Phase 0: Enter Plan Mode

> **Rules:** 5, 8
> **Goal:** Write the durable operating plan that survives context compaction.
> **Inputs:** User's idea (from conversation), UBIQUITOUS_LANGUAGE.md (if exists)
> **Outputs:** Plan (in plan mode), UBIQUITOUS_LANGUAGE.md confirmed read or noted absent

Run `/mission-init` to verify that all Darkfactory skills are installed and required CLI tools are available. If `/mission-init` reports any issues, resolve them before proceeding.

Enter plan mode immediately. Write a durable operating manual as the plan. This plan survives context compaction and is the orchestrator's source of truth.

### Plan Template

Write this plan, filling in values as they become known. Read the plan structure from [templates/plan.md](templates/plan.md).

The plan must include a `## Review Checkpoints` section for recording the Phase 2 review result before the change directory exists.

To fill in the **Darkfactory SHA** field, resolve the Darkfactory repo root from the skill symlink: `DARKFACTORY_ROOT="$(cd "$(readlink "$HOME/.claude/skills/mission")/../.." 2>/dev/null && pwd -P)"`. Then run `git -C "$DARKFACTORY_ROOT" rev-parse HEAD`; if `git -C "$DARKFACTORY_ROOT" status --porcelain` returns anything, append `-dirty` to the SHA. Paste the result into the plan.

### First Action

Check if `UBIQUITOUS_LANGUAGE.md` exists in the repo root. If it does, read it now so all phases use consistent terminology.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] `/mission-init` passed (all skills symlinked, CLI tools present)
- [ ] Plan is written and active in plan mode
- [ ] Darkfactory SHA captured in plan
- [ ] UBIQUITOUS_LANGUAGE.md checked (read if exists, noted if absent)

---

## Phase 1: Grill for Alignment

> **Rules:** --
> **Goal:** Align on what is being built, why, and what "done" looks like.
> **Inputs:** User's idea (from conversation)
> **Outputs:** Grill conversation (in context, used by Phases 2 and 3), key decisions summarized in plan

Run `/grill` to interview the user about their idea. The grill session should drive toward shared understanding of what is being built, why, and what "done" looks like.

If the user says "skip":
- Warn: "Skipping the grill means the contract will be based on whatever I can infer. Contract quality depends on alignment quality. Are you sure?"
- If they confirm, proceed to Phase 2.

After the grill completes, summarize the key decisions in the plan.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] Grill complete or explicitly skipped (with warning acknowledged)
- [ ] Key decisions summarized in plan

---

## Phase 2: Establish Shared Language

> **Rules:** 5
> **Goal:** Establish shared terminology for all subsequent phases.
> **Inputs:** Grill conversation (from Phase 1, in context), UBIQUITOUS_LANGUAGE.md (if exists from Phase 0 check)
> **Outputs:** UBIQUITOUS_LANGUAGE.md (created or updated)

Run `/language` to create or update `UBIQUITOUS_LANGUAGE.md`.

- If the file already existed and was read in Phase 0, this phase updates it with any new terms from the grill session.
- If the file did not exist, this phase creates it. Read the file after creation so all subsequent phases use the terms.

Run `/artifact-review` on the generated UBIQUITOUS_LANGUAGE.md before presenting to the human.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] UBIQUITOUS_LANGUAGE.md exists and is loaded into context
- [ ] Review checkpoint recorded in plan (REVIEWED or FAILED)

---

## Review Pattern (used at every review point in planning)

Every artifact review follows this pattern:
1. **Run `/artifact-review`** -- dispatch the artifact to Codex for review (per Rule 11)
2. **Write checkpoint** -- persist the review result before continuing:
   - **Phase 2** (before the change directory exists): add a structured entry to the plan's Review Checkpoints section: `- Phase 2 (<artifact>): REVIEWED | FAILED — <timestamp> — <finding count or failure reason>`
   - **Phase 3 onward** (change directory exists): write the review output (or failure notice) to `reviews/<phase>-<artifact>.md` in the change directory, using the format in [templates/review-checkpoint.md](templates/review-checkpoint.md). If the `/artifact-review` dispatch fails, write a FAILED checkpoint with the dispatch error.
   The checkpoint must exist before presenting the digest.
3. **Present the gate digest** -- do NOT paste the artifact content or the raw review findings into chat. Both are already on disk and are reachable as clickable file links. Present this block:

   ```
   Created:     <one line: what this artifact is>
   Review:      <checkpoint path>  — <"no material findings" | "FAILED: <reason>" | one line per material finding>
   For review:  <artifact path(s)>

   Approve and continue? Reply `yes`, or `no` with what you'd change.
   ```

   - `Created` -- a single line naming what was produced.
   - `Review` -- link the checkpoint file (`reviews/<phase>-<artifact>.md`) and append a **material finding** line for each material finding (see below). If none, write `no material findings`. If `/artifact-review` itself failed, write `FAILED: <reason>` -- never render a failed review as clean. **Phase 2 exception:** the change directory does not exist yet, so there is no durable checkpoint file and nothing else persists the review. Point `Review` at the plan's Review Checkpoints section and surface *every* finding in the digest, not just material ones.
   - `For review` -- the artifact file path(s) the human opens. For multi-file gates (Phase 4.1 specs), give one `Review` line per spec checkpoint and list every spec path -- e.g. `Review: reviews/phase4.1-spec-auth.md — HIGH auth/2: ... (fixed)` and `reviews/phase4.1-spec-billing.md — no material findings`, then `For review: specs/auth/spec.md, specs/billing/spec.md`.

   **Material finding** -- a finding is material, and only then gets a one-line synopsis in the digest, if it is (a) **CRITICAL** or **HIGH** severity (the artifact-review severity labels), (b) a finding you are dismissing as a false positive (surface it so a real gotcha can never hide under "reviewed clean"), or (c) a finding of any severity that you cannot confidently resolve or wave off yourself. Severity is assigned by the reviewer, not by what the human needs to see -- (c) is the safety valve for a MEDIUM or LOW that you suspect actually matters. MEDIUM and LOW findings you can confidently resolve stay in the linked checkpoint, unsurfaced. **Fix before you present:** resolve every CRITICAL/HIGH at the source and re-run `/artifact-review` before showing the digest -- the approve prompt must never sit above an open, unfixed CRITICAL/HIGH. So a material line reports a settled state, not a promise: `HIGH billing/2: "appropriate error" was too vague — fixed, pinned to the 402 message`, or `CRITICAL auth/3: false positive, the scenario already covers it`, or for a (c) item `MEDIUM export/1: unsure if the 10MB cap is intended — flagging for your call`.
4. **Human decides** -- approve (`yes`), or request changes (`no` with what to change).

The human can always reply `yes` to keep momentum. If changes are requested, revise and re-present the digest.

---

## Phase 3: Write the Proposal

> **Rules:** 5
> **Goal:** Author proposal.md -- the single intent document -- in the change directory.
> **Inputs:** Grill conversation (from Phase 1, in context), UBIQUITOUS_LANGUAGE.md, plan's Review Checkpoints section (for the retroactive Phase 2 checkpoint)
> **Outputs:** Change directory created, reviews/ set up, proposal.md human-approved

The proposal is the mission's only intent artifact -- there is no separate PRD. Author it once, directly from the grill conversation. Use terms from `UBIQUITOUS_LANGUAGE.md` consistently.

1. **Create the change directory:** `openspec new change "<name>"`. If the change directory already exists (a previous session was interrupted between creating it and Phase 6), do not re-run `openspec new change` -- continue with the existing directory and pick up at the first incomplete step below.

2. **Set up the reviews directory and write the Phase 2 checkpoint retroactively:**
   ```bash
   mkdir -p openspec/changes/<change-name>/reviews
   ```
   Read the plan's Review Checkpoints section and write its Phase 2 entry to `reviews/phase2-ubiquitous-language.md` using the format in [templates/review-checkpoint.md](templates/review-checkpoint.md). The retroactive checkpoint carries the plan entry's summary (status, timestamp, finding count); the full Phase 2 review output is not preserved across sessions, so leave the body as the summary line. Write only the Phase 2 checkpoint here -- Phase 3's own checkpoint is written fresh in step 4, never retroactively.

3. **Author `proposal.md`** in the change directory with exactly these four sections, each read with a distinct review lens:
   - **Problem** -- what is broken or missing, why now, and what success looks like. (Lens: is this a real problem?)
   - **Approach** -- the high-level solution, written as a bullet list of discrete commitments (each bullet one verifiable promise about what will exist or change). The coverage audit (Phase 4.5) enumerates these bullets, so prose paragraphs that bury commitments are not acceptable here. (Lens: is this the right approach?)
   - **Rejected alternatives** -- what was considered and dropped, and why. This keeps solution bias visible. (Lens: were alternatives genuinely weighed?)
   - **Capabilities** -- the kebab-case capability index (e.g., `user-auth`, `data-export`). Each entry becomes a `specs/<name>/spec.md` file in Phase 4, so this section is load-bearing: it is the contract between the proposal and the specs phase. Check `openspec/specs/` for existing capability names before naming new ones, and list a modified capability only when its spec-level behavior changes.

   OpenSpec does not validate proposal.md section names (its config rules cover only design.md and tasks.md), so this four-section format replaces the schema template's default sections.

4. **Review and gate:** run `/artifact-review` on `proposal.md`, write the checkpoint to `reviews/phase3-proposal.md`, and present to the human following the Review Pattern above.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] Change directory created (`openspec new change`)
- [ ] reviews/ created and reviews/phase2-ubiquitous-language.md written
- [ ] proposal.md authored with all four sections (Problem, Approach, Rejected alternatives, Capabilities)
- [ ] proposal.md human-approved
- [ ] Review checkpoint exists: reviews/phase3-proposal.md

---

## Phase 4: Create and Review OpenSpec Artifacts

> **Rules:** 5
> **Goal:** Generate and get human approval on the remaining OpenSpec artifacts: specs, design, tasks.
> **Inputs:** proposal.md (authored in Phase 3), UBIQUITOUS_LANGUAGE.md
> **Outputs:** specs/, design.md, tasks.md -- all human-approved

Read `proposal.md` from the change directory (authored and approved in Phase 3). It is the source of truth for all artifact generation. Use terms from `UBIQUITOUS_LANGUAGE.md` consistently in all artifacts.

The `spec-driven` schema orders its artifacts `proposal -> specs -> design -> tasks`; proposal.md is already written, so this phase generates the remaining three. Generate each artifact one at a time, present it to the human for review, and get approval before generating the next.

**Do NOT run `/opsx:propose` as a single pass.** Instead, orchestrate artifact creation using the openspec CLI commands directly. The change directory already exists (created in Phase 3) -- do not re-run `openspec new change`.

1. `openspec status --change "<name>" --json` -- get artifact order and dependencies
2. For each remaining artifact in order:
   a. `openspec instructions <artifact-id> --change "<name>" --json` -- get instructions for this artifact
   b. Create the artifact file following the instructions and template
   c. `openspec status --change "<name>" --json` -- re-run status to refresh the artifact list (newly unblocked artifacts and per-capability spec IDs appear only after their dependencies are written)

### Phase 4.1: Generate and Review Specs

Generate `specs/<capability>/spec.md` files (one per capability listed in the proposal). Each spec contains formal WHEN/THEN requirements using SHALL/MUST language.

Run `/artifact-review` on each generated spec file. Present the whole spec set in one digest following the review pattern: one `Review` line per `reviews/phase4.1-spec-<capability>.md` checkpoint (with its material findings, if any), and every spec path listed under `For review`. Specs are the formal requirements that flow into tasks and the contract. Missing or vague specs are the primary cause of missed requirements in the final build.

If approved, proceed to Phase 4.2.

### Phase 4.2: Generate and Review Design

Generate `design.md` (how -- architecture decisions, alternatives considered, risks). Reference the proposal for motivation and specs for requirements.

Design is required for every mission. The build loop and worker briefs depend on design.md existing -- skipping it would break downstream phases.

Follow the required structure in `templates/design-template.md`. The design must include these six section headings: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points, and Test Strategy.

#### Module Depth

For each major component or module in the design, consider its depth. A deep module has a small interface hiding significant implementation. A shallow module has a large interface with thin logic behind it. Prefer deep modules -- they are more testable at the boundary, easier for workers to implement in a single task, and easier for subsequent workers to use correctly.

For each module, describe:
- **Interface** -- what callers see (public API, CLI flags, function signatures)
- **What it hides** -- the complexity behind the interface that callers don't need to know about
- **Why this boundary** -- what makes this the right place to draw the line

If a module has a large interface relative to its implementation, ask whether it should be split differently or whether some of its interface belongs in a higher-level module. A good test: can a worker implement this module in a single task and a validator test it at the boundary? If not, the module may be too shallow or too broad.

#### Existing Codebases

When the mission modifies an existing codebase (not greenfield), the design must evaluate the current code structure against the planned architecture. For each module the design touches, read the existing code and assess whether it aligns with the target design. If existing modules are shallow, tightly coupled, or conflict with the planned architecture, the design should call this out explicitly and recommend refactoring as part of the implementation. These refactoring decisions flow naturally into task ordering in Phase 4.4 -- refactoring tasks go before the feature tasks that depend on them.

Before presenting to the human, run the Phase 4.2 design gate:
- Verify `design.md` contains all six required section headings from `templates/design-template.md`: Architecture Decision, Module Inventory, Type Catalog, Error Strategy, Integration Points, and Test Strategy. If any are missing, regenerate `design.md` before human review.
- For greenfield missions or missions with explicit architecture changes, verify the Architecture Decision section contains 2-3 scored architecture candidates using the template rubric.
- For existing codebase missions without architecture changes, scoring is not required. Verify the Architecture Decision section documents the current architecture and explains how the planned changes fit.
- Enforce the hard floor: no recommended candidate can score 0 on any scoring dimension. If a candidate scores 0 on any dimension, flag it and recommend against it.
- If candidates tie on total score, present the tied candidates to the human for discussion and selection. Do not apply an arbitrary tie-breaker.

After the design gate passes, run `/artifact-review` on the generated `design.md`. Present it to the human following the review pattern.

After the human approves design.md, populate 5 of the 6 Deterministic Anchors in the plan: workspace layout, primary domain types, error handling strategy, test strategy, and chosen architecture. Source values from design.md's Architecture Decision, Type Catalog, Error Strategy, and Test Strategy sections. Each anchor is a one-liner pointer with a source reference. The sixth anchor (task decomposition strategy) is populated in Phase 4.3 after the human approves the task strategy.

If approved, proceed to Phase 4.3.

### Phase 4.3: Task Strategy Approval

Before generating tasks.md, present a task decomposition strategy for human approval:

1. **Decomposition approach:** layered (types and domain logic first, then transport/CLI), vertical (each task cuts end-to-end through all layers), or hybrid (foundation layer first, then vertical slices). State which and why.
2. **Target task count range:** e.g., "6-8 feature tasks + 4-5 closing tasks." Justify the range based on the module inventory from design.md.
3. **Dependency ordering rationale:** which tasks must come first and why (e.g., "Task 1 defines core types that all subsequent tasks import").

#### Frontend Task Decomposition (TypeScript React projects only)

For TypeScript React projects, use the specific task layer vocabulary:

| Layer | Skill | Rationale |
|---|---|---|
| `frontend/component` | `/frontend` | Visual React component work |
| `frontend/page` | `/frontend` | Page composition |
| `frontend/layout` | `/frontend` | Visual structure |
| `frontend/hooks` | `/tdd` | Hooks have testable interfaces |
| `backend` | `/tdd` | APIs, server logic |
| `shared/types` | `/tdd` | Type contracts |
| `shared/schema` | `/tdd` | Zod validation, pure logic |
| `shared/utils` | `/tdd` | Pure utility functions |

**Mechanical rule:** `/frontend` only when the layer starts with `frontend/` AND is not `frontend/hooks`. Everything else gets `/tdd`.

Auto-set `browser_tools: true` in config.yaml when any task has `Skill: /frontend`.

When a frontend feature contains complex logic (validation, state machines, data transformation), split it: one `/tdd` task for the logic as a hook/utility, one `/frontend` task for the component that consumes it. The `/tdd` task comes first in the task list.

The human approves this strategy before tasks.md is generated. If rejected, revise and re-present.

After approval, populate the task decomposition strategy anchor in the plan's Deterministic Anchors section.

If approved, proceed to Phase 4.4.

### Phase 4.4: Generate and Review Tasks

Generate `tasks.md` (ordered checklist of implementation tasks).

#### Task Quality Standard

Each task must produce a concrete, testable output. When the approved strategy (Phase 4.3) is vertical, tasks cross all layers. When the strategy is layered or hybrid, tasks may be scoped to a single layer, provided each task still has a clear done state and a test surface.

**One task = one worker = one commit = one validation cycle.**

A task is good if you can demo it and verify it independently after it lands.

#### Task Topology

Each task in tasks.md must include topology metadata in this fixed format:

```markdown
- [ ] Task N: <name>
  Depends on: <prior task numbers, or "none">
  Consumes: <surfaces from prior tasks -- types, interfaces, endpoints>
  Produces: <surfaces this task creates -- types, modules, interfaces, endpoints>
  Layers: <logical architectural layers touched, e.g. domain, transport, storage>
```

#### Skill and Constraint Validation (TypeScript React projects only)

Task topology for TypeScript React projects includes two additional fields:

```markdown
Skill: </tdd or /frontend>
Constraints: </ui-system, /react-perf, or "none">
```

Validation rules:
- Exactly one `Skill:` value per task -- either `/tdd` or `/frontend`
- Zero or more `Constraints:` values
- `/ui-system` and `/react-perf` are only valid as mission constraints with `Skill: /frontend`. (Both skills are also independently invokable by humans via `/ui-system` or `/react-perf` outside of missions — the constraint restriction applies only to mission task topology.)
- Flag invalid combinations during Phase 4.4 human review

Layers are logical groupings of modules (domain, transport, storage, configuration, etc.) defined in the design narrative, not columns from the Module Inventory table.

When the approved strategy (Phase 4.3) is layered or hybrid and a task's Layers field lists more than 2 architectural layers, flag the task for human review with a recommendation to split or approve as-is. When the approved strategy is vertical, the layer count flag does not apply since vertical tasks are expected to cross all layers.

When a task adds public surface area to a module (new exports, API endpoints, CLI flags), include the implementation behind that surface in the same task. Do not split a module's interface from its implementation across tasks -- the per-task validator judges depth from a single diff and will flag an interface without the work behind it.

#### Mandatory Closing Tasks

Every mission's tasks.md MUST include these closing tasks after all feature tasks:

1. **Smoke test (mandatory).** Write or update one end-to-end test that runs the project's primary user flow against real dependencies. No project-internal mocks. Place it where the project's standard test command runs it. If the mission produces no runnable user flow, note that in the handoff.

2. **Cleanup task (mandatory).** Remove unused dependencies, dead code, leftover scaffolding, temporary artifacts, and any backwards-compatibility shims. Workers sometimes leave debris that per-task validation does not catch. This task cleans up the codebase before the shape re-check so the re-check does not flag soon-to-be-deleted surface area.

3. **Module shape re-check (mandatory for missions with 5+ feature tasks; optional otherwise).** Re-read design.md's module descriptions. For each module, compare the description to what the code actually exposes -- in whatever form is natural for the project (public functions, React props, HTTP endpoints, CLI flags, exported types, data schemas, message topics, job parameters, config keys, whatever contract callers actually depend on). Verify that every domain type in the Type Catalog exists in the committed code with a matching name. If reality has drifted meaningfully from the description, either update design.md to match (deliberate change) or refactor the code back (unintended drift). If Type Catalog names have drifted, either update design.md to match the code (deliberate change) or refactor the code to match design.md (unintended drift). "Meaningful drift" means: any public surface item added, removed, or renamed that is not reflected in design.md; any change to the vocabulary or contract callers depend on; or a cumulative widening where a module that was described as having N operations now has noticeably more. Temporary shims and internal helpers do not count. For small missions (under 5 feature tasks) a one-line "no drift observed" note in the handoff is sufficient -- this step exists to catch cumulative drift across many tasks, which rarely occurs in short missions. This catches drift that per-task scrutiny cannot see.

   Write the findings to `mission/validations/module-shape-audit.md` -- a short free-form note listing each module, whether drift was found, and what was done about it ("no drift," "design.md updated," or "refactored"). No required format; the file exists as a record that the check happened.

4. **CLAUDE.md + ARCH.md generation (mandatory).** If the project root does not have a CLAUDE.md, create one from [templates/claude-md.md](templates/claude-md.md) using the actual built codebase (real commands, real repo structure, real conventions). If a CLAUDE.md already exists, do not modify it. Similarly, if no ARCH.md exists, create one from design.md's architecture decisions as a living document. If ARCH.md already exists, do not modify it.

5. **Documentation task (mandatory).** Write or update documentation for what was built. The orchestrator recommends the documentation scope based on what the feature tasks produce -- this varies by mission type. Not every mission needs install instructions, architecture diagrams, or API docs. Use terms from `UBIQUITOUS_LANGUAGE.md` consistently. Any commands shown in docs must be executed against the built artifacts and the actual output pasted -- don't show output that wasn't produced. If unsure what documentation is needed, ask the human during this review.

6. **CI configuration task (optional).** Add or update CI/CD pipeline configuration. The orchestrator recommends whether to include this based on the change scope. The human decides during this review.

If the design (Phase 4.2) identified refactoring needed for existing code, those tasks go before the feature tasks in tasks.md.

Run `/artifact-review` on the generated `tasks.md`. Present it following the review pattern (the digest links `tasks.md`; do not paste the task list). If approved, proceed to Phase 4.5.

After all artifacts are reviewed and approved, update the plan with the actual artifact paths.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] All specs/<capability>/spec.md files human-approved
- [ ] Review checkpoint exists for every spec: reviews/phase4.1-spec-<capability>.md matches specs/ one-to-one
- [ ] design.md human-approved
- [ ] Review checkpoint exists: reviews/phase4.2-design.md
- [ ] Task strategy approved by human (Phase 4.3)
- [ ] tasks.md (with closing tasks) human-approved
- [ ] Review checkpoint exists: reviews/phase4.4-tasks.md
- [ ] Plan updated with actual artifact paths

---

## Phase 4.5: Spec Coverage Audit

> **Rules:** 5
> **Goal:** Verify every proposal requirement maps to at least one task before writing the contract.
> **Inputs:** proposal.md (from the change directory), specs/, tasks.md
> **Outputs:** coverage-audit.md, updated specs/tasks if gaps found

Before writing the contract, verify that every proposal requirement is covered by at least one task.

A **proposal requirement** is each bullet in the proposal's Approach section (one discrete commitment per bullet) plus each entry in its Capabilities index. Enumerate them in document order -- this fixed extraction rule means two sessions auditing the same proposal build the same coverage table.

### Steps

1. **Read proposal.md** from the change directory (authored in Phase 3). Extract the proposal requirements per the rule above.
2. **Read tasks.md** and all spec files from `specs/`.
3. **Build a coverage table** mapping every proposal requirement to the task(s) that implement it:

Use the format in [templates/coverage-audit.md](templates/coverage-audit.md).

The "Prerequisites met?" column applies to Substitutions only. For Interpretations and Removals, write "--".

4. **Flag gaps and classify deviations.** For each requirement that is not directly covered:
  **If GAP (no plan to implement):** Add task (and spec requirement if missing) / Defer (out of scope, track for later) / Skip (intentionally not implementing, document why).

  **If DEVIATION (implementing differently than spec), classify:**
    **Interpretation** (spec is ambiguous, reasonable reading) -- batch-approve with COVERED items.
    **Substitution** (different approach) -- explicit human approval per item. State the trade-off: "The spec says X. We are doing Y instead because Z. This means [concrete consequence]."
    **Removal** (not implementing) -- explicit human approval per item. State impact: "Users who need X will not be able to use the tool for that purpose."

  #### Prerequisite check for substitutions
  For every Substitution, ask: "Does the deviated approach require implementation work that the mapped task will not cover?" Break down the deviated approach into its prerequisites. If any prerequisite is not covered by the mapped task or another existing task, route it through gap remediation (Add task / Defer / Skip). The deviation cannot be approved until all prerequisites are resolved. Resolution options:
    **Add a task** to cover the missing prerequisite (triggers spec/task re-review)
    **Expand the mapped task** to include the prerequisite (update tasks.md, re-present for review)
    **Reclassify as Removal** (forces the human to acknowledge the requirement is being dropped, not substituted)

  Mark approved deviations as **APPROVED-DEVIATION** in the coverage table (not COVERED). This status persists through the contract and is re-surfaced during the spec delta check (Phase 9).

  **Present substitutions and removals in a dedicated review step.** The human approves or rejects each individually. Do not mix them with the COVERED items review. Interpretations may be batch-approved with the main table.

5. **Persist the coverage audit to disk.** Write the coverage table to `openspec/changes/<change-name>/coverage-audit.md`. This file is needed for mission resume (so a resumed session can see what was already mapped without re-running the audit) and for contract writing (Phase 5 cross-references it). For each Substitution deviation, include the prerequisite analysis (which prerequisites were checked, which were covered, which were gaps, and how they were resolved). This ensures a resumed session knows why a substitution was approved safe.

6. Run `/artifact-review` on the persisted `coverage-audit.md` and present it following the review pattern (the digest links `coverage-audit.md`; do not paste the table). The per-item substitution and removal approvals stay in their dedicated step above -- the digest covers the audit artifact, not those decisions.

7. **If specs change** (new spec requirements added to cover gaps), re-present the affected spec files for human review before proceeding. The human must approve any new or modified specs.

8. **If tasks change** (new tasks added to cover gaps), re-present the updated tasks.md for review. The human must approve the final task list before proceeding.

9. **If specs or tasks changed in steps 7-8,** re-persist the updated `coverage-audit.md` to disk and re-run `/artifact-review` on it. Overwrite the existing checkpoint file (`reviews/phase4.5-coverage-audit.md`) with the new review result. Phase 5 cross-references `coverage-audit.md` when writing the contract -- the persisted audit must reflect the final approved state.

10. **Record the coverage audit** in the plan. Update the Spec Coverage Audit section with the gap count and resolutions.

If approved, proceed to Phase 5.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] Coverage audit complete -- every proposal requirement mapped
- [ ] All gaps resolved (added, deferred, or skipped)
- [ ] All deviations classified and approved by human
- [ ] Changed specs/tasks re-approved by human (if any changed)
- [ ] coverage-audit.md persisted to disk
- [ ] Review checkpoint exists: reviews/phase4.5-coverage-audit.md
- [ ] Plan updated with audit results

---

## Phase 5: Write Contract

> **Rules:** 2, 5
> **Goal:** Write the validation contract that workers never see.
> **Inputs:** coverage-audit.md, specs/, tasks.md, proposal.md
> **Outputs:** contract.md (human-approved, testability pre-flight passed)

Create `contract.md` in the change directory (`openspec/changes/<change-name>/contract.md`).

### Contract Rules

1. Group checks by task, matching the task list in tasks.md.
2. Checks should describe **observable behavior** verifiable from outside the code (HTTP response, CLI output, file on disk, UI state, build passing, etc.). Where this is genuinely not possible, use `[inspection]` checks -- see the verification method tags in the contract template for caps.
3. Do NOT reference implementation details (function names, variable names, internal data structures).
4. Be specific and unambiguous. A validator reading only the contract and the diff should be able to verify every check without guessing.
5. Workers never see this file.
6. **Write the minimum set of checks needed to verify the task is complete and correct.** Each check should trace to a specific spec scenario (e.g., `add-todo/1`), including error behaviors specified in the specs. Do not add checks for edge cases, nice-to-haves, or defensive behaviors unless the proposal or specs explicitly require them. If a check would not cause you to reject the task when it fails, do not include it. Place checks on the earliest task where they become verifiable -- if a behavior spans multiple tasks, put the check on the task where it can first be checked, not where the work begins.

### Cross-Referencing Specs

For each task, use the coverage audit from Phase 4.5 to find which spec capabilities map to it. Then read the actual spec files to identify specific scenarios. Write checks that capture the specific details from those scenarios -- not generic behavioral checks, but checks that would fail if the specific scenario were not implemented. Each check must trace back to a specific spec capability and scenario number (e.g., `add-todo/1`).

**Process for each task:**
1. Look up the spec capabilities mapped to this task in the coverage table (e.g., `add-todo`, `search`).
2. Read the actual spec files for those capabilities (`specs/<capability>/spec.md`). Identify which specific WHEN/THEN scenarios are relevant to this task. Number the scenarios within each spec file (1, 2, 3...) to form the `capability/N` identifiers used in contract comments.
3. Write checks that verify those specific behaviors with specific values, endpoints, messages, or states from the specs.
4. If the spec mentions specific constraints (limits, formats, error messages, status codes), those constraints must appear in the checks verbatim.

### Contract Template

Each check must include a **verification method** tag indicating how a validator should check it:
- `[execution]` -- verifiable by running a command, HTTP request, or test script (strongest)
- `[output]` -- verifiable by checking command output, file contents, or API response
- `[inspection]` -- verifiable only by reading code (weakest)

Cap `[inspection]` checks at **20% of total per task**. If more than 20% require inspection, rewrite them as `[execution]` or `[output]` where possible. If a check genuinely cannot be made executable, it is acceptable -- but flag it during contract review.

Use the format in [templates/contract.md](templates/contract.md).

Reference spec coverage with a `<!-- Specs: capability/N, capability/M -->` comment, where `capability` is the spec folder name and `N` is the scenario number within that spec file.

When a spec scenario traces to an APPROVED-DEVIATION item in the coverage audit, mark it with `(DEVIATED)` in the Specs comment. The check itself describes the actual implemented behavior (the deviation), not the original spec requirement. This gives the human visibility during contract review into which checks exist because of an approved deviation.

### Contract Review

After writing the contract, run `/artifact-review` on the generated `contract.md`. Present it to the human following the review pattern. In the contract gate's `Created` line, give the task count and total check count (a count that feels too high suggests over-specification; too low suggests gaps). If any spec scenario was hard to express as an observable check, add a material-finding line for it -- a weak or vague check is a real quality risk worth the human's eye. Scenario-to-check traceability already lives in the `<!-- Specs: capability/N -->` comments in the contract file the human opens.

If the human requests changes, revise and re-present the digest.

### Contract Testability Pre-flight

After the human approves the contract, run a quick testability audit before proceeding. For each check, verify:

1. **Executable checks** (`[execution]`) -- confirm the check describes a concrete action and expected result that a validator could script (e.g., "POST /api/users with {email: 'a@b.com'} returns 201"). If the check is too vague to script, flag it.
2. **Output checks** (`[output]`) -- confirm the check names a specific file, value, or output to check. If it says "appropriate error message" instead of a specific message, flag it.
3. **Inspection checks** (`[inspection]`) -- verify whether the check could be rewritten as `[execution]` or `[output]`. If yes, rewrite it. If genuinely not possible (e.g., "no direct database access outside the repository layer"), keep it.

If any checks are flagged as ambiguous:
- Rewrite them to be specific and re-present only the changed checks to the human for quick approval
- Do not re-review the entire contract

Log the testability breakdown to progress.md: `<timestamp> -- Contract testability pre-flight: <N> execution, <M> output, <P> inspection (<Q>% inspection).`

Update the plan with the contract path after approval.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] contract.md human-approved
- [ ] Review checkpoint exists: reviews/phase5-contract.md
- [ ] Testability pre-flight passed (inspection checks <=20%)
- [ ] Testability breakdown logged to progress.md
- [ ] Plan updated with contract path

---

## Phase 6: Create Mission Directory

> **Rules:** 5
> **Goal:** Create the mission directory with config and progress tracking files.
> **Inputs:** Change name (from plan), engine preferences (from plan's Engine Config section, or defaults below)
> **Outputs:** mission/ directory with config.yaml, progress.md, and subdirectories

Read the engine preferences from the plan's Engine Config section. If not set by the human, use the defaults shown below.

Create the mission directory at `openspec/changes/<change-name>/mission/` with this structure:

```
mission/
  config.yaml
  progress.md
  briefs/
  handoffs/
  validations/
  issues/
  logs/
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

# Auto-set to true during Phase 4.3 when any task has Skill: /frontend.
browser_tools: false

# Absolute path to the durable plan file (the one written in Phase 0, normally
# ~/.claude/plans/<name>.md). mission-resume reads this directly to find the
# plan without grepping. Fill in the real absolute path when creating this file.
plan_file: <absolute path to the plan file>

# The commit the mission builds on top of -- HEAD when the build branch was
# established in Phase 7, before any task commit. Read by Phase 9 (adversarial
# review diff scope) and mission-resume orphaned-commit detection. Stored as a
# SHA, not a branch name, so the diff scope stays correct even when the branch
# moves or the mission runs on a pre-existing (non-default) branch.
base_sha: <full HEAD SHA, captured and written in Phase 7>
```

Set `plan_file` to the absolute path of the plan written in Phase 0. Leave `base_sha` as the placeholder here; Phase 7 captures and writes it when the build branch is established.

### progress.md

```markdown
# Mission: <change-name>

Started: <ISO 8601 timestamp>

## Log

- <timestamp> -- Mission directory created. Awaiting approval.
```

Create the five subdirectories with one command:

```bash
mkdir -p openspec/changes/<change-name>/mission/{briefs,handoffs,validations,issues,logs}
```

`logs/` is load-bearing: the worker and adversarial dispatch scripts redirect to `mission/logs/*.log` and will fail if the directory does not exist.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] mission/ directory exists with config.yaml and progress.md
- [ ] briefs/, handoffs/, validations/, issues/, logs/ subdirectories created

---

## Phase 7: Launch Confirmation

> **Rules:** 5
> **Goal:** Get human confirmation before starting the build.
> **Inputs:** tasks.md, contract.md, coverage-audit.md, config.yaml
> **Outputs:** Human approval, progress.md updated, plan status = BUILDING

Ensure a feature branch. Resolve the default branch with `gh api repos/{owner}/{repo} --jq .default_branch` (abort Phase 7 if that gh call fails -- it means no GitHub remote or bad auth). If `git rev-parse --abbrev-ref HEAD` equals the default branch, run `git checkout -b mission/<change-name>` -- if that branch already exists, use `mission/<change-name>-$(date -u +%Y%m%d%H%M%S)`. Log the checkout to progress.md.

Regardless of whether a checkout happened, capture the build base: run `git rev-parse HEAD` and write the result into config.yaml's `base_sha` field, replacing the placeholder. This is the commit the first task builds on top of (a `checkout -b` does not change it). Phase 9 and `mission-resume` use this pinned SHA to scope the mission diff, so it must be captured before the first task commit.

Show the user a summary dashboard:
1. **Mission overview** -- change name, total tasks (feature + closing), total contract checks
2. **Coverage audit result** -- number of proposal requirements covered, any deferred/skipped items
3. **Engine config** -- which engine runs each role (from config.yaml)

Ask: "Everything has been reviewed. Ready to start the build?"

- If yes: append `<timestamp> -- Plan approved. <N> tasks, <M> checks in contract.` to progress.md. Update plan status to BUILDING. Proceed to Phase 8.
- If the human wants to revisit something: loop back to the specific phase they name. Do not re-review everything.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] Human confirms "ready to start the build"
- [ ] progress.md updated with approval timestamp and task/check counts
- [ ] Plan status = BUILDING

---

## Phase 8: Build Loop

> **Rules:** 1, 2, 3, 4, 5, 6, 8, 9
> **Goal:** Build all tasks with per-task validation.
> **Inputs:** tasks.md, contract.md, design.md, proposal.md, config.yaml, UBIQUITOUS_LANGUAGE.md
> **Outputs:** Committed code per task, validation reports in mission/validations/. Codebase deltas are internal (populate subsequent task briefs within this phase).

For each unchecked task (`- [ ]`) in tasks.md, run Steps 1-7. Process tasks in order, one at a time (serial execution). Skip tasks marked `- [~]` (skipped) and `- [x]` (complete).

### Step 1: Assemble Brief

Write a brief to `mission/briefs/task-N.md`.

#### Brief Template

Read [templates/worker-brief.md](templates/worker-brief.md).

**The brief NEVER contains contract.md content.** 

#### Skill Invocation (mandatory)

The Procedures section of every brief — both initial worker briefs and fix briefs — must instruct the worker to **invoke** the task's skill (e.g. `/tdd`, `/frontend`) before writing any code. Use the exact language from the templates — "invoke", not "use" or "follow". The worker has access to all darkfactory skills via its skill loader; this instruction ensures it actually loads the skill's full workflow rather than treating the skill name as a vague hint. The same applies to constraint skills (`/ui-system`, `/react-perf`) — each must be explicitly invoked.

Substitute the task's `Skill:` value directly into the placeholder (it already includes the leading slash). If the task has no `Skill:` field, substitute `/tdd`. Fix briefs must carry forward the original task's `Skill:` and `Constraints:` values — never hardcode `/tdd`.

#### Browser Tools Override

If `browser_tools: false` in config.yaml (human override), omit visual verification procedures from the worker brief even for `/frontend` tasks. The worker follows the `/frontend` workflow but skips the chrome-devtools-mcp visual verify step in the Build phase.

### Step 2: Dispatch Worker

Read the engine from `mission/config.yaml`.

#### Browser Tools Pre-flight (Skill: /frontend tasks only, browser_tools: true)

Skip this check if `browser_tools: false` in config.yaml or the task does not have `Skill: /frontend`.

Before dispatching a `/frontend` task with `browser_tools: true`, check that chrome-devtools-mcp is available in **both** environments:

1. **Claude Code (orchestrator):** Check that `mcp__chrome-devtools__*` tools are in the available tool list.
2. **Codex (dispatch workers):** Run `grep -q '\[mcp_servers\.chrome-devtools\]' "$HOME/.codex/config.toml"` to verify the MCP server is configured for `codex exec`.

If both pass: proceed with dispatch, include visual verification in the brief.
If either fails: block dispatch. Log to progress.md: `<timestamp> -- Task N: chrome-devtools-mcp unavailable at dispatch. Pausing.` Surface to human with options: retry (check again), skip visual verify for this task, or abort mission.

#### If engine is `codex`:

Before dispatching, run `git rev-parse HEAD` and store the result as `PRE_TASK_SHA`. This value is used in Step 7 to generate the codebase delta — store it somewhere you can reference after the background worker returns (e.g. in the plan file or a local variable).

Use the Bash tool with `run_in_background: true` to run the worker. Use this exact bash script:

```bash
BRIEF_PATH="<absolute path to mission/briefs/task-N.md>"
HANDOFF_PATH="<absolute path to mission/handoffs/task-N.md>"
LOG_PATH="<absolute path to mission/logs/task-N-codex.log>"
PRE_TASK_SHA=$(git rev-parse HEAD)
# Clean stale temp files and directories from previous workers to avoid filling /tmp
find /tmp -mindepth 1 -maxdepth 1 -user "$(whoami)" -mmin +30 -not -name "claude-*" \( -type f -delete -o -type d -exec rm -rf {} + \) 2>/dev/null

# Dispatch via darkfactory-dispatch: process supervisor with timeout, heartbeat, and structured JSON result
DISPATCH_RESULT=$(darkfactory-dispatch \
  --model gpt-5.6-terra \
  --effort high \
  --timeout 1800 \
  --sandbox workspace-write \
  --config 'sandbox_workspace_write.writable_roots=[".git"]' \
  --prompt-file "$BRIEF_PATH" \
  --output "$HANDOFF_PATH" \
  --cwd "$(pwd)" \
  --log "$LOG_PATH" \
  --json 2>>"$LOG_PATH")

if [ -z "$DISPATCH_RESULT" ]; then
  echo "TASK_FAILED: dispatcher crashed with no output. Log: $LOG_PATH"
else
  DISPATCH_STATUS=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.status')
  DISPATCH_ERROR=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.error.message // empty')

  if [ "$DISPATCH_STATUS" = "completed" ] && [ -f "$HANDOFF_PATH" ]; then
    echo "TASK_COMPLETE: handoff written"
  else
    echo "TASK_FAILED: dispatch status $DISPATCH_STATUS${DISPATCH_ERROR:+; error: $DISPATCH_ERROR}; handoff $([ -f "$HANDOFF_PATH" ] && echo present || echo missing). Log: $LOG_PATH"
  fi
fi
```

The Bash tool returns a shell task ID immediately and notifies you when it completes. On completion, call BashOutput on that shell task ID to retrieve stdout. The only thing on stdout is the single final line -- either `TASK_COMPLETE` or `TASK_FAILED` -- because all codex output was redirected to `$LOG_PATH`.

- `TASK_COMPLETE` -> proceed to Step 3
- `TASK_FAILED` -> proceed to Step 3 (failure path). Read `$LOG_PATH` for failure context before asking the human.

**Human visibility:** To watch dispatch progress live, run `tail -f <absolute path to mission/logs/task-N-codex.log>` in another terminal. The log contains both codex output and dispatcher events (heartbeats every 15 seconds, timeout warnings, tool activity). If the configured timeout is reached, the dispatcher kills the codex process automatically (SIGTERM at soft timeout, SIGKILL after grace period).

#### If engine is `claude`:

Use the Agent tool with `subagent_type=darkfactory-claude` to spawn a subagent. Pass the full brief content as the prompt. Instruct the subagent to write its handoff to the same path (`mission/handoffs/task-N.md`).

### Step 3: Handle Result

Read the handoff file at `mission/handoffs/task-N.md`.

- **TASK_COMPLETE** (handoff exists and contains a commit SHA): Extract the commit SHA. **Guard -- keep mission bookkeeping out of the task commit.** The worker commit must not touch `openspec/changes/<change-name>/`: the plan and `mission/` files are bookkeeping the orchestrator owns, and committing them pollutes the `base_sha..HEAD` mission diff (the review and orphaned-commit scope). Check it:

  ```bash
  git show --name-only --format= <SHA> | grep "^openspec/changes/<change-name>/"
  ```

  If that prints any path, the worker violated the staging rule. The worker commit is HEAD (serial execution), so strip those paths and re-commit code-only, preserving the original message, then use the new SHA from here on:

  ```bash
  git reset --soft HEAD~1
  git restore --staged -- 'openspec/changes/<change-name>/'
  git commit -C <SHA>
  ```

  The stripped files stay on disk uncommitted, as intended. Re-read HEAD as the new commit SHA and log: `<timestamp> -- Task N: stripped mission bookkeeping from worker commit; re-committed as <new SHA>.`

  Then log to progress.md: `<timestamp> -- Task N: worker completed. Commit: <SHA>` (the re-committed SHA if the guard fired, otherwise the original). Proceed to Step 4.
- **TASK_FAILED** (handoff missing or no commit SHA):
  1. If engine is `codex` and this is the first failure for this task: log to progress.md: `<timestamp> -- Task N: codex worker failed. Falling back to claude worker.` Append a line to `mission/fallbacks.log`: `task-N|<task-name>|<reason for failure>`. Then dispatch a claude worker using the existing "If engine is `claude`" dispatch path with the same brief. Wait for the result.
     - If the claude fallback produces `TASK_COMPLETE`: log to progress.md: `<timestamp> -- Task N: claude fallback succeeded. Commit: <SHA>.` Proceed to Step 4.
     - If the claude fallback also fails: fall through to the human escalation below.
  2. Human escalation (reached when: engine is `claude`, engine is `codex` and both attempts failed, or any second failure): log to progress.md: `<timestamp> -- Task N: worker FAILED. <reason>.` Ask the human: retry, skip, or abort. If skip: mark the task as `- [~]` in tasks.md and log `<timestamp> -- Task N: SKIPPED by human.` to progress.md. The build loop ignores `- [~]` tasks.
  3. On the next task, revert to the configured engine. This fallback is per-task only. Before starting the next task, re-read `engine` from `openspec/changes/<change-name>/mission/config.yaml` so the engine resets to the configured value after any Codex -> Claude fallback.

### Step 4: Scrutiny Validation

Spawn a Claude subagent (via the Agent tool, `subagent_type=darkfactory-claude`) with this prompt:

#### Scrutiny Validator Prompt

Read [prompts/scrutiny-validator.md](prompts/scrutiny-validator.md) and use it as the subagent prompt. Fill in the commit SHA and contract checks for this task. The prompt runs 15 gates: test presence, build, tests, lint, code review, module depth, frontend depth review, code standards, simplify, security (if applicable), contract checks, behavioral conformance, silent skip detection, diff coverage, test quality review.

Read the validator's report from `mission/validations/task-N-scrutiny.md`.

### Step 5: Handle Scrutiny Validation Result

- **PASS**: Log to progress.md: `<timestamp> -- Task N: scrutiny validation PASSED.` Proceed to Step 6.
- **FAIL**: Proceed to Step 5a (fix loop).

### Step 5a: Fix Loop

Track the attempt number (starts at 1 for the initial worker, increments on each fix).

1. **Write failure details** to `mission/issues/task-N-v<attempt>.md`:

```markdown
# Issue: Task N - <name> (Attempt <attempt>)

## Failed Gates
<list of failed gates from validator report>

## Observed Symptoms
<describe what went wrong in terms of behavior -- error messages, test failures, unexpected output. Do NOT copy contract check text here.>

## Validator Recommendations
<recommendations from validator>
```

2. **Circuit breaker check:** If attempt >= 3, stop the loop. Log to progress.md: `<timestamp> -- Task N: circuit breaker triggered after <attempt> attempts.` Update plan status to PAUSED. Ask the human: "Task N has failed validation 3 times. Options: retry (allows one more attempt, counter continues from current value), skip (mark as skipped with `- [~]`, continue to next task), abort (stop the mission)."

   - **Retry:** Increment the attempt counter by 1 (e.g., attempt 3 becomes attempt 4). Allow one more fix attempt. If it fails again, the circuit breaker triggers again at the new count. Retry does NOT reset the counter.
   - **Skip:** Mark the task as `- [~]` in tasks.md. Log to progress.md: `<timestamp> -- Task N: SKIPPED by human after <attempt> failed attempts.` Continue to the next unchecked task. The build loop ignores `- [~]` tasks (they are neither unchecked nor complete).
   - **Abort:** Update plan status to PAUSED. Stop the build loop. Report to the human.

3. **Diagnostic triage.** Spawn a Claude subagent (`subagent_type=darkfactory-claude`) to diagnose the failure. The subagent receives:
   - The validator report (from `mission/validations/task-N-scrutiny.md` or `task-N-usertest.md` -- whichever triggered this fix loop entry)
   - The worker's diff (`git show <commit-sha>`)

   The subagent does NOT receive `contract.md`, `proposal.md`, or `design.md`. It describes what the code does wrong, not what the contract expected.

   Subagent prompt: "You are diagnosing a validation failure. Read the validator report and the code diff. For each failure, identify the probable root cause: which file and line, what the code does, and why that causes the observed symptom. Be specific -- name files, lines, variables, and values. Describe what the code does wrong, not what it should do. Write your triage to: `<path to mission/issues/task-N-v<attempt>-triage.md>`"

   The triage output is included in the fix brief (Step 4 below) as an additional "Diagnostic Triage" section. If the triage subagent fails or times out, skip it and proceed with the fix brief without it -- triage is helpful but not blocking.

4. **Write fix brief** to `mission/briefs/task-N-fix-v<attempt+1>.md`:

#### Fix Brief Template

Read [templates/worker-fix-brief.md](templates/worker-fix-brief.md).

**The fix brief NEVER contains contract.md content.** 

The fix brief must carry forward the original task's `Skill:` and `Constraints:` values into its Procedures section. See "Skill Invocation (mandatory)" in Step 1 — the same rules apply to fix briefs.

5. **Dispatch fix worker** using the same engine and method as Step 2, but with the fix brief.

6. **After fix completes:** Go back to **Step 4** (re-run scrutiny validation with the new commit SHA). Both per-task validators must re-run after every fix (scrutiny and user-test), not just the one that failed.

### Step 6: User-Test Validation

Spawn a Claude subagent (via the Agent tool, `subagent_type=darkfactory-claude`) with this prompt:

#### User-Test Validator Prompt

Read [prompts/usertest-validator.md](prompts/usertest-validator.md) and use it as the subagent prompt. Fill in the commit SHA and contract checks for this task.

Read the validator's report from `mission/validations/task-N-usertest.md`.

- **PASS**: Log to progress.md: `<timestamp> -- Task N: user-test validation PASSED.` Proceed to Step 6a (demo video).
- **FAIL**: Enter the fix loop (Step 5a) with the user-test failure details. After the fix, re-run **both** validators (Step 4, then Step 6).
- **SKIP** (all checks skipped): Log to progress.md: `<timestamp> -- Task N: user-test validation SKIPPED (all checks infrastructure-only).` Proceed to Step 6a (demo video).

### Step 6a: Demo Video (Frontend Only)

The orchestrator decides whether this task has a frontend component by checking the diff. If the task touches UI files (components, pages, styles, templates), it has a frontend component.

**If frontend component exists:**

Spawn a Claude subagent (`subagent_type=darkfactory-claude`) to run `/demo-video`. The subagent should save the recording to `mission/validations/task-N-demo.mp4`.

**Demo failure is non-blocking.** If the demo fails to record:
- Log to progress.md: `<timestamp> -- Task N: demo video FAILED to record. <reason>. Continuing.`
- Proceed to Step 7.

**If no frontend component:** Skip this step. Proceed to Step 7.

### Step 7: Track Progress

1. Mark the task complete in `tasks.md`: change `- [ ]` to `- [x]` for this task.

2. **Generate codebase delta for subsequent briefs.** Run `git diff $PRE_TASK_SHA..HEAD` (where `PRE_TASK_SHA` is the value recorded in Step 2 before this task was dispatched) to extract what this task added. Summarize into a codebase delta entry containing: files created/modified (paths), public function/endpoint signatures, key types, and patterns used. Store this delta in the plan under a `## Codebase Deltas` section. The next task's brief (Step 1) reads accumulated deltas from the plan and writes them into the "What Previous Tasks Built" section.

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

4a. **Foundation audit (after task 1 only).** If this is task 1 and it passed both validators, run a foundation check before proceeding to task 2:

   - Read task 1's Produces field from its topology metadata in tasks.md
   - Extract module names from the Produces field. Look up those modules in design.md's Module Inventory. Verify that corresponding directories or files exist in the committed code with matching names.
   - Extract type names from the Produces field. Look up those types in design.md's Type Catalog. Verify that corresponding type definitions exist in the committed code with matching names.

   If drift is detected (module structure or type names do not match the design for items task 1 declared it would produce):
   - Generate a corrective task 1a
   - Insert it in tasks.md between task 1 and task 2
   - Add checks for it to contract.md
   - Present both to the human for approval
   - The corrective task goes through the normal build loop before task 2 starts

   If no drift: log to progress.md: `<timestamp> -- Foundation audit PASSED. Module structure and type names confirmed for task 1 produces.`

   This audit runs only after task 1. It does not run after any other task.

5. **If more tasks remain:** Go back to Step 1 for the next task.

6. **If all tasks are done:** Proceed to Phase 9.

**Gate -- proceed when:**
- [ ] Plan's `## Orchestrator Phase` field updated to this phase, step, and timestamp
- [ ] All tasks in tasks.md are `- [x]` (complete) or `- [~]` (skipped)
- [ ] No unchecked `- [ ]` tasks remain
- [ ] Each completed task has passing scrutiny and user-test reports
- [ ] Plan codebase deltas are current

---

## Phase 9: Mission Completion

> **Rules:** 1, 2, 5, 7
> **Goal:** Final gates, draft PR, and dual adversarial review.
> **Inputs:** All committed code (from Phase 8), contract.md, coverage-audit.md
> **Outputs:** Draft PR, adversarial review reports, plan status = COMPLETE

### Final Gates

Re-run whole-repo quality checks.

Run the project's build, test, lint, and format commands against the full repository:
- Build: does it compile/build cleanly?
- Tests: do all tests pass?
- Lint: zero warnings (e.g., `cargo clippy -- -D warnings`, `eslint .`)
- Format: zero diffs (e.g., `cargo fmt --check`, `prettier --check .`)

Also check `mission/deferrals.md`. If the file exists and contains entries, present them to the human: each line is a SKIP'd check from an earlier task that was supposed to be picked up later. For each, the human decides: accept (the deferral is fine), resolve (spawn a fix task to cover it), or dispute (the check was never actually deferred). Clear the file before proceeding once the human has ruled on every entry.

**If all gates pass:** Log to progress.md: `<timestamp> -- Final gates PASSED.` Proceed to the spec delta check.

**If any gate fails:** Spawn a fix task targeting only the gate failures. This fix task:
- Gets a brief describing the specific failures (error messages, warnings, format diffs)
- Goes through scrutiny validation only (skip user-test -- this is a hygiene fix)
- Follows the standard circuit breaker (3 attempts). If the circuit breaker triggers, ask the human.
- After the fix passes, re-run all four gates to confirm no new regressions

### Spec Delta Check

After final gates pass, verify that the built codebase actually contains what the coverage audit promised.

Spawn a Claude subagent (`subagent_type=darkfactory-claude`). Read [prompts/spec-delta-checker.md](prompts/spec-delta-checker.md) and use it as the subagent prompt. Fill in the coverage-audit.md content and the report output path.

Read the delta check report from `mission/validations/spec-delta.md`.

**If CLEAN (no discrepancies):** Log to progress.md: `<timestamp> -- Spec delta check CLEAN. <N> items verified, <M> approved deviations listed.` Proceed to create the draft PR.

**If DISCREPANCIES FOUND:** Present each discrepancy to the human with three options:
- **Add fix task** -- create a new task to implement the missing item. Write checks for this task and append them to contract.md (present to human for quick approval). The task then goes through the normal build loop (brief, worker, scrutiny validation, user-test validation). After completion, re-run final gates and the spec delta check.
- **Reclassify as APPROVED-DEVIATION** -- the item was covered in planning but the implementation intentionally diverged. The human provides a reason and classification (Interpretation/Substitution/Removal). Update the coverage audit on disk.
- **Accept gap** -- acknowledge the gap and proceed. Log it in progress.md as an accepted gap.

Log the human's decision for each discrepancy to progress.md.

### Create Draft PR

Create a **draft** pull request using `gh pr create --draft`. Never create a ready-for-review PR. Use the PR body template below.

#### PR Body Template

Read [templates/pr-body.md](templates/pr-body.md) and fill in the values.

### Dual Adversarial Review

After the draft PR is created, run a final adversarial review that walks every mission commit one at a time, groups findings by task, and does a final cross-task pass. Reviewers do not read the full diff in one go -- that stalls on large missions. The per-commit loop keeps each read bounded.

Run two independent reviewers in parallel:
- **Reviewer A** (engine from `adversarial_reviewer_a` in config.yaml, default: codex) -- dispatched via Bash with `run_in_background: true`
- **Reviewer B** (engine from `adversarial_reviewer_b` in config.yaml, default: claude) -- dispatched via Agent tool with `subagent_type=darkfactory-claude`

Both receive the **pinned base SHA** (`base_sha` from config.yaml, captured in Phase 7), the full contract.md, and the report path. The prompt instructs each reviewer to run `git log --reverse --format="%h %s" <base-sha>..HEAD` itself and iterate. Pinning to a SHA keeps the review scope fixed even if branches move during the review.

Before dispatching either reviewer, read `base_sha` from `openspec/changes/<change-name>/mission/config.yaml`:

```bash
BASE_SHA=$(awk -F: '/^[[:space:]]*base_sha[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' openspec/changes/<change-name>/mission/config.yaml)
if ! git cat-file -e "${BASE_SHA}^{commit}" 2>/dev/null; then
  echo "ABORT: base_sha missing or not a valid commit in openspec/changes/<change-name>/mission/config.yaml"
  exit 1
fi
```

Pass `$BASE_SHA` into both reviewer prompts.

#### Adversarial Reviewer Prompt (used for both A and B)

Read [prompts/adversarial-reviewer.md](prompts/adversarial-reviewer.md) and use it as the subagent prompt. Fill in the base SHA (from `$BASE_SHA` above), the full contract.md content, and the absolute report path.

#### Reviewer A Dispatch Script (codex)

```bash
REPORT_PATH="<absolute path to mission/validations/adversarial-a.md>"
LOG_PATH="<absolute path to mission/logs/adversarial-a-codex.log>"
PROMPT_PATH=$(mktemp /tmp/mission-prompt.XXXXXX)

# Guard mission-prompt* from the stale temp cleanup
find /tmp -mindepth 1 -maxdepth 1 -user "$(whoami)" -mmin +30 -not -name "claude-*" -not -name "*mission-prompt*" \( -type f -delete -o -type d -exec rm -rf {} + \) 2>/dev/null

# Write the adversarial-reviewer.md prompt content to $PROMPT_PATH with $BASE_SHA, contract.md content, and report path filled in

# Dispatch via darkfactory-dispatch: process supervisor with timeout, heartbeat, and structured JSON result
DISPATCH_RESULT=$(darkfactory-dispatch \
  --model gpt-5.6-terra \
  --effort high \
  --timeout 2700 \
  --sandbox workspace-write \
  --prompt-file "$PROMPT_PATH" \
  --output "$REPORT_PATH" \
  --cwd "$(pwd)" \
  --log "$LOG_PATH" \
  --json 2>>"$LOG_PATH")

rm -f "$PROMPT_PATH"

if [ -z "$DISPATCH_RESULT" ]; then
  echo "REVIEW_FAILED: dispatcher crashed with no output. Log: $LOG_PATH"
else
  DISPATCH_STATUS=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.status')
  DISPATCH_ERROR=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.error.message // empty')

  if [ "$DISPATCH_STATUS" = "completed" ] && [ -f "$REPORT_PATH" ]; then
    echo "REVIEW_COMPLETE: report written"
  else
    echo "REVIEW_FAILED: dispatch status $DISPATCH_STATUS${DISPATCH_ERROR:+; error: $DISPATCH_ERROR}; report $([ -f "$REPORT_PATH" ] && echo present || echo missing). Log: $LOG_PATH"
  fi
fi
```

The Bash tool returns a shell task ID immediately. Reviewer B runs synchronously via the Agent tool. To run them in parallel: **dispatch Reviewer A first** (non-blocking, returns a shell task ID), **then dispatch Reviewer B** (blocks until it returns). When B returns, call BashOutput on A's shell task ID -- if A is done, the final line is `REVIEW_COMPLETE` or `REVIEW_FAILED`; if not, wait for A's completion notification before reading.

**Human visibility:** To watch Reviewer A progress live, run `tail -f <absolute path to mission/logs/adversarial-a-codex.log>` in another terminal.

### Present to Human

Log to progress.md: `<timestamp> -- Draft PR created: <URL>. Adversarial reviews complete.`

Read `mission/fallbacks.log` if it exists. If it contains entries, build the fallback summary section. If the file does not exist or is empty, omit the section entirely.

Present the results to the user:

```
Mission complete.

Draft PR: <URL>

[Only if mission/fallbacks.log has entries:]
Codex Fallback Summary:
- <count> of <total tasks> tasks required Claude fallback:
  - Task N: <reason> (<task-name>)
  - Task M: <reason> (<task-name>)

Adversarial Review A (Codex):
<summary of findings -- count by severity>

Adversarial Review B (Claude):
<summary of findings -- count by severity>

Full reports at:
- mission/validations/adversarial-a.md
- mission/validations/adversarial-b.md

Review the PR and the adversarial findings. You decide what to address.
If you decide not to ship this mission (reviewers found blockers, priorities changed), close the draft PR yourself: `gh pr close <PR number> --delete-branch`. Stop here -- do not proceed to the After merge steps below. The orchestrator only handles the happy path and circuit-breaker abort.
After merge:
1. Run /opsx:archive to move the change files to the archive directory
2. Commit and push the archive move:
   - If main is unprotected: `git checkout main && git pull && git add openspec/ && git commit -m "chore: archive <change-name>" && git push`
   - If main is protected: create a branch (`git checkout -b archive/<change-name>`), commit, push, open a PR, merge it
```

Update plan status to COMPLETE.

**Gate -- mission is COMPLETE when:**
- [ ] Final gates passed (build, test, lint, format)
- [ ] Spec delta check clean (no unresolved discrepancies)
- [ ] Draft PR created
- [ ] Both adversarial reviews complete and presented to human
- [ ] Plan status = COMPLETE
