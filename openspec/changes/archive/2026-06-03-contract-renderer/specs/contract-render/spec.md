## ADDED Requirements

### Requirement: Render checks grouped by task

The renderer SHALL emit each spec-derived check under the task named by its acceptance entry's `task` link, formatted as `- [ ] [<method>] <statement>`. The check statement MUST be carried verbatim from the acceptance entry, with no rewording, strengthening, or invention.

#### Scenario: Check appears under its task
- **WHEN** the renderer runs over scenarios whose acceptance entries link to tasks
- **THEN** each check appears under the `## Task N` section named by its `task` link

#### Scenario: Statement carried verbatim
- **WHEN** an acceptance entry has a check statement
- **THEN** the rendered check text equals that statement exactly

#### Scenario: Deterministic ordering within a task
- **WHEN** a task has multiple checks
- **THEN** the checks render in Spec Reference order (by capability name, then N)

### Requirement: Render one Spec Reference comment per task

For each task, the renderer SHALL emit a single `<!-- Specs: ... -->` comment listing the Spec References of all spec-derived checks under that task, de-duplicated and ordered by capability name then N.

#### Scenario: References de-duplicated and ordered
- **WHEN** a task's checks reference several scenarios, including a repeated reference
- **THEN** the task's `<!-- Specs: ... -->` comment lists each reference once, ordered by capability then N

#### Scenario: Deviated reference marked
- **WHEN** a referenced scenario's status is `deviated`
- **THEN** that reference is marked `(DEVIATED)` within the comment

### Requirement: Include operational checks from the closing-task fragment

The renderer SHALL source the mandatory operational checks (smoke test, cleanup, module-shape re-check, CLAUDE.md/ARCH.md, documentation) from a standing closing-task fragment and render them under their closing tasks. The closing-task fragment declares which tasks are closing tasks; every task not declared by the fragment is a feature task. Operational checks are not spec-derived and MUST NOT carry a Spec Reference comment. An acceptance entry MUST NOT link to a closing task, so each task section has a single check source.

#### Scenario: Closing-task checks rendered from the fragment
- **WHEN** the renderer composes a contract
- **THEN** the mandatory closing-task checks appear under their closing tasks, sourced from the closing-task fragment

#### Scenario: Operational checks carry no Spec Reference
- **WHEN** a closing-task check is rendered
- **THEN** it carries no `<!-- Specs: ... -->` reference

#### Scenario: Acceptance entry links to a closing task
- **WHEN** an acceptance entry links to a task the fragment declares as a closing task
- **THEN** the renderer exits with an error naming the entry and the closing task

### Requirement: Produce deterministic output

The renderer SHALL produce byte-identical output for identical input. Task sections SHALL render in task-list order; checks within a task SHALL render in Spec Reference order (by capability name, then N).

#### Scenario: Repeated runs match
- **WHEN** the renderer runs twice over the same scenarios, task list, and fragment
- **THEN** the two `contract.md` outputs are byte-identical

#### Scenario: Task sections follow task-list order
- **WHEN** the renderer emits multiple task sections
- **THEN** the sections appear in the order the tasks appear in the task list

### Requirement: Fail loudly on inconsistency

The renderer SHALL exit with an error, rather than emit a partial contract, when it encounters an acceptance entry whose `task` link names no task in the task list, a duplicate Spec Reference, or a feature task with zero checks. A closing task with no spec-derived checks is not an error because it receives checks from the fragment.

#### Scenario: Dangling task link
- **WHEN** an acceptance entry links to a task that is absent from the task list
- **THEN** the renderer exits with an error naming the entry and the missing task

#### Scenario: Feature task with zero checks
- **WHEN** a feature task ends up with no checks
- **THEN** the renderer exits with an error naming the task

#### Scenario: Duplicate Spec Reference at render time
- **WHEN** two scenarios share a Spec Reference
- **THEN** the renderer exits with an error naming the duplicated reference

### Requirement: Match the existing contract format

The rendered `contract.md` SHALL use the existing contract format: `## Task N: <name>` sections, `<!-- Specs: capability/N -->` comments, and `- [ ] [execution|output|inspection] <statement>` checks, so that validators and downstream phases consume it unchanged.

#### Scenario: Format fidelity
- **WHEN** the renderer emits a contract
- **THEN** its task headers, Spec Reference comments, and check lines match the current contract template's structure

### Requirement: Warn when inspection checks exceed the cap

The renderer SHALL emit a non-fatal warning when a task's `[inspection]` checks exceed 20% of that task's total checks.

#### Scenario: Inspection checks over the cap
- **WHEN** a task's `[inspection]` checks exceed 20% of that task's checks
- **THEN** the renderer emits a non-fatal warning naming the task

#### Scenario: Inspection checks at or under the cap
- **WHEN** a task's `[inspection]` checks are at or below 20% of that task's checks
- **THEN** the renderer emits no inspection-cap warning for that task
