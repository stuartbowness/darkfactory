//! Contract rendering for `darkfactory-render` (design D2/D5/D6/D7).
//!
//! This is the deep core: [`build_model`] assembles the three parsed inputs
//! (spec scenarios, the task list, and the closing-task fragment) into a typed
//! [`ContractModel`], and [`render`] projects that model into the deterministic
//! `contract.md` body. The grouping by task, the Spec Reference comment grammar
//! (dedupe / order / `(DEVIATED)`), check formatting, the closing-kind ->
//! fragment join, and the inspection-cap warning all live behind these two
//! calls. The renderer carries authored check statements verbatim and never
//! invents or rewords a check.
//!
//! It is fail-loud: a dangling task link, an acceptance entry attached to a
//! closing task, a feature task with zero checks, an unknown closing kind, and a
//! duplicate spec reference each surface as a typed [`RenderError`] with no
//! partial contract emitted. The inspection-cap breach is the one non-fatal
//! signal: it is reported through [`RenderedContract::warnings`] while the render
//! still succeeds.

use crate::core::parse::validate_closing_kinds;
use crate::core::types::{
    ClosingTaskFragment, RenderError, RenderedContract, RequirementScenario, SpecReference, Task,
    TaskList, VerificationMethod,
};

/// The contract title line.
const TITLE: &str = "# Contract";

/// Strictly-greater-than threshold for the inspection-cap warning: a task whose
/// `[inspection]` checks exceed 20% of its checks is flagged.
const INSPECTION_CAP_PERCENT: u32 = 20;

/// The assembled, fully ordered contract ready to render.
///
/// `build_model` produces this; `render` projects it to markdown. It holds one
/// [`TaskSection`] per task in task-list order; each section already carries its
/// deduped/ordered Spec Reference comment (feature tasks only) and its check
/// lines in render order, so `render` is a pure string join with no remaining
/// decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractModel {
    /// The task sections in task-list order.
    sections: Vec<TaskSection>,
}

/// One rendered task section: its heading parts, an optional Specs comment, and
/// its ordered check lines.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskSection {
    /// The task number for the `## Task <N>:` heading.
    number: u32,
    /// The task name for the heading.
    name: String,
    /// The `<!-- Specs: ... -->` comment body for a feature task, or `None` for
    /// a closing task (closing tasks emit no Specs comment).
    specs_comment: Option<String>,
    /// The check lines, each already formatted as `[<method>] <statement>`.
    checks: Vec<CheckLine>,
}

/// One rendered check: its verbatim `[<method>] <statement>` text plus the
/// verification method (kept so the inspection-cap warning can be computed
/// without re-parsing the text).
#[derive(Debug, Clone, PartialEq, Eq)]
struct CheckLine {
    /// The verbatim `[<method>] <statement>` body (no `- [ ] ` prefix).
    body: String,
    /// The verification method of this check.
    method: VerificationMethod,
}

/// Assemble the typed [`ContractModel`] from the three parsed inputs.
///
/// `scenarios` are the spec scenarios (source order from `parse_specs`), `tasks`
/// is the task list (render order), and `fragment` supplies the closing-task
/// checks. This is where grouping by task, the Spec Reference comment grammar,
/// the closing-kind -> fragment join, deterministic ordering, and all fail-loud
/// validation happen, so [`render`] is a pure projection.
///
/// # Errors
///
/// Returns a [`RenderError`] with no partial model on any malformed case:
/// [`DanglingTaskLink`](RenderError::DanglingTaskLink),
/// [`AcceptanceLinksClosingTask`](RenderError::AcceptanceLinksClosingTask),
/// [`FeatureTaskNoChecks`](RenderError::FeatureTaskNoChecks), or
/// [`UnknownClosingKind`](RenderError::UnknownClosingKind). A
/// [`DuplicateSpecReference`](RenderError::DuplicateSpecReference) is detected
/// upstream by `parse_specs` and surfaces through the full pipeline.
pub fn build_model(
    scenarios: &[RequirementScenario],
    tasks: &TaskList,
    fragment: &ClosingTaskFragment,
) -> Result<ContractModel, RenderError> {
    // Design D5: an unknown closing kind fails loud in the render pipeline.
    validate_closing_kinds(tasks, fragment)?;

    // Design D2/D6: every acceptance entry attaches to the task named by its
    // task_link. Validate the link target before grouping, so a dangling or
    // closing-task link fails loud rather than silently dropping the check.
    for placed in placed_entries(scenarios) {
        let Some(task) = task_by_number(tasks, placed.task_link) else {
            return Err(RenderError::DanglingTaskLink(placed.task_link));
        };
        if task.closing_kind.is_some() {
            return Err(RenderError::AcceptanceLinksClosingTask(placed.task_link));
        }
    }

    let mut sections = Vec::with_capacity(tasks.tasks.len());
    for task in &tasks.tasks {
        let section = match &task.closing_kind {
            Some(kind) => closing_section(task, kind, fragment),
            None => feature_section(task, scenarios)?,
        };
        sections.push(section);
    }

    Ok(ContractModel { sections })
}

/// Project a built [`ContractModel`] into the deterministic `contract.md` body
/// and its non-fatal warnings.
///
/// Output is byte-identical across repeat renders: the model is fully ordered by
/// `build_model`, so this is a pure string join with no nondeterministic source.
/// The body lives in [`RenderedContract::body`]; the inspection-cap warnings live
/// in [`RenderedContract::warnings`].
pub fn render(model: &ContractModel) -> Result<RenderedContract, RenderError> {
    let mut body = String::from(TITLE);
    body.push('\n');
    for section in &model.sections {
        body.push('\n');
        body.push_str(&format!("## Task {}: {}\n", section.number, section.name));
        if let Some(comment) = &section.specs_comment {
            body.push_str(&format!("<!-- Specs: {comment} -->\n"));
        }
        for check in &section.checks {
            body.push_str(&format!("- [ ] {}\n", check.body));
        }
    }

    let warnings = model
        .sections
        .iter()
        .filter_map(inspection_warning)
        .collect();

    Ok(RenderedContract { body, warnings })
}

/// One acceptance entry paired with the source position that breaks Spec
/// Reference ties (scenario index, then entry index within the scenario).
struct PlacedEntry<'a> {
    spec_ref: &'a SpecReference,
    deviated: bool,
    statement: &'a str,
    method: VerificationMethod,
    task_link: u32,
    scenario_index: usize,
    entry_index: usize,
}

/// Flatten every scenario's acceptance entries into placeable units, recording
/// each entry's source position (scenario order, then entry order) for the
/// deterministic tie-break (design D7).
fn placed_entries(scenarios: &[RequirementScenario]) -> Vec<PlacedEntry<'_>> {
    let mut placed = Vec::new();
    for (scenario_index, scenario) in scenarios.iter().enumerate() {
        let deviated = scenario
            .status
            .map(|status| status.is_deviated())
            .unwrap_or(false);
        for (entry_index, entry) in scenario.acceptance.iter().enumerate() {
            placed.push(PlacedEntry {
                spec_ref: &scenario.spec_ref,
                deviated,
                statement: &entry.statement,
                method: entry.method,
                task_link: entry.task_link,
                scenario_index,
                entry_index,
            });
        }
    }
    placed
}

/// The task with the given number, or `None`.
fn task_by_number(tasks: &TaskList, number: u32) -> Option<&Task> {
    tasks.tasks.iter().find(|task| task.number == number)
}

/// Build a feature task's section: its checks gathered from acceptance entries
/// linking to it, ordered by Spec Reference then source order, plus the one
/// deduped/ordered Specs comment.
///
/// # Errors
///
/// [`FeatureTaskNoChecks`](RenderError::FeatureTaskNoChecks) when no acceptance
/// entry links to this feature task (design D6).
fn feature_section(
    task: &Task,
    scenarios: &[RequirementScenario],
) -> Result<TaskSection, RenderError> {
    let mut placed: Vec<PlacedEntry<'_>> = placed_entries(scenarios)
        .into_iter()
        .filter(|entry| entry.task_link == task.number)
        .collect();

    if placed.is_empty() {
        return Err(RenderError::FeatureTaskNoChecks(task.number));
    }

    // Design D7: order by Spec Reference (capability asc, then N asc), ties
    // broken by source order (scenario index, then entry index). A stable sort
    // on the source-ordered vec keeps equal keys in source order.
    placed.sort_by(|a, b| {
        spec_ref_key(a.spec_ref)
            .cmp(&spec_ref_key(b.spec_ref))
            .then(a.scenario_index.cmp(&b.scenario_index))
            .then(a.entry_index.cmp(&b.entry_index))
    });

    let specs_comment = Some(specs_comment(&placed));
    let checks = placed
        .iter()
        .map(|entry| CheckLine {
            body: format!("[{}] {}", method_token(entry.method), entry.statement),
            method: entry.method,
        })
        .collect();

    Ok(TaskSection {
        number: task.number,
        name: task.name.clone(),
        specs_comment,
        checks,
    })
}

/// Build a closing task's section: the fragment's checks for its kind, in
/// fragment order, with no Specs comment (design D5).
fn closing_section(task: &Task, kind: &str, fragment: &ClosingTaskFragment) -> TaskSection {
    let checks = fragment
        .kinds
        .iter()
        .find(|known| known.name == kind)
        .map(|known| {
            known
                .checks
                .iter()
                .map(|check| CheckLine {
                    body: check.clone(),
                    method: fragment_check_method(check),
                })
                .collect()
        })
        .unwrap_or_default();

    TaskSection {
        number: task.number,
        name: task.name.clone(),
        specs_comment: None,
        checks,
    }
}

/// The deduped, ordered `<!-- Specs: ... -->` body for a feature task's checks.
///
/// `placed` is already sorted by Spec Reference then source order, so a single
/// pass that skips a ref equal to the previous one yields the dedupe in the
/// correct order. A ref whose scenario is `deviated` gets ` (DEVIATED)`.
fn specs_comment(placed: &[PlacedEntry<'_>]) -> String {
    let mut refs: Vec<String> = Vec::new();
    let mut last_key: Option<(String, u32)> = None;
    for entry in placed {
        let key = spec_ref_key(entry.spec_ref);
        if last_key.as_ref() == Some(&key) {
            continue;
        }
        let mut text = format!("{}/{}", entry.spec_ref.capability, entry.spec_ref.number);
        if entry.deviated {
            text.push_str(" (DEVIATED)");
        }
        refs.push(text);
        last_key = Some(key);
    }
    refs.join(", ")
}

/// The sort key for a Spec Reference: capability ascending, then N ascending.
fn spec_ref_key(spec_ref: &SpecReference) -> (String, u32) {
    (spec_ref.capability.clone(), spec_ref.number)
}

/// The bare method token (`execution` / `output` / `inspection`).
fn method_token(method: VerificationMethod) -> &'static str {
    match method {
        VerificationMethod::Execution => "execution",
        VerificationMethod::Output => "output",
        VerificationMethod::Inspection => "inspection",
    }
}

/// The verification method named by a verbatim fragment check's `[<method>]`
/// tag. The fragment was validated by `parse_fragment`, so the tag is always a
/// known method; an unexpected shape falls back to a non-inspection method
/// rather than panicking (it cannot inflate the inspection-cap count).
fn fragment_check_method(check: &str) -> VerificationMethod {
    let token = check
        .strip_prefix('[')
        .and_then(|rest| rest.split(']').next())
        .unwrap_or("");
    VerificationMethod::parse(token).unwrap_or(VerificationMethod::Execution)
}

/// The inspection-cap warning for a section, or `None` when it is at or under
/// the 20% cap.
///
/// The ratio is computed with integer math: `inspection * 100 > total * 20`
/// avoids floating point so the boundary is exact. A section with no checks
/// never warns.
fn inspection_warning(section: &TaskSection) -> Option<String> {
    let total = section.checks.len() as u32;
    if total == 0 {
        return None;
    }
    let inspection = section
        .checks
        .iter()
        .filter(|check| check.method == VerificationMethod::Inspection)
        .count() as u32;
    if inspection * 100 > total * INSPECTION_CAP_PERCENT {
        let percent = inspection * 100 / total;
        Some(format!(
            "Task {}: {}% of checks are [inspection] (exceeds {}% cap)",
            section.number, percent, INSPECTION_CAP_PERCENT
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{
        AcceptanceEntry, ClosingTaskKind, RequirementScenario, ScenarioStatus, SpecReference, Task,
        TaskList, VerificationMethod,
    };

    fn scenario(
        capability: &str,
        number: u32,
        status: Option<ScenarioStatus>,
        acceptance: Vec<AcceptanceEntry>,
    ) -> RequirementScenario {
        RequirementScenario {
            spec_ref: SpecReference {
                capability: capability.to_string(),
                number,
            },
            status,
            acceptance,
        }
    }

    fn entry(statement: &str, method: VerificationMethod, task_link: u32) -> AcceptanceEntry {
        AcceptanceEntry {
            statement: statement.to_string(),
            method,
            task_link,
        }
    }

    fn feature(number: u32, name: &str) -> Task {
        Task {
            number,
            name: name.to_string(),
            closing_kind: None,
        }
    }

    fn closing(number: u32, name: &str, kind: &str) -> Task {
        Task {
            number,
            name: name.to_string(),
            closing_kind: Some(kind.to_string()),
        }
    }

    fn render_to_markdown(
        scenarios: &[RequirementScenario],
        tasks: &TaskList,
        fragment: &ClosingTaskFragment,
    ) -> RenderedContract {
        let model = build_model(scenarios, tasks, fragment).expect("model builds");
        render(&model).expect("render succeeds")
    }

    #[test]
    fn a_feature_task_check_renders_under_its_task_with_a_specs_comment() {
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry(
                "`widget greet` prints a greeting and exits 0",
                VerificationMethod::Execution,
                1,
            )],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Greet")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.to_markdown(),
            "\
# Contract

## Task 1: Greet
<!-- Specs: widget/1 -->
- [ ] [execution] `widget greet` prints a greeting and exits 0
"
        );
    }

    #[test]
    fn feature_checks_order_by_spec_reference_then_source_order() {
        // Source order places capability `b`/5 FIRST and `a`/3 second (with two
        // entries). Render order must instead be a/3's two entries (a < b), with
        // their relative order kept by the source-order tie-break, then b/5. This
        // proves the Spec Reference sort drives placement, not source order.
        let scenarios = vec![
            scenario(
                "b",
                5,
                None,
                vec![entry("b5 only", VerificationMethod::Execution, 1)],
            ),
            scenario(
                "a",
                3,
                None,
                vec![
                    entry("a3 first", VerificationMethod::Execution, 1),
                    entry("a3 second", VerificationMethod::Output, 1),
                ],
            ),
        ];
        let tasks = TaskList {
            tasks: vec![feature(1, "Mixed")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.to_markdown(),
            "\
# Contract

## Task 1: Mixed
<!-- Specs: a/3, b/5 -->
- [ ] [execution] a3 first
- [ ] [output] a3 second
- [ ] [execution] b5 only
"
        );
    }

    #[test]
    fn a_deviated_scenarios_ref_is_marked_and_deduped_once() {
        // One deviated scenario with two entries on the same task: the ref must
        // appear exactly once, carrying (DEVIATED), and both checks render.
        let scenarios = vec![scenario(
            "acceptance-field",
            4,
            Some(ScenarioStatus::Deviated),
            vec![
                entry("first deviated check", VerificationMethod::Execution, 9),
                entry("second deviated check", VerificationMethod::Output, 9),
            ],
        )];
        let tasks = TaskList {
            tasks: vec![feature(9, "Acceptance field")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.to_markdown(),
            "\
# Contract

## Task 9: Acceptance field
<!-- Specs: acceptance-field/4 (DEVIATED) -->
- [ ] [execution] first deviated check
- [ ] [output] second deviated check
"
        );
    }

    #[test]
    fn a_closing_task_renders_fragment_checks_with_no_specs_comment() {
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("a feature check", VerificationMethod::Execution, 1)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Feature"), closing(2, "Cleanup", "cleanup")],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "cleanup".to_string(),
                checks: vec![
                    "[execution] the linter is clean".to_string(),
                    "[inspection] no leftover scaffolding remains".to_string(),
                ],
            }],
        };

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.to_markdown(),
            "\
# Contract

## Task 1: Feature
<!-- Specs: widget/1 -->
- [ ] [execution] a feature check

## Task 2: Cleanup
- [ ] [execution] the linter is clean
- [ ] [inspection] no leftover scaffolding remains
"
        );
    }

    #[test]
    fn repeat_render_is_byte_identical() {
        let scenarios = vec![
            scenario(
                "beta",
                2,
                None,
                vec![entry("beta check", VerificationMethod::Execution, 1)],
            ),
            scenario(
                "alpha",
                7,
                None,
                vec![entry("alpha check", VerificationMethod::Output, 1)],
            ),
            scenario(
                "alpha",
                1,
                Some(ScenarioStatus::Deviated),
                vec![entry("alpha one", VerificationMethod::Inspection, 1)],
            ),
        ];
        let tasks = TaskList {
            tasks: vec![feature(1, "Many refs"), closing(2, "Smoke", "smoke-test")],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "smoke-test".to_string(),
                checks: vec!["[execution] the flow runs".to_string()],
            }],
        };

        let first = render_to_markdown(&scenarios, &tasks, &fragment);
        let second = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            first.to_markdown().as_bytes(),
            second.to_markdown().as_bytes(),
            "rendering the same inputs twice must be byte-identical"
        );
        // And the model itself is stable across builds.
        let model_a = build_model(&scenarios, &tasks, &fragment).expect("model builds");
        let model_b = build_model(&scenarios, &tasks, &fragment).expect("model builds");
        assert_eq!(model_a, model_b);
    }

    #[test]
    fn tasks_render_in_task_list_order_not_link_order() {
        // Scenarios link to task 3 then task 1, but task-list order is 1 then 3.
        let scenarios = vec![
            scenario(
                "widget",
                3,
                None,
                vec![entry("for task three", VerificationMethod::Execution, 3)],
            ),
            scenario(
                "widget",
                1,
                None,
                vec![entry("for task one", VerificationMethod::Execution, 1)],
            ),
        ];
        let tasks = TaskList {
            tasks: vec![feature(1, "First"), feature(3, "Third")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.to_markdown(),
            "\
# Contract

## Task 1: First
<!-- Specs: widget/1 -->
- [ ] [execution] for task one

## Task 3: Third
<!-- Specs: widget/3 -->
- [ ] [execution] for task three
"
        );
    }

    #[test]
    fn a_dangling_task_link_fails_loud() {
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("orphan check", VerificationMethod::Execution, 99)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Only task")],
        };
        let fragment = ClosingTaskFragment::default();

        assert_eq!(
            build_model(&scenarios, &tasks, &fragment),
            Err(RenderError::DanglingTaskLink(99))
        );
    }

    #[test]
    fn an_acceptance_entry_linking_to_a_closing_task_fails_loud() {
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("misattached check", VerificationMethod::Execution, 2)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Feature"), closing(2, "Cleanup", "cleanup")],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "cleanup".to_string(),
                checks: vec!["[execution] the linter is clean".to_string()],
            }],
        };

        assert_eq!(
            build_model(&scenarios, &tasks, &fragment),
            Err(RenderError::AcceptanceLinksClosingTask(2))
        );
    }

    #[test]
    fn a_feature_task_with_no_checks_fails_loud() {
        // Task 2 is a feature task that no acceptance entry links to.
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("for task one", VerificationMethod::Execution, 1)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Has checks"), feature(2, "Empty")],
        };
        let fragment = ClosingTaskFragment::default();

        assert_eq!(
            build_model(&scenarios, &tasks, &fragment),
            Err(RenderError::FeatureTaskNoChecks(2))
        );
    }

    #[test]
    fn an_unknown_closing_kind_fails_loud_in_the_pipeline() {
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("a feature check", VerificationMethod::Execution, 1)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Feature"), closing(2, "Mystery", "bogus-kind")],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "cleanup".to_string(),
                checks: vec!["[execution] the linter is clean".to_string()],
            }],
        };

        assert_eq!(
            build_model(&scenarios, &tasks, &fragment),
            Err(RenderError::UnknownClosingKind("bogus-kind".to_string()))
        );
    }

    #[test]
    fn a_task_over_the_inspection_cap_warns() {
        // Task 9: 1 of 2 checks are [inspection] = 50% > 20% -> warns. Render
        // still succeeds (non-fatal).
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![
                entry("an execution check", VerificationMethod::Execution, 9),
                entry("an inspection check", VerificationMethod::Inspection, 9),
            ],
        )];
        let tasks = TaskList {
            tasks: vec![feature(9, "Inspection heavy")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.warnings(),
            ["Task 9: 50% of checks are [inspection] (exceeds 20% cap)".to_string()]
        );
    }

    #[test]
    fn a_task_at_the_inspection_cap_does_not_warn() {
        // Task 5: exactly 1 of 5 checks are [inspection] = 20%, which is NOT
        // strictly greater than 20% -> no warning.
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![
                entry("c1", VerificationMethod::Execution, 5),
                entry("c2", VerificationMethod::Execution, 5),
                entry("c3", VerificationMethod::Output, 5),
                entry("c4", VerificationMethod::Output, 5),
                entry("c5", VerificationMethod::Inspection, 5),
            ],
        )];
        let tasks = TaskList {
            tasks: vec![feature(5, "At the cap")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert!(
            rendered.warnings().is_empty(),
            "exactly 20% inspection must not warn, got: {:?}",
            rendered.warnings()
        );
    }

    #[test]
    fn a_closing_tasks_inspection_cap_reads_the_fragment_method_tag() {
        // The closing task's checks come from the fragment as verbatim
        // `[method] statement` strings; the cap must read the tag, not a typed
        // method field. 2 of 2 inspection = 100% -> warns for the closing task.
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry("a feature check", VerificationMethod::Execution, 1)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Feature"), closing(8, "Docs", "documentation")],
        };
        let fragment = ClosingTaskFragment {
            kinds: vec![ClosingTaskKind {
                name: "documentation".to_string(),
                checks: vec![
                    "[inspection] documentation exists".to_string(),
                    "[inspection] the README is accurate".to_string(),
                ],
            }],
        };

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert_eq!(
            rendered.warnings(),
            ["Task 8: 100% of checks are [inspection] (exceeds 20% cap)".to_string()]
        );
    }

    #[test]
    fn statements_render_verbatim_without_rewording() {
        // A statement carrying backticks, brackets, and a percent sign must
        // survive the render character-for-character.
        let statement = "`render --shadow [x]` exits 0 when 100% of checks pass";
        let scenarios = vec![scenario(
            "widget",
            1,
            None,
            vec![entry(statement, VerificationMethod::Output, 1)],
        )];
        let tasks = TaskList {
            tasks: vec![feature(1, "Verbatim")],
        };
        let fragment = ClosingTaskFragment::default();

        let rendered = render_to_markdown(&scenarios, &tasks, &fragment);
        assert!(
            rendered
                .to_markdown()
                .contains(&format!("- [ ] [output] {statement}\n")),
            "the statement must render verbatim, got:\n{}",
            rendered.to_markdown()
        );
    }
}
