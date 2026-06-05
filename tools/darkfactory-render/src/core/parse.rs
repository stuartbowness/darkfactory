//! Spec-scenario parsing for `darkfactory-render`.
//!
//! Turns a capability's spec markdown into typed [`RequirementScenario`]
//! values, hiding the acceptance-block grammar (design D3) and the
//! spec-ref/status comment grammar (design D4) behind one narrow function. It
//! does no file IO: the CLI reads files and hands strings to this module, so
//! every grammar rule is unit-testable on in-memory strings.
//!
//! The parser is fail-loud: every malformed case returns a typed
//! [`RenderError`] rather than guessing or silently dropping content.

use std::collections::HashSet;

use crate::core::types::{
    AcceptanceEntry, ClosingTaskFragment, ClosingTaskKind, RenderError, RequirementScenario,
    ScenarioStatus, SpecReference, Task, TaskList, VerificationMethod,
};

const SCENARIO_HEADING: &str = "#### Scenario:";
const SPEC_REF_PREFIX: &str = "<!-- spec-ref:";
const STATUS_PREFIX: &str = "<!-- status:";
const ACCEPTANCE_MARKER: &str = "**Acceptance:**";

/// Parse one capability's spec markdown into its scenarios.
///
/// `capability` is the spec folder name; it is the capability half of every
/// scenario's [`SpecReference`] (the parser never invents it). `spec_markdown`
/// is the full text of one capability's `spec.md`.
///
/// A scenario begins at a `#### Scenario:` heading. Directly under the heading
/// two optional HTML comments may appear: `<!-- spec-ref: N -->` (sets the
/// scenario's Spec Reference to `capability/N`) and `<!-- status: <value> -->`.
/// After the WHEN/THEN bullets, an optional `**Acceptance:**` block lists one
/// or more `- [task:N] [<method>] <statement>` entries. Each acceptance entry
/// inherits the scenario's Spec Reference.
///
/// Scenarios without an acceptance block parse successfully and contribute no
/// entries; only scenarios that carry acceptance require a `spec-ref`.
///
/// # Errors
///
/// Returns a [`RenderError`] for any malformed scenario:
/// [`EmptyStatement`](RenderError::EmptyStatement),
/// [`EmptyAcceptanceList`](RenderError::EmptyAcceptanceList),
/// [`MissingSpecRef`](RenderError::MissingSpecRef),
/// [`DuplicateSpecReference`](RenderError::DuplicateSpecReference),
/// [`UnknownVerificationMethod`](RenderError::UnknownVerificationMethod), or
/// [`UnknownStatus`](RenderError::UnknownStatus).
pub fn parse_specs(
    capability: &str,
    spec_markdown: &str,
) -> Result<Vec<RequirementScenario>, RenderError> {
    let mut scenarios = Vec::new();
    let mut seen_refs: HashSet<u32> = HashSet::new();

    for block in scenario_blocks(spec_markdown) {
        // Every scenario is retained, including acceptance-less ones. Duplicate
        // detection applies only to scenarios that carry a Spec Reference.
        let scenario = parse_scenario(capability, block)?;
        if let Some(spec_ref) = &scenario.spec_ref {
            let number = spec_ref.number;
            if !seen_refs.insert(number) {
                return Err(RenderError::DuplicateSpecReference(format!(
                    "{capability}/{number}"
                )));
            }
        }
        scenarios.push(scenario);
    }

    Ok(scenarios)
}

const CLOSING_PREFIX: &str = "Closing:";

/// Parse `tasks.md` content into a [`TaskList`].
///
/// A task begins at a `Task N: <name>` heading line (optionally prefixed with a
/// `- [ ]`/`- [x]`/`- [~]` checkbox). The heading's number and name are read
/// from the `Task N:` part. A `Closing: <kind>` line anywhere in the task's body
/// (before the next heading) sets that task's [`Task::closing_kind`]; a task
/// without one is a feature task (`None`).
///
/// Content before the first task heading is ignored. This does no file IO.
pub fn parse_tasks(tasks_markdown: &str) -> Result<TaskList, RenderError> {
    let mut tasks: Vec<Task> = Vec::new();
    for line in tasks_markdown.lines() {
        if let Some((number, name)) = task_heading(line) {
            tasks.push(Task {
                number,
                name: name.to_string(),
                closing_kind: None,
            });
        } else if let Some(kind) = closing_field(line) {
            if let Some(current) = tasks.last_mut() {
                current.closing_kind = Some(kind.to_string());
            }
        }
    }
    Ok(TaskList { tasks })
}

/// The `<kind>` of a `Closing: <kind>` body line, or `None`.
fn closing_field(line: &str) -> Option<&str> {
    let kind = line.trim().strip_prefix(CLOSING_PREFIX)?.trim();
    (!kind.is_empty()).then_some(kind)
}

const KIND_HEADING: &str = "### ";

/// Parse the closing-task fragment into a [`ClosingTaskFragment`].
///
/// The fragment groups operational checks under per-kind sections. A section
/// begins at a `### <kind>` heading; the lines beneath it, up to the next `###`
/// heading, are its checks. Each check is a `- [<method>] <statement>` bullet
/// stored verbatim as its `[<method>] <statement>` text (the `- ` bullet prefix
/// is stripped); the renderer (Task 4) prefixes `- [ ] `. Non-bullet lines and
/// content before the first `### ` heading are ignored. This does no file IO.
///
/// # Errors
///
/// A check whose `[<method>]` tag is not a recognized [`VerificationMethod`]
/// surfaces as [`RenderError::UnknownVerificationMethod`], matching `parse_specs`.
pub fn parse_fragment(fragment_markdown: &str) -> Result<ClosingTaskFragment, RenderError> {
    let mut kinds: Vec<ClosingTaskKind> = Vec::new();
    for line in fragment_markdown.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix(KIND_HEADING) {
            kinds.push(ClosingTaskKind {
                name: name.trim().to_string(),
                checks: Vec::new(),
            });
        } else if let Some(check) = fragment_check(trimmed) {
            validate_check_method(check)?;
            if let Some(current) = kinds.last_mut() {
                current.checks.push(check.to_string());
            }
        }
    }
    Ok(ClosingTaskFragment { kinds })
}

/// Check every closing task's kind against the fragment's declared kinds.
///
/// A closing task is one whose [`Task::closing_kind`] is `Some`. Each such
/// kind must name a [`ClosingTaskKind`] in `fragment`; the first that does not
/// surfaces as [`RenderError::UnknownClosingKind`] carrying the offending kind.
/// Feature tasks (`closing_kind == None`) are ignored.
pub fn validate_closing_kinds(
    tasks: &TaskList,
    fragment: &ClosingTaskFragment,
) -> Result<(), RenderError> {
    for task in &tasks.tasks {
        let Some(kind) = &task.closing_kind else {
            continue;
        };
        if !fragment.kinds.iter().any(|known| &known.name == kind) {
            return Err(RenderError::UnknownClosingKind(kind.clone()));
        }
    }
    Ok(())
}

/// The verbatim `[<method>] <statement>` text of a `- [<method>] ...` bullet,
/// or `None` for a non-bullet line.
fn fragment_check(line: &str) -> Option<&str> {
    let body = line.strip_prefix("- ")?.trim_start();
    body.starts_with('[').then_some(body)
}

/// Fail loud if a fragment check's `[<method>]` tag is not a known method.
fn validate_check_method(check: &str) -> Result<(), RenderError> {
    let malformed = || RenderError::UnknownVerificationMethod(check.to_string());
    let (method_token, _) = bracketed(check).ok_or_else(malformed)?;
    VerificationMethod::parse(method_token)?;
    Ok(())
}

/// Parse a `Task N: <name>` heading into its number and name.
///
/// The line may carry a checkbox prefix (`- [ ]`, `- [x]`, `- [~]`); it is
/// stripped before reading `Task N:`. Returns `None` for any non-heading line.
fn task_heading(line: &str) -> Option<(u32, &str)> {
    let after_checkbox = strip_checkbox(line.trim_start());
    let rest = after_checkbox.strip_prefix("Task ")?;
    let colon = rest.find(':')?;
    let number = rest[..colon].trim().parse::<u32>().ok()?;
    let name = rest[colon + 1..].trim();
    Some((number, name))
}

/// Strip a leading task checkbox (`- [ ]`, `- [x]`, `- [~]`) if present.
fn strip_checkbox(line: &str) -> &str {
    let Some(rest) = line.strip_prefix("- [") else {
        return line;
    };
    let Some(close) = rest.find(']') else {
        return line;
    };
    rest[close + 1..].trim_start()
}

/// Split the markdown into one slice of lines per `#### Scenario:` heading.
///
/// Content before the first scenario heading (e.g. requirement prose) is
/// ignored, matching the OpenSpec spec layout.
fn scenario_blocks(markdown: &str) -> Vec<Vec<&str>> {
    let mut blocks: Vec<Vec<&str>> = Vec::new();
    for line in markdown.lines() {
        if line.trim_start().starts_with(SCENARIO_HEADING) {
            blocks.push(vec![line]);
        } else if let Some(current) = blocks.last_mut() {
            current.push(line);
        }
    }
    blocks
}

/// Parse one scenario block into a [`RequirementScenario`].
///
/// Every scenario is retained, including an acceptance-less one (it contributes
/// no entries and carries `spec_ref == None` unless it declares a `spec-ref`
/// comment). An acceptance-carrying scenario without a Spec Reference still
/// fails loud with [`RenderError::MissingSpecRef`].
fn parse_scenario(capability: &str, lines: Vec<&str>) -> Result<RequirementScenario, RenderError> {
    let title = scenario_title(lines[0]);

    let mut spec_ref_number: Option<u32> = None;
    let mut status_token: Option<&str> = None;
    let mut acceptance_start: Option<usize> = None;
    let mut behavior: Vec<String> = Vec::new();

    for (index, raw) in lines.iter().enumerate().skip(1) {
        let line = raw.trim();
        if let Some(rest) = comment_value(line, SPEC_REF_PREFIX) {
            spec_ref_number = Some(parse_spec_ref_number(rest)?);
        } else if let Some(rest) = comment_value(line, STATUS_PREFIX) {
            status_token = Some(rest);
        } else if line == ACCEPTANCE_MARKER {
            acceptance_start = Some(index + 1);
            break;
        } else if is_behavior_bullet(line) {
            behavior.push(line.to_string());
        }
    }

    let status = ScenarioStatus::parse_optional(status_token)?;

    let Some(acceptance_start) = acceptance_start else {
        // No acceptance block: a retained scenario that contributes no entries.
        // It is never required to carry a Spec Reference.
        return Ok(RequirementScenario {
            title: title.to_string(),
            capability: capability.to_string(),
            behavior,
            spec_ref: spec_ref_number.map(|number| SpecReference {
                capability: capability.to_string(),
                number,
            }),
            status,
            acceptance: Vec::new(),
        });
    };

    let identifier = scenario_identifier(capability, spec_ref_number, title);
    let acceptance = parse_acceptance(&lines[acceptance_start..], &identifier)?;
    if acceptance.is_empty() {
        return Err(RenderError::EmptyAcceptanceList(identifier));
    }

    let number = spec_ref_number.ok_or(RenderError::MissingSpecRef)?;

    Ok(RequirementScenario {
        title: title.to_string(),
        capability: capability.to_string(),
        behavior,
        spec_ref: Some(SpecReference {
            capability: capability.to_string(),
            number,
        }),
        status,
        acceptance,
    })
}

/// Whether a trimmed line is a behavior bullet: `- **WHEN**`, `- **THEN**`, or
/// `- **AND**`. Blank lines, HTML comments, and other prose are excluded.
fn is_behavior_bullet(line: &str) -> bool {
    line.starts_with("- **WHEN**")
        || line.starts_with("- **THEN**")
        || line.starts_with("- **AND**")
}

/// The scenario title after the `#### Scenario:` heading.
fn scenario_title(heading: &str) -> &str {
    heading
        .trim_start()
        .trim_start_matches(SCENARIO_HEADING)
        .trim()
}

/// The inner text of an HTML comment `<!-- prefix VALUE -->`, or `None`.
fn comment_value<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(prefix)?;
    let inner = rest.strip_suffix("-->")?;
    Some(inner.trim())
}

/// Parse the integer N from a `spec-ref` comment value.
///
/// A non-numeric value yields no usable Spec Reference, so it surfaces as
/// [`RenderError::MissingSpecRef`] rather than inventing a number. `RenderError`
/// has no dedicated "malformed spec-ref" variant in Stage 1; missing and
/// malformed both mean "this acceptance-carrying scenario has no valid ref".
fn parse_spec_ref_number(value: &str) -> Result<u32, RenderError> {
    value
        .parse::<u32>()
        .map_err(|_| RenderError::MissingSpecRef)
}

/// Parse the acceptance entry lines following the `**Acceptance:**` marker.
///
/// `identifier` names the enclosing scenario (`capability/N` or its title) so a
/// per-entry error can point the author at the offending scenario.
///
/// Stops at the next heading (or the end of the scenario block). Blank lines and
/// non-entry prose between or after entries are skipped, so an acceptance block
/// may separate its entries with blank lines without dropping any. A line that
/// looks like an entry (`- ...`) but is malformed still fails loud via
/// [`parse_acceptance_entry`].
fn parse_acceptance(lines: &[&str], identifier: &str) -> Result<Vec<AcceptanceEntry>, RenderError> {
    let mut entries = Vec::new();
    for raw in lines {
        let line = raw.trim();
        if line.starts_with("#### ") || line.starts_with("### ") {
            break;
        }
        if !line.starts_with("- ") {
            // Blank lines and trailing prose between/after entries are skipped
            // rather than terminating the block, so a blank line never silently
            // drops the entries that follow it.
            continue;
        }
        entries.push(parse_acceptance_entry(line, identifier)?);
    }
    Ok(entries)
}

/// Parse one acceptance entry `- [task:N] [<method>] <statement>`.
///
/// An unrecognized method surfaces as
/// [`RenderError::UnknownVerificationMethod`]. A genuinely empty or
/// whitespace-only statement surfaces as
/// [`RenderError::EmptyStatement`], carrying `identifier` so the error names the
/// offending scenario. A structurally broken entry (missing `[task:N]` or
/// `[<method>]` brackets, or a non-numeric task number) is NOT an empty
/// statement: Stage 1's `RenderError` has no dedicated "malformed entry"
/// variant, so such a line fails loud as
/// [`RenderError::UnknownVerificationMethod`] carrying the offending entry
/// text. Reporting the verbatim line keeps the error accurate — the parser
/// could not extract a valid verification method from it — instead of
/// mislabelling a populated line as "empty".
fn parse_acceptance_entry(line: &str, identifier: &str) -> Result<AcceptanceEntry, RenderError> {
    let body = line.trim_start_matches("- ").trim_start();

    let malformed = || RenderError::UnknownVerificationMethod(body.to_string());

    let (task_token, after_task) = bracketed(body).ok_or_else(malformed)?;
    let task_link = parse_task_link(task_token).ok_or_else(malformed)?;

    let (method_token, after_method) = bracketed(after_task.trim_start()).ok_or_else(malformed)?;
    let method = VerificationMethod::parse(method_token)?;

    let statement = after_method.trim();
    if statement.is_empty() {
        return Err(RenderError::EmptyStatement(identifier.to_string()));
    }

    Ok(AcceptanceEntry {
        statement: statement.to_string(),
        method,
        task_link,
    })
}

/// Split a leading `[token]` from the rest of a line.
///
/// Returns the bare token (without brackets) and the remainder, or `None` when
/// the text does not begin with a closed `[...]`.
fn bracketed(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix('[')?;
    let close = rest.find(']')?;
    Some((&rest[..close], &rest[close + 1..]))
}

/// Parse the `task:N` token into the integer link N, or `None` if malformed.
fn parse_task_link(token: &str) -> Option<u32> {
    token.strip_prefix("task:")?.trim().parse::<u32>().ok()
}

/// A human-readable scenario identifier for error messages.
///
/// Prefers `capability/N` when a Spec Reference is present, else the title.
fn scenario_identifier(capability: &str, number: Option<u32>, title: &str) -> String {
    match number {
        Some(number) => format!("{capability}/{number}"),
        None => title.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{ScenarioStatus, SpecReference, VerificationMethod};

    #[test]
    fn parse_tasks_extracts_number_and_name_from_a_feature_task() {
        let markdown = "\
- [ ] Task 1: Crate skeleton
  Depends on: none
";
        let tasks = parse_tasks(markdown).expect("valid tasks parse");
        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].number, 1);
        assert_eq!(tasks.tasks[0].name, "Crate skeleton");
        assert_eq!(tasks.tasks[0].closing_kind, None);
    }

    #[test]
    fn parse_fragment_yields_kinds_mapped_to_verbatim_checks() {
        let markdown = "\
# Closing-task checks

### smoke-test
- [execution] the primary user flow runs end to end and passes

### cleanup
- [execution] the linter is clean
- [inspection] no leftover scaffolding or temporary artifacts remain
";
        let fragment = parse_fragment(markdown).expect("valid fragment parses");
        assert_eq!(fragment.kinds.len(), 2);

        assert_eq!(fragment.kinds[0].name, "smoke-test");
        assert_eq!(
            fragment.kinds[0].checks,
            vec!["[execution] the primary user flow runs end to end and passes".to_string()],
            "a check is stored verbatim as its `[<method>] <statement>` text"
        );

        assert_eq!(fragment.kinds[1].name, "cleanup");
        assert_eq!(
            fragment.kinds[1].checks,
            vec![
                "[execution] the linter is clean".to_string(),
                "[inspection] no leftover scaffolding or temporary artifacts remain".to_string(),
            ]
        );
    }

    #[test]
    fn validate_closing_kinds_accepts_a_fragment_known_kind() {
        // Downstream integration check from the brief: a feature task (no kind)
        // and a closing task tagged with a fragment-known kind validate OK.
        let tasks = TaskList {
            tasks: vec![
                Task {
                    number: 1,
                    name: "Feature".to_string(),
                    closing_kind: None,
                },
                Task {
                    number: 7,
                    name: "Smoke test".to_string(),
                    closing_kind: Some("smoke-test".to_string()),
                },
            ],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "smoke-test".to_string(),
                checks: vec!["[execution] the flow runs".to_string()],
            }],
        };
        assert_eq!(validate_closing_kinds(&tasks, &fragment), Ok(()));
    }

    #[test]
    fn validate_closing_kinds_rejects_a_kind_absent_from_the_fragment() {
        let tasks = TaskList {
            tasks: vec![Task {
                number: 7,
                name: "Smoke test".to_string(),
                closing_kind: Some("bogus".to_string()),
            }],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "smoke-test".to_string(),
                checks: vec!["[execution] the flow runs".to_string()],
            }],
        };
        assert_eq!(
            validate_closing_kinds(&tasks, &fragment),
            Err(RenderError::UnknownClosingKind("bogus".to_string()))
        );
    }

    #[test]
    fn parse_fragment_rejects_an_unknown_verification_method() {
        let markdown = "\
### smoke-test
- [telepathy] verified by mind reading
";
        assert_eq!(
            parse_fragment(markdown),
            Err(RenderError::UnknownVerificationMethod(
                "telepathy".to_string()
            ))
        );
    }

    #[test]
    fn parse_tasks_handles_all_checkbox_prefixes_and_closing_kind() {
        let markdown = "\
- [x] Task 1: Done feature
  Depends on: none

- [~] Task 6: In-progress feature
  Depends on: Task 1

- [ ] Task 7: Smoke test
  Depends on: Task 6
  Closing: smoke-test

  End-to-end smoke test of the primary flow.
";
        let tasks = parse_tasks(markdown).expect("valid tasks parse");
        assert_eq!(tasks.tasks.len(), 3);

        assert_eq!(tasks.tasks[0].number, 1);
        assert_eq!(tasks.tasks[0].name, "Done feature");
        assert_eq!(tasks.tasks[0].closing_kind, None);

        assert_eq!(tasks.tasks[1].number, 6);
        assert_eq!(tasks.tasks[1].name, "In-progress feature");
        assert_eq!(tasks.tasks[1].closing_kind, None);

        assert_eq!(tasks.tasks[2].number, 7);
        assert_eq!(tasks.tasks[2].name, "Smoke test");
        assert_eq!(
            tasks.tasks[2].closing_kind,
            Some("smoke-test".to_string()),
            "the Closing: field attaches to its own task, not an earlier one"
        );
    }

    const ONE_ENTRY: &str = "\
#### Scenario: Greets a caller
<!-- spec-ref: 1 -->
- **WHEN** a caller greets
- **THEN** the widget greets back

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0
";

    #[test]
    fn parses_a_single_acceptance_entry() {
        let scenarios = parse_specs("widget", ONE_ENTRY).expect("valid spec parses");
        assert_eq!(scenarios.len(), 1);
        let scenario = &scenarios[0];
        assert_eq!(
            scenario.spec_ref,
            Some(SpecReference {
                capability: "widget".to_string(),
                number: 1,
            })
        );
        assert_eq!(scenario.status, None);
        assert_eq!(scenario.acceptance.len(), 1);
        let entry = &scenario.acceptance[0];
        assert_eq!(entry.task_link, 2);
        assert_eq!(entry.method, VerificationMethod::Execution);
        assert_eq!(
            entry.statement,
            "`widget greet` prints a greeting and exits 0"
        );
    }

    #[test]
    fn parses_multiple_entries_with_independent_links_and_methods() {
        let markdown = "\
#### Scenario: Does several things
<!-- spec-ref: 4 -->
<!-- status: deviated -->
- **WHEN** invoked
- **THEN** it does several things

**Acceptance:**
- [task:2] [execution] first check runs
- [task:3] [output] second check inspects output
";
        let scenarios = parse_specs("widget", markdown).expect("valid spec parses");
        let scenario = &scenarios[0];
        assert_eq!(scenario.status, Some(ScenarioStatus::Deviated));
        assert_eq!(scenario.acceptance.len(), 2);
        assert_eq!(scenario.acceptance[0].task_link, 2);
        assert_eq!(scenario.acceptance[0].method, VerificationMethod::Execution);
        assert_eq!(scenario.acceptance[1].task_link, 3);
        assert_eq!(scenario.acceptance[1].method, VerificationMethod::Output);
        // Both entries inherit the scenario's Spec Reference.
        assert_eq!(scenario.spec_ref.as_ref().map(|r| r.number), Some(4));
    }

    #[test]
    fn blank_line_after_the_marker_then_entries_are_parsed() {
        // A blank line between **Acceptance:** and the first entry must NOT be
        // read as an empty acceptance list.
        let markdown = "\
#### Scenario: Blank after the marker
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** it works

**Acceptance:**

- [task:1] [execution] the check runs
";
        let scenarios = parse_specs("widget", markdown).expect("blank after marker is valid");
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].acceptance.len(), 1);
        assert_eq!(scenarios[0].acceptance[0].task_link, 1);
    }

    #[test]
    fn blank_lines_between_entries_do_not_drop_any() {
        // The key fail-loud fix: a blank line between entries must not silently
        // drop the entries that follow it.
        let markdown = "\
#### Scenario: Blank-separated entries
<!-- spec-ref: 2 -->
- **WHEN** invoked
- **THEN** it works

**Acceptance:**
- [task:1] [execution] first check

- [task:2] [output] second check
";
        let scenarios = parse_specs("widget", markdown).expect("blank-separated entries are valid");
        assert_eq!(
            scenarios[0].acceptance.len(),
            2,
            "no entry is dropped after a blank line"
        );
        assert_eq!(scenarios[0].acceptance[0].task_link, 1);
        assert_eq!(scenarios[0].acceptance[1].task_link, 2);
    }

    #[test]
    fn non_entry_prose_between_entries_is_skipped_not_terminating() {
        // Trailing/interleaved non-bullet prose is skipped; entries before and
        // after such prose are all kept.
        let markdown = "\
#### Scenario: Entries with interleaved prose
<!-- spec-ref: 3 -->
- **WHEN** invoked
- **THEN** it works

**Acceptance:**
- [task:1] [execution] first check

A clarifying note that is not a bullet.
- [task:2] [output] second check
";
        let scenarios = parse_specs("widget", markdown).expect("prose between entries is skipped");
        assert_eq!(scenarios[0].acceptance.len(), 2);
        assert_eq!(scenarios[0].acceptance[1].task_link, 2);
    }

    #[test]
    fn acceptance_less_scenario_is_retained_with_title_and_behavior() {
        // Stage 2: a scenario with no **Acceptance:** block is no longer dropped.
        // It is retained so coverage can see it, carrying its title, its verbatim
        // WHEN/THEN behavior bullets, no Spec Reference, and no entries.
        let markdown = "\
#### Scenario: Just a plain scenario
- **WHEN** something happens
- **THEN** it is observed
";
        let scenarios = parse_specs("widget", markdown).expect("acceptance-less scenario is valid");
        assert_eq!(
            scenarios.len(),
            1,
            "an acceptance-less scenario is retained"
        );
        let scenario = &scenarios[0];
        assert_eq!(scenario.title, "Just a plain scenario");
        assert_eq!(
            scenario.behavior,
            vec![
                "- **WHEN** something happens".to_string(),
                "- **THEN** it is observed".to_string(),
            ],
            "the verbatim WHEN/THEN bullets are captured in order"
        );
        assert_eq!(
            scenario.spec_ref, None,
            "an acceptance-less scenario without a spec-ref comment carries no Spec Reference"
        );
        assert!(
            scenario.acceptance.is_empty(),
            "an acceptance-less scenario contributes no entries"
        );
    }

    #[test]
    fn behavior_text_includes_and_excludes_comments_and_stops_at_acceptance() {
        // Behavior text is the verbatim WHEN/THEN/AND bullets before
        // **Acceptance:**, with the spec-ref/status comments and blank lines
        // excluded, and nothing captured from the acceptance block.
        let markdown = "\
#### Scenario: A richer behavior
<!-- spec-ref: 2 -->
<!-- status: deviated -->
- **WHEN** invoked
- **AND** a flag is set
- **THEN** it does the thing

**Acceptance:**
- [task:3] [execution] the check runs
";
        let scenarios = parse_specs("widget", markdown).expect("valid spec parses");
        let scenario = &scenarios[0];
        assert_eq!(scenario.title, "A richer behavior");
        assert_eq!(
            scenario.behavior,
            vec![
                "- **WHEN** invoked".to_string(),
                "- **AND** a flag is set".to_string(),
                "- **THEN** it does the thing".to_string(),
            ],
            "AND is captured, comments and blanks excluded, acceptance bullets excluded"
        );
        assert_eq!(
            scenario.spec_ref.as_ref().map(|r| r.number),
            Some(2),
            "an acceptance-carrying scenario carries Some(spec_ref)"
        );
    }

    #[test]
    fn empty_statement_fails_loud_naming_the_offender() {
        let markdown = "\
#### Scenario: Has an empty statement
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [execution]
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::EmptyStatement("widget/1".to_string()))
        );
    }

    #[test]
    fn whitespace_only_statement_fails_loud_naming_the_offender() {
        // A statement that is only whitespace is as empty as a missing one,
        // and the error must still name the offending scenario.
        let markdown = "\
#### Scenario: Has a whitespace-only statement
<!-- spec-ref: 5 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [execution]    \t
";
        let err = parse_specs("widget", markdown).expect_err("whitespace statement fails loud");
        assert_eq!(err, RenderError::EmptyStatement("widget/5".to_string()));
        assert!(
            err.to_string().contains("widget/5"),
            "the error message must name the offending scenario, got: {err}"
        );
    }

    #[test]
    fn structurally_malformed_entry_is_not_reported_as_empty_statement() {
        // A non-numeric task link is a structurally broken entry, not an empty
        // statement: it must fail loud with an accurate, non-empty-statement
        // error that names the offending line.
        let markdown = "\
#### Scenario: Has a malformed entry
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:two] [execution] a real-looking statement
";
        let err = parse_specs("widget", markdown).expect_err("malformed entry fails loud");
        assert!(
            !matches!(err, RenderError::EmptyStatement(_)),
            "a non-empty malformed entry must not be reported as an empty statement, got: {err:?}"
        );
        assert_eq!(
            err,
            RenderError::UnknownVerificationMethod(
                "[task:two] [execution] a real-looking statement".to_string()
            )
        );
    }

    #[test]
    fn entry_missing_method_bracket_is_not_reported_as_empty_statement() {
        // Missing the [<method>] bracket is structural breakage, not an empty
        // statement.
        let markdown = "\
#### Scenario: Missing method bracket
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] no method bracket here
";
        let err = parse_specs("widget", markdown).expect_err("malformed entry fails loud");
        assert!(
            !matches!(err, RenderError::EmptyStatement(_)),
            "a non-empty malformed entry must not be reported as an empty statement, got: {err:?}"
        );
        assert_eq!(
            err,
            RenderError::UnknownVerificationMethod("[task:2] no method bracket here".to_string())
        );
    }

    #[test]
    fn present_but_empty_acceptance_list_fails_loud() {
        let markdown = "\
#### Scenario: Empty acceptance block
<!-- spec-ref: 7 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**

#### Scenario: Next
- **WHEN** x
- **THEN** y
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::EmptyAcceptanceList("widget/7".to_string()))
        );
    }

    #[test]
    fn acceptance_without_spec_ref_fails_loud() {
        let markdown = "\
#### Scenario: Acceptance but no ref
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [execution] a real check
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::MissingSpecRef)
        );
    }

    #[test]
    fn duplicate_spec_ref_within_capability_fails_loud() {
        let markdown = "\
#### Scenario: First
<!-- spec-ref: 3 -->
- **WHEN** a
- **THEN** b

**Acceptance:**
- [task:2] [execution] first check

#### Scenario: Second
<!-- spec-ref: 3 -->
- **WHEN** c
- **THEN** d

**Acceptance:**
- [task:2] [execution] second check
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::DuplicateSpecReference("widget/3".to_string()))
        );
    }

    #[test]
    fn unknown_method_fails_loud() {
        let markdown = "\
#### Scenario: Bad method
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [telepathy] a check by mind reading
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::UnknownVerificationMethod(
                "telepathy".to_string()
            ))
        );
    }

    #[test]
    fn unknown_status_fails_loud() {
        let markdown = "\
#### Scenario: Bad status
<!-- spec-ref: 1 -->
<!-- status: pondered -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [execution] a check
";
        assert_eq!(
            parse_specs("widget", markdown),
            Err(RenderError::UnknownStatus("pondered".to_string()))
        );
    }

    #[test]
    fn two_entries_round_trip_for_the_renderer() {
        // Downstream integration check from the brief: task links 2 and 3
        // survive so the renderer can place each check under its task.
        let markdown = "\
#### Scenario: Round trip
<!-- spec-ref: 1 -->
- **WHEN** invoked
- **THEN** done

**Acceptance:**
- [task:2] [execution] check for task two
- [task:3] [inspection] check for task three
";
        let scenarios = parse_specs("widget", markdown).expect("valid spec parses");
        let links: Vec<u32> = scenarios[0]
            .acceptance
            .iter()
            .map(|entry| entry.task_link)
            .collect();
        assert_eq!(links, vec![2, 3]);
        assert_eq!(
            scenarios[0].acceptance[1].method,
            VerificationMethod::Inspection
        );
    }
}
