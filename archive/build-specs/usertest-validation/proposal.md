## Why

The scrutiny validator catches code-level issues (build failures, lint errors, review findings) but can't verify that the feature actually works from a user's perspective. A worker might produce code that compiles and passes lint but doesn't render correctly, returns wrong API responses, or breaks user flows. This change adds the user-test validator -- the second validator that tests behavior end-to-end.

## What Changes

- After scrutiny validation passes, the orchestrator spawns a user-test validator subagent
- The user-test validator receives the commit SHA and contract assertions
- It decides whether to run API tests (HTTP requests), browser tests (agent-browser), or both based on what the diff changed
- It writes a validation report to `mission/validations/task-N-usertest.md`
- On failure, the existing fix loop handles retries (re-runs BOTH validators after each fix)

## Capabilities

### New Capabilities
- `usertest-validator`: The Claude subagent that tests behavior by hitting APIs and/or driving a browser, checking contract assertions against actual runtime behavior

### Modified Capabilities
- `mission-start`: The /mission skill's Step 6 placeholder is replaced with actual user-test validation logic

## Impact

- Modified files: `.claude/skills/mission/SKILL.md` (replace Step 6 placeholder with user-test validation)
- Dependencies: `/test-app` skill (agent-browser), project must be running locally for browser/API tests
