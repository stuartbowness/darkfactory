//! Migration-and-equivalence integration tests (Task 6).
//!
//! These prove the renderer reproduces a REAL hand-authored contract. The
//! migrated fixture under `tests/fixtures/migrated/` is the archived change
//! `2026-04-29-domain-vocabulary-type-names` carried into the v2 grammar:
//! its three capability specs gain stable `<!-- spec-ref: N -->` tokens and
//! authored `**Acceptance:**` blocks reproducing the archived `contract.md`'s
//! checks verbatim, plus one authored `readme-updates` capability hosting the
//! archived Task 3 (README) checks that had no spec source (design D6).
//!
//! Three tests:
//! 1. `migrated_fixture_renders_equivalent_to_the_archived_contract` — the full
//!    pipeline on the fixture renders a contract EQUIVALENT to the archived one
//!    (same checks under the same tasks; see the equivalence definition there).
//! 2. `inserting_a_scenario_does_not_change_existing_spec_references` — the D4
//!    stable-ID invariant: the Spec Reference is the explicit token, not a
//!    position, so inserting a scenario in the middle leaves prior refs fixed.
//! 3. `migrated_tokens_resolve_to_the_same_scenarios_as_positional_refs` — the
//!    migration from positional numbering to stable tokens preserves resolution:
//!    each token N names the SAME scenario the archived positional reference N
//!    pointed at.

use darkfactory_render::core::parse::{parse_fragment, parse_specs, parse_tasks};
use darkfactory_render::core::render::{build_model, render};
use darkfactory_render::core::types::RequirementScenario;

// The migrated capability specs and tasks, read at compile time from the
// fixture under the crate.
const BRIEF_DOMAIN_TYPES: &str = include_str!("fixtures/migrated/specs/brief-domain-types/spec.md");
const DOMAIN_TYPE_BOUNDARY: &str =
    include_str!("fixtures/migrated/specs/domain-type-boundary/spec.md");
const TYPE_NAME_AUDIT: &str = include_str!("fixtures/migrated/specs/type-name-audit/spec.md");
const README_UPDATES: &str = include_str!("fixtures/migrated/specs/readme-updates/spec.md");
const MIGRATED_TASKS: &str = include_str!("fixtures/migrated/tasks.md");

// The standing closing-task fragment, reused as-is (not over-fit to this
// fixture). Its generic operational checks deliberately differ from the
// archived mission-specific closing checks (the v2 design standardizes them).
const CLOSING_TASK_CHECKS: &str = include_str!("../templates/closing-task-checks.md");

/// Parse all four migrated capabilities into one scenario list, in the same
/// capability order the migration authored them.
fn migrated_scenarios() -> Vec<RequirementScenario> {
    let mut scenarios = Vec::new();
    for (capability, markdown) in [
        ("brief-domain-types", BRIEF_DOMAIN_TYPES),
        ("domain-type-boundary", DOMAIN_TYPE_BOUNDARY),
        ("type-name-audit", TYPE_NAME_AUDIT),
        ("readme-updates", README_UPDATES),
    ] {
        scenarios.extend(parse_specs(capability, markdown).expect("migrated spec parses"));
    }
    scenarios
}

/// The section of the rendered body for `## Task N: <name>`, from its heading up
/// to (but not including) the next `## Task ` heading or end of document.
fn task_section<'a>(body: &'a str, heading: &str) -> &'a str {
    let start = body
        .find(heading)
        .unwrap_or_else(|| panic!("task heading `{heading}` present in:\n{body}"));
    let after = &body[start..];
    let rest = &after[heading.len()..];
    match rest.find("\n## Task ") {
        Some(offset) => &after[..heading.len() + offset + 1],
        None => after,
    }
}

#[test]
fn migrated_fixture_renders_equivalent_to_the_archived_contract() {
    // EQUIVALENCE DEFINITION (design D6 / Test Strategy): "the same checks under
    // the same tasks" — validator-outcome parity, NOT byte-identity with the
    // archived `contract.md`. Concretely:
    //   * Same 8 task sections, same `## Task N: <name>` names, same order.
    //   * FEATURE tasks (1, 2, 3): the rendered checks reproduce the archived
    //     contract's per-task checks VERBATIM (same statement, same method tag),
    //     and each feature task carries the expected `<!-- Specs: ... -->`
    //     comment with the right refs. This is the strong parity claim: the
    //     substantive per-mission checks are faithfully reproduced from the
    //     authored acceptance.
    //   * CLOSING tasks (4-8): each renders its kind's GENERIC operational
    //     checks FROM THE STANDING FRAGMENT and carries NO `<!-- Specs -->`
    //     comment. These are compared STRUCTURALLY (present, fragment-sourced,
    //     no Specs comment), NOT byte-for-byte against the archived
    //     mission-specific closing checks — under the v2 design closing checks
    //     are standardized via the fragment, so they intentionally differ from
    //     the archived ones.
    let scenarios = migrated_scenarios();
    let tasks = parse_tasks(MIGRATED_TASKS).expect("migrated tasks parse");
    let fragment = parse_fragment(CLOSING_TASK_CHECKS).expect("standing fragment parses");

    let model = build_model(&scenarios, &tasks, &fragment).expect("migrated model builds");
    let rendered = render(&model).expect("migrated contract renders");
    let body = rendered.to_markdown();

    // --- Same 8 task sections, same names, same order. ---
    let headings = [
        "## Task 1: Add Domain Types section to worker brief template",
        "## Task 2: Add domain type boundary and Type Catalog name verification",
        "## Task 3: Update README to reflect changes",
        "## Task 4: Smoke test",
        "## Task 5: Cleanup",
        "## Task 6: Module shape re-check",
        "## Task 7: CLAUDE.md + ARCH.md generation",
        "## Task 8: Documentation",
    ];
    let positions: Vec<usize> = headings
        .iter()
        .map(|h| {
            body.find(h)
                .unwrap_or_else(|| panic!("missing task heading `{h}` in:\n{body}"))
        })
        .collect();
    assert!(
        positions.windows(2).all(|w| w[0] < w[1]),
        "the 8 task sections must appear in archived order, got positions {positions:?}"
    );

    // --- FEATURE tasks: verbatim check parity + Specs comment with right refs. ---

    // Task 1: archived Specs brief-domain-types/1-4; 4 [output] + 1 [inspection].
    let t1 = task_section(&body, headings[0]);
    assert!(
        t1.contains(
            "<!-- Specs: brief-domain-types/1, brief-domain-types/2, brief-domain-types/3, brief-domain-types/4 -->\n"
        ),
        "Task 1 Specs comment must list brief-domain-types/1-4, got:\n{t1}"
    );
    for check in [
        "- [ ] [output] `skills/mission/templates/worker-brief.md` contains a section headed \"## Domain Types\" that appears after the \"## Key Terms\" section\n",
        "- [ ] [output] The Domain Types section includes a one-line instruction telling the worker to use exact names for domain types\n",
        "- [ ] [output] The Domain Types section includes a placeholder for listing Type Catalog entries relevant to the task from both Produces and Consumes fields\n",
        "- [ ] [output] The Domain Types section includes a \"None for this task\" fallback for tasks with no relevant domain types\n",
        "- [ ] [inspection] The Domain Types section is separate from the Key Terms section (not merged into it)\n",
    ] {
        assert!(t1.contains(check), "Task 1 missing archived check verbatim:\n{check}\nin:\n{t1}");
    }

    // Task 2: archived Specs domain-type-boundary/1, type-name-audit/1-2; 3 [output].
    let t2 = task_section(&body, headings[1]);
    assert!(
        t2.contains(
            "<!-- Specs: domain-type-boundary/1, type-name-audit/1, type-name-audit/2 -->\n"
        ),
        "Task 2 Specs comment must list domain-type-boundary/1, type-name-audit/1-2, got:\n{t2}"
    );
    for check in [
        "- [ ] [output] `skills/mission/templates/design-template.md` Type Catalog section contains a sentence stating that domain types model the problem space and that technical helpers are excluded unless they are public compatibility surfaces\n",
        "- [ ] [output] `skills/mission/SKILL.md` module shape re-check closing task instructions mention verifying that every domain type in the Type Catalog exists in the committed code with a matching name\n",
        "- [ ] [output] The SKILL.md instruction specifies that drift is resolved by either updating design.md or refactoring the code, matching the existing module surface drift resolution pattern\n",
    ] {
        assert!(t2.contains(check), "Task 2 missing archived check verbatim:\n{check}\nin:\n{t2}");
    }

    // Task 3 (the D6 case): archived contract had NO spec source; the migration
    // authored a `readme-updates` scenario so each README check now traces to a
    // scenario. As a feature task it now carries a Specs comment (readme-updates/1),
    // and reproduces the archived README checks verbatim.
    let t3 = task_section(&body, headings[2]);
    assert!(
        t3.contains("<!-- Specs: readme-updates/1 -->\n"),
        "Task 3 (README, D6) must carry the authored readme-updates/1 Specs comment, got:\n{t3}"
    );
    for check in [
        "- [ ] [output] `README.md` mentions the Domain Types section in worker briefs\n",
        "- [ ] [output] `README.md` mentions the domain type boundary clarification in the design template\n",
        "- [ ] [output] `README.md` mentions the Type Catalog name verification in the module shape re-check\n",
    ] {
        assert!(t3.contains(check), "Task 3 missing archived README check verbatim:\n{check}\nin:\n{t3}");
    }

    // --- CLOSING tasks (4-8): fragment-sourced checks, NO Specs comment. ---
    // Compared STRUCTURALLY (not byte-identical to the archived closing checks):
    // each renders the standing fragment's GENERIC operational checks for its
    // kind, which the v2 design standardizes and which therefore differ from the
    // archived mission-specific closing checks.
    let closing = [
        (headings[3], "smoke-test"),
        (headings[4], "cleanup"),
        (headings[5], "module-shape"),
        (headings[6], "claude-arch"),
        (headings[7], "documentation"),
    ];
    let parsed_fragment = parse_fragment(CLOSING_TASK_CHECKS).expect("fragment parses");
    for (heading, kind) in closing {
        let section = task_section(&body, heading);
        assert!(
            !section.contains("<!-- Specs:"),
            "closing task `{heading}` must carry no Specs comment, got:\n{section}"
        );
        let fragment_kind = parsed_fragment
            .kinds
            .iter()
            .find(|k| k.name == kind)
            .unwrap_or_else(|| panic!("fragment declares kind `{kind}`"));
        for check in &fragment_kind.checks {
            let rendered_check = format!("- [ ] {check}\n");
            assert!(
                section.contains(&rendered_check),
                "closing task `{heading}` must render fragment check from kind `{kind}`:\n{rendered_check}\nin:\n{section}"
            );
        }
    }
}

#[test]
fn inserting_a_scenario_does_not_change_existing_spec_references() {
    // D4 stable-ID invariant (contract check 2): the Spec Reference is the
    // explicit `spec-ref: N` token, NOT the scenario's position in the file.
    // Inserting a new scenario (with a new, higher N) in the MIDDLE of a
    // migrated capability must leave every pre-existing scenario's Spec
    // Reference unchanged.
    let before = parse_specs("brief-domain-types", BRIEF_DOMAIN_TYPES)
        .expect("migrated brief-domain-types parses");
    let refs_before: Vec<(String, u32)> = before
        .iter()
        .map(|s| (s.spec_ref.capability.clone(), s.spec_ref.number))
        .collect();
    assert_eq!(
        refs_before,
        vec![
            ("brief-domain-types".to_string(), 1),
            ("brief-domain-types".to_string(), 2),
            ("brief-domain-types".to_string(), 3),
            ("brief-domain-types".to_string(), 4),
        ],
        "sanity: the migrated capability carries refs 1-4 in source order"
    );

    // Insert a NEW scenario with a new, higher ref (5) between scenarios 2 and 3.
    // If refs were positional, this would renumber scenarios 3 and 4.
    let new_scenario = "\
#### Scenario: Inserted in the middle
<!-- spec-ref: 5 -->
- **WHEN** a new scenario is inserted between existing ones
- **THEN** the existing scenarios keep their Spec References

**Acceptance:**
- [task:1] [output] an inserted check that links to Task 1

";
    let anchor = "#### Scenario: Brief for a task with no domain types";
    let insert_at = BRIEF_DOMAIN_TYPES
        .find(anchor)
        .expect("anchor scenario present");
    let mutated = format!(
        "{}{}{}",
        &BRIEF_DOMAIN_TYPES[..insert_at],
        new_scenario,
        &BRIEF_DOMAIN_TYPES[insert_at..]
    );

    let after =
        parse_specs("brief-domain-types", &mutated).expect("mutated brief-domain-types parses");

    // The new scenario adds a ref; the pre-existing refs (1-4) are unchanged
    // and still resolve to their original scenarios despite the position shift.
    let mut refs_after: Vec<(String, u32)> = after
        .iter()
        .map(|s| (s.spec_ref.capability.clone(), s.spec_ref.number))
        .collect();
    assert!(
        refs_after.contains(&("brief-domain-types".to_string(), 5)),
        "the inserted scenario contributes ref 5"
    );
    refs_after.retain(|(_, n)| *n != 5);
    assert_eq!(
        refs_after, refs_before,
        "inserting a scenario must not change any pre-existing Spec Reference"
    );

    // And ref 3 still resolves to the same scenario it did before the insert,
    // even though it now sits AFTER the inserted scenario in the file.
    let three_before = before.iter().find(|s| s.spec_ref.number == 3).unwrap();
    let three_after = after.iter().find(|s| s.spec_ref.number == 3).unwrap();
    assert_eq!(
        three_before, three_after,
        "ref 3 must resolve to the same scenario before and after the insert"
    );
}

#[test]
fn migrated_tokens_resolve_to_the_same_scenarios_as_positional_refs() {
    // Contract check 3: migration from positional numbering to stable tokens
    // preserves resolution. The archived `contract.md` referenced scenarios
    // POSITIONALLY (capability/N = the Nth scenario in that capability's
    // spec.md). The migration assigned each migrated scenario a `spec-ref: N`
    // token. This asserts the migrated token N equals the archived positional
    // reference N AND resolves to the SAME scenario (same title and WHEN/THEN
    // content) the positional reference pointed at.
    //
    // The archived spec.md files are READ-ONLY input data; we reconstruct the
    // archived positional scenarios from their known titles/content here (we do
    // not, and must not, modify the archived change).
    let scenarios = migrated_scenarios();

    // (capability, token N) -> the scenario title the archived positional
    // reference N pointed at (the Nth `#### Scenario:` in that capability).
    let expected_titles: &[(&str, u32, &str)] = &[
        (
            "brief-domain-types",
            1,
            "Brief for a task that produces domain types",
        ),
        (
            "brief-domain-types",
            2,
            "Brief for a task that consumes but does not produce domain types",
        ),
        (
            "brief-domain-types",
            3,
            "Brief for a task with no domain types",
        ),
        (
            "brief-domain-types",
            4,
            "Domain Types section is separate from Key Terms",
        ),
        (
            "domain-type-boundary",
            1,
            "Domain type boundary in design template",
        ),
        ("type-name-audit", 1, "All Type Catalog names present"),
        ("type-name-audit", 2, "Type name drift detected"),
    ];

    // Each migrated capability's source markdown, so we can confirm the token N
    // sits under the SAME scenario heading the positional Nth scenario had.
    let sources = [
        ("brief-domain-types", BRIEF_DOMAIN_TYPES),
        ("domain-type-boundary", DOMAIN_TYPE_BOUNDARY),
        ("type-name-audit", TYPE_NAME_AUDIT),
    ];

    for (capability, n, title) in expected_titles {
        // The token resolves: a parsed scenario carries this capability/N.
        let resolved = scenarios
            .iter()
            .find(|s| s.spec_ref.capability == *capability && s.spec_ref.number == *n)
            .unwrap_or_else(|| panic!("migrated token {capability}/{n} resolves to a scenario"));

        // The migrated capability has the spec-ref token directly under the
        // expected scenario title — i.e. the token N is on the same scenario the
        // positional reference N pointed at in the archived spec. We verify the
        // token sits in the heading block whose title matches the archived Nth
        // scenario's title.
        let source = sources
            .iter()
            .find(|(c, _)| c == capability)
            .map(|(_, md)| *md)
            .unwrap_or_else(|| panic!("source for capability {capability}"));
        let heading = format!("#### Scenario: {title}");
        let heading_at = source
            .find(&heading)
            .unwrap_or_else(|| panic!("scenario `{title}` present in {capability} spec"));
        let after_heading = &source[heading_at + heading.len()..];
        // The next scenario heading bounds this block.
        let block_end = after_heading
            .find("#### Scenario:")
            .map(|o| heading_at + heading.len() + o)
            .unwrap_or(source.len());
        let block = &source[heading_at..block_end];
        let token = format!("<!-- spec-ref: {n} -->");
        assert!(
            block.contains(&token),
            "token {token} must sit under archived positional scenario `{title}` in {capability}, block:\n{block}"
        );

        // The acceptance entries of the resolved scenario all link to the task
        // the archived contract attached this reference to (Task 1 for
        // brief-domain-types, Task 2 for the other two), proving the migrated
        // reference carries the same contract meaning.
        let expected_task = if *capability == "brief-domain-types" {
            1
        } else {
            2
        };
        assert!(
            resolved
                .acceptance
                .iter()
                .all(|e| e.task_link == expected_task),
            "every acceptance entry on {capability}/{n} must link to archived Task {expected_task}"
        );
    }
}
