## ADDED Requirements

### Requirement: Orchestrator spawns user-test validator after scrutiny passes
The orchestrator SHALL spawn a user-test validator subagent after the scrutiny validator reports PASS.

#### Scenario: User-test validator spawned
- **WHEN** the scrutiny validator reports overall PASS
- **THEN** the orchestrator spawns a user-test validator with the commit SHA and contract assertions

### Requirement: Validator receives only commit SHA and contract assertions
The user-test validator SHALL NOT receive the proposal, design, worker brief, or worker reasoning.

#### Scenario: Adversarial separation maintained
- **WHEN** the user-test validator is spawned
- **THEN** its prompt contains only the commit SHA, contract assertions, and test instructions

### Requirement: Validator decides test strategy from the diff
The user-test validator SHALL read the commit diff and decide whether to run API tests, browser tests, or both.

#### Scenario: Backend-only change
- **WHEN** the diff only contains backend files (API routes, database, server logic)
- **THEN** the validator runs HTTP API tests only

#### Scenario: Frontend-only change
- **WHEN** the diff only contains frontend files (components, pages, styles)
- **THEN** the validator runs browser tests via agent-browser only

#### Scenario: Full-stack change
- **WHEN** the diff contains both backend and frontend files
- **THEN** the validator runs both API tests and browser tests

### Requirement: API tests make HTTP requests and check responses
The validator SHALL construct HTTP requests based on contract assertions and verify the responses.

#### Scenario: API assertion tested
- **WHEN** a contract assertion describes API behavior
- **THEN** the validator makes the relevant HTTP request and checks the response status, body, or headers

### Requirement: Browser tests use agent-browser
The validator SHALL use the /test-app skill (agent-browser) to interact with the UI when testing frontend assertions.

#### Scenario: UI assertion tested
- **WHEN** a contract assertion describes user-facing behavior
- **THEN** the validator drives a browser session, performs the described action, and checks the result

### Requirement: Validator can skip untestable assertions
The validator SHALL report SKIP for assertions it cannot test, with an explanation.

#### Scenario: Assertion skipped
- **WHEN** the validator cannot determine how to test an assertion with its available tools
- **THEN** it reports SKIP with a reason, and the orchestrator does not treat this as a failure

### Requirement: Validator writes structured report
The user-test validator SHALL write its report to `mission/validations/task-N-usertest.md`.

#### Scenario: Report written
- **WHEN** the validator completes all checks
- **THEN** a file exists at `mission/validations/task-N-usertest.md` with: overall PASS/FAIL, test strategy chosen, results per assertion (PASS/FAIL/SKIP with evidence), and a summary

### Requirement: Failure triggers existing fix loop
On user-test failure, the orchestrator SHALL enter the fix loop with the user-test failures, and re-run BOTH validators after any fix.

#### Scenario: User-test failure enters fix loop
- **WHEN** the user-test validator reports overall FAIL
- **THEN** the orchestrator enters the fix loop (Step 5a) with the user-test failure details
