# Darkfactory

A lightweight multi-agent mission system built entirely from Claude Code primitives. No custom IDE, no app, no framework. Just skills, subagents, plan mode, and markdown files.

Inspired by [Factory's Missions architecture](https://www.factory.ai/) and [Matt Pocock's AI development skills](https://github.com/mattpocock/skills).

## What it does

Darkfactory turns a goal into working, validated code -- autonomously. You describe what you want to build, the system plans it, dispatches workers to implement it, validates the results against a contract, and delivers a draft PR.

```
idea -> spec -> build -> verify -> ship -> merge
```

The inner loop runs per task: a worker builds a vertical slice, validators check it against the contract, and if it fails, a fix worker addresses the issues. This repeats until the task passes or a circuit breaker asks for human input.

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
     /opsx:propose             1. Assemble brief
     Write contract            2. Dispatch worker -----> Codex (or Claude)
     Create mission dir        3. Scrutiny validator --> Claude subagent
              |                4. User-test validator -> Claude subagent
     Human approves plan       5. Adversarial review --> Codex + Claude
              |                6. Demo video (frontend)
              +--------->      7. Mark complete
                                    |
                               All tasks done
                                    |
                              Draft PR via /pr
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
| `/prd-to-plan` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/prd-to-plan) | Spec phase: vertical slice pattern |
| `/review` | Your project's Claude skills | Scrutiny validator: code review |
| `/code-simplify` | Your project's Claude skills | Scrutiny validator: complexity check |
| `/security-review` | Your project's Claude skills | Scrutiny validator: security audit |
| `/test-app` | Your project's Claude skills | User-test validator: browser tests |
| `/demo-video` | Your project's Claude skills | Demo recording for frontend tasks |
| `/pr` | Your project's Claude skills | Completion: draft PR creation |
| `/opsx:propose` | Installed by `openspec init` | Spec phase: create artifacts |
| `/opsx:apply` | Installed by `openspec init` | Manual implementation (non-mission) |
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
2. Check for and install supporting skills (`grill-me`, `ubiquitous-language`, `prd-to-plan`) from [layercodedev/skills](https://github.com/layercodedev/skills) (pinned fork of mattpocock/skills)
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
git clone --depth 1 https://github.com/layercodedev/skills.git /tmp/layercode-skills
cd /tmp/layercode-skills && git checkout 651eab033bdf8f7fd535c274f8cbe839075aba5e
cp -r /tmp/layercode-skills/grill-me ~/.claude/skills/
cp -r /tmp/layercode-skills/ubiquitous-language ~/.claude/skills/
cp -r /tmp/layercode-skills/prd-to-plan ~/.claude/skills/
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
2. Grill you with 20-100 questions until aligned
3. Create a shared language glossary
4. Create OpenSpec artifacts (proposal, design, tasks)
5. Write a validation contract
6. Create the mission directory
7. Present the plan for your approval

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

## When to use a mission vs regular development

| Scope | Approach |
|-------|----------|
| Quick bug fix | Just fix it in Claude Code |
| Small feature | `/opsx:propose` then `/opsx:apply` |
| Multi-task feature | `/mission` |
| Large refactor | `/mission` |
| Overnight build | `/mission` in tmux |

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
