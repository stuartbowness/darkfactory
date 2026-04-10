## Why

The worker loop can dispatch workers and track progress, but it has no way to verify that the work is correct. Without validation, the orchestrator blindly trusts that each worker built the right thing. This change adds the scrutiny validator -- the first line of defense that checks build/test/lint and runs code review against the validation contract.

## What Changes

- After each worker completes, the orchestrator spawns a scrutiny validator subagent
- The scrutiny validator receives the commit SHA and the relevant contract assertions
- It runs build, test, lint, `/review`, and `/code-simplify` against the diff
- It writes a validation report to `mission/validations/task-N-scrutiny.md`
- On failure, the orchestrator spawns a fix worker with the failure details, then re-runs both validators
- Circuit breaker after 3 failed attempts on the same task -- asks the human

## Capabilities

### New Capabilities
- `scrutiny-validator`: The Claude subagent that runs build/test/lint and code review against the contract assertions for a given commit
- `fix-loop`: The orchestrator's ability to spawn fix workers on validation failure and re-validate, with a circuit breaker after 3 attempts

### Modified Capabilities
- `mission-start`: The /mission skill gains validation steps between worker completion and task marking

## Impact

- Modified files: `.claude/skills/mission/SKILL.md` (add validation after worker, add fix loop)
- Dependencies: existing skills (`/review`, `/code-simplify`, `/security-review`), Claude Code Agent tool for spawning validator subagents
