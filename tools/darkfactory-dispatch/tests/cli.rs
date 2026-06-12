use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command as StdCommand, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use assert_cmd::Command;
use nix::libc;
use predicates::prelude::*;
use serde_json::Value;
use ulid::Ulid;

static TEST_ID: AtomicU64 = AtomicU64::new(1);

#[test]
fn help_lists_dispatch_flags() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.arg("--help").assert().success().stdout(
        predicate::str::contains("--model")
            .and(predicate::str::contains("--effort"))
            .and(predicate::str::contains("--timeout"))
            .and(predicate::str::contains("--grace"))
            .and(predicate::str::contains("--heartbeat"))
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
fn whitespace_only_prompt_exits_with_cli_error() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");

    cmd.args(["--prompt", "   "])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("prompt is empty"));
}

#[test]
fn empty_prompt_file_exits_with_cli_error() {
    let path = std::env::temp_dir().join(format!(
        "darkfactory-dispatch-empty-prompt-{}.md",
        Ulid::new()
    ));
    std::fs::write(&path, "  \n\t\n").expect("write prompt file");

    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");
    cmd.args(["--prompt-file", path.to_str().expect("utf8 path")])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("is empty"));

    std::fs::remove_file(&path).ok();
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

#[test]
fn status_unknown_id_exits_nonzero() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");
    let dispatch_id = unknown_dispatch_id();

    cmd.args(["status", &dispatch_id])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("dispatch not found")
                .and(predicate::str::contains(dispatch_id)),
        );
}

#[test]
fn result_unknown_id_exits_nonzero() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");
    let dispatch_id = unknown_dispatch_id();

    cmd.args(["result", &dispatch_id]).assert().failure();
}

#[test]
fn wait_unknown_id_exits_nonzero() {
    let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");
    let dispatch_id = unknown_dispatch_id();

    cmd.args(["wait", &dispatch_id]).assert().failure();
}

#[test]
fn async_dispatch_flow_reports_status_and_result() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("async-flow")?;
    let output = env.home.join("result.md");
    let mut child = env.dispatch_command();
    child
        .args([
            "--async",
            "--prompt",
            "echo test",
            "--output",
            output.to_str().expect("utf-8 output path"),
        ])
        .env("CODEX_MOCK_MODE", "delayed_success")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child.spawn()?;
    let stdout = child.stdout.take().expect("stdout piped");
    let mut lines = BufReader::new(stdout).lines();
    let ack: Value = serde_json::from_str(&lines.next().expect("ack line")?)?;
    assert_eq!(ack["kind"], "async_started");
    let dispatch_id = ack["dispatch_id"]
        .as_str()
        .expect("dispatch id")
        .to_string();

    let status = env
        .dispatch_command()
        .args(["status", &dispatch_id])
        .output()?;
    assert!(
        status.status.success(),
        "status stderr: {:?}",
        status.stderr
    );
    let status_json: Value = serde_json::from_slice(&status.stdout)?;
    assert_eq!(status_json["state"], "running");

    let wait = env
        .dispatch_command()
        .args(["wait", &dispatch_id])
        .output()?;
    assert!(wait.status.success(), "wait stderr: {:?}", wait.stderr);

    let result = env
        .dispatch_command()
        .args(["result", &dispatch_id])
        .output()?;
    assert!(
        result.status.success(),
        "result stderr: {:?}",
        result.stderr
    );
    let result_json: Value = serde_json::from_slice(&result.stdout)?;
    assert_eq!(result_json["status"], "completed");
    assert_eq!(result_json["dispatch_id"], dispatch_id);

    assert!(child.wait()?.success());
    assert!(env
        .home
        .join(".darkfactory/dispatches")
        .join(dispatch_id)
        .join("events.jsonl")
        .exists());

    Ok(())
}

#[test]
fn status_reports_orphaned_when_async_dispatcher_dies() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("orphan")?;
    let output = env.home.join("result.md");
    let pid_file = env.home.join("codex.pid");
    let mut child = env.dispatch_command();
    child
        .args([
            "--async",
            "--prompt",
            "sleep",
            "--timeout",
            "60",
            "--output",
            output.to_str().expect("utf-8 output path"),
        ])
        .env("CODEX_MOCK_MODE", "sleep")
        .env("CODEX_MOCK_PID_FILE", &pid_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child.spawn()?;
    let stdout = child.stdout.take().expect("stdout piped");
    let mut lines = BufReader::new(stdout).lines();
    let ack: Value = serde_json::from_str(&lines.next().expect("ack line")?)?;
    let dispatch_id = ack["dispatch_id"]
        .as_str()
        .expect("dispatch id")
        .to_string();

    child.kill()?;
    let _ = child.wait()?;

    let status = env
        .dispatch_command()
        .args(["status", &dispatch_id])
        .output()?;
    assert!(
        status.status.success(),
        "status stderr: {:?}",
        status.stderr
    );
    let status_json: Value = serde_json::from_slice(&status.stdout)?;
    assert_eq!(status_json["state"], "orphaned");

    if let Ok(pid) = std::fs::read_to_string(pid_file) {
        kill_process_group(pid.trim().parse()?);
    }

    Ok(())
}

#[test]
fn async_dispatch_prunes_old_store_directories() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("cleanup")?;
    let store_root = env.home.join(".darkfactory/dispatches");
    std::fs::create_dir_all(&store_root)?;
    let old_id =
        Ulid::from_datetime(SystemTime::now() - Duration::from_secs(8 * 24 * 60 * 60)).to_string();
    let old_dir = store_root.join(old_id);
    std::fs::create_dir_all(&old_dir)?;

    let output = env.home.join("result.md");
    let mut child = env.dispatch_command();
    child
        .args([
            "--async",
            "--prompt",
            "echo test",
            "--output",
            output.to_str().expect("utf-8 output path"),
        ])
        .env("CODEX_MOCK_MODE", "success")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child.spawn()?;
    let stdout = child.stdout.take().expect("stdout piped");
    let mut lines = BufReader::new(stdout).lines();
    let ack: Value = serde_json::from_str(&lines.next().expect("ack line")?)?;
    assert_eq!(ack["kind"], "async_started");
    assert!(!old_dir.exists());
    assert!(child.wait()?.success());

    Ok(())
}

#[test]
fn json_stdout_is_single_object() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("json-single-object")?;
    let output = env.home.join("result.md");
    let result = env
        .dispatch_command()
        .args([
            "--json",
            "--prompt",
            "review",
            "--output",
            output.to_str().expect("utf-8 output path"),
            "--cwd",
            env.home.to_str().expect("utf-8 home path"),
        ])
        .env("CODEX_MOCK_MODE", "special_chars")
        .output()?;

    assert!(result.status.success(), "stderr: {:?}", result.stderr);
    // Stdout must be exactly one parseable DispatchResult object -- no event or
    // diagnostic leakage -- and special characters must round-trip through serde.
    let parsed: Value = serde_json::from_slice(&result.stdout)?;
    assert_eq!(parsed["status"], "completed");
    let commands = parsed["activity"]["commands_run"]
        .as_array()
        .expect("commands_run array");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], r#"grep -n "foo\bar" baz | jq .x"#);

    Ok(())
}

struct MockEnv {
    home: PathBuf,
    bin_dir: PathBuf,
}

impl MockEnv {
    fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root = temp_test_dir(name)?;
        let home = root.join("home");
        let bin_dir = root.join("bin");
        std::fs::create_dir_all(&home)?;
        std::fs::create_dir_all(&bin_dir)?;
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_codex_mock.sh");
        let target = bin_dir.join("codex");
        std::fs::copy(source, &target)?;
        let mut permissions = std::fs::metadata(&target)?.permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&target, permissions)?;
        Ok(Self { home, bin_dir })
    }

    fn dispatch_command(&self) -> StdCommand {
        let mut command = StdCommand::new(env!("CARGO_BIN_EXE_darkfactory-dispatch"));
        let path = std::env::var("PATH").unwrap_or_default();
        command
            .env("HOME", &self.home)
            .env("PATH", format!("{}:{path}", self.bin_dir.display()));
        command
    }
}

fn temp_test_dir(name: &str) -> std::io::Result<PathBuf> {
    let id = TEST_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "darkfactory-dispatch-cli-{}-{name}-{id}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn unknown_dispatch_id() -> String {
    format!("unknown-id-{}", Ulid::new())
}

fn kill_process_group(pid: i32) {
    if pid > 0 {
        let _ = unsafe { libc::kill(-pid, libc::SIGKILL) };
    }
}
