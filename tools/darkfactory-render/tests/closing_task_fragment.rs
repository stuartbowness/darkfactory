//! Integration test: the committed standing closing-task fragment parses into
//! the five mandatory kinds. This pins the on-disk `templates/` file to the
//! contract the renderer (Task 4) and CLI (Task 5) depend on.

use darkfactory_render::core::parse::{parse_fragment, parse_tasks, validate_closing_kinds};

/// The committed standing fragment (read at compile time from `templates/`).
const CLOSING_TASK_CHECKS: &str = include_str!("../templates/closing-task-checks.md");

/// A real `tasks.md` (copied from this change) carrying feature and closing
/// tasks tagged with all five mandatory kinds.
const TASKS_SAMPLE: &str = include_str!("fixtures/tasks_sample.md");

#[test]
fn committed_fragment_declares_the_five_mandatory_kinds() {
    let fragment = parse_fragment(CLOSING_TASK_CHECKS).expect("committed fragment parses");

    let names: Vec<&str> = fragment
        .kinds
        .iter()
        .map(|kind| kind.name.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            "smoke-test",
            "cleanup",
            "module-shape",
            "claude-arch",
            "documentation",
        ],
        "the standing fragment must declare exactly the five mandatory closing-task kinds"
    );

    for kind in &fragment.kinds {
        assert!(
            !kind.checks.is_empty(),
            "kind `{}` must declare at least one check",
            kind.name
        );
    }
}

#[test]
fn real_tasks_validate_against_the_committed_fragment() {
    let tasks = parse_tasks(TASKS_SAMPLE).expect("real tasks.md parses");
    let fragment = parse_fragment(CLOSING_TASK_CHECKS).expect("committed fragment parses");

    // The sample carries 11 tasks; the last five are closing tasks tagged
    // smoke-test, cleanup, module-shape, claude-arch, documentation.
    assert_eq!(tasks.tasks.len(), 11);
    assert_eq!(tasks.tasks[0].number, 1);
    assert_eq!(tasks.tasks[6].closing_kind, Some("smoke-test".to_string()));
    assert_eq!(
        tasks.tasks[10].closing_kind,
        Some("documentation".to_string())
    );

    // Every closing kind in the real list is known to the committed fragment.
    assert_eq!(validate_closing_kinds(&tasks, &fragment), Ok(()));
}
