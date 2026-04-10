## Context

The scrutiny validator (previous change) catches code-level problems. This change adds the user-test validator -- the behavioral testing layer that verifies features work end-to-end by interacting with the running application.

Factory's talk emphasized that most mission time is spent here, not on code generation. The user-test validator spawns the app, interacts with it, and checks that things actually work. This is what catches the gap between "code looks right" and "code works right."

## Goals / Non-Goals

**Goals:**
- Replace the Step 6 placeholder with user-test validation
- Validator decides API tests, browser tests, or both based on the diff
- API tests: make HTTP requests and assert responses
- Browser tests: use agent-browser (via /test-app skill) to interact with the UI
- Validator writes report to mission/validations/task-N-usertest.md
- On failure, existing fix loop handles retries (re-runs both validators)

**Non-Goals:**
- Demo video recording (change: completion-phase -- runs after validation, not during)
- Modifying the fix loop logic (already implemented in scrutiny-validation change)

## Decisions

### User-test validator is a Claude subagent
Like the scrutiny validator, it's spawned via the Agent tool with clean context. It receives the commit SHA, contract assertions, and instructions. It does NOT receive the proposal, design, or worker reasoning.

### Validator decides its own test strategy
The validator reads the commit diff and determines:
- If only backend files changed -> HTTP API tests only
- If only frontend files changed -> browser tests only (agent-browser)
- If both changed -> both API tests and browser tests

This is a judgment call by the validator, not a tag in the contract. The orchestrator doesn't need to classify tasks upfront.

**Alternative considered:** Tagging assertions in the contract with test types. Rejected during the grill -- it adds complexity to the contract and the validator is smart enough to decide.

### The application must be running for user-test validation
The validator needs a live app to test against. The validator's instructions include running the project's dev server if it's not already running. This is project-specific -- the instructions reference the project's dev command.

**Alternative considered:** Having the validator start the app itself. This works but the validator needs to know the dev command. We include it in the validator prompt, derived from the project's CLAUDE.md or package.json.

### API tests use curl or similar HTTP tools
The validator constructs HTTP requests based on the contract assertions. For example, if the assertion says "creating a channel with a duplicate name returns an error," the validator sends a POST to create a channel, sends the same POST again, and checks for an error response.

### Browser tests use /test-app (agent-browser)
The validator drives a browser session using the /test-app skill. It navigates to pages, fills forms, clicks buttons, and checks that the UI behaves as the contract asserts.

## Risks / Trade-offs

- **[App may not start cleanly]** -> The validator reports startup failure. The orchestrator asks the human. This is a project setup issue, not a code issue.
- **[Browser tests are slow and flaky]** -> Factory noted most mission time is spent here. Accept the cost -- behavioral validation is the point. Flaky failures get caught by the circuit breaker.
- **[Validator may not know how to test an assertion]** -> It reports SKIP with an explanation instead of PASS/FAIL. The orchestrator logs it but doesn't treat it as a failure.
