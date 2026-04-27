## ADDED Requirements

### Requirement: /adversarial skill exists
A skill named `adversarial` SHALL exist at `skills/adversarial/SKILL.md` and be user-invokable.

#### Scenario: Skill metadata is correct
- **WHEN** inspecting `skills/adversarial/SKILL.md`
- **THEN** the frontmatter `name` is `adversarial`
- **AND** `user-invokable` is `true`
- **AND** the description mentions dual-agent review and Claude + Codex

### Requirement: /adversarial accepts a review target
The skill SHALL accept an optional argument specifying what to review: a PR number, a file path, or no argument (defaults to current diff).

#### Scenario: No argument reviews current diff
- **WHEN** `/adversarial` is invoked with no arguments
- **THEN** the skill reviews uncommitted changes (`git diff` + `git diff --staged`)

#### Scenario: PR number argument reviews PR diff
- **WHEN** `/adversarial <number>` is invoked with a PR number
- **THEN** the skill fetches the PR diff with `gh pr diff <number>` before spawning reviewers

#### Scenario: File path argument reviews that file
- **WHEN** `/adversarial <path>` is invoked with a file path
- **THEN** the skill reads the specified file and uses it as the review target

### Requirement: Two agents run concurrently
The skill SHALL spawn a Codex reviewer and a Claude reviewer in parallel so both complete in approximately the same wall time as the slower reviewer.

#### Scenario: Codex reviewer dispatched async first
- **WHEN** `/adversarial` executes
- **THEN** the Codex reviewer is dispatched via `darkfactory-dispatch --async` before the Claude reviewer is launched
- **AND** the Claude reviewer is launched via the Agent tool after the Codex dispatch returns a job ID

#### Scenario: Both reviewers receive identical input
- **WHEN** both reviewers are spawned
- **THEN** each receives the same review target content (diff or file)
- **AND** each receives the same review instructions

### Requirement: Reviewer prompts are adversarial and independent
Each reviewer SHALL be instructed to review independently, without seeing the other's output.

#### Scenario: Adversarial framing in prompts
- **WHEN** each reviewer's prompt is inspected
- **THEN** it instructs the reviewer to be independent and opinionated
- **AND** it does not include the other reviewer's output or conclusions

### Requirement: Findings grouped by severity
Each reviewer's output SHALL group findings into severity levels.

#### Scenario: Severity levels present
- **WHEN** a reviewer produces output
- **THEN** findings are grouped as: critical, warning, nit
- **AND** each finding includes a `file:line` reference where applicable

### Requirement: Reports written to files
Both reviewer reports SHALL be written to the local filesystem.

#### Scenario: Codex report file
- **WHEN** `/adversarial` completes
- **THEN** the Codex reviewer's report is written to `mission/validations/adversarial-codex.md` (or a temp path if run outside a mission)

#### Scenario: Claude report file
- **WHEN** `/adversarial` completes
- **THEN** the Claude reviewer's report is written to `mission/validations/adversarial-claude.md` (or a temp path if run outside a mission)

### Requirement: Disagreements surfaced
After both reports are collected, the skill SHALL identify and surface points where the two reviewers disagree.

#### Scenario: Disagreements highlighted in summary
- **WHEN** both reviewer reports are collected
- **THEN** the skill presents a synthesis to the human that calls out findings mentioned by one reviewer but not the other, or findings where the severity assessment differs
