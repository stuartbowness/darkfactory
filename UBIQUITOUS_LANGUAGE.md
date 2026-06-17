# Ubiquitous Language

> Scope: this file holds **ambiguity-prone domain terms** only — the words people and agents would otherwise confuse. How-it-works mechanics live in `ARCH.md`; skill-specific vocabulary and rules live in the owning skill; dispatcher internals live in `tools/darkfactory-dispatch/README.md`. A term earns a place here only if it is domain (not a mechanism), ambiguity-prone, definable in one sentence as what it IS, and shared across roles. Mechanics, tool internals, file/config names, and coding rules do not belong here.

## Mission system

- **Mission** -- An autonomous run of the full plan-build-validate cycle for a single OpenSpec change. *Avoid:* job, run, session, task
- **Orchestrator** -- The main Claude Code session that plans work, dispatches workers, reads results, and decides what to do next. *Avoid:* coordinator, manager, controller
- **Worker** -- A subagent (default: Codex) that implements a single task and commits the result. *Avoid:* builder, implementer, coder
- **Validator** -- A Claude subagent that checks a worker's output against the contract, never having seen the code before. *Avoid:* reviewer, checker, tester

## Planning and contracts

- **Contract** -- A set of checks written during planning that define "done" independently of implementation, consumed only by validators. *Avoid:* test plan, acceptance criteria, spec
- **Check** -- A single verifiable statement in a contract that a validator can confirm from outside the code. *Avoid:* assertion (in contract context), test case, requirement
- **Brief** -- A self-contained markdown file the orchestrator writes for a worker, containing the task, project context, and procedures. *Avoid:* prompt, instructions, spec
- **Handoff** -- A structured report a worker writes to disk after a task, including the commit SHA, approach, and concerns. *Avoid:* report, summary, output
- **Deviation classification** -- How a spec deviation is categorized during the coverage audit: Interpretation, Substitution, or Removal (the latter two require human approval). *Avoid:* deviation type, deviation level
- **Vertical slice** -- A task that cuts through all layers to a concrete, testable output. *Avoid:* horizontal slice, layer
- **Spec delta check** -- The Phase 9 step that verifies every COVERED coverage-audit item is actually present in the built code. *Avoid:* final spec check, compliance check, coverage verification

## Validation

- **Gate** -- A numbered step in the scrutiny validator's sequence (e.g., gate 3: tests, gate 11: contract checks), reporting PASS, FAIL, WARN, SKIP, or N/A. *Avoid:* check (in validator-step context), step, stage
- **Scrutiny validator** -- The first validator: runs build, test, lint, code review, and test-health gates against the contract. *Avoid:* code reviewer, linter, static validator
- **User-test validator** -- The second validator: tests behavior by hitting the API (HTTP tests) or driving a browser via `/test-app`. *Avoid:* QA, E2E tester, integration tester
- **Fix loop** -- The cycle where a validation failure spawns a fix worker, then both validators re-run, up to 3 attempts before asking the human. *Avoid:* retry, repair cycle
- **Circuit breaker** -- The rule that stops the fix loop after 3 failed attempts on a task and asks the human. *Avoid:* retry limit, fail-safe

_Gate-by-gate mechanics are in `skills/mission/prompts/scrutiny-validator.md`; the build loop is in `ARCH.md`._

## Test health

- **Silent skip** -- A test that conditionally returns early without testing anything yet reports PASS, hiding missing coverage. *Avoid:* conditional skip, soft skip, hidden skip
- **Diff coverage** -- Line coverage measured only on the added lines in a task's git diff, not the whole codebase. *Avoid:* new code coverage, incremental coverage
- **Tautological assertion** -- A test assertion that passes regardless of whether the code is correct (e.g., `assert!(true)`). *Avoid:* empty assertion, no-op test, fake test
- **Test quality review** -- The `/test-quality` subagent that judges whether tests verify meaningful behavior. *Avoid:* test review, test audit, test assessment

## Execution model

- **Serial execution** -- The rule that only one worker runs at a time. *Avoid:* sequential, single-threaded
- **Engine** -- The configurable AI model/tool used for a role (e.g., Codex for workers, Claude for validators). *Avoid:* model, provider, backend
- **Plan mode** -- The Claude Code plan that serves as the orchestrator's durable operating manual, surviving context compaction. *Avoid:* system prompt, instructions

## File artifacts

- **Mission directory** -- The `mission/` folder inside an OpenSpec change directory holding all runtime state for a mission. *Avoid:* state folder, workspace
- **Progress log** -- The `progress.md` file where the orchestrator records task status, decisions, and issues. *Avoid:* status file, mission log
- **Mission config** -- The `config.yaml` specifying which engine to use for each role. *Avoid:* settings, preferences

## OpenSpec integration

- **Change** -- An OpenSpec unit of work with its own proposal, design, tasks, contract, and mission directory. *Avoid:* feature, epic, project
- **Proposal** -- The mission's single intent document and first OpenSpec artifact, authored in Phase 3 from the grill (Problem, Approach, Rejected alternatives, Capabilities). *Avoid:* PRD, intent doc, requirements doc
- **Design** -- The OpenSpec artifact describing how the change will be built. *Avoid:* architecture doc, technical spec
- **Tasks** -- The OpenSpec artifact listing tasks as checkboxes, each one worker's worth of work. *Avoid:* checklist, todo list, backlog
- **Feature task** -- A task that implements required behavior. *Avoid:* implementation task, work task
- **Closing task** -- A mandatory end-of-mission task (smoke test, cleanup, module shape re-check, CLAUDE.md/ARCH.md, documentation, optional CI) that implements no requirement. *Avoid:* wrap-up task, final task

## Dispatch

- **Dispatcher** -- The `darkfactory-dispatch` Rust binary that supervises a Codex worker as a process supervisor. *Avoid:* wrapper, harness, runner, proxy
- **Dispatch** -- The act of spawning a supervised Codex subprocess through the dispatcher. *Avoid:* launch, invoke, run

_Supervisor internals (two-phase timeout, heartbeat, process group, result schema) are in `tools/darkfactory-dispatch/README.md`._

## Planning vocabulary

- **Type Catalog** -- A design.md section listing domain types by name, purpose, and owning module; names are drawn from this glossary when a matching term exists. *Avoid:* type registry, type list, types.md
- **Domain type** -- A type that models the problem space, excluding technical helpers unless they are public compatibility surfaces. *Avoid:* helper type, utility type, internal type
- **Scenario** -- One WHEN/THEN behavior inside a capability's spec.md, numbered to form its Spec Reference (capability/N). *Avoid:* case, example
- **Spec Reference** -- A capability/scenario identifier (e.g., add-todo/1) that traces a contract check back to a formal spec requirement. *Avoid:* REQ-ID, requirement ID, spec ID

_Design-artifact mechanics (the scoring rubric, deterministic anchors, task topology, foundation audit) are in `ARCH.md`._

## Frontend

- **Methodology** -- The implementation workflow assigned to a task: `/tdd` (red-green-refactor) or `/frontend` (design-implement-test), mutually exclusive per task. *Avoid:* implementation skill, build skill, workflow
- **Constraint** -- An additive design-guidance skill layered on a methodology (`/ui-system`, `/react-perf`); zero or more per task. *Avoid:* supplementary skill, pattern skill
- **Task layer** -- The task-topology value that picks the methodology: `frontend/component|page|layout` map to `/frontend`; `frontend/hooks`, `backend`, `shared/*` map to `/tdd`. *Avoid:* task type, task category
- **Deep component** -- A React component with a small prop interface that hides significant logic and owns one user-facing workflow end-to-end. *Avoid:* smart component, container component, feature component

_The four deep-component rules and the component workflow are in the `/frontend` skill (`DEEP-COMPONENTS.md`)._

## Relationships

- A **Mission** belongs to exactly one **Change**.
- A **Change** contains one **Proposal**, one **Design**, one **Tasks** list, one **Contract**, and one **Mission directory**.
- Each **Task** maps to one **Worker** invocation, one git commit, and one validation cycle.
- A **Contract** contains **Checks** grouped by **Task**.
- The **Orchestrator** writes **Briefs** and reads **Handoffs**; it spawns Codex **Workers** through the **Dispatcher**, and Claude **Workers** and **Validators** through the Agent tool.
- **Workers** read **Briefs** and write **Handoffs** -- they never see the **Contract**.
- **Validators** read the **Contract** and the commit diff -- they never see the **Proposal** or **Design**.
- The **Fix loop** re-runs both **Validators**; the **Circuit breaker** halts it and returns control to the human.
- A **Task** has exactly one **Methodology** and zero or more **Constraints**.

## Example dialogue

> **Dev:** "I want to add real-time notifications. Should I start a **Mission**?"
>
> **Orchestrator:** "How many **Tasks**? Each should be a **Vertical slice** -- a concrete, testable output."
>
> **Dev:** "Three -- WebSocket setup, delivery, and the UI."
>
> **Orchestrator:** "Good fit. I'll **/grill** you, create the **Change**, and write the **Contract** with its **Checks** before any **Workers** start. **Workers** never see the **Contract** -- the **Validators** do."
>
> **Dev:** "What if the Codex **Worker** hangs?"
>
> **Orchestrator:** "The **Dispatcher** supervises it, times out cleanly, and returns a result; I fall back to a Claude **Worker**. You can watch the **Progress log** or run **/mission-status** while it runs."

## Flagged ambiguities

- **"template"** refers to two different files in the mission skill: `design-template.md` (the design.md sections and scoring rubric) and `claude-md.md` (the CLAUDE.md closing-task template). Use the full filename when context is ambiguous.

## Resolved ambiguities

- **"task"** meant both a micro-step and a bounded unit of worker work. **Resolved:** "task" is the standard term (unchanged from OpenSpec); each task should meet the task quality standard. "Slice" is not a standalone noun -- it's a quality standard for tasks.
- **"spec"** meant both SDD spec files and the planning phase. **Resolved:** in Darkfactory, **Spec** refers to the planning phase (`align -> spec -> contract -> build -> validate -> ship`) where the orchestrator creates OpenSpec artifacts and the contract. SDD is a separate workflow, not part of the vocabulary.
- **"validation"** meant both the contract checks and the act of running validators. **Resolved:** the **Contract** contains **Checks**; **Validation** is a **Validator** verifying those checks against the code. Don't use "validation" for the contract itself.
- **"coverage"** could mean line coverage or spec coverage. **Resolved:** **Diff coverage** is line coverage on added lines in a git diff; **Coverage audit** is the spec-to-task mapping in Phase 4.5. Don't use "coverage" without the qualifier.
- **"check" / "assertion" / "gate"** overlapped across contract items, validator steps, and test assertions. **Resolved:** a **Gate** is a numbered scrutiny-validator step; a **Check** is a single verifiable statement in a contract; an **Assertion** is reserved for code test assertions. Don't cross them.
- **"skill"** could mean a methodology or a constraint. **Resolved** with a two-axis model: a **Methodology** is the implementation workflow (`/tdd` or `/frontend`, one per task); a **Constraint** is additive design guidance (`/ui-system`, `/react-perf`, zero or more). Both are skills, but the axes are orthogonal.
- **"frontend"** as a task layer could mean component work (`/frontend`) or hook work (`/tdd`). **Resolved** with specific layer vocabulary: `frontend/component|page|layout` map to `/frontend`; `frontend/hooks` maps to `/tdd`. Don't use bare "frontend" as a task layer.
