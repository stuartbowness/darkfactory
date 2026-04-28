# Ubiquitous Language

## Mission system

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Mission** | An autonomous run of the full plan-build-validate cycle for a single OpenSpec change | Job, run, session, task |
| **Orchestrator** | The main Claude Code session that plans work, dispatches workers, reads results, and decides what to do next | Coordinator, manager, controller |
| **Worker** | A subagent (default: Codex) that implements a single task and commits the result | Builder, implementer, coder |
| **Validator** | A Claude subagent that checks a worker's output against the contract without having seen the code before | Reviewer, checker, tester |

## Planning and contracts

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Contract** | A set of assertions written during planning that define "done" independently of implementation, consumed only by validators | Test plan, acceptance criteria, spec |
| **Assertion** | A single verifiable statement in a contract that a validator can check from outside the code | Test case, requirement, check |
| **Brief** | A self-contained markdown file the orchestrator writes for a worker, containing the task description, project context, and procedures | Prompt, instructions, spec |
| **Handoff** | A structured report a worker writes to disk after completing a task, including the commit SHA, approach taken, and any concerns | Report, summary, output |
| **Vertical slice** | A best practice for how to write good tasks -- each task should be a thin tracer-bullet path through all layers (schema, API, UI, tests) that delivers narrow but complete functionality | Horizontal slice, layer |
| **Deviation classification** | Categorization of spec deviations during the coverage audit as Interpretation (ambiguous spec, reasonable reading), Substitution (replacing specified approach with a different one), or Removal (not implementing a specified feature). Substitutions and removals require explicit human approval. | Deviation type, deviation level |
| **Spec delta check** | A post-build verification step (Phase 9) where a Claude subagent checks that every item marked COVERED in the coverage audit is actually present in the built codebase. APPROVED-DEVIATION items are listed but not re-litigated. | Final spec check, compliance check, coverage verification |
| **Concern boundary** | A prose statement in design.md naming which concerns within a module belong in separate files (e.g., "The JSON-RPC module separates protocol handling, dispatch, action handlers, and tests into distinct files"). Named concerns only — no prescribed filenames. | File boundary, separation rule, module split |
| **Multi-task module** | A module or file referenced by two or more tasks in tasks.md, which requires at least one concern boundary in design.md before the design can be approved. | Shared module, hot module |
| **Concern separation scan** | The orchestrator step in Phase 4.4 that reads the newly generated tasks.md, identifies multi-task modules, and either proposes concern boundaries or blocks tasks.md approval until they are added. | Design review, module scan, boundary check |

## Validation

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Scrutiny validator** | The first validator type that runs build, test, lint, code review, code simplify, and test health checks against the contract | Code reviewer, linter, static validator |
| **User-test validator** | The second validator type that tests behavior by hitting the API (HTTP tests) or driving a browser (agent-browser) | QA, E2E tester, integration tester |
| **Behavioral conformance** | A scrutiny validator check (check 11) that verifies observable behavior by writing and executing a test script rather than reading the code. Falls back to "verified by inspection" if execution is inconclusive. | Behavioral test, execution test, runtime check |
| **Final gates** | Whole-repo build, test, lint, and format checks run after the last task completes but before PR creation. Catches regressions introduced by later tasks (e.g., documentation or CI tasks) after the cleanup task ran. | Final checks, post-build gates, repo gates |
| **Demo video** | A recording of frontend functionality captured after both validators pass, created for human review | Screencast, screen recording |
| **Fix loop** | The cycle where a validation failure spawns a fix worker, then both validators re-run, up to 3 attempts before asking the human | Retry, repair cycle |
| **Circuit breaker** | The rule that stops the fix loop after 3 failed attempts on the same task and asks the human for guidance | Retry limit, fail-safe |

## Test health

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Silent skip** | A test that conditionally returns early without testing anything but reports PASS to the test runner, hiding missing coverage | Conditional skip, soft skip, hidden skip |
| **Diff coverage** | Line coverage measured only on added lines in a task's git diff, not the entire codebase | New code coverage, incremental coverage |
| **Error handling smell** | A language-specific anti-pattern in error handling code (e.g., `.unwrap()` in Rust, `catch {}` in TypeScript, `except: pass` in Python) counted per KLOC of changed code | Code smell, anti-pattern, bad practice |
| **File size pre-pass** | A deterministic line count on touched files injected into the module depth review, ensuring the LLM reviewer cannot overlook large files | Line count check, size gate |
| **Graduated file size threshold** | The two-level file size check in Check 6 of the scrutiny validator: WARNING at 600 lines (module growing toward danger zone), hard FAIL at 1000 lines (unconditional — the validator does not read design.md; the orchestrator reads design.md after the FAIL and may override to PASS if a single-file justification exists) | Size threshold, line limit, file size gate |
| **Test quality review** | An LLM-based semantic review (owned skill: `/test-quality`) that evaluates whether tests verify meaningful behavior, checking for tautological assertions, implementation coupling, untested branches, and missing edge cases | Test review, test audit, test assessment |
| **Tautological assertion** | A test assertion that passes regardless of whether the code works correctly (e.g., `assert!(true)`, `expect(x).toBeDefined()` without checking the value) | Empty assertion, no-op test, fake test |
| **Self-gating check** | A scrutiny check that naturally skips itself when the diff contains no relevant files, requiring no explicit closing-task exemption | Auto-skip, conditional check |

## Execution model

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Serial execution** | The rule that only one worker runs at a time to prevent conflicts | Sequential, single-threaded |
| **Engine** | The configurable AI model/tool used for a given role (e.g. Codex for workers, Claude for validators) | Model, provider, backend |
| **Plan mode** | The Claude Code plan that serves as the orchestrator's durable operating manual, guaranteed to survive context compaction | System prompt, instructions |
| **Proactive compaction** | A `/compact` step the orchestrator runs at phase boundaries to shed intermediate tool output (validation reports, git diffs, fix loop history) while the plan file and progress.md carry forward as compacted state. Fires at two points: planning-to-build transition and build-to-completion transition. | Auto-compact, context cleanup |
| **Smart zone** | The portion of the context window (roughly under 40% utilisation) where LLM decision quality remains high. The orchestrator aims to stay here by compacting proactively. | Safe zone, green zone |
| **Dumb zone** | The portion of the context window (roughly above 40% utilisation) where accumulated noise degrades LLM decision quality. Automatic compaction at 80-95% is too late -- quality has already degraded for several tasks. | Danger zone, red zone |

## File artifacts

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Mission directory** | The `mission/` folder nested inside an OpenSpec change directory that holds all runtime state for a mission | State folder, workspace |
| **Progress log** | The `progress.md` file where the orchestrator records task status, decisions, and issues as the mission runs | Status file, mission log |
| **Mission config** | The `config.yaml` file specifying which engine to use for each role in the mission | Settings, preferences |

## OpenSpec integration

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Change** | An OpenSpec unit of work with its own proposal, design, tasks, and now contract and mission directory | Feature, epic, project |
| **Proposal** | The OpenSpec artifact describing what is being built and why | PRD, requirements doc |
| **Design** | The OpenSpec artifact describing how it will be built (architecture decisions) | Architecture doc, technical spec |
| **Tasks** | The OpenSpec artifact listing tasks as checkboxes, each representing one worker's worth of work (should follow the vertical slice best practice) | Checklist, todo list, backlog |

## Dispatch

| Term | Definition | Aliases to avoid |
| --- | --- | --- |
| **Dispatcher** | The `darkfactory-dispatch` Rust binary that sits between the orchestrator and a Codex worker as a process supervisor | Wrapper, harness, runner, proxy |
| **Dispatch** | The act of spawning a supervised Codex subprocess through the dispatcher, including process group setup, timeout management, and structured result reporting | Launch, invoke, run |
| **Heartbeat** | A periodic NDJSON event emitted by the dispatcher to stderr, carrying elapsed time and last activity, used to distinguish "working" from "dead" | Ping, keepalive, pulse |
| **Two-phase timeout** | The dispatcher's timeout strategy: a soft timeout sends SIGTERM and arms a grace period (default: half the timeout, minimum 1 second), then a hard timeout sends SIGKILL | Timeout, deadline, kill timer |
| **Process group** | An OS-level grouping (via `setpgid`) that ensures killing the Codex process also kills any grandchild processes it spawned | Process tree, child processes |
| **Dispatch store** | A minimal filesystem directory at `~/.darkfactory/dispatches/<ulid>/` holding `status.json`, `result.json`, `host.pid`, and `events.jsonl` for async dispatch lifecycle management | State directory, result store |
| **Dispatch result** | A structured JSON object emitted on stdout when a dispatch completes, containing status, exit code, signal, duration, output path, token usage, and activity | Result, outcome, response |

## Relationships

- A **Mission** belongs to exactly one **Change**
- A **Change** contains one **Proposal**, one **Design**, one **Tasks** list, one **Contract**, and one **Mission directory**
- Each **Task** maps to one **Worker** invocation, one git commit, and one validation cycle
- A **Contract** contains **Assertions** grouped by **Task**
- The **Orchestrator** writes **Briefs** and reads **Handoffs**
- The **Orchestrator** spawns Codex **Workers** through the **Dispatcher**, and Claude **Workers** and **Validators** through the Agent tool
- The **Dispatcher** supervises the Codex process: it manages the **Process group**, emits **Heartbeats**, enforces the **Two-phase timeout**, and returns a **Dispatch result**
- **Workers** read **Briefs** and write **Handoffs** -- they never see the **Contract**
- **Validators** read the **Contract** and the commit diff -- they never see the **Proposal** or **Design**
- A **Fix loop** produces versioned **Handoffs** (task-N-fix-v2.md) and re-runs both **Validators**
- The **Circuit breaker** halts the **Fix loop** and returns control to the human
- The **Dispatch store** is independent of the **Mission directory** -- it holds dispatcher metadata, not mission state
- The **Scrutiny validator** runs deterministic checks (build, lint, **Silent skip** detection, **Diff coverage**, **Error handling smell** density, **File size pre-pass**) and LLM checks (code review, module depth, **Test quality review**)
- **Diff coverage** measures added lines only -- a worker is responsible for covering the code they wrote, not backfilling pre-existing debt
- The **Test quality review** (`/test-quality`) is an owned skill invoked by the **Scrutiny validator** as a subagent, following the same pattern as `/review` and `/simplify`
- A **Tautological assertion** is worse than no test because it gives false confidence -- the **Scrutiny validator** treats it as a hard FAIL
- A **Multi-task module** requires at least one **Concern boundary** in design.md; the **Concern separation scan** enforces this as a blocking gate during Phase 4.4
- The **Graduated file size threshold** is the runtime enforcement of **Concern boundaries**: a WARNING at 600 lines prompts verification; a FAIL at 1000 lines is unconditional — the orchestrator then checks design.md for a single-file justification and may override to PASS

## Example dialogue

> **Dev:** "I want to add real-time notifications to Toyo. Should I start a **Mission**?"
>
> **Orchestrator:** "That depends on the scope. How many **Tasks** do you think it involves?"
>
> **Dev:** "At least three -- WebSocket setup, notification delivery, and the UI. Each is a **Vertical slice** cutting through the API and frontend."
>
> **Orchestrator:** "That's a good fit. Let me **/grill** you on the requirements first, then I'll create the **Change** via **/opsx:propose** and write the **Contract** before any **Workers** start."
>
> **Dev:** "What if the Codex **Worker** hangs during the WebSocket task?"
>
> **Orchestrator:** "The **Dispatcher** handles that. It emits **Heartbeats** every 15 seconds so I can tell if the worker is still active. If no progress happens, the **Two-phase timeout** fires -- first SIGTERM, then after the grace period SIGKILL to the entire **Process group**. I get a **Dispatch result** with status `timed_out` and fall back to a Claude **Worker**."
>
> **Dev:** "And I can check on progress while it runs?"
>
> **Orchestrator:** "Yes -- tail the **Dispatcher's** log for **Heartbeat** events, read the **Progress log** directly, or run **/mission-status** in another session for a formatted summary."

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

## Example dialogue (concern separation)

> **Dev:** "Three tasks all touch the JSON-RPC module. Will that cause problems?"
>
> **Orchestrator:** "It makes it a **Multi-task module**. Before I let you approve tasks.md, I'll run the **Concern separation scan** and require at least one **Concern boundary** for it -- something like 'The JSON-RPC module separates protocol handling, dispatch, action handlers, and tests into distinct files.'"
>
> **Dev:** "What if a worker ignores that and puts everything in one file anyway?"
>
> **Orchestrator:** "The **Graduated file size threshold** catches it. At 600 lines the **Scrutiny validator** emits a WARNING -- module growing toward the danger zone. At 1000 lines it's an unconditional FAIL. After that fires, I check design.md: if there's a single-file justification I override to PASS; otherwise it goes to the fix loop."
>
> **Dev:** "Can I intentionally keep it as one file?"
>
> **Orchestrator:** "Yes -- write a one-line justification in design.md: 'intentionally one file because X.' The validator accepts that as a PASS."

## Resolved ambiguities

- **"task"** was used early in the conversation to mean both a micro-step within a feature and a vertical slice of work. **Resolved:** keep "task" as the standard term. The guidance is that each task should follow the **Vertical slice** best practice (thin, end-to-end, demoable), but the term "task" is unchanged from OpenSpec. "Slice" is not a standalone noun -- it's a quality standard for tasks.
- **"spec"** was used to mean both SDD spec files and the planning phase of the mission flow. **Resolved:** SDD specs are not part of Darkfactory's vocabulary. In Darkfactory, **Spec** refers exclusively to the planning phase (`align -> spec -> contract -> build -> validate -> ship`) where the orchestrator creates OpenSpec artifacts and the contract. SDD is a separate workflow for documenting existing code after the fact.
- **"validation"** was used for both the contract assertions and the act of running validators. **Resolved:** the **Contract** contains **Assertions**. **Validation** is the act of a **Validator** checking those assertions against the code. Be consistent: don't use "validation" to refer to the contract itself.
- **"coverage"** could mean line coverage (percentage of lines executed during tests) or spec coverage (percentage of PRD requirements mapped to tasks). **Resolved:** **Diff coverage** means line coverage on added lines in a git diff. **Coverage audit** means the spec-to-task mapping in Phase 4a. Don't use "coverage" without the qualifier.
- **"check"** could mean a scrutiny validator step or a contract assertion. **Resolved:** a **Check** is a step in the scrutiny validator's sequence (e.g., check 3: tests, check 12: silent skip detection). An **Assertion** is a statement in the contract. Don't use "check" to refer to contract assertions.
