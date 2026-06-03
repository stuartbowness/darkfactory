//! End-to-end CLI tests for `darkfactory-render` (Task 5).
//!
//! These drive the REAL built binary via `std::process::Command`, located with
//! `env!("CARGO_BIN_EXE_darkfactory-render")` (no `assert_cmd`). They prove the
//! `render` and `shadow` subcommands do file IO, map results to the documented
//! exit codes and stderr, and that `shadow` is strictly read-only.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::SystemTime;

/// The fixture root committed under the crate.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli")
}

/// The absolute path to a committed fixture file or directory.
fn fixture(rel: &str) -> String {
    fixtures().join(rel).to_string_lossy().into_owned()
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
fn render_emits_the_contract_to_stdout_and_exits_zero() {
    let output = run(&[
        "render",
        "--specs",
        &fixture("specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
    ]);
    assert_eq!(code(&output), 0, "render must exit 0 on a valid input set");

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.starts_with("# Contract\n"), "stdout:\n{stdout}");
    assert!(
        stdout.contains("## Task 2: Greet command\n"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("<!-- Specs: widget/1 -->\n"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("- [ ] [execution] `widget greet` prints a greeting and exits 0\n"),
        "stdout:\n{stdout}"
    );
    // The closing task renders with no Specs comment.
    assert!(
        stdout.contains("## Task 7: Smoke test\n"),
        "stdout:\n{stdout}"
    );

    // A clean render emits nothing to stderr.
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.is_empty(),
        "a clean render must emit no stderr, got:\n{stderr}"
    );
}

#[test]
fn render_byte_matches_the_captured_contract_fixture() {
    // The committed `contract_identical.md` was captured from this binary; a
    // drift in render output (or the CLI's stdout handling) breaks this.
    let output = run(&[
        "render",
        "--specs",
        &fixture("specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
    ]);
    assert_eq!(code(&output), 0);
    let expected = fs::read(fixtures().join("contract_identical.md")).expect("read fixture");
    assert_eq!(
        output.stdout, expected,
        "render stdout must byte-match the captured contract fixture"
    );
}

#[test]
fn render_maps_a_render_error_to_stderr_and_a_nonzero_exit() {
    let output = run(&[
        "render",
        "--specs",
        &fixture("bad_specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
    ]);
    assert_ne!(
        code(&output),
        0,
        "a RenderError must produce a non-zero exit"
    );
    assert!(
        output.stdout.is_empty(),
        "no partial contract may be printed on a RenderError"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("task 99"),
        "the error must name the offending dangling link, got:\n{stderr}"
    );
}

#[test]
fn render_reports_the_inspection_cap_warning_to_stderr_with_exit_zero() {
    let output = run(&[
        "render",
        "--specs",
        &fixture("inspection_specs"),
        "--tasks",
        &fixture("inspection_tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
    ]);
    assert_eq!(
        code(&output),
        0,
        "the inspection-cap breach is non-fatal: render still exits 0"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("Task 2") && stderr.contains("[inspection]") && stderr.contains("cap"),
        "the inspection-cap warning must go to stderr, got:\n{stderr}"
    );
    // The contract itself still lands on stdout.
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.starts_with("# Contract\n"), "stdout:\n{stdout}");
}

#[test]
fn an_unknown_subcommand_is_a_usage_error() {
    let output = run(&["frobnicate"]);
    assert_ne!(code(&output), 0, "an unknown subcommand must exit non-zero");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage:"), "stderr:\n{stderr}");
}

#[test]
fn shadow_against_an_identical_contract_reports_no_differences_and_exits_zero() {
    let output = run(&[
        "shadow",
        "--specs",
        &fixture("specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
        "--existing",
        &fixture("contract_identical.md"),
    ]);
    assert_eq!(code(&output), 0, "identical shadow must exit 0");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("no differences"),
        "identical shadow must say so, got:\n{stdout}"
    );
}

#[test]
fn shadow_against_a_different_contract_prints_a_diff_and_exits_one() {
    let output = run(&[
        "shadow",
        "--specs",
        &fixture("specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
        "--existing",
        &fixture("contract_different.md"),
    ]);
    assert_eq!(
        code(&output),
        1,
        "a differing shadow must exit 1 so the cutover gate can script it"
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    // The existing-only (old) line is marked `-`, the rendered-only (new) `+`.
    assert!(
        stdout.contains("- - [ ] [execution] `widget greet` prints a DIFFERENT greeting"),
        "the existing-only line must be marked removed, got:\n{stdout}"
    );
    assert!(
        stdout.contains("+ - [ ] [execution] `widget greet` prints a greeting and exits 0"),
        "the rendered-only line must be marked added, got:\n{stdout}"
    );
}

#[test]
fn shadow_maps_a_render_error_to_a_distinct_nonzero_exit_and_stderr() {
    let output = run(&[
        "shadow",
        "--specs",
        &fixture("bad_specs"),
        "--tasks",
        &fixture("tasks.md"),
        "--fragment",
        &fixture("fragment.md"),
        "--existing",
        &fixture("contract_identical.md"),
    ]);
    // Distinct from the "differences found" exit (1): a RenderError is 2.
    assert_eq!(
        code(&output),
        2,
        "a shadow RenderError must use a distinct non-zero exit, not the diff exit"
    );
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("task 99"),
        "the RenderError must reach stderr, got:\n{stderr}"
    );
}

#[test]
fn shadow_is_read_only_it_modifies_no_file_and_creates_none() {
    // Copy the whole fixture tree to a scratch dir, snapshot every file's bytes
    // and mtime, run shadow against the DIFFERENT contract (the path that does
    // the most work), and assert nothing on disk changed and no file appeared.
    let scratch = scratch_copy_of_fixtures();

    let before = snapshot(&scratch);

    let output = Command::new(env!("CARGO_BIN_EXE_darkfactory-render"))
        .args([
            "shadow",
            "--specs",
            scratch.join("specs").to_string_lossy().as_ref(),
            "--tasks",
            scratch.join("tasks.md").to_string_lossy().as_ref(),
            "--fragment",
            scratch.join("fragment.md").to_string_lossy().as_ref(),
            "--existing",
            scratch
                .join("contract_different.md")
                .to_string_lossy()
                .as_ref(),
        ])
        .output()
        .expect("spawn darkfactory-render");
    assert_eq!(
        output.status.code(),
        Some(1),
        "the read-only check uses the differing path, which exits 1"
    );

    let after = snapshot(&scratch);
    assert_eq!(
        before, after,
        "shadow must not write, modify, or create ANY file"
    );

    fs::remove_dir_all(&scratch).ok();
}

/// One file's recorded identity for the read-only assertion: its relative path,
/// byte content, and modification time. Equality of the whole set before and
/// after a `shadow` run proves nothing was touched.
type FileSnapshot = Vec<(PathBuf, Vec<u8>, SystemTime)>;

/// Snapshot every file under `root`, sorted by relative path for stable
/// comparison.
fn snapshot(root: &Path) -> FileSnapshot {
    let mut files: FileSnapshot = Vec::new();
    collect_files(root, root, &mut files);
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

/// Recursively gather `(relative_path, bytes, mtime)` for every file under `dir`.
fn collect_files(base: &Path, dir: &Path, out: &mut FileSnapshot) {
    for entry in fs::read_dir(dir).expect("read scratch dir") {
        let entry = entry.expect("scratch dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_files(base, &path, out);
        } else {
            let rel = path
                .strip_prefix(base)
                .expect("path under base")
                .to_path_buf();
            let bytes = fs::read(&path).expect("read scratch file");
            let mtime = fs::metadata(&path)
                .expect("scratch file metadata")
                .modified()
                .expect("scratch file mtime");
            out.push((rel, bytes, mtime));
        }
    }
}

/// Copy the committed CLI fixture tree into a unique scratch dir so the
/// read-only test can snapshot it without touching the working tree.
fn scratch_copy_of_fixtures() -> PathBuf {
    let dest = std::env::temp_dir().join(format!(
        "darkfactory-render-shadow-ro-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    copy_tree(&fixtures(), &dest);
    dest
}

/// Recursively copy a directory tree from `src` to `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create scratch dir");
    for entry in fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("fixture dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy fixture file");
        }
    }
}
