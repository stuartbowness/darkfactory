## Context

The orchestrator skeleton change delivered the spec phase (/mission skill, plan mode, grill, propose, contract, mission directory). This change adds the build phase -- the loop where the orchestrator iterates through tasks and dispatches workers to implement them.

The worker loop is the core execution engine of Darkfactory. It must handle: assembling briefs from artifacts, invoking Codex (or Claude) non-interactively, waiting for completion without burning tokens, reading handoff reports, and tracking progress on disk.

All building blocks exist: Codex CLI (`codex exec`), Claude Code Monitor tool, the mission directory structure with briefs/ and handoffs/ directories.

## Goals / Non-Goals

**Goals:**
- Add build phase to the /mission skill (runs after plan approval)
- Assemble brief files from OpenSpec artifacts
- Dispatch workers via Codex CLI with Monitor for completion notification
- Read handoff reports and track progress in tasks.md and progress.md
- Support configurable engine (codex or claude) per mission config

**Non-Goals:**
- Validation (change: scrutiny-validation, usertest-validation)
- Fix loop and circuit breaker (depends on validation)
- Demo video recording (change: completion-phase)
- Draft PR creation (change: completion-phase)
- Resume from interrupted build (change: resume-and-status)

## Decisions

### Worker is invoked via `codex exec` with file-based communication
The orchestrator writes a brief to `mission/briefs/task-N.md`. It then runs `codex exec --full-auto --sandbox workspace-write -m gpt-5.4 -c model_reasoning_effort="high" -o mission/handoffs/task-N.md` with a prompt that points Codex at the brief. Codex reads the brief, implements the task, commits, and writes the handoff report to the output path.

**Alternative considered:** Using the Codex plugin's `/codex:rescue` command. Rejected because we need more control over the exact invocation flags, output path, and sandbox settings. `codex exec` gives us that.

### Monitor watches for worker completion
The orchestrator uses the Claude Code Monitor tool to kick off Codex in the background and watch for the handoff file to appear. This saves tokens -- Claude goes idle while Codex works instead of blocking on a Bash call.

The Monitor script:
1. Kicks off Codex in the background
2. Polls for the handoff file every 2 seconds
3. Reports completion or failure when Codex exits

**Alternative considered:** `Bash(run_in_background=true)` for simpler one-shot notification. Rejected because we agreed on Monitor as the single pattern throughout Darkfactory for consistency, and Monitor allows intermediate status reporting in future enhancements.

### Brief is assembled by reading and excerpting OpenSpec artifacts
The orchestrator reads proposal.md, design.md, and the specific task from tasks.md, then writes a self-contained brief. It extracts relevant sections rather than dumping entire files. The brief never includes content from contract.md (adversarial separation).

**Alternative considered:** Passing the full artifact files to Codex and letting it figure out what's relevant. Rejected because it wastes Codex's context window and may cause it to lose focus on the specific task.

### Claude subagent is the fallback engine
If `mission/config.yaml` has `worker: claude`, the orchestrator uses the Agent tool to spawn a Claude subagent instead of Codex. The subagent receives the same brief content in its prompt. It writes the handoff report to the same file path. The interface is identical -- only the invocation mechanism changes.

**Alternative considered:** Always using Codex. Rejected because model flexibility is a core design principle (Factory's "right model in each role"). The config makes it easy to swap.

### Each worker commits exactly once
The worker makes a single commit containing all changes for the task. The commit message follows the format `[task-N] description`. This makes validation clean -- the validator gets one commit SHA to review.

**Alternative considered:** Allowing multiple commits per task. Rejected because it complicates validation (which commit to review?) and handoff (which SHA to report?).

## Risks / Trade-offs

- **[Codex may fail silently]** -> The Monitor script checks the exit code. If Codex exits non-zero and no handoff file exists, the orchestrator logs the failure and asks the human. Without the validation change, there's no automatic retry yet.
- **[Brief may be too large for Codex context]** -> The orchestrator excerpts relevant sections, not full files. For very large proposals/designs, this is a prompt quality problem we iterate on.
- **[Codex may not follow handoff format]** -> The brief includes the exact format. If the handoff is malformed, the orchestrator logs it and asks the human. Structured enforcement comes with experience.
- **[Worker may make multiple commits]** -> The brief explicitly says "single commit." If violated, the orchestrator uses the most recent commit. Not ideal but recoverable.

## Open Questions

- What timeout should the Monitor use for a worker? Default to 600s (10 minutes) with a note that complex tasks may need longer. Can be adjusted per-project.
