use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_lists_dispatch_flags() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.arg("--help").assert().success().stdout(
        predicate::str::contains("--model")
            .and(predicate::str::contains("--effort"))
            .and(predicate::str::contains("--timeout"))
            .and(predicate::str::contains("--grace"))
            .and(predicate::str::contains("--sandbox"))
            .and(predicate::str::contains("--config"))
            .and(predicate::str::contains("--prompt"))
            .and(predicate::str::contains("--prompt-file"))
            .and(predicate::str::contains("--output"))
            .and(predicate::str::contains("--cwd"))
            .and(predicate::str::contains("--log"))
            .and(predicate::str::contains("--json"))
            .and(predicate::str::contains("--async"))
            .and(predicate::str::contains("--version")),
    );
}

#[test]
fn version_prints_package_version() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_prompt_exits_with_cli_error() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.assert()
        .code(2)
        .stderr(predicate::str::contains("missing prompt"));
}

#[test]
fn prompt_and_prompt_file_are_mutually_exclusive() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.args(["--prompt", "test", "--prompt-file", "prompt.md"])
        .assert()
        .code(2);
}

#[test]
fn json_and_async_are_mutually_exclusive() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.args(["--json", "--async", "--prompt", "test"])
        .assert()
        .code(2);
}
