# Architecture

Living architecture document. Update when you change architecture.

## Three-role agent model

Darkfactory uses three agent roles, each running as a separate AI conversation:

**Orchestrator** -- the main Claude Code session. Plans the work, dispatches one worker at a time, runs validators, manages fix loops and circuit breakers. Uses plan mode to keep instructions alive across long conversations. Defined by `skills/mission/SKILL.md` (10 phases, 0-9). In basic mode (`--basic`), the orchestrator also acts as the builder -- no workers are dispatched. Defined by `skills/mission/basic/SKILL.md`.

**Workers** -- each builds a single task from a brief. Default engine is Codex (via `darkfactory-dispatch`); Claude is an alternative. A worker receives only its brief, never the contract. Makes changes in one commit and writes a handoff report.

**Validators** -- check each worker's output. Two run back-to-back per task: scrutiny (build, test, lint, code review, contract check, execution check, test health) and user-test (tests the feature as a user would). Validators see the contract and the code, never the proposal or design.

## Information walls

Workers never see the contract. Validators never see the design.

This forces validation to test what the code *does*, not what someone *intended*. Fix workers receive symptom descriptions, never the contract, so they solve real problems instead of gaming a checklist.

## File-backed state

All state lives on disk, not in memory. Sessions can crash and resume. Progress is visible from a second terminal.

```
openspec/changes/<change-name>/
  proposal.md              # What and why
  specs/<cap>/spec.md      # WHEN/THEN requirements
  design.md                # Architecture decisions, module inventory, type catalog
  tasks.md                 # Ordered checklist with topology metadata
  coverage-audit.md        # Proposal-to-task traceability
  contract.md              # Observable behavior checks, traced to specs
  mission/
    config.yaml            # Engine per role (codex or claude)
    progress.md            # Running log
    briefs/task-N.md       # What each worker receives
    handoffs/task-N.md     # What each worker reports
    validations/           # Validator reports
    issues/                # Failure details per attempt
    logs/                  # Streaming output (tail -f)
```

## Skill structure

Skills live in `skills/` at the repo root. `install.sh` symlinks each into `~/.claude/skills/` for Claude Code discovery.

Each skill is a self-contained module. Public interface: frontmatter (name, description). Implementation: SKILL.md body. No skill imports from another at the filesystem level.

The mission skill is intentionally monolithic (~900 lines). Companion files are read on demand:
- `templates/` -- artifact formats (plan, contract, worker brief, design, etc.)
- `prompts/` -- instructions handed to validator and reviewer subagents

23 skills total: mission (with basic sub-skill), mission-resume, mission-status, mission-init, grill, language, prd, tdd, test-quality, security, adversarial, dispatch, artifact-review, lint-audit, skill-review, arch-review, domain-model, module-design, qa, write-a-skill, frontend, ui-system, react-perf.

lint-audit audits a TypeScript project's oxlint config and tsconfig strictness against an agent-friendly ruleset. Part of the mechanical enforcement stack alongside the ts-quality.sh PostToolUse hook: the hook catches patterns at edit time (grep-based), the linter catches them at build time (AST-level via oxlint), and the compiler catches them at type-check time (tsconfig strictness). Gate 4 of the scrutiny validator runs the project's linter, so rules added by lint-audit are automatically enforced during missions.

## Build loop

For each task: dispatch worker, run scrutiny validator, run user-test validator. On failure: diagnostic triage, then fix worker with symptoms only (information wall maintained). Circuit breaker after 3 attempts (1 build + 2 fixes) -- pauses for human decision.

After task 1 passes, a foundation audit checks module structure and type names against the design.

### Basic mode build loop

`/mission --basic` is a simplified version of the full build loop designed for single-task work like bug fixes and small features. Instead of dispatching separate worker and validator agents, the orchestrator handles everything itself in one session.

The flow has 5 phases: orient (read the codebase, detect build/test tooling, generate a mini-contract of 3-8 expected behaviors), build (TDD by default, or direct implementation with `--no-tdd`), validate (5 sequential gates: build, test, lint, format, contract), fix (diagnose and retry on failure), and commit (single commit at the end).

The circuit breaker works the same way -- 3 total attempts before stopping -- but unlike the full loop, it doesn't pause to ask the user. It commits whatever it has and reports what failed. There are no information walls since there's only one agent doing all the work.

Defined by `skills/mission/basic/SKILL.md`. The main `skills/mission/SKILL.md` routes to it when `--basic` is present in the arguments.

## Planning constraints

Design phase scores 2-3 architecture candidates on a four-dimension rubric (separation of concerns, testability, task independence, module depth). A mandatory gate verifies the design includes an architecture decision, module inventory, type catalog, error strategy, integration points, and test strategy.

Before task generation, the orchestrator presents a decomposition strategy (layered, vertical, or hybrid) for human approval. Each task carries topology metadata: depends-on, consumes, produces, layers.

## Configuration

Global defaults in `~/.darkfactory/config.yaml`. Per-mission engine assignments in `mission/config.yaml` (worker, scrutiny_validator, usertest_validator, adversarial reviewers).

## Versioning

One version for the whole repo: the GitHub release tag (v1.x). Internal tools (darkfactory-dispatch) have their own Cargo.toml version pinned at 1.0.0, which is never bumped. When releasing, increment the repo tag only.

## External dependencies

- Claude Code (orchestrator, Claude workers, validators)
- Codex CLI (Codex workers, adversarial reviewer)
- darkfactory-dispatch (Rust supervisor: process isolation, timeout, heartbeat, structured results) -- pinned at v1.0.0
- OpenSpec CLI (planning artifact generation)
- GitHub CLI (draft PRs)
