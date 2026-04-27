## ADDED Requirements

### Requirement: /dispatch skill exists
A skill named `dispatch` SHALL exist at `skills/dispatch/SKILL.md` and be user-invokable.

#### Scenario: Skill metadata is correct
- **WHEN** inspecting `skills/dispatch/SKILL.md`
- **THEN** the frontmatter `name` is `dispatch`
- **AND** `user-invokable` is `true`
- **AND** the description mentions wrapping darkfactory-dispatch

### Requirement: /dispatch accepts a task description
The skill SHALL accept a task description (from arguments or conversation context) and pass it to `darkfactory-dispatch` as the prompt.

#### Scenario: Task from arguments
- **WHEN** `/dispatch <task description>` is invoked with inline text
- **THEN** the skill uses that text as the prompt passed to `darkfactory-dispatch`

#### Scenario: Task from context
- **WHEN** `/dispatch` is invoked with no arguments
- **THEN** the skill asks the user for the task description before dispatching

### Requirement: /dispatch calls darkfactory-dispatch with supervision
The skill SHALL invoke `darkfactory-dispatch` (not `codex exec` directly) so that timeout, heartbeat, and process group isolation are enforced.

#### Scenario: Uses darkfactory-dispatch binary
- **WHEN** `/dispatch` runs a task
- **THEN** it calls `darkfactory-dispatch` with a prompt file and `--json` for structured output
- **AND** it does not call `codex exec` directly

### Requirement: /dispatch presents the result
After the dispatch completes, the skill SHALL present the result to the user in a readable format.

#### Scenario: Successful dispatch result shown
- **WHEN** `darkfactory-dispatch` completes with status `completed`
- **THEN** the skill prints the summary of what the worker did (from the structured JSON result)
- **AND** it reports the duration and any files written

#### Scenario: Failed dispatch result shown
- **WHEN** `darkfactory-dispatch` completes with status `failed` or `timed_out`
- **THEN** the skill reports the failure status and any error details from the result JSON
- **AND** it does not suppress the failure

### Requirement: /dispatch surfaces the output file path
The skill SHALL tell the user where the full output was written.

#### Scenario: Output path reported
- **WHEN** `/dispatch` completes
- **THEN** the skill prints the path to the output file containing the full worker output
