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
    AcceptanceEntry, RenderError, RequirementScenario, ScenarioStatus, SpecReference,
    VerificationMethod,
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
        // Acceptance-less scenarios yield `None` and carry no Spec Reference,
        // so duplicate detection only ever sees acceptance-carrying scenarios.
        if let Some(scenario) = parse_scenario(capability, block)? {
            let number = scenario.spec_ref.number;
            if !seen_refs.insert(number) {
                return Err(RenderError::DuplicateSpecReference(format!(
                    "{capability}/{number}"
                )));
            }
            scenarios.push(scenario);
        }
    }

    Ok(scenarios)
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
/// Returns `Ok(None)` for an acceptance-less scenario (it contributes no
/// entries and is not required to carry a Spec Reference).
fn parse_scenario(
    capability: &str,
    lines: Vec<&str>,
) -> Result<Option<RequirementScenario>, RenderError> {
    let title = scenario_title(lines[0]);

    let mut spec_ref_number: Option<u32> = None;
    let mut status_token: Option<&str> = None;
    let mut acceptance_start: Option<usize> = None;

    for (index, raw) in lines.iter().enumerate().skip(1) {
        let line = raw.trim();
        if let Some(rest) = comment_value(line, SPEC_REF_PREFIX) {
            spec_ref_number = Some(parse_spec_ref_number(rest)?);
        } else if let Some(rest) = comment_value(line, STATUS_PREFIX) {
            status_token = Some(rest);
        } else if line == ACCEPTANCE_MARKER {
            acceptance_start = Some(index + 1);
            break;
        }
    }

    let status = ScenarioStatus::parse_optional(status_token)?;

    let Some(acceptance_start) = acceptance_start else {
        // No acceptance block: a valid scenario that contributes no entries.
        return Ok(None);
    };

    let identifier = scenario_identifier(capability, spec_ref_number, title);
    let acceptance = parse_acceptance(&lines[acceptance_start..], &identifier)?;
    if acceptance.is_empty() {
        return Err(RenderError::EmptyAcceptanceList(identifier));
    }

    let number = spec_ref_number.ok_or(RenderError::MissingSpecRef)?;

    Ok(Some(RequirementScenario {
        spec_ref: SpecReference {
            capability: capability.to_string(),
            number,
        },
        status,
        acceptance,
    }))
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
/// Stops at the first blank line or next heading; non-entry lines inside the
/// block are ignored so trailing prose does not break parsing.
fn parse_acceptance(lines: &[&str], identifier: &str) -> Result<Vec<AcceptanceEntry>, RenderError> {
    let mut entries = Vec::new();
    for raw in lines {
        let line = raw.trim();
        if line.is_empty() {
            break;
        }
        if line.starts_with("#### ") || line.starts_with("### ") {
            break;
        }
        if !line.starts_with("- ") {
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
            SpecReference {
                capability: "widget".to_string(),
                number: 1,
            }
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
        assert_eq!(scenario.spec_ref.number, 4);
    }

    #[test]
    fn acceptance_less_scenario_parses_and_contributes_no_entries() {
        let markdown = "\
#### Scenario: Just a plain scenario
- **WHEN** something happens
- **THEN** it is observed
";
        let scenarios = parse_specs("widget", markdown).expect("acceptance-less scenario is valid");
        assert!(
            scenarios.is_empty(),
            "a scenario with no acceptance contributes no entries"
        );
    }

    #[test]
    fn acceptance_less_scenario_does_not_require_a_spec_ref() {
        // No spec-ref comment and no acceptance: must NOT be MissingSpecRef.
        let markdown = "\
#### Scenario: No ref, no acceptance
- **WHEN** something happens
- **THEN** it is observed
";
        let result = parse_specs("widget", markdown);
        assert_eq!(result, Ok(Vec::new()));
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
