# computed-coverage Specification

## Purpose
TBD - created by archiving change computed-coverage. Update Purpose after archive.
## Requirements
### Requirement: Coverage reports a gap for every uncovered scenario

Computed coverage SHALL consider every **Scenario** in the specs, including a scenario that omits its `acceptance` field. A scenario SHALL be reported as a **Coverage gap** when none of its acceptance entries links to a valid **Feature task**. A scenario with at least one acceptance entry linking to a valid Feature task SHALL NOT be reported as a gap, even if its other acceptance entries carry unresolved links — those surface at render time, not in coverage.

#### Scenario: Scenario omits acceptance
- **WHEN** a scenario declares no `acceptance` field
- **THEN** it is reported as a Coverage gap

#### Scenario: Only link points to a non-existent task
- **WHEN** every acceptance entry of a scenario links to a task number absent from the task list
- **THEN** the scenario is reported as a Coverage gap

#### Scenario: Only link points to a closing task
- **WHEN** every acceptance entry of a scenario links to a **Closing task**
- **THEN** the scenario is reported as a Coverage gap

#### Scenario: Scenario has at least one valid feature-task link
- **WHEN** a scenario has one acceptance entry linking to a valid Feature task
- **THEN** the scenario is NOT reported as a Coverage gap

#### Scenario: Covered scenario with an unresolved sibling link
- **WHEN** a scenario has one acceptance entry linking to a valid Feature task and another entry whose link is unresolved
- **THEN** the scenario is NOT reported as a Coverage gap
- **AND** the unresolved sibling link is left for the strict render-time check, not coverage

### Requirement: Each gap declares its gap kinds

Each Coverage gap SHALL declare the distinct reasons it is uncovered, drawn from exactly this set: no acceptance authored, a dangling task link, a link to a closing task. When a scenario has several invalid acceptance entries with different reasons, the gap SHALL list each distinct reason once, in this fixed order: no acceptance, dangling task link, closing-task link.

#### Scenario: No-acceptance scenario declares one kind
- **WHEN** a gap scenario has no acceptance field
- **THEN** its gap kind is exactly "no acceptance"

#### Scenario: Mixed invalid links declare each distinct kind
- **WHEN** a gap scenario has one acceptance entry with a dangling link and another linking to a closing task, and no valid link
- **THEN** the gap lists both "dangling task link" and "closing-task link", in the fixed order

### Requirement: Gap report content and determinism

The **Gap report** SHALL identify each Coverage gap by its capability and its **Spec Reference**. When a scenario has no Spec Reference, coverage SHALL identify it by its capability and scenario title rather than failing — coverage is tolerant (see the tolerance requirement), and an acceptance-less scenario without a Spec Reference is exactly the kind of gap coverage exists to surface. Each gap SHALL show its identifier (Spec Reference, or capability and title when absent), its scenario title, its behavior text, and its **Gap kind**(s). **Behavior text** is the verbatim, ordered list of the scenario's behavior bullets — the lines beginning `- **WHEN**`, `- **THEN**`, or `- **AND**` that precede `**Acceptance:**` — excluding HTML comments and blank lines. Gaps SHALL be ordered by capability name (ascending) then by document position of the scenario within that capability's `spec.md`. For identical input the report SHALL be byte-identical.

#### Scenario: Gaps in one capability are ordered by document position
- **WHEN** two scenarios in the same capability are both gaps
- **THEN** they appear in the report in the order they occur in the spec file

#### Scenario: Gaps across capabilities are ordered by capability name
- **WHEN** gaps exist in more than one capability
- **THEN** the capabilities appear in ascending name order

#### Scenario: Output is stable across runs
- **WHEN** the coverage report is produced twice from the same specs and task list
- **THEN** the two outputs are byte-identical

#### Scenario: Acceptance-less scenario without a Spec Reference is still reported
- **WHEN** a gap scenario has no acceptance field and no Spec Reference
- **THEN** it is reported as a gap, identified by its capability and scenario title, not skipped or errored

### Requirement: Canonical Gap report format

The Gap report SHALL follow this exact markdown layout: a `# Coverage Gap Report` title, a `## Summary` section with one line `<T> scenarios, <C> covered, <G> gaps`, then a `## Gaps` section. When there are no gaps the Gaps section SHALL contain the single line `None.`. Otherwise each gap SHALL render as a `### <identifier> — <scenario title>` heading (identifier is the Spec Reference `<capability>/<N>`, or `<capability>` when no Spec Reference exists), a `- Gap kind: <kinds>` line listing the distinct kinds comma-separated in the fixed order, and a `- Behavior:` line followed by each behavior bullet indented two spaces and rendered verbatim. Gap-kind labels SHALL be exactly `no acceptance`, `dangling task link`, and `closing-task link`. Sections SHALL be separated by exactly one blank line.

#### Scenario: Report with gaps matches the canonical layout
- **WHEN** capability `gadget` has scenario `gadget/1` "Sync gadget" whose only acceptance links to a missing task, capability `widget` has covered scenario `widget/1` and acceptance-less scenario `widget/2` "Delete widget"
- **THEN** the report is exactly:
  ```
  # Coverage Gap Report

  ## Summary
  3 scenarios, 1 covered, 2 gaps

  ## Gaps

  ### gadget/1 — Sync gadget
  - Gap kind: dangling task link
  - Behavior:
    - **WHEN** a gadget is edited
    - **THEN** it syncs

  ### widget/2 — Delete widget
  - Gap kind: no acceptance
  - Behavior:
    - **WHEN** the user deletes a widget
    - **THEN** it is removed
  ```

#### Scenario: Report with no gaps
- **WHEN** every scenario links to a valid Feature task
- **THEN** the Gaps section contains exactly the line `None.`
- **AND** the Summary line reports `0 gaps`

### Requirement: Summary line

The Gap report SHALL include exactly one summary line stating the total number of scenarios considered, the number covered, and the number of gaps.

#### Scenario: Summary reflects gaps
- **WHEN** the specs contain scenarios of which some are gaps
- **THEN** the summary line states the total scenario count, the covered count, and the gap count

#### Scenario: Summary with no gaps
- **WHEN** every scenario links to a valid Feature task
- **THEN** the summary line reports zero gaps

### Requirement: Coverage subcommand and exit codes

`darkfactory-render` SHALL provide a `coverage` subcommand taking `--specs <dir>` and `--tasks <md>` and writing the Gap report to standard output. It SHALL exit `0` when there are no gaps, `3` when one or more gaps are found, and `2` on a usage or parse error. Exit `3` is a tolerant, non-failing result, not an error.

#### Scenario: No gaps exits zero
- **WHEN** coverage finds no gaps
- **THEN** the process exits `0`

#### Scenario: Gaps found exits three
- **WHEN** coverage finds one or more gaps
- **THEN** the Gap report is written to stdout
- **AND** the process exits `3`

#### Scenario: Missing required argument exits two
- **WHEN** the `coverage` subcommand is invoked without `--specs` or without `--tasks`
- **THEN** the process exits `2`

### Requirement: Tolerant of unresolved links, strict on malformed input

Computed coverage SHALL tolerate unresolved acceptance links (a dangling task number or a link to a closing task) and omitted acceptance by reporting the scenario as a gap, never failing for those reasons. An acceptance-less scenario that also lacks a Spec Reference SHALL still be reported, identified by title. An acceptance-carrying scenario without a Spec Reference remains malformed input (exit `2`), unchanged from the strict parser. It SHALL fail with exit `2` on malformed input: an unrecognised verification method, a malformed task link, a duplicate Spec Reference within a capability, a duplicate task number in the task list, or an unreadable or missing input path. A task link is malformed when its `task:` token does not resolve to a `u32` — coverage SHALL accept exactly what the existing acceptance parser accepts, so coverage and the Renderer never disagree about a link (`0` is a valid `u32`, so `[task:0]` is a normal link, resolved against the task list like any other).

#### Scenario: Dangling link is a gap, not an error
- **WHEN** an acceptance entry links to a non-existent task
- **THEN** coverage reports a gap and does not exit `2` for that reason

#### Scenario: Acceptance-carrying scenario without a Spec Reference fails loudly
- **WHEN** a scenario declares acceptance but no Spec Reference
- **THEN** the process exits `2`

#### Scenario: Unrecognised verification method fails loudly
- **WHEN** a spec contains an acceptance entry with a verification method outside `[execution]`, `[output]`, `[inspection]`
- **THEN** the process exits `2`

#### Scenario: Duplicate task number fails loudly
- **WHEN** the task list contains two tasks with the same number
- **THEN** the process exits `2`

#### Scenario: Malformed task link fails loudly
- **WHEN** an acceptance entry's `task:` token does not resolve to a `u32` (e.g. `[Task:1]` with the wrong prefix, `[task:abc]`, or a missing task token)
- **THEN** the process exits `2`

### Requirement: Existing render and shadow behavior is unchanged

The `render` and `shadow` subcommands SHALL produce the same output and exit codes as before this change. Retaining acceptance-less scenarios and behavior text in the parser SHALL NOT alter the rendered contract or the shadow comparison for any input.

#### Scenario: Render output is unchanged
- **WHEN** `render` is run on an input that produced a given contract before this change
- **THEN** it produces the identical contract

#### Scenario: Shadow result is unchanged
- **WHEN** `shadow` is run on an input that produced a given result before this change
- **THEN** it produces the identical result

