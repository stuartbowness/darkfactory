## Context

Darkfactory's architecture is fully designed (see docs/design-decisions.md) but nothing is built yet. This change implements the first executable piece: the orchestrator's planning phase. The orchestrator is a Claude Code session that guides a user from a goal description to approved planning artifacts and a validation contract.

The orchestrator does NOT implement the worker loop or validation in this change. It only handles: idea -> spec. The build -> verify -> ship -> merge phases come in later changes.

All building blocks already exist as installed skills: `/grill-me`, `/ubiquitous-language`, `/opsx:propose`. This change composes them into a single `/mission` skill and adds the new artifacts (contract.md, mission directory).

## Goals / Non-Goals

**Goals:**
- Create the `/mission` skill as the entry point for starting a mission
- Enter Claude Code plan mode with a durable orchestrator operating manual
- Run the spec phase: grill -> ubiquitous language -> propose -> contract -> approval
- Create the mission directory structure with config.yaml and progress.md
- Produce all artifacts needed for the worker loop to pick up later

**Non-Goals:**
- Worker invocation (change: worker-loop)
- Validation (changes: scrutiny-validation, usertest-validation)
- Demo video recording (change: completion-phase)
- Draft PR creation (change: completion-phase)
- Mission resume / mission status (change: resume-and-status)
- tmux integration

## Decisions

### The `/mission` skill is a single SKILL.md file
The skill is a prompt, not code. It instructs Claude on the sequence of steps to follow. This matches Factory's approach where "almost all orchestration logic is defined in prompts and skills instead of hard-coded logic." When models improve, the skill just works better.

**Alternative considered:** A bash script that orchestrates Claude Code sessions. Rejected because the orchestrator needs to have an interactive conversation with the user during the grill phase, which doesn't work well from a script.

### Plan mode is entered immediately on `/mission` invocation
The Claude Code plan is the durable operating manual. It must be written first, before any other work, because it's the one piece of context guaranteed to survive compaction. The plan template is embedded in the skill's instructions.

**Alternative considered:** Writing the plan after the grill phase. Rejected because the plan needs to exist before compaction could happen, and a long grill session could trigger compaction.

### The contract.md is a new file, not an OpenSpec artifact type
We write contract.md directly into the change directory alongside proposal.md, design.md, and tasks.md. We don't modify OpenSpec's schema or CLI to support it. It's just a markdown file that the orchestrator creates after `/opsx:propose` finishes.

**Alternative considered:** Extending OpenSpec with a custom artifact type. Rejected for now -- adds complexity to OpenSpec config for something that's just a markdown file. Can revisit if the contract needs tooling support later.

### The skill invokes existing skills by instructing Claude to run them
The `/mission` skill doesn't call `/grill-me` programmatically. It instructs the orchestrator (Claude) to follow a sequence: "first, grill the user... then, run /ubiquitous-language... then, run /opsx:propose..." Claude reads these instructions from the plan and executes them as tool calls in the conversation.

**Alternative considered:** A meta-skill that chains skill invocations. Claude Code doesn't support skill-calls-skill natively, so the orchestrator prompt is the composition layer.

### Mission config defaults to Codex workers, Claude validators
The config.yaml template pre-fills with these defaults. The user can edit before approving. This matches the current toolchain (Codex for coding, Claude for review).

## Risks / Trade-offs

- **[Long grill sessions may fill context]** -> The plan is written first and survives compaction. Mission state on disk means the orchestrator can re-read context after compaction.
- **[User may skip the grill]** -> The skill should allow but discourage skipping. The grill is what makes the contract good.
- **[Contract quality depends on orchestrator reasoning]** -> This is a prompt quality problem. We iterate on the skill's instructions as we learn what works. Factory had the same challenge and solved it with ~700 lines of prompts.
- **[OpenSpec propose may not match our exact needs]** -> We use it as-is for proposal/design/tasks. The contract is a separate step. If propose's output needs adjustment, we can add guidance to the skill prompt.

## Open Questions

- Should the skill support resuming an interrupted spec phase? (e.g., grill was done but propose hasn't run yet) -- defer to resume-and-status change.
