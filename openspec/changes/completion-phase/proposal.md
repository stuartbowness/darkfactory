## Why

The build phase can implement and validate tasks, but it has no completion step. There's no demo video for frontend work and no draft PR for the human to review. This change adds the finishing touches: demo video recording for frontend tasks after validation passes, and a draft PR with mission summary when all tasks are done.

## What Changes

- After both validators pass on a frontend task, the orchestrator spawns a demo video step
- The demo video uses the /demo-video skill to record the feature in action
- Videos are saved to `mission/validations/task-N-demo.mp4`
- After all tasks are complete, the orchestrator creates a draft PR via /pr with a mission summary including: tasks completed, validation results, demo video links, and any issues encountered

## Capabilities

### New Capabilities
- `demo-video-step`: The orchestrator's ability to record a demo video for frontend tasks after both validators pass
- `mission-completion`: The orchestrator's ability to create a draft PR with a full mission summary when all tasks are done

### Modified Capabilities
- `mission-start`: The /mission skill gains a demo video step between validation and task marking, plus a completion phase after the build loop

## Impact

- Modified files: `.claude/skills/mission/SKILL.md` (add demo video step, add completion phase)
- Dependencies: `/demo-video` skill, `/pr` skill, `gh` CLI for draft PR creation
