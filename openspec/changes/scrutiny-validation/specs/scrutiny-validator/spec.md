## ADDED Requirements

### Requirement: Orchestrator spawns scrutiny validator after worker completes
The orchestrator SHALL spawn a Claude subagent via the Agent tool after reading a successful worker handoff.

#### Scenario: Validator spawned
- **WHEN** the orchestrator reads a valid handoff report with a commit SHA
- **THEN** it spawns a scrutiny validator subagent with the commit SHA and contract assertions for this task

### Requirement: Validator receives only commit SHA and contract assertions
The scrutiny validator SHALL NOT receive the proposal, design, worker brief, or worker handoff reasoning. It receives only what it needs to validate.

#### Scenario: Adversarial separation maintained
- **WHEN** the validator subagent is spawned
- **THEN** its prompt contains only the commit SHA, the contract assertions for this task, and instructions for what checks to run

### Requirement: Validator runs build, test, and lint
The scrutiny validator SHALL run the project's build, test, and lint commands and report their exit codes.

#### Scenario: Build/test/lint pass
- **WHEN** the validator runs build, test, and lint
- **THEN** the report includes pass status and exit codes for each

#### Scenario: Build/test/lint fail
- **WHEN** any of build, test, or lint fail
- **THEN** the report includes fail status, exit codes, and the relevant error output

### Requirement: Validator runs /review against the diff
The scrutiny validator SHALL run the /review skill against the commit diff, checking for bugs, security issues, and logic errors.

#### Scenario: Code review completed
- **WHEN** the validator reviews the diff
- **THEN** the report includes review findings grouped by severity (critical, warning, nit)

### Requirement: Validator runs /code-simplify against the diff
The scrutiny validator SHALL run the /code-simplify skill to check for unnecessary complexity in the changes.

#### Scenario: Simplicity check completed
- **WHEN** the validator checks the diff for complexity
- **THEN** the report includes any simplification findings

### Requirement: Validator runs /security-review for sensitive tasks
The scrutiny validator SHALL run the /security-review skill when the task touches auth, sessions, or sensitive data paths.

#### Scenario: Security review triggered
- **WHEN** the commit diff touches auth middleware, session handling, or credential-related code
- **THEN** the report includes security review findings

### Requirement: Validator checks each contract assertion
The scrutiny validator SHALL evaluate each assertion from the contract for this task and report pass or fail with evidence.

#### Scenario: Assertion passes
- **WHEN** the validator can confirm an assertion is satisfied by the code
- **THEN** the report marks it as PASS with a brief explanation

#### Scenario: Assertion fails
- **WHEN** the validator cannot confirm an assertion is satisfied
- **THEN** the report marks it as FAIL with the specific reason

### Requirement: Validator writes structured report
The scrutiny validator SHALL write its report to `mission/validations/task-N-scrutiny.md`.

#### Scenario: Report written
- **WHEN** the validator completes all checks
- **THEN** a file exists at `mission/validations/task-N-scrutiny.md` with: overall pass/fail, build/test/lint results, review findings, assertion results, and a summary
