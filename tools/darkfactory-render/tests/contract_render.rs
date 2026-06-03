//! Pipeline integration tests for the contract renderer (Task 4).
//!
//! These exercise the real `parse_* -> build_model -> render` path on in-memory
//! markdown, proving the deep core composes end to end and that upstream
//! fail-loud errors (here `DuplicateSpecReference`) surface through the full
//! pipeline rather than being swallowed by the renderer.

use darkfactory_render::core::parse::{parse_fragment, parse_specs, parse_tasks};
use darkfactory_render::core::render::{build_model, render};
use darkfactory_render::core::types::RenderError;

const SPEC: &str = "\
# widget spec

#### Scenario: Widget greets a caller
<!-- spec-ref: 1 -->
- **WHEN** asked to greet
- **THEN** it greets

**Acceptance:**
- [task:2] [execution] `widget greet` prints a greeting and exits 0
- [task:3] [output] the greeting output contains the caller's name
";

const TASKS: &str = "\
# Tasks

- [ ] Task 2: Greet command
  Depends on: none

- [ ] Task 3: Greeting output
  Depends on: Task 2

- [ ] Task 7: Smoke test
  Depends on: Task 3
  Closing: smoke-test
";

const FRAGMENT: &str = "\
### smoke-test
- [execution] the primary user flow runs end to end and passes
";

#[test]
fn parse_then_build_then_render_produces_the_expected_contract_shape() {
    let scenarios = parse_specs("widget", SPEC).expect("spec parses");
    let tasks = parse_tasks(TASKS).expect("tasks parse");
    let fragment = parse_fragment(FRAGMENT).expect("fragment parses");

    let model = build_model(&scenarios, &tasks, &fragment).expect("model builds");
    let rendered = render(&model).expect("render succeeds");
    let body = rendered.to_markdown();

    // The brief's minimum integration: the markdown carries the
    // `## Task N` / `<!-- Specs -->` / `- [ ] [method]` shape.
    assert!(body.starts_with("# Contract\n"), "body:\n{body}");
    assert!(body.contains("## Task 2: Greet command\n"), "body:\n{body}");
    assert!(body.contains("<!-- Specs: widget/1 -->\n"), "body:\n{body}");
    assert!(
        body.contains("- [ ] [execution] `widget greet` prints a greeting and exits 0\n"),
        "body:\n{body}"
    );
    assert!(
        body.contains("## Task 3: Greeting output\n"),
        "body:\n{body}"
    );
    assert!(
        body.contains("- [ ] [output] the greeting output contains the caller's name\n"),
        "body:\n{body}"
    );

    // The closing task carries its fragment check and NO Specs comment.
    assert!(body.contains("## Task 7: Smoke test\n"), "body:\n{body}");
    assert!(
        body.contains("- [ ] [execution] the primary user flow runs end to end and passes\n"),
        "body:\n{body}"
    );
    let smoke_section = body
        .split("## Task 7: Smoke test\n")
        .nth(1)
        .expect("smoke section present");
    assert!(
        !smoke_section.contains("<!-- Specs:"),
        "a closing task must carry no Specs comment, got:\n{smoke_section}"
    );

    // This balanced set (2 execution, 1 output) trips no inspection-cap warning.
    assert!(
        rendered.warnings().is_empty(),
        "warnings: {:?}",
        rendered.warnings()
    );
}

#[test]
fn duplicate_spec_reference_surfaces_through_the_full_pipeline() {
    // Two acceptance-carrying scenarios share `spec-ref: 1` within one
    // capability; `parse_specs` detects it and the error must reach the caller
    // unchanged through the render pipeline (the renderer does not re-detect it).
    let duped_spec = "\
#### Scenario: First
<!-- spec-ref: 1 -->
- **WHEN** a
- **THEN** b

**Acceptance:**
- [task:1] [execution] first check

#### Scenario: Second
<!-- spec-ref: 1 -->
- **WHEN** c
- **THEN** d

**Acceptance:**
- [task:1] [execution] second check
";

    let result = parse_specs("widget", duped_spec).and_then(|scenarios| {
        let tasks = parse_tasks("- [ ] Task 1: Feature\n").expect("tasks parse");
        let fragment = parse_fragment("").expect("empty fragment parses");
        let model = build_model(&scenarios, &tasks, &fragment)?;
        render(&model)
    });

    assert_eq!(
        result,
        Err(RenderError::DuplicateSpecReference("widget/1".to_string())),
        "a duplicate spec-ref must surface through parse -> build_model -> render"
    );
}
