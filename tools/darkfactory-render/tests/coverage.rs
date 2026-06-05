//! End-to-end CLI tests for the `coverage` subcommand (Task 4).
//!
//! These drive the REAL built binary via `std::process::Command`, located with
//! `env!("CARGO_BIN_EXE_darkfactory-render")` (no `assert_cmd`). They prove the
//! `coverage` subcommand reads the specs directory and tasks file, runs the
//! `parse_specs -> parse_tasks -> check_duplicate_tasks -> analyze ->
//! render_report` pipeline, writes the Gap report to stdout, and maps the result
//! to the documented exit codes: `0` (no gaps), `3` (gaps found), `2`
//! (usage/parse error).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The CLI fixture root committed under the crate.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli")
}

/// The absolute path to a committed CLI fixture file or directory.
fn fixture(rel: &str) -> String {
    fixtures().join(rel).to_string_lossy().into_owned()
}

/// The absolute path to a file or directory under the migrated fixture tree.
fn migrated(rel: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/migrated")
        .join(rel)
        .to_string_lossy()
        .into_owned()
}

/// Run the built binary with `args` and return its captured output.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_darkfactory-render"))
        .args(args)
        .output()
        .expect("spawn darkfactory-render")
}

/// The exit code of a finished process (the test only runs on Unix CI/dev).
fn code(output: &Output) -> i32 {
    output.status.code().expect("process returned an exit code")
}

#[test]
fn coverage_with_no_gaps_exits_zero_and_prints_the_report() {
    // The `covered_specs` fixture's lone scenario links to feature Task 2.
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("covered_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        0,
        "a fully covered spec set must exit 0, stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.starts_with("# Coverage Gap Report\n"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("1 scenarios, 1 covered, 0 gaps\n"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("None.\n"), "stdout:\n{stdout}");
}

#[test]
fn coverage_with_gaps_exits_three_and_prints_the_gap_on_stdout() {
    // The `gap_specs` fixture has an acceptance-less scenario: a NoAcceptance gap.
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("gap_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        3,
        "a gaps-found run must exit 3 (distinct from usage/parse 2 and shadow 1), stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("## Gaps\n"),
        "the report lands on stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("- Gap kind: no acceptance\n"),
        "the gap kind must be reported:\n{stdout}"
    );
    assert!(
        !stdout.contains("None."),
        "a gaps-found run must not print the no-gaps body:\n{stdout}"
    );
}

#[test]
fn a_dangling_acceptance_link_is_a_gap_not_a_parse_error() {
    // The `bad_specs` fixture links an acceptance entry to task 99, which is not
    // in the tasks file. For `render` this is a fatal RenderError; for
    // `coverage` it is a tolerable DanglingTaskLink GAP (exit 3, not 2).
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("bad_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        3,
        "a dangling link is a gap (exit 3), not a parse error (exit 2), stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("- Gap kind: dangling task link\n"),
        "the dangling link must surface as a gap kind:\n{stdout}"
    );
}

#[test]
fn coverage_is_byte_identical_across_two_runs() {
    // Full-pipeline determinism: two runs on the same inputs must produce
    // byte-identical stdout, so fixtures and CI diffs stay reliable.
    let args = [
        "coverage",
        "--specs",
        &fixture("gap_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ];
    let first = run(&args);
    let second = run(&args);
    assert_eq!(code(&first), code(&second), "exit codes must match");
    assert_eq!(
        first.stdout, second.stdout,
        "coverage stdout must be byte-identical across runs"
    );
}

#[test]
fn coverage_on_the_migrated_fixture_reports_a_stable_gap_count() {
    // Stage 1's migrated fixture is already in the acceptance-field format. Its
    // `domain-type-boundary` capability has three acceptance-less scenarios that
    // are NoAcceptance gaps; every other scenario links to a feature task. The
    // count is pinned so a regression in parsing or analysis trips this test.
    let output = run(&[
        "coverage",
        "--specs",
        &migrated("specs"),
        "--tasks",
        &migrated("tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        3,
        "the migrated fixture has gaps, so it must exit 3, stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("11 scenarios, 8 covered, 3 gaps\n"),
        "the migrated fixture gap count is pinned at 3, got:\n{stdout}"
    );
    // All three gaps are in the domain-type-boundary capability.
    assert_eq!(
        stdout.matches("### domain-type-boundary").count(),
        3,
        "all three gaps belong to domain-type-boundary, got:\n{stdout}"
    );
}

#[test]
fn missing_specs_flag_is_a_usage_error() {
    let output = run(&["coverage", "--tasks", &fixture("covered_tasks.md")]);
    assert_eq!(code(&output), 2, "a missing --specs flag is a usage error");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("missing required flag `--specs`"),
        "stderr:\n{stderr}"
    );
    assert!(stderr.contains("usage:"), "usage must be shown:\n{stderr}");
}

#[test]
fn missing_tasks_flag_is_a_usage_error() {
    let output = run(&["coverage", "--specs", &fixture("covered_specs")]);
    assert_eq!(code(&output), 2, "a missing --tasks flag is a usage error");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("missing required flag `--tasks`"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn an_unreadable_specs_path_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("does_not_exist"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(code(&output), 2, "an unreadable --specs path exits 2");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.to_lowercase().contains("specs"),
        "the error must name the unreadable specs path:\n{stderr}"
    );
    assert!(
        output.stdout.is_empty(),
        "no report may be printed on a read error"
    );
}

#[test]
fn a_missing_tasks_path_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("covered_specs"),
        "--tasks",
        &fixture("no_such_tasks.md"),
    ]);
    assert_eq!(code(&output), 2, "a missing --tasks path exits 2");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.to_lowercase().contains("cannot read"),
        "the error must report the unreadable tasks path:\n{stderr}"
    );
}

#[test]
fn an_unrecognised_verification_method_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("bad_method_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(code(&output), 2, "an unknown method is a parse error");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("unknown verification method"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn a_malformed_task_link_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("malformed_link_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(code(&output), 2, "a malformed task link is a parse error");
    assert!(
        output.stdout.is_empty(),
        "no report may be printed on a parse error"
    );
}

#[test]
fn a_duplicate_spec_reference_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("dup_specref_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        2,
        "a duplicate Spec Reference within a capability is a parse error"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("duplicate spec reference"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn an_acceptance_carrying_scenario_with_no_spec_ref_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("missing_specref_specs"),
        "--tasks",
        &fixture("covered_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        2,
        "an acceptance-carrying scenario with no Spec Reference is a parse error"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("missing a spec reference"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn a_duplicate_task_number_is_a_parse_error() {
    let output = run(&[
        "coverage",
        "--specs",
        &fixture("covered_specs"),
        "--tasks",
        &fixture("dup_task_number_tasks.md"),
    ]);
    assert_eq!(
        code(&output),
        2,
        "a duplicate task number is a parse error (exit 2), not a gap"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("duplicate task number"),
        "stderr:\n{stderr}"
    );
}
