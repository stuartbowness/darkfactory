## Context

The build phase now implements tasks and validates them with both scrutiny and user-test validators. This change adds two final pieces: demo videos for frontend tasks and a draft PR for the human to review when the mission is done.

## Goals / Non-Goals

**Goals:**
- Record demo videos for frontend tasks after validation passes
- Create a draft PR with mission summary after all tasks complete
- The orchestrator decides which tasks are frontend-facing (based on the task description and diff)

**Non-Goals:**
- Full PR review process (the human reviews the draft PR manually)
- Auto-merging (the human promotes the draft to ready and merges)
- Archival (change: resume-and-status -- /opsx:archive runs after merge)

## Decisions

### Orchestrator decides which tasks get demo videos
During the grill, we decided the orchestrator makes this call, not the validator. The orchestrator knows the full plan and what each task involves. If a task has any frontend component, it gets a demo video.

### Demo video is a Claude subagent
The demo video step spawns a subagent that runs /demo-video. It needs the app running (which it should be from user-test validation) and a description of what to demo.

### Draft PR uses /pr skill
The orchestrator runs /pr to create the PR. The PR body includes a mission summary: list of tasks, validation results per task, links to demo videos, any issues that were fixed, and the total attempt count.

### PR is always a draft
The mission system never creates a ready PR. The human must review and promote it. This matches the Tier 3 review pattern -- anything big enough to warrant a mission warrants human review.

## Risks / Trade-offs

- **[Demo video may fail if app is in a bad state]** -> The demo is a nice-to-have, not a blocker. If it fails, log it and skip. Don't enter the fix loop for a demo failure.
- **[PR summary may be very long for large missions]** -> Keep it to key facts per task. Link to the full reports in the mission directory.
