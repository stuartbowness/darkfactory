//! `openspec validate` compatibility test (design D3).
//!
//! The acceptance block authored under a `#### Scenario:` heading is extra
//! content that `openspec validate` must ignore. This test stands up a minimal
//! OpenSpec project, drops a spec carrying an acceptance block into it, and
//! asserts `openspec validate` exits 0. The same fixture also round-trips
//! through `parse_specs`, proving the authored acceptance is both
//! validate-safe and parseable.

use std::fs;
use std::path::Path;
use std::process::Command;

use darkfactory_render::core::parse::parse_specs;

/// The fixture spec carrying an acceptance block (committed under the crate).
const ACCEPTANCE_SPEC: &str = include_str!("fixtures/acceptance_spec.md");

/// Whether `openspec` is resolvable on PATH in this environment.
fn openspec_available() -> bool {
    Command::new("openspec")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Write `contents` to `path`, creating parent directories.
fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, contents).expect("write file");
}

#[test]
fn acceptance_carrying_spec_passes_openspec_validate() {
    if !openspec_available() {
        // Env limitation, not a code defect: skip loudly via the test name.
        // The fixture is still exercised by `parse_specs` below in CI envs
        // where openspec is present. See the handoff for why this can occur.
        eprintln!("openspec not found on PATH; skipping validate-compatibility assertion");
        return;
    }

    let project = tempdir();
    let openspec_root = project.join("openspec");

    // Minimal OpenSpec project: a config and one spec carrying acceptance.
    write_file(&openspec_root.join("config.yaml"), "schema: spec-driven\n");
    write_file(&openspec_root.join("specs/widget/spec.md"), ACCEPTANCE_SPEC);

    let output = Command::new("openspec")
        .arg("validate")
        .arg("widget")
        .arg("--type")
        .arg("spec")
        .current_dir(&project)
        .output()
        .expect("run openspec validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "openspec validate must exit 0 on an acceptance-carrying spec.\n\
         status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        stdout,
        stderr,
    );

    fs::remove_dir_all(&project).ok();
}

#[test]
fn fixture_spec_parses_through_parse_specs() {
    // The same validate-safe fixture must parse into typed scenarios.
    let scenarios = parse_specs("widget", ACCEPTANCE_SPEC).expect("fixture spec parses");
    assert_eq!(scenarios.len(), 1);
    assert_eq!(scenarios[0].spec_ref.number, 1);
    assert_eq!(scenarios[0].acceptance.len(), 2);
    let links: Vec<u32> = scenarios[0]
        .acceptance
        .iter()
        .map(|entry| entry.task_link)
        .collect();
    assert_eq!(links, vec![2, 3]);
}

/// Create a unique temp directory for one test run.
fn tempdir() -> std::path::PathBuf {
    let base = std::env::temp_dir();
    let unique = format!(
        "darkfactory-render-openspec-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    );
    let dir = base.join(unique);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}
