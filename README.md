# Darkfactory

A lightweight multi-agent mission system built entirely from Claude Code primitives. No custom IDE, no app, no framework. Just skills, subagents, plan mode, and markdown files.

Inspired by [Factory's Missions architecture](https://www.factory.ai/) and [Matt Pocock's AI development skills](https://github.com/mattpocock/skills).

## What it does

Darkfactory turns a goal into working, validated code -- autonomously. You describe what you want to build, the system plans it, dispatches workers to implement it, validates the results against a contract, and delivers a draft PR.

A mission has three stages:

1. **You and the orchestrator plan together.** The orchestrator grills you on what you want, establishes shared terminology, then creates OpenSpec artifacts sequentially (proposal, specs, design, tasks) -- reviewing each with you before proceeding. It runs a spec coverage audit against the PRD to catch missed requirements, classifying any deviations from the spec (substitutions and removals require your explicit approval). It writes a PRD-traced contract where every assertion is tagged with a verification method (execution, output, or inspection) and runs a testability pre-flight to catch vague assertions before they waste fix cycles. You review and confirm launch.

2. **The orchestrator builds and validates autonomously.** For each task, it dispatches a worker (Codex or Claude, configurable), then runs two layers of validation: scrutiny (test presence gate, build/test/lint, code review, code standards, simplify, security review, behavioral conformance) and user-testing (API and browser tests). If anything fails, a fix worker addresses the issues and both validators re-run. This repeats until the task passes or a circuit breaker asks for your input.

3. **You review and merge.** After all tasks complete, the orchestrator runs final gates (whole-repo build/test/lint/format) and a spec delta check (verifying the built code matches what the coverage audit promised). It then creates a draft PR and runs a dual adversarial review (Codex + Claude in parallel) against the full diff. Both reports are presented to you alongside the PR link. You decide what to address, promote the PR to ready, and merge.

## Architecture

```
                                You
                                 │
                          /mission skill
                                 │
                           Orchestrator
                                 │
                                 ▼
    ┌────────────────────────────────────────────────────┐
    │  Spec phase                                        │
    │                                                    │
    │  Grill you for alignment, establish shared         │
    │  language, write a PRD, then generate and          │
    │  review each artifact: proposal, specs,            │
    │  design, tasks. Run a coverage audit that          │
    │  classifies any deviations from the spec.          │
    │  Write a validation contract.                      │
    │                                                    │
    │  ◄── you review and approve each step              │
    └──────────────────────────┬─────────────────────────┘
                               ▼
    ┌────────────────────────────────────────────────────┐
    │  Build phase                                       │
    │                                                    │
    │  For each task: dispatch a worker (Codex or        │
    │  Claude), then run scrutiny validation (build,     │
    │  test, lint, code review, contract assertions,     │
    │  behavioral conformance) and user-test             │
    │  validation (HTTP and browser checks). On          │
    │  failure, a fix worker addresses the issues        │
    │  and both validators re-run.                       │
    │                                                    │
    │  Autonomous until circuit breaker (3 attempts)     │
    └──────────────────────────┬─────────────────────────┘
                               ▼
    ┌────────────────────────────────────────────────────┐
    │  Completion                                        │
    │                                                    │
    │  Re-run build, test, lint, and format gates        │
    │  on the full repo. Verify every covered item       │
    │  from the spec audit is actually in the code.      │
    │  Create a draft PR. Run dual adversarial           │
    │  review (Codex + Claude in parallel).              │
    │                                                    │
    │  ◄── you review findings and merge                 │
    └────────────────────────────────────────────────────┘
```

### Three roles

**Orchestrator** -- the main Claude Code session. Plans the work, dispatches workers one at a time, reads results, decides what to do next. Runs in plan mode so its operating manual survives context compaction.

**Workers** -- implement one task each. Default engine: Codex (GPT-5.4). Get a self-contained brief with the task description, project context, procedures, and a codebase delta showing what previous tasks built (file paths, function signatures, types -- facts only, never reasoning from other workers). Commit in a single commit. Write a structured handoff report. Never see the contract.

**Validators** -- check each task against the contract. Two types run sequentially per task:
- **Scrutiny validator** (Claude) -- test presence gate, build, tests, lint, code review, code standards, simplify, security review (conditional), contract assertions, behavioral conformance (attempts to verify observable behavior by execution, falls back to inspection)
- **User-test validator** (Claude) -- hits the API with HTTP tests and/or drives a browser with agent-browser

After all tasks complete, **final gates** re-run whole-repo build, test, lint, and format checks (catching regressions from late tasks), then a **spec delta check** verifies every COVERED item in the coverage audit is actually present in the built code. Finally, a **dual adversarial review** runs once against the full diff. Both Codex and Claude independently review the code and their reports are presented to the human. No automated fix loop -- the human decides what to address.

### Adversarial separation

Workers never see the contract. Validators never see the proposal or design. This is by design -- it prevents the validator from rubber-stamping code that "looks like it should work" and forces it to check what actually works.

### Fix loop

When validation fails, a diagnostic triage agent reads the validator report and the diff to identify probable root causes (file, line, what the code does wrong). The orchestrator then writes a fix brief containing the symptoms and triage (not contract assertions -- adversarial separation is maintained) and dispatches a fix worker. After the fix, both per-task validators (scrutiny + user-test) re-run. Circuit breaker after 3 attempts -- asks the human to retry, skip, or abort.

### File-backed state

All mission state lives on disk:

```
openspec/changes/<change-name>/
  proposal.md              # What and why
  specs/<capability>/spec.md  # Formal WHEN/THEN requirements (one per capability)
  design.md                # How (architecture decisions)
  tasks.md                 # Task checklist (includes mandatory closing tasks)
  coverage-audit.md        # PRD requirement-to-task mapping
  contract.md              # Validation assertions (traced to PRD, tagged by verification method)
  mission/
    config.yaml            # Engine settings per role
    progress.md            # Orchestrator's running log
    briefs/task-N.md       # Worker briefs
    handoffs/task-N.md     # Worker handoff reports
    validations/           # Validator reports, demo videos
    issues/                # Failure details per attempt
```

This means missions survive session death (resume from file state) and can be monitored from a separate terminal.

## Skills

| Skill | Purpose |
|-------|---------|
| `/mission` | Start an autonomous mission. Grills you, plans, dispatches workers, validates, creates draft PR. |
| `/mission-resume` | Resume an interrupted mission from file state. Detects where it left off. |
| `/mission-status` | Read-only progress check. Run from another session while a mission runs. |
| `/darkfactory-init` | Check project setup. Reports what's installed and what's missing. |

### Required external skills

These skills are referenced by `/mission` but are **not bundled** in this repo. You need to install them separately.

| Skill | Source | Used by |
|-------|--------|---------|
| `/grill-me` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/grill-me) | Spec phase: alignment interview |
| `/ubiquitous-language` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/ubiquitous-language) | Spec phase: shared glossary |
| `/write-a-prd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/write-a-prd) | Spec phase: structured PRD from grill conversation |
| `/tdd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/tdd) | Build phase: test-driven development for workers |
| `/review` | Built-in Claude Code (required) | Scrutiny validator: code review |
| `/simplify` | Built-in Claude Code (required) | Scrutiny validator: complexity check |
| `/security-review` | Your global Claude skills (conditional) | Scrutiny validator: security audit (runs when diff touches auth/secrets/input handling) |
| `/test-app` | Your project's Claude skills (optional) | User-test validator: browser tests |
| `/demo-video` | Your project's Claude skills (optional) | Demo recording for frontend tasks |
| `openspec` CLI | Installed by `npm install -g openspec` | Spec phase: create artifacts sequentially |
| `/opsx:archive` | Installed by `openspec init` | Post-merge: archive the change |

`/review` and `/simplify` are mandatory -- the scrutiny validator always runs them. `/security-review` is conditional (runs when the diff touches auth, secrets, or input handling). `/test-app` and `/demo-video` are optional and project-specific.

## Setup

### Prerequisites

- [Claude Code](https://docs.anthropic.com/en/docs/claude-code) installed and authenticated
- [OpenSpec CLI](https://github.com/Fission-AI/OpenSpec) installed (`npm install -g openspec`)
- [Codex CLI](https://github.com/openai/codex) installed (`npm install -g @openai/codex`) and authenticated (`codex login`)
- [GitHub CLI](https://cli.github.com/) installed and authenticated (`gh auth login`) -- used for draft PR creation
- Git repository initialized

### First-time install

Clone the darkfactory repo and symlink the skills globally:

```bash
git clone https://github.com/layercodedev/darkfactory.git ~/.darkfactory
ln -sf ~/.darkfactory/.claude/skills/mission ~/.claude/skills/mission
ln -sf ~/.darkfactory/.claude/skills/mission-resume ~/.claude/skills/mission-resume
ln -sf ~/.darkfactory/.claude/skills/mission-status ~/.claude/skills/mission-status
ln -sf ~/.darkfactory/.claude/skills/darkfactory-init ~/.claude/skills/darkfactory-init
```

Restart Claude Code, then from any project run:

```
/darkfactory-init
```

It checks what's installed and tells you exactly what's missing. It will:
1. Check core and supporting skills are installed globally
2. Tell you the exact commands to install anything missing
3. Initialize OpenSpec in the current project if needed
4. Verify CLI dependencies (codex, gh, openspec)
5. Report what's ready

### Updating

Since skills are symlinked, just pull:

```bash
cd ~/.darkfactory && git pull
```

### Configuration

Each mission creates its own `config.yaml` with engine defaults. You can change these before approving the plan:

```yaml
engines:
  worker: codex                    # or claude
  scrutiny_validator: claude
  usertest_validator: claude
  adversarial_reviewer_a: codex
  adversarial_reviewer_b: claude
  demo_video: claude
```

## Usage

### Start a mission

```
/mission Build a user authentication system with email/password login, session management, and a profile page
```

The orchestrator will:
1. Enter plan mode (durable operating manual)
2. Grill you until aligned (`/grill-me`)
3. Create a shared language glossary (`/ubiquitous-language`)
4. Write a structured PRD (`/write-a-prd`)
5. Create OpenSpec artifacts one at a time, each reviewed by you: proposal, specs, design, tasks
6. Run a spec coverage audit (maps every PRD requirement to a task, flags gaps)
7. Write a PRD-traced contract (you review)
8. Create the mission directory
9. Confirm launch

After you approve, it runs the build loop autonomously.

### Check progress

From another terminal while a mission runs:

```
/mission-status
```

### Resume after interruption

If your session dies:

```
/mission-resume
```

(This is a separate skill, not a flag on `/mission`.)

### When it's done

The orchestrator creates a draft PR. Review it, promote to ready, merge, then:

```
/opsx:archive
```

## When to use a mission

Use `/mission` for everything. The scope adapts naturally -- a small bug fix produces a short grill, one task, and full validation. A large feature produces a long grill, many tasks, and the same validation per task. One workflow regardless of size.

| Scope | What happens |
|-------|-------------|
| Quick bug fix | Short grill, 1 task, full validation, draft PR |
| Small feature | Medium grill, 1-2 tasks, full validation, draft PR |
| Multi-task feature | Long grill, many tasks, full validation per task, draft PR |
| Large refactor | Long grill (40+ questions), many tasks, full validation, draft PR |
| Overnight build | Same as above, run in tmux |

## How it was built

Darkfactory was built using its own principles. The v1 skills were hand-written during a grill-me session that produced the design decisions and ubiquitous language. Then Darkfactory built itself (v2) -- Claude workers generated all three skills independently from the specs, a Codex adversarial review found 7 issues, and a fix worker addressed them. The v2 fixed version is what ships.

The full build history is in `archive/` (ignored by Claude via `.claudeignore`):
- `archive/build-specs/` -- the 6 OpenSpec changes that designed darkfactory
- `archive/v1-skills/` -- the original hand-written skills
- `archive/self-build/` -- the v2 mission (briefs, handoffs, validation reports, adversarial review)

## Key references

- [Factory Missions talk](docs/reference_factory_talk.md) -- the multi-agent architecture that inspired this
- [Matt Pocock skills talk](docs/reference_pocock_talk.md) -- design alignment, ubiquitous language, TDD patterns
- [Provenance](docs/provenance.md) -- how Darkfactory relates to both sources (what's faithful, what's different, what's new)
- [Design decisions](docs/design-decisions.md) -- every architectural choice with rationale
- [Ubiquitous language](UBIQUITOUS_LANGUAGE.md) -- shared terminology (25 terms)

## License

MIT
