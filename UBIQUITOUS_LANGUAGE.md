# Ubiquitous Language

## Mission system

- **Mission** -- An autonomous run of the full plan-build-validate cycle for a single OpenSpec change. *Avoid:* job, run, session, task
- **Orchestrator** -- The main Claude Code session that plans work, dispatches workers, reads results, and decides what to do next. *Avoid:* coordinator, manager, controller
- **Worker** -- A subagent (default: Codex) that implements a single task and commits the result. *Avoid:* builder, implementer, coder
- **Validator** -- A Claude subagent that checks a worker's output against the contract without having seen the code before. *Avoid:* reviewer, checker, tester

## Planning and contracts

- **Contract** -- A set of checks written during planning that define "done" independently of implementation, consumed only by validators. *Avoid:* test plan, acceptance criteria, spec
- **Check** -- A single verifiable statement in a contract that a validator can verify from outside the code. *Avoid:* assertion (in contract context), test case, requirement
- **Brief** -- A self-contained markdown file the orchestrator writes for a worker, containing the task description, project context, and procedures. *Avoid:* prompt, instructions, spec
- **Handoff** -- A structured report a worker writes to disk after completing a task, including the commit SHA, approach taken, and any concerns. *Avoid:* report, summary, output
- **Vertical slice** -- The default task quality standard. Each task must produce a concrete, testable output. When the approved strategy is vertical, tasks cross all layers. When layered or hybrid, tasks may be scoped to a single layer with a clear done state and test surface. *Avoid:* horizontal slice, layer
- **Deviation classification** -- Categorization of spec deviations during the coverage audit as Interpretation (ambiguous spec, reasonable reading), Substitution (replacing specified approach with a different one), or Removal (not implementing a specified feature). Substitutions and removals require explicit human approval. *Avoid:* deviation type, deviation level
- **Spec delta check** -- A post-build verification step (Phase 9) where a Claude subagent checks that every item marked COVERED in the coverage audit is actually present in the built codebase. APPROVED-DEVIATION items are listed but not re-litigated. *Avoid:* final spec check, compliance check, coverage verification

## Validation

- **Gate** -- A numbered step in the scrutiny validator's sequence (e.g., gate 3: tests, gate 11: contract checks). Gates report PASS, FAIL, WARN, SKIP, or N/A. *Avoid:* check (in validator-step context), step, stage
- **Scrutiny validator** -- The first validator type that runs build, test, lint, code review, code simplify, and test health gates against the contract. *Avoid:* code reviewer, linter, static validator
- **User-test validator** -- The second validator type that tests behavior by hitting the API (HTTP tests) or driving a browser (agent-browser). *Avoid:* QA, E2E tester, integration tester
- **Behavioral conformance** -- A scrutiny validator gate (gate 12) that verifies observable behavior by writing and executing a test script rather than reading the code. Falls back to "verified by inspection" if execution is inconclusive. *Avoid:* behavioral test, execution test, runtime check
- **Final gates** -- Whole-repo build, test, lint, and format gates run after the last task completes but before PR creation. Catches regressions introduced by later tasks (e.g., documentation or CI tasks) after the cleanup task ran. *Avoid:* final checks, post-build gates, repo gates
- **Module shape re-check** -- A mandatory closing task that compares design.md module descriptions and Type Catalog names to what the code actually exposes, catching cumulative drift across tasks that per-task scrutiny cannot see. *Avoid:* shape check, surface audit, drift check
- **Demo video** -- A recording of frontend functionality captured after both validators pass, created for human review. *Avoid:* screencast, screen recording
- **Fix loop** -- The cycle where a validation failure spawns a fix worker, then both validators re-run, up to 3 attempts before asking the human. *Avoid:* retry, repair cycle
- **Circuit breaker** -- The rule that stops the fix loop after 3 failed attempts on the same task and asks the human for guidance. *Avoid:* retry limit, fail-safe

## Test health

- **Silent skip** -- A test that conditionally returns early without testing anything but reports PASS to the test runner, hiding missing coverage. *Avoid:* conditional skip, soft skip, hidden skip
- **Diff coverage** -- Line coverage measured only on added lines in a task's git diff, not the entire codebase. *Avoid:* new code coverage, incremental coverage
- **Error handling smell** -- A language-specific anti-pattern in error handling code (e.g., `.unwrap()` in Rust, `catch {}` in TypeScript, `except: pass` in Python) counted per KLOC of changed code. *Avoid:* code smell, anti-pattern, bad practice
- **File size pre-pass** -- A deterministic line count on touched files injected into the module depth review, ensuring the LLM reviewer cannot overlook large files. *Avoid:* line count check, size gate
- **Test quality review** -- An LLM-based semantic review (owned skill: `/test-quality`) that evaluates whether tests verify meaningful behavior, checking for tautological assertions, implementation coupling, untested branches, and missing edge cases. *Avoid:* test review, test audit, test assessment
- **Tautological assertion** -- A test assertion that passes regardless of whether the code works correctly (e.g., `assert!(true)`, `expect(x).toBeDefined()` without checking the value). *Avoid:* empty assertion, no-op test, fake test
- **Self-skipping gate** -- A scrutiny gate that naturally skips itself when the diff contains no relevant files, requiring no explicit closing-task exemption. *Avoid:* auto-skip, conditional gate, self-gating check

## Execution model

- **Serial execution** -- The rule that only one worker runs at a time to prevent conflicts. *Avoid:* sequential, single-threaded
- **Engine** -- The configurable AI model/tool used for a given role (e.g. Codex for workers, Claude for validators). *Avoid:* model, provider, backend
- **Plan mode** -- The Claude Code plan that serves as the orchestrator's durable operating manual, guaranteed to survive context compaction. *Avoid:* system prompt, instructions

## File artifacts

- **Mission directory** -- The `mission/` folder nested inside an OpenSpec change directory that holds all runtime state for a mission. *Avoid:* state folder, workspace
- **Progress log** -- The `progress.md` file where the orchestrator records task status, decisions, and issues as the mission runs. *Avoid:* status file, mission log
- **Mission config** -- The `config.yaml` file specifying which engine to use for each role in the mission. *Avoid:* settings, preferences

## OpenSpec integration

- **Change** -- An OpenSpec unit of work with its own proposal, design, tasks, and now contract and mission directory. *Avoid:* feature, epic, project
- **Proposal** -- The mission's single intent document and first OpenSpec artifact, authored in Phase 3 from the grill: Problem, Approach, Rejected alternatives, and the Capabilities index the specs are built from. Replaces the earlier separate PRD GitHub issue. *Avoid:* PRD, intent doc, requirements doc
- **Design** -- The OpenSpec artifact describing how it will be built (architecture decisions). *Avoid:* architecture doc, technical spec
- **Tasks** -- The OpenSpec artifact listing tasks as checkboxes, each representing one worker's worth of work (should follow the approved task quality standard). *Avoid:* checklist, todo list, backlog
- **Feature task** -- A task in **Tasks** that implements required behavior; the only kind an **Acceptance** `task` link may target. *Avoid:* implementation task, work task
- **Closing task** -- A mandatory end-of-mission task (smoke test, cleanup, module shape re-check, CLAUDE.md/ARCH.md, documentation, optional CI) that implements no requirement and carries a `closing_kind`; an **Acceptance** link never points at one. *Avoid:* wrap-up task, final task

## Dispatch

- **Dispatcher** -- The `darkfactory-dispatch` Rust binary that sits between the orchestrator and a Codex worker as a process supervisor. *Avoid:* wrapper, harness, runner, proxy
- **Dispatch** -- The act of spawning a supervised Codex subprocess through the dispatcher, including process group setup, timeout management, and structured result reporting. *Avoid:* launch, invoke, run
- **Heartbeat** -- A periodic NDJSON event emitted by the dispatcher to stderr, carrying elapsed time and last activity, used to distinguish "working" from "dead". *Avoid:* ping, keepalive, pulse
- **Two-phase timeout** -- The dispatcher's timeout strategy: a soft timeout sends SIGTERM and arms a grace period (default: half the timeout, minimum 1 second), then a hard timeout sends SIGKILL. *Avoid:* timeout, deadline, kill timer
- **Process group** -- An OS-level grouping (via `setpgid`) that ensures killing the Codex process also kills any grandchild processes it spawned. *Avoid:* process tree, child processes
- **Dispatch store** -- A minimal filesystem directory at `~/.darkfactory/dispatches/<ulid>/` holding `status.json`, `result.json`, `host.pid`, and `events.jsonl` for async dispatch lifecycle management. *Avoid:* state directory, result store
- **Dispatch result** -- A structured JSON object emitted on stdout when a dispatch completes, containing status, exit code, signal, duration, output path, token usage, and activity. *Avoid:* result, outcome, response

## Planning determinism

- **Deterministic Anchors** -- One-liner pointers in the plan to the six highest-leverage design decisions, positioned to survive context compaction. *Avoid:* anchors, plan summary, design cache
- **Architecture Candidate Scoring** -- Evaluating 2-3 architecture options against a four-dimension rubric (separation of concerns, testability, task independence, module depth) to select an architecture deterministically. *Avoid:* architecture review, arch comparison
- **Task Independence** -- A scoring dimension measuring how well an architecture allows tasks to be completed without cross-task coordination on conventions. *Avoid:* worker task-ability, decomposability
- **Task Topology** -- Per-task metadata declaring which prior tasks it depends on, which surfaces it consumes, which surfaces it produces, and which architectural layers it touches. *Avoid:* task graph, dependency map
- **Foundation Audit** -- A one-time check after task 1 passes validation, verifying that committed code matches the design's module inventory and type catalog for task 1's declared produces. *Avoid:* post-task-1 check, foundation check
- **Type Catalog** -- A design.md section listing domain types with name, purpose, and owning module -- no field-level detail. Type names are drawn from this glossary when a matching term exists; types not in the glossary are named by the worker. *Avoid:* type registry, type list, types.md
- **Domain type** -- A type that models the problem space. Excludes technical helpers (CLI parsers, test fixtures, generic containers) unless they are public compatibility surfaces. *Avoid:* helper type, utility type, internal type
- **Module Inventory** -- A design.md section listing modules with interface width, what they hide, and why the boundary exists. *Avoid:* module table, module list
- **Design Template** -- A reference document defining mandatory design.md sections and scoring rubrics, used by the orchestrator alongside openspec config rules. *Avoid:* design spec, design schema
- **Spec Reference** -- A capability/scenario identifier (e.g., add-todo/1) used to trace contract checks back to formal spec requirements. *Avoid:* REQ-ID, requirement ID, spec ID
- **Scenario** -- One WHEN/THEN behavior inside a capability's spec.md, numbered within the file to form its **Spec Reference** (capability/N); the unit **Computed coverage** counts and reports gaps against. *Avoid:* case, example

## Frontend architecture

- **Deep component** -- A React component with a small prop interface that hides significant state, effects, and rendering logic, owning one user-facing workflow end-to-end. *Avoid:* smart component, container component, feature component
- **Allowed-shallow exception** -- A component that is intentionally thin and exempt from depth rules: design system leaves, route/layout glue, adapter components, or headless behavior primitives. *Avoid:* dumb component, presentational component, wrapper
- **Component contract** -- A lightweight per-component declaration written before implementation, specifying name, deep/shallow classification, prop interface, and key render states. *Avoid:* component spec, component design, component plan
- **Design-implement-test** -- The `/frontend` skill's implementation workflow: design component interfaces, build with visual verification, then write behavioral tests. Replaces red-green-refactor for React components. *Avoid:* frontend TDD, component TDD
- **Visual verification** -- Using chrome-devtools-mcp during the Build phase to confirm a component renders correctly before writing tests. *Avoid:* visual testing, screenshot testing, visual QA
- **Frontend depth gate** -- A self-skipping scrutiny validator gate (gate 7) that evaluates React component structure against the four deep-component rules when `.tsx`/`.jsx` files are in the diff. *Avoid:* component review, React lint, frontend lint
- **Methodology** -- The implementation workflow skill assigned to a task -- either `/tdd` (red-green-refactor) or `/frontend` (design-implement-test). Mutually exclusive per task. *Avoid:* implementation skill, build skill, workflow
- **Constraint** -- An additive design-guidance skill layered on top of a methodology -- `/ui-system`, `/react-perf`, or future `/backend-ts`. Zero or more per task. *Avoid:* supplementary skill, pattern skill
- **Task layer** -- A specific vocabulary value in task topology metadata that determines which methodology applies. `frontend/component`, `frontend/page`, `frontend/layout` map to `/frontend`; `frontend/hooks`, `backend`, `shared/*` map to `/tdd`. *Avoid:* task type, task category
- **Browser tools** -- A mission config flag (`browser_tools: true/false`) indicating whether chrome-devtools-mcp is required. Auto-set by the orchestrator when any task has `Skill: /frontend`. *Avoid:* browser testing, visual tools

## Frontend: deep component rules

- **Small semantic interface** -- A component prop interface where each prop represents a domain concept, not an implementation knob. Count concepts, not props. Audit hidden inputs (context, URL, cache). *Avoid:* narrow props, minimal API
- **Derive don't sync** -- Computing values during render (useMemo, inline computation) instead of syncing state with useEffect. Effects that transform data without I/O are a depth violation. *Avoid:* computed state, reactive state
- **Composition over configuration** -- Using compound component slots and children instead of config objects with many options. Callers compose behavior without understanding internals. *Avoid:* compound components, slots pattern
- **Collapse coordination** -- A deep component owns one workflow end-to-end, exposing semantic events (onSubmit) not coordination knobs (onValidate, onStep1Complete). Parents choose what happens, not how. *Avoid:* single responsibility, workflow ownership

## Mechanical enforcement

- **Mechanical enforcement** -- Binary pass/fail rules enforced by tooling (hooks, linters, compiler flags) rather than advisory instructions the agent might forget. Inspired by Armin Ronacher / Christina's friction talk. *Avoid:* best practices, guidelines, conventions
- **Lint audit** -- A darkfactory skill (`/lint-audit`) that audits a TypeScript project's oxlint config and tsconfig strictness against an agent-friendly ruleset, reports gaps, and offers to apply fixes. *Avoid:* lint check, config review
- **Agent-friendly ruleset** -- The recommended set of oxlint rules and tsconfig settings that mechanically enforce patterns agents tend to violate: type safety bypasses, silent failures, error suppression, dead code. *Avoid:* recommended rules, best practice rules
- **ts-quality hook** -- A Claude Code PostToolUse hook (`ts-quality.sh`) that rejects Edit/Write of `.ts`/`.tsx` files containing banned patterns via `fileRejection`. Complementary to the lint audit -- hook catches patterns at edit time, linter catches them at build time. *Avoid:* TypeScript hook, quality hook
- **Strictness settings** -- tsconfig.json compiler options that strengthen the type system: `strict`, `noUncheckedIndexedAccess`, `erasableSyntaxOnly`. Without these, the compiler accepts code that the linter and hook assume is rejected. *Avoid:* strict mode, compiler flags
- **Migration scope** -- The number of compiler errors surfaced by enabling a strictness setting in an existing project, measured by `tsc --strict --noEmit 2>&1 | wc -l`. Used to estimate effort before committing to a strictness migration. *Avoid:* error count, migration cost

## Requirement-as-atom planning (Darkfactory 2)

- **Requirement record** -- The single source-of-truth unit for one required behavior, holding `id`, `behavior`, one or more `acceptance` entries, and `status`; each acceptance entry owns its own `task` link (any record-level task association is their computed union). All planning views render from it. *Avoid:* atom, requirement atom, spec item
- **Behavior (field)** -- The requirement field that tells the builder the observable user outcome to produce, carrying no validator internals. *Avoid:* description, spec text, requirement text
- **Acceptance (field)** -- The requirement field holding the falsifiable check that proves the behavior, carrying its **Verification method** tag, `task` link, and **Spec Reference**; rendered into the contract and never shown to the worker. *Avoid:* criteria, acceptance criteria, test, check, contract check
- **Verification method** -- The tag on an acceptance entry declaring how a validator checks it: `[execution]` (run a command), `[output]` (inspect output, file, or response), or `[inspection]` (read code); `[inspection]` is capped at 20% of a task's checks. *Avoid:* check type, verification tag
- **Projection** -- A read-only view rendered from the requirement record rather than authored: the contract, the worker brief's behavior slice, and the **Gap report**. *Avoid:* view, generated artifact, derived doc
- **Render** -- To mechanically produce a projection from the requirement record, as opposed to authoring it with an LLM. *Avoid:* generate, project (verb)
- **Renderer** -- The deterministic tool that renders the contract from acceptance fields grouped by task, carrying check text verbatim and never inventing checks. *Avoid:* generator, contract generator, projector
- **Shadow mode** -- Running a renderer alongside the existing authoring step and recording text diffs against the hand-authored artifact for inspection, without replacing anything; cutover parity is judged by **Validator-outcome parity**, not by text match. *Avoid:* dry run, preview mode
- **Cutover** -- Switching a projection from hand-authored to rendered once the cutover gate is met, deleting the authoring step. *Avoid:* switchover, migration
- **Cutover gate** -- The criteria required before a cutover: validator-outcome parity across diverse mission shapes (small CLI, large multi-task, frontend, deviation-heavy), acceptance and control contract authored in separate sessions, a seeded failure plus at least one real missed or weak requirement caught, and a rollback path. *Avoid:* go/no-go, migration gate
- **Validator-outcome parity** -- The cutover test that a rendered contract makes validators pass or fail the same tasks as the hand-authored contract, rather than matching its text. *Avoid:* text parity, diff match
- **Duplicated-artifact drift** -- Divergence between documents that re-author the same fact (e.g. specs vs contract); the specific failure the requirement record removes, distinct from behavior-vs-acceptance and acceptance-vs-code drift, which remain. *Avoid:* drift, document drift
- **Deviated status** -- A terminal requirement `status` recording an approved divergence, keeping the original acceptance immutable and adding a deviation sub-field (actual behavior, reason, classification, approver, date, temporary/permanent). *Avoid:* APPROVED-DEVIATION, waiver
- **Computed coverage** -- The Stage 2 projection (EPIC #199; see docs/darkfactory-2.md) that renders a **Gap report** from the **Requirement records**: a tolerant, scenario-level pass listing every **Coverage gap** plus a total/covered/gaps summary, computed instead of authored as a table. Reports gaps; never errors on them. The v2 replacement for the hand-authored **Coverage audit** -- during shadow mode both exist. *Avoid:* coverage table, gap audit
- **Coverage gap** -- A spec scenario none of whose acceptance entries link to a valid feature task; the unit **Computed coverage** reports. *Avoid:* missing requirement, uncovered item, hole
- **Gap kind** -- The reason a scenario is a **Coverage gap**: no acceptance authored, a dangling task link, or a link to a closing task. *Avoid:* gap type, gap reason
- **Gap report** -- The **Projection** rendered by **Computed coverage** -- the list of **Coverage gaps** plus one total/covered/gaps summary line -- emitted as markdown with a three-way exit code distinguishing no gaps (success), gaps found (a tolerant, non-failing result), and a usage error; the exact codes are fixed in the spec. *Avoid:* coverage report, gap list

## Relationships

- A **Mission** belongs to exactly one **Change**
- A **Change** contains one **Proposal**, one **Design**, one **Tasks** list, one **Contract**, and one **Mission directory**
- Each **Task** maps to one **Worker** invocation, one git commit, and one validation cycle
- A **Contract** contains **Checks** grouped by **Task**
- The **Orchestrator** writes **Briefs** and reads **Handoffs**
- The **Orchestrator** spawns Codex **Workers** through the **Dispatcher**, and Claude **Workers** and **Validators** through the Agent tool
- The **Dispatcher** supervises the Codex process: it manages the **Process group**, emits **Heartbeats**, enforces the **Two-phase timeout**, and returns a **Dispatch result**
- **Workers** read **Briefs** and write **Handoffs** -- they never see the **Contract**
- **Validators** read the **Contract** and the commit diff -- they never see the **Proposal** or **Design**
- A **Fix loop** produces versioned **Handoffs** (task-N-fix-v2.md) and re-runs both **Validators**
- The **Circuit breaker** halts the **Fix loop** and returns control to the human
- The **Dispatch store** is independent of the **Mission directory** -- it holds dispatcher metadata, not mission state
- The **Scrutiny validator** runs deterministic **Gates** (build, lint, **Silent skip** detection, **Diff coverage**, **Error handling smell** density, **File size pre-pass**) and LLM **Gates** (code review, module depth, **Test quality review**)
- **Diff coverage** measures added lines only -- a worker is responsible for covering the code they wrote, not backfilling pre-existing debt
- The **Test quality review** (`/test-quality`) is an owned skill invoked by the **Scrutiny validator** as a subagent, following the same pattern as `/review` and `/simplify`
- A **Tautological assertion** is worse than no test because it gives false confidence -- the **Scrutiny validator** treats it as a hard FAIL
- **Deterministic Anchors** are populated from design.md during Phase 4.2 and stored in the **Plan** -- they reference the **Type Catalog**, **Module Inventory**, and **Architecture Candidate Scoring** results
- The **Type Catalog** draws **Domain type** names from this glossary. **Worker briefs** list relevant **Type Catalog** entries as exact names to use in code. The **Foundation Audit** checks task 1 names against the **Type Catalog**. The **Module shape re-check** verifies all **Type Catalog** names at the end.
- The **Foundation Audit** runs once after task 1, comparing committed code against the **Module Inventory** and **Type Catalog** from design.md
- **Spec References** in the **Contract** trace **Checks** back to specific **Spec** capability/scenario pairs (e.g., add-todo/1), replacing arbitrary row numbers
- **Task Topology** metadata is written per task in tasks.md and reviewed during Phase 4.3 -- tasks touching more than 2 architectural layers are flagged for human review
- The **Design Template** is a reference document in `templates/` -- the actual design.md generation is driven by openspec config rules that require the same sections
- A **Task** has exactly one **Methodology** (`/tdd` or `/frontend`) and zero or more **Constraints** (`/ui-system`, `/react-perf`)
- The **Task layer** determines the **Methodology**: `frontend/component`, `frontend/page`, `frontend/layout` map to `/frontend`; all other layers map to `/tdd`
- `frontend/hooks` maps to `/tdd` despite being frontend code -- hooks have testable interfaces suited to red-green-refactor
- The **Frontend depth gate** (gate 7) is a **Self-skipping gate** that only fires when `.tsx`/`.jsx` files are in the diff
- **Visual verification** requires **Browser tools** (`browser_tools: true` in **Mission config**), checked at mission-init and at dispatch time
- A **Deep component** satisfies all four rules: **Small semantic interface**, **Derive don't sync**, **Composition over configuration**, **Collapse coordination**
- An **Allowed-shallow exception** is exempt from depth rules but must be classified: design system leaf, route/layout glue, adapter, or headless behavior primitive
- The **Component contract** is written per component during the `/frontend` workflow's Contract phase, before the Build phase begins
- A **Requirement record** has one **Behavior** and one or more **Acceptance** entries, and is the source the **Contract** is rendered from
- The **Renderer** projects the **Contract** from **Acceptance** fields grouped by **Task** -- a rendered **Check** corresponds one-to-one to an **Acceptance** entry
- **Workers** never see **Acceptance** or the rendered **Contract**; their **Brief** projects **Behavior** only
- **Shadow mode** precedes **Cutover**, and **Cutover** is allowed only when the **Cutover gate** holds (chiefly **Validator-outcome parity**)
- The **Requirement record** removes **Duplicated-artifact drift** but not behavior-vs-acceptance or acceptance-vs-code drift, which independent review checks must police
- A **Deviated status** keeps the original **Acceptance** immutable so the divergence is recorded, not laundered
- **Computed coverage** renders a **Gap report** from the same **Acceptance** task links the **Renderer** consumes, but tolerates missing links and reports each as a **Coverage gap** instead of failing -- it finds *missing* links (the mechanical join), not *wrong* ones, which stay a Codex judgment

## Example dialogue

> **Dev:** "I want to add real-time notifications to Toyo. Should I start a **Mission**?"
>
> **Orchestrator:** "That depends on the scope. How many **Tasks** do you think it involves?"
>
> **Dev:** "At least three -- WebSocket setup, notification delivery, and the UI. Each is a **Vertical slice** cutting through the API and frontend."
>
> **Orchestrator:** "That's a good fit. Let me **/grill** you on the requirements first, then I'll create the **Change** via **/opsx:propose** and write the **Contract** with its **Checks** before any **Workers** start."
>
> **Dev:** "What if the Codex **Worker** hangs during the WebSocket task?"
>
> **Orchestrator:** "The **Dispatcher** handles that. It emits **Heartbeats** every 15 seconds so I can tell if the worker is still active. If no progress happens, the **Two-phase timeout** fires -- first SIGTERM, then after the grace period SIGKILL to the entire **Process group**. I get a **Dispatch result** with status `timed_out` and fall back to a Claude **Worker**."
>
> **Dev:** "And I can check on progress while it runs?"
>
> **Orchestrator:** "Yes -- tail the **Dispatcher's** log for **Heartbeat** events, read the **Progress log** directly, or run **/mission-status** in another session for a formatted summary."

## Example dialogue (planning determinism)

> **Dev:** "Two missions against the same proposal produced completely different architectures. How do we prevent that?"
>
> **Orchestrator:** "The **Design Template** now requires **Architecture Candidate Scoring**. I evaluate 2-3 options on four dimensions -- separation of concerns, testability, **Task Independence**, and module depth -- each scored 0-3. No candidate can score 0 on any dimension. If two tie, you pick."
>
> **Dev:** "What stops the design from using different type names each run?"
>
> **Orchestrator:** "The **Type Catalog** in design.md names every **Domain type** using terms from the ubiquitous language. The **Worker brief** lists the relevant **Type Catalog** entries so the worker uses exact names. The **Foundation Audit** checks after task 1 that the committed code matches. At the end, the module shape re-check verifies all **Type Catalog** names again."
>
> **Dev:** "And if context compacts mid-mission and you lose the design details?"
>
> **Orchestrator:** "The **Deterministic Anchors** in the plan survive compaction. They're one-liner pointers to the six key decisions -- workspace layout, task strategy, domain types, error handling, test strategy, and chosen architecture. I can re-read design.md if I need the full detail, but the anchors tell me what to look for."

## Example dialogue (test health)

> **Dev:** "The benchmark shows irontodo-v2 has 97 tests but only 49 actually run. What happened?"
>
> **Orchestrator:** "Two tests are **Silent skips** -- they check for a database URL and `return` early if it's missing, reporting PASS while testing nothing. The other 46 panic on missing PostgreSQL. The **Scrutiny validator** would now catch the **Silent skips** with a deterministic grep and flag them as FAIL."
>
> **Dev:** "What about coverage? The tests that do run might not even cover the new code."
>
> **Orchestrator:** "That's what **Diff coverage** checks. It runs the language-appropriate coverage tool and measures only the added lines in the task's diff. Below 70% is a FAIL. Below 80% is a WARNING."
>
> **Dev:** "And what if the tests do cover the code but the assertions are meaningless?"
>
> **Orchestrator:** "The **Test quality review** catches that. It's an LLM subagent that reads the test diff alongside the production diff and flags **Tautological assertions** -- tests that pass regardless of correctness. A P0 finding like `assert!(true)` is a hard FAIL."

## Example dialogue (frontend architecture)

> **Dev:** "I need to build a payment form. Should that be one task or two?"
>
> **Orchestrator:** "Two. The validation logic (card format, expiry, required fields) is a hook with a testable interface -- that's a `/tdd` task with **Task layer** `frontend/hooks`. The form component itself is visual and owns the submission workflow -- that's a `/frontend` task with **Task layer** `frontend/component`."
>
> **Dev:** "How does the `/frontend` worker know what to build?"
>
> **Orchestrator:** "The workflow starts with a Scan (read existing components), then a **Component contract** -- name the component, declare it **Deep** or **Allowed-shallow**, define the prop interface, list render states. Then Build with **Visual verification** via chrome-devtools-mcp, then behavioral tests."
>
> **Dev:** "What if the worker builds a component with 15 props?"
>
> **Orchestrator:** "The deep-component audit checklist catches it before commit -- **Small semantic interface** says count concepts, not props. If the worker misses it, the **Frontend depth gate** (gate 7) flags it during scrutiny validation."
>
> **Dev:** "And if the project is Rust, none of this applies?"
>
> **Orchestrator:** "Correct. Every task gets `/tdd`, no **Constraints**, no **Browser tools**. The **Frontend depth gate** self-skips because there are no `.tsx` files in the diff."

## Example dialogue (requirement-as-atom planning)

> **Dev:** "In Darkfactory 2, who writes the **Contract**?"
>
> **Orchestrator:** "Nobody writes it directly anymore. Each scenario carries an **Acceptance (field)** -- a falsifiable check with its method tag and task link. The **Renderer** reads those and projects the **Contract**, grouped by task. It renders authored acceptance; it never invents a check."
>
> **Dev:** "Then what does the **Worker** see?"
>
> **Orchestrator:** "Only the **Behavior (field)**, projected into the **Brief**. The **Acceptance** and the rendered **Contract** stay hidden -- the same separation as v1."
>
> **Dev:** "If the renderer is deterministic, how do we trust it before we rely on it?"
>
> **Orchestrator:** "**Shadow mode**. It runs beside the hand-authored contract and we check **Validator-outcome parity** -- do validators pass and fail the same tasks. Only when the **Cutover gate** holds do we **Cutover** and delete the authoring step."
>
> **Dev:** "And this kills drift?"
>
> **Orchestrator:** "It kills **Duplicated-artifact drift** -- specs and contract can no longer disagree. It does not touch behavior-vs-acceptance or acceptance-vs-code drift; independent checks handle those."
>
> **Dev:** "And the coverage audit -- who writes that now?"
>
> **Orchestrator:** "Nobody. **Computed coverage** renders a **Gap report** from the records. It reads the same **Acceptance** task links the **Renderer** uses, but where the renderer fails loud on a bad link, coverage tolerates it and reports a **Coverage gap** -- one per scenario that no acceptance line points to a real feature task."
>
> **Dev:** "So it catches everything the hand-authored table did?"
>
> **Orchestrator:** "It catches the *missing* links -- the mechanical join. It does not catch a *wrong* link, a requirement piled on a task that can't satisfy it. The **Gap kind** tells you why each scenario is uncovered -- no acceptance, a dangling link, or a link to a closing task -- but judging whether a present link is the *right* one stays a Codex review."

## Flagged ambiguities

- **"template"** is used for two different things in the mission skill: the **Design Template** (a reference document defining mandatory design.md sections and scoring rubrics) and the **CLAUDE.md template** (a fill-in-the-blanks document for the closing task). Both live in `templates/` but serve different purposes. **Resolved:** use the full name ("design template" or "CLAUDE.md template") when context is ambiguous. The file names distinguish them: `design-template.md` vs `claude-md.md`.
- **"contract"** now spans two production methods: the v1 hand-authored `contract.md` and the v2 **Projection** rendered from **Acceptance** fields. **Resolved:** the **Contract** is the same artifact with the same definition; only its production changes. Say "rendered contract" when contrasting with the v1 authored one.
- **"check" vs "acceptance"** overlap across the record and the contract. **Resolved:** **Acceptance** is the source field on a **Requirement record**; a **Check** is what it becomes once rendered into the **Contract**. They correspond one-to-one; do not use "check" for the source field.
- **"generate" vs "render"** -- v1 uses "generate" for the orchestrator's LLM authoring of artifacts; v2 **Render** is deterministic. **Resolved:** reserve "render" for the deterministic projection and avoid "generate" for it, so the mechanical step is never confused with LLM authoring.
- **"atom" vs "record"** -- the model is named "requirement-as-atom," but the object has internal fields and is not indivisible. **Resolved:** use **Requirement record** in operational text; reserve "atom" for naming the model.

## Resolved ambiguities

- **"task"** was used early in the conversation to mean both a micro-step within a feature and a bounded unit of worker work. **Resolved:** keep "task" as the standard term. The guidance is that each task should follow the approved task quality standard, but the term "task" is unchanged from OpenSpec. "Slice" is not a standalone noun -- it's a quality standard for tasks.
- **"spec"** was used to mean both SDD spec files and the planning phase of the mission flow. **Resolved:** SDD specs are not part of Darkfactory's vocabulary. In Darkfactory, **Spec** refers exclusively to the planning phase (`align -> spec -> contract -> build -> validate -> ship`) where the orchestrator creates OpenSpec artifacts and the contract. SDD is a separate workflow for documenting existing code after the fact.
- **"validation"** was used for both the contract checks and the act of running validators. **Resolved:** the **Contract** contains **Checks**. **Validation** is the act of a **Validator** verifying those checks against the code. Be consistent: don't use "validation" to refer to the contract itself.
- **"coverage"** could mean line coverage (percentage of lines executed during tests) or spec coverage (percentage of proposal requirements mapped to tasks). **Resolved:** **Diff coverage** means line coverage on added lines in a git diff. **Coverage audit** means the spec-to-task mapping in Phase 4a. Don't use "coverage" without the qualifier.
- **"check" / "assertion" / "gate"** had overlapping usage across contract items, validator steps, and code test assertions. **Resolved** with a three-way split: a **Gate** is a numbered step in the scrutiny validator's sequence (e.g., gate 3: tests, gate 11: contract checks). A **Check** is a single verifiable statement in a contract. An **Assertion** is reserved for code test assertions only (e.g., **Tautological assertion**, `assert_eq!`, `expect().toBe()`). Don't use "assertion" for contract items or "check" for validator steps.
- **"skill"** could mean both a methodology (implementation workflow) and a constraint (design guidance). **Resolved** with a two-axis model: a **Methodology** is the implementation workflow (`/tdd` or `/frontend`, mutually exclusive per task). A **Constraint** is additive design guidance (`/ui-system`, `/react-perf`, zero or more per task). Both are skills, but the axes are orthogonal. Don't say "skill" when you mean methodology or constraint specifically.
- **"frontend"** as a task layer could mean component work (visual, needs `/frontend`) or hook work (logic, needs `/tdd`). **Resolved** with specific layer vocabulary: `frontend/component`, `frontend/page`, `frontend/layout` map to `/frontend`; `frontend/hooks` maps to `/tdd`. Don't use bare "frontend" as a task layer.
