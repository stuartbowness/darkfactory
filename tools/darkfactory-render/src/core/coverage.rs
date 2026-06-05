//! Tolerant coverage analysis for `darkfactory-render`.
//!
//! [`analyze`] is the tolerant counterpart to the strict
//! [`build_model`](crate::core::render::build_model): where `build_model` fails
//! loud on a dangling or closing-task link, `analyze` records it as a
//! [`GapKind`]. It classifies which spec scenarios have no task covering them
//! and is **infallible** — unresolved links become gap kinds, never errors, so
//! it can run on incomplete planning input.
//!
//! Malformed *input* (a duplicate task number) is a separate concern handled by
//! [`check_duplicate_tasks`], which the CLI calls before analyzing. It is kept
//! out of `parse_tasks` so `render`/`shadow` stay permissive and unchanged.

use crate::core::types::{
    CoverageGap, CoverageReport, GapKind, RenderError, RequirementScenario, TaskList,
};

/// Classify which spec scenarios have no task covering them.
///
/// A scenario is a **gap** when NONE of its acceptance entries links to a valid
/// feature task (a [`Task`](crate::core::types::Task) whose `closing_kind ==
/// None`). A scenario with at
/// least one entry resolving to a feature task is **covered**, even if a sibling
/// entry has an unresolved link.
///
/// This is **infallible**: unresolved links are recorded as [`GapKind`]s, never
/// returned as errors.
pub fn analyze(scenarios: &[RequirementScenario], tasks: &TaskList) -> CoverageReport {
    let total = scenarios.len();
    let mut gaps: Vec<(usize, CoverageGap)> = Vec::new();

    for (position, scenario) in scenarios.iter().enumerate() {
        if let Some(kinds) = classify(scenario, tasks) {
            gaps.push((
                position,
                CoverageGap {
                    capability: scenario.capability.clone(),
                    spec_ref: scenario.spec_ref.clone(),
                    title: scenario.title.clone(),
                    behavior: scenario.behavior.clone(),
                    kinds,
                },
            ));
        }
    }

    // Order by capability name, then by document position within that capability
    // (which is the source order from `parse_specs`). `position` is already the
    // global source index, so a stable sort by capability preserves it.
    gaps.sort_by(|a, b| a.1.capability.cmp(&b.1.capability).then(a.0.cmp(&b.0)));

    let covered = total - gaps.len();
    CoverageReport {
        gaps: gaps.into_iter().map(|(_, gap)| gap).collect(),
        total,
        covered,
    }
}

/// Classify a scenario, returning its distinct gap kinds when it is a gap, or
/// `None` when it is covered.
///
/// A scenario is covered when at least one acceptance entry resolves to a
/// feature task. Otherwise it is a gap, and the returned kinds are the distinct
/// reasons present, in the fixed order `NoAcceptance`, `DanglingTaskLink`,
/// `ClosingTaskLink`.
fn classify(scenario: &RequirementScenario, tasks: &TaskList) -> Option<Vec<GapKind>> {
    if scenario.acceptance.is_empty() {
        return Some(vec![GapKind::NoAcceptance]);
    }

    let mut dangling = false;
    let mut closing = false;
    for entry in &scenario.acceptance {
        match tasks
            .tasks
            .iter()
            .find(|task| task.number == entry.task_link)
        {
            None => dangling = true,
            Some(task) if task.closing_kind.is_some() => closing = true,
            Some(_) => return None, // resolves to a feature task: covered.
        }
    }

    let mut kinds = Vec::new();
    if dangling {
        kinds.push(GapKind::DanglingTaskLink);
    }
    if closing {
        kinds.push(GapKind::ClosingTaskLink);
    }
    Some(kinds)
}

/// Project a [`CoverageReport`] into the canonical Gap report markdown.
///
/// A pure formatter: `CoverageReport -> String`, no I/O and no errors, mirroring
/// how [`render`](crate::core::render::render) projects a contract model to a
/// string. The output is deterministic and byte-identical so fixtures and diffs
/// stay reliable.
///
/// Layout: a `# Coverage Gap Report` title, a `## Summary` line
/// `<total> scenarios, <covered> covered, <gaps> gaps`, then a `## Gaps`
/// section. With no gaps the section body is the single line `None.`; otherwise
/// one block per gap, separated by a blank line. Each block has an
/// `### <identifier> — <title>` heading (the em dash is U+2014), a
/// `- Gap kind: <kinds>` line, and a `- Behavior:` line followed by each stored
/// behavior bullet indented two spaces.
pub fn render_report(report: &CoverageReport) -> String {
    let mut out = String::from("# Coverage Gap Report\n");
    out.push('\n');
    out.push_str("## Summary\n");
    out.push_str(&format!(
        "{} scenarios, {} covered, {} gaps\n",
        report.total,
        report.covered,
        report.gaps.len()
    ));
    out.push('\n');
    out.push_str("## Gaps\n");

    if report.gaps.is_empty() {
        out.push('\n');
        out.push_str("None.\n");
        return out;
    }

    for gap in &report.gaps {
        out.push('\n');
        out.push_str(&format!("### {} — {}\n", gap_identifier(gap), gap.title));
        out.push_str(&format!("- Gap kind: {}\n", gap_kinds(&gap.kinds)));
        out.push_str("- Behavior:\n");
        for line in &gap.behavior {
            out.push_str(&format!("  {line}\n"));
        }
    }

    out
}

/// The `### ` heading identifier for a gap: `<capability>/<N>` when the gap
/// carries a Spec Reference, otherwise the bare `<capability>`.
fn gap_identifier(gap: &CoverageGap) -> String {
    match &gap.spec_ref {
        Some(spec_ref) => format!("{}/{}", gap.capability, spec_ref.number),
        None => gap.capability.clone(),
    }
}

/// Render the gap's distinct kinds as comma-separated labels, in stored order.
///
/// Labels map exactly: `NoAcceptance` -> `no acceptance`,
/// `DanglingTaskLink` -> `dangling task link`,
/// `ClosingTaskLink` -> `closing-task link`.
fn gap_kinds(kinds: &[GapKind]) -> String {
    kinds
        .iter()
        .map(|kind| match kind {
            GapKind::NoAcceptance => "no acceptance",
            GapKind::DanglingTaskLink => "dangling task link",
            GapKind::ClosingTaskLink => "closing-task link",
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Report a duplicate task number as a fail-loud error.
///
/// Used by the CLI path (Task 4) before [`analyze`]. Kept out of `parse_tasks`
/// so render/shadow stay permissive.
///
/// # Errors
///
/// [`DuplicateTaskNumber`](RenderError::DuplicateTaskNumber) when two tasks
/// share a number.
pub fn check_duplicate_tasks(tasks: &TaskList) -> Result<(), RenderError> {
    let mut seen: Vec<u32> = Vec::with_capacity(tasks.tasks.len());
    for task in &tasks.tasks {
        if seen.contains(&task.number) {
            return Err(RenderError::DuplicateTaskNumber(
                task.number,
                task.name.clone(),
            ));
        }
        seen.push(task.number);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse::parse_specs;
    use crate::core::types::{AcceptanceEntry, SpecReference, Task, VerificationMethod};

    fn entry(task_link: u32) -> AcceptanceEntry {
        AcceptanceEntry {
            statement: "check".to_string(),
            method: VerificationMethod::Execution,
            task_link,
        }
    }

    fn scenario(
        capability: &str,
        number: u32,
        acceptance: Vec<AcceptanceEntry>,
    ) -> RequirementScenario {
        RequirementScenario {
            title: format!("{capability} scenario {number}"),
            capability: capability.to_string(),
            behavior: vec![format!("WHEN {capability}")],
            spec_ref: Some(SpecReference {
                capability: capability.to_string(),
                number,
            }),
            status: None,
            acceptance,
        }
    }

    fn feature(number: u32) -> Task {
        Task {
            number,
            name: format!("feature {number}"),
            closing_kind: None,
        }
    }

    fn closing(number: u32, kind: &str) -> Task {
        Task {
            number,
            name: format!("closing {number}"),
            closing_kind: Some(kind.to_string()),
        }
    }

    fn tasks(tasks: Vec<Task>) -> TaskList {
        TaskList { tasks }
    }

    #[test]
    fn scenario_with_no_acceptance_entries_is_a_no_acceptance_gap() {
        let scenarios = vec![scenario("add-todo", 1, vec![])];
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        assert_eq!(report.total, 1);
        assert_eq!(report.covered, 0);
        assert_eq!(report.gaps.len(), 1);
        assert_eq!(report.gaps[0].kinds, vec![GapKind::NoAcceptance]);
        assert_eq!(report.gaps[0].capability, "add-todo");
        assert_eq!(report.gaps[0].title, "add-todo scenario 1");
    }

    #[test]
    fn scenario_linking_only_to_a_missing_task_is_a_dangling_gap() {
        let scenarios = vec![scenario("add-todo", 1, vec![entry(99)])];
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        assert_eq!(report.gaps.len(), 1);
        assert_eq!(report.gaps[0].kinds, vec![GapKind::DanglingTaskLink]);
        assert_eq!(report.covered, 0);
    }

    #[test]
    fn scenario_linking_only_to_a_closing_task_is_a_closing_gap() {
        let scenarios = vec![scenario("add-todo", 1, vec![entry(2)])];
        let report = analyze(
            &scenarios,
            &tasks(vec![feature(1), closing(2, "smoke-test")]),
        );

        assert_eq!(report.gaps.len(), 1);
        assert_eq!(report.gaps[0].kinds, vec![GapKind::ClosingTaskLink]);
        assert_eq!(report.covered, 0);
    }

    #[test]
    fn one_valid_feature_link_covers_the_scenario_despite_an_unresolved_sibling() {
        // First entry dangles (task 99 missing); second resolves to feature 1.
        let scenarios = vec![scenario("add-todo", 1, vec![entry(99), entry(1)])];
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        assert!(report.gaps.is_empty(), "a valid feature link covers it");
        assert_eq!(report.total, 1);
        assert_eq!(report.covered, 1);
    }

    #[test]
    fn distinct_kinds_are_listed_once_each_in_fixed_order() {
        // Two dangling entries and one closing entry: DanglingTaskLink and
        // ClosingTaskLink each appear once, dangling before closing.
        let scenarios = vec![scenario(
            "add-todo",
            1,
            vec![entry(99), entry(98), entry(2)],
        )];
        let report = analyze(
            &scenarios,
            &tasks(vec![feature(1), closing(2, "smoke-test")]),
        );

        assert_eq!(report.gaps.len(), 1);
        assert_eq!(
            report.gaps[0].kinds,
            vec![GapKind::DanglingTaskLink, GapKind::ClosingTaskLink]
        );
    }

    #[test]
    fn gaps_are_ordered_by_capability_then_document_position() {
        // Document order: beta/1, alpha/2, alpha/1. Expected gap order:
        // alpha/2, alpha/1 (document position within capability), then beta/1.
        let scenarios = vec![
            scenario("beta", 1, vec![]),
            scenario("alpha", 2, vec![]),
            scenario("alpha", 1, vec![]),
        ];
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        let order: Vec<_> = report
            .gaps
            .iter()
            .map(|gap| {
                let spec = gap.spec_ref.as_ref().expect("spec-ref present");
                (gap.capability.clone(), spec.number)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                ("alpha".to_string(), 2),
                ("alpha".to_string(), 1),
                ("beta".to_string(), 1),
            ]
        );
    }

    #[test]
    fn acceptance_less_scenario_with_a_spec_ref_is_a_no_acceptance_gap_carrying_its_ref() {
        // A scenario with a `<!-- spec-ref: N -->` but no acceptance entries:
        // a NoAcceptance gap identified by its Spec Reference, not just title.
        let scenarios = vec![RequirementScenario {
            title: "untitled".to_string(),
            capability: "add-todo".to_string(),
            behavior: vec![],
            spec_ref: Some(SpecReference {
                capability: "add-todo".to_string(),
                number: 7,
            }),
            status: None,
            acceptance: vec![],
        }];
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        assert_eq!(report.gaps.len(), 1);
        assert_eq!(report.gaps[0].kinds, vec![GapKind::NoAcceptance]);
        let spec = report.gaps[0].spec_ref.as_ref().expect("spec-ref retained");
        assert_eq!(spec.capability, "add-todo");
        assert_eq!(spec.number, 7);
    }

    #[test]
    fn acceptance_less_scenario_without_spec_ref_gaps_by_its_capability() {
        // The common gap case: a scenario with WHEN/THEN jotted but no
        // acceptance and no `<!-- spec-ref: N -->`. It parses to
        // `spec_ref: None`, yet its gap must still identify by its real
        // capability (the spec directory name), not the empty string.
        let markdown = "\
#### Scenario: A plain jotted scenario
- **WHEN** something happens
- **THEN** it is observed
";
        let scenarios = parse_specs("widget", markdown).expect("acceptance-less scenario parses");
        let report = analyze(&scenarios, &tasks(vec![feature(1)]));

        assert_eq!(report.gaps.len(), 1);
        let gap = &report.gaps[0];
        assert_eq!(gap.kinds, vec![GapKind::NoAcceptance]);
        assert_eq!(gap.title, "A plain jotted scenario");
        assert_eq!(
            gap.capability, "widget",
            "a no-spec-ref gap must identify by its capability, not the empty string"
        );
        assert!(
            gap.spec_ref.is_none(),
            "the scenario carries no spec-ref, so the gap carries none either"
        );
    }

    fn gap(
        capability: &str,
        number: Option<u32>,
        title: &str,
        behavior: Vec<&str>,
        kinds: Vec<GapKind>,
    ) -> CoverageGap {
        CoverageGap {
            capability: capability.to_string(),
            spec_ref: number.map(|number| SpecReference {
                capability: capability.to_string(),
                number,
            }),
            title: title.to_string(),
            behavior: behavior.into_iter().map(str::to_string).collect(),
            kinds,
        }
    }

    #[test]
    fn render_report_emits_the_canonical_gaps_layout() {
        let report = CoverageReport {
            gaps: vec![
                gap(
                    "gadget",
                    Some(1),
                    "Sync gadget",
                    vec!["- **WHEN** a gadget is edited", "- **THEN** it syncs"],
                    vec![GapKind::DanglingTaskLink],
                ),
                gap(
                    "widget",
                    Some(2),
                    "Delete widget",
                    vec![
                        "- **WHEN** the user deletes a widget",
                        "- **THEN** it is removed",
                    ],
                    vec![GapKind::NoAcceptance],
                ),
            ],
            total: 3,
            covered: 1,
        };

        let expected = "\
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
";

        assert_eq!(render_report(&report), expected);
    }

    #[test]
    fn render_report_emits_the_canonical_no_gaps_layout() {
        let report = CoverageReport {
            gaps: vec![],
            total: 3,
            covered: 3,
        };

        let expected = "\
# Coverage Gap Report

## Summary
3 scenarios, 3 covered, 0 gaps

## Gaps

None.
";

        assert_eq!(render_report(&report), expected);
    }

    #[test]
    fn render_report_uses_bare_capability_when_a_gap_has_no_spec_ref() {
        let report = CoverageReport {
            gaps: vec![gap(
                "widget",
                None,
                "A plain jotted scenario",
                vec!["- **WHEN** something happens"],
                vec![GapKind::NoAcceptance],
            )],
            total: 1,
            covered: 0,
        };

        let rendered = render_report(&report);
        assert!(
            rendered.contains("### widget — A plain jotted scenario\n"),
            "a no-spec-ref gap heading must be the bare capability, got:\n{rendered}"
        );
    }

    #[test]
    fn render_report_lists_multiple_kinds_comma_separated_in_stored_order() {
        let report = CoverageReport {
            gaps: vec![gap(
                "gadget",
                Some(1),
                "Mixed gap",
                vec!["- **WHEN** x"],
                vec![GapKind::DanglingTaskLink, GapKind::ClosingTaskLink],
            )],
            total: 1,
            covered: 0,
        };

        assert!(
            render_report(&report).contains("- Gap kind: dangling task link, closing-task link\n"),
            "distinct kinds render comma-separated in stored order with exact labels"
        );
    }

    #[test]
    fn check_duplicate_tasks_accepts_distinct_numbers() {
        let list = tasks(vec![feature(1), feature(2), closing(3, "smoke-test")]);
        assert_eq!(check_duplicate_tasks(&list), Ok(()));
    }

    #[test]
    fn check_duplicate_tasks_rejects_a_shared_number() {
        let list = tasks(vec![feature(1), feature(1)]);
        let err = check_duplicate_tasks(&list).expect_err("duplicate number is an error");
        assert_eq!(
            err,
            RenderError::DuplicateTaskNumber(1, "feature 1".to_string())
        );
    }
}
