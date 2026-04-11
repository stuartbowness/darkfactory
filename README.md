# Darkfactory

A lightweight multi-agent mission system built entirely from Claude Code primitives. No custom IDE, no app, no framework. Just skills, subagents, plan mode, and markdown files.

Inspired by [Factory's Missions architecture](https://www.factory.ai/) and [Matt Pocock's AI development skills](https://github.com/mattpocock/skills).

## What it does

Darkfactory turns a goal into working, validated code -- autonomously. You describe what you want to build, the system plans it, dispatches workers to implement it, validates the results against a contract, and delivers a draft PR.

A mission has three stages:

1. **You and the orchestrator plan together.** The orchestrator grills you on what you want, establishes shared terminology, creates a proposal/design/tasks via OpenSpec, writes a validation contract, and presents everything for your approval.

2. **The orchestrator builds and validates autonomously.** For each task, it dispatches a worker (Codex or Claude, configurable), then runs three layers of validation: scrutiny (build/test/lint/code review), user-testing (API and browser tests), and dual adversarial review (Codex + Claude). If anything fails, a fix worker addresses the issues and all validators re-run. This repeats until the task passes or a circuit breaker asks for your input.

3. **You review and merge.** The orchestrator creates a draft PR with a mission summary. You review it, promote to ready, and merge.

## Architecture

```
                          You
                           |
                    /mission skill
                           |
                     Orchestrator
                  (Claude Code session
                    in plan mode)
                           |
              +------------+------------+
              |                         |
         Spec Phase                Build Phase
              |                         |
     /grill-me (align)          For each task:
     /ubiquitous-language           |
     /write-a-prd (PRD)        1. Assemble brief
     /opsx:propose
     Write contract            2. Dispatch worker -----> Codex (or Claude)
     Create mission dir        3. Scrutiny validator --> Claude subagent
              |                4. User-test validator -> Claude subagent
     Human approves plan       5. Adversarial review --> Codex + Claude
              |                6. Demo video (frontend)
              +--------->      7. Mark complete
                                    |
                               All tasks done
                                    |
                           Draft PR via gh CLI
```

### Three roles

**Orchestrator** -- the main Claude Code session. Plans the work, dispatches workers one at a time, reads results, decides what to do next. Runs in plan mode so its operating manual survives context compaction.

**Workers** -- implement one task each. Default engine: Codex (GPT-5.4). Get a self-contained brief with the task description, project context, and procedures. Commit in a single commit. Write a structured handoff report. Never see the validation contract.

**Validators** -- check each task against the contract. Two types run sequentially:
- **Scrutiny validator** (Claude) -- build, test, lint, code review, simplify, security review, contract assertions
- **User-test validator** (Claude) -- hits the API with HTTP tests and/or drives a browser with agent-browser

Plus a **dual adversarial review** where both Codex and Claude independently review the code. The orchestrator synthesizes their findings. Different models catch different things.

### Adversarial separation

Workers never see the contract. Validators never see the proposal or design. This is by design -- it prevents the validator from rubber-stamping code that "looks like it should work" and forces it to check what actually works.

### Fix loop

When validation fails, the orchestrator writes a fix brief (what failed, validator output, commit SHA) and dispatches a fix worker. After the fix, ALL validators re-run. Circuit breaker after 3 attempts -- asks the human to retry, skip, or abort.

### File-backed state

All mission state lives on disk:

```
openspec/changes/<change-name>/
  proposal.md              # What and why
  design.md                # How (architecture decisions)
  tasks.md                 # Task checklist (vertical slices)
  contract.md              # Validation assertions
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

### Required external skills

These skills are referenced by `/mission` but are **not bundled** in this repo. You need to install them separately.

| Skill | Source | Used by |
|-------|--------|---------|
| `/grill-me` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/grill-me) | Spec phase: alignment interview |
| `/ubiquitous-language` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/ubiquitous-language) | Spec phase: shared glossary |
| `/write-a-prd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/write-a-prd) | Spec phase: structured PRD from grill conversation |
| `/prd-to-plan` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/prd-to-plan) | Spec phase: vertical slice pattern |
| `/tdd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/tdd) | Build phase: test-driven development for workers |
| `/review` | Your project's Claude skills | Scrutiny validator: code review |
| `/simplify` | Built-in (ships with Claude Code) | Scrutiny validator: complexity check |
| `/security-review` | Your global Claude skills | Scrutiny validator: security audit |
| `/test-app` | Your project's Claude skills | User-test validator: browser tests |
| `/demo-video` | Your project's Claude skills | Demo recording for frontend tasks |
| `/opsx:propose` | Installed by `openspec init` | Spec phase: create artifacts |
| `/opsx:archive` | Installed by `openspec init` | Post-merge: archive the change |

Skills marked "Your project's Claude skills" are project-specific. If your project doesn't have them, the validator will skip those checks. The mission will still run -- these are best-effort enhancements, not hard blockers.

## Setup

### Prerequisites

- [Claude Code](https://docs.anthropic.com/en/docs/claude-code) installed and authenticated
- [OpenSpec CLI](https://github.com/Fission-AI/OpenSpec) installed (`npm install -g openspec`)
- [Codex CLI](https://github.com/openai/codex) installed (`npm install -g @openai/codex`) and authenticated (`codex login`)
- [GitHub CLI](https://cli.github.com/) installed and authenticated (`gh auth login`) -- used for draft PR creation
- Git repository initialized

### Install

From the darkfactory repo directory, run:

```
/darkfactory-init
```

This will:
1. Install `/mission`, `/mission-resume`, `/mission-status` to `~/.claude/skills/` (global, works in any project)
2. Check for and install supporting skills (`grill-me`, `ubiquitous-language`, `write-a-prd`, `prd-to-plan`, `tdd`) from [layercodedev/skills](https://github.com/layercodedev/skills) (pinned fork of mattpocock/skills)
3. Initialize OpenSpec in the target project if needed
4. Verify CLI dependencies (codex, gh, openspec) are installed and authenticated
5. Report what's ready and what needs manual action

After install, restart your Claude Code session so the new skills register.

### Manual install

If you prefer to set up manually:

1. Copy skills globally:
```bash
cp -r .claude/skills/mission ~/.claude/skills/
cp -r .claude/skills/mission-resume ~/.claude/skills/
cp -r .claude/skills/mission-status ~/.claude/skills/
```

2. Install supporting skills from [layercodedev/skills](https://github.com/layercodedev/skills) (pinned fork of mattpocock/skills):
```bash
git clone https://github.com/layercodedev/skills.git /tmp/layercode-skills
cd /tmp/layercode-skills && git checkout 651eab033bdf8f7fd535c274f8cbe839075aba5e
cp -r /tmp/layercode-skills/grill-me ~/.claude/skills/
cp -r /tmp/layercode-skills/ubiquitous-language ~/.claude/skills/
cp -r /tmp/layercode-skills/write-a-prd ~/.claude/skills/
cp -r /tmp/layercode-skills/prd-to-plan ~/.claude/skills/
cp -r /tmp/layercode-skills/tdd ~/.claude/skills/
```

3. Initialize OpenSpec in your project:
```bash
cd /path/to/your/project
openspec init --tools claude,codex
```

4. Verify dependencies:
```bash
codex login          # Ensure Codex is authenticated
gh auth status       # Ensure GitHub CLI is authenticated
openspec --version   # Ensure OpenSpec is installed
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
5. Create OpenSpec artifacts from the PRD (`/opsx:propose`)
6. Write a validation contract
7. Create the mission directory
8. Present the plan for your approval

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
| Quick bug fix | Short grill (3-5 questions), 1 task, full validation, draft PR |
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
- [Design decisions](docs/design-decisions.md) -- every architectural choice with rationale
- [Ubiquitous language](UBIQUITOUS_LANGUAGE.md) -- shared terminology (22 terms)

## License

MIT
