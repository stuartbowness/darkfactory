use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use ulid::Ulid;

static TEST_ID: AtomicU64 = AtomicU64::new(1);
static STORE_PATH_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

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
fn async_flag_is_rejected_without_dispatch_files() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("async-rejected")?;
    let result = env
        .dispatch_command()
        .args(["--async", "--prompt", "x"])
        .output()?;

    assert_eq!(result.status.code(), Some(2));
    assert!(result.stdout.is_empty());
    let stderr = String::from_utf8(result.stderr)?;
    assert!(stderr.contains("unexpected argument") || stderr.contains("Usage:"));
    assert!(!stderr.contains("dispatch not found"));
    assert!(!String::from_utf8_lossy(&result.stdout).contains("async_started"));
    assert!(!home_dispatch_root(&env.home).exists());

    Ok(())
}

#[test]
fn lifecycle_subcommands_are_rejected_by_parser() -> Result<(), Box<dyn std::error::Error>> {
    for subcommand in ["status", "wait", "result"] {
        let mut cmd = Command::cargo_bin("darkfactory-dispatch").expect("binary exists");
        let result = cmd.args([subcommand, "01TEST"]).output()?;
        assert_eq!(result.status.code(), Some(2), "{subcommand}");
        let stderr = String::from_utf8(result.stderr)?;
        assert!(
            stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
            "{subcommand}: {stderr}"
        );
        assert!(
            !stderr.contains("dispatch not found"),
            "{subcommand}: {stderr}"
        );
        assert!(result.stdout.is_empty(), "{subcommand}");
    }

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

#[test]
fn json_dispatch_without_store_paths_creates_no_dispatch_files(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = STORE_PATH_LOCK.lock().expect("store path mutex poisoned");
    let env = MockEnv::new("no-store-files")?;
    let tmp_store = tmp_dispatch_root();
    std::fs::remove_dir_all(&tmp_store).ok();
    std::fs::remove_dir_all(home_dispatch_root(&env.home)).ok();

    let result = env
        .dispatch_command()
        .args([
            "--json",
            "--prompt",
            "review",
            "--cwd",
            env.home.to_str().expect("utf-8 home path"),
        ])
        .env("CODEX_MOCK_MODE", "success")
        .output()?;

    assert!(result.status.success(), "stderr: {:?}", result.stderr);
    let parsed: Value = serde_json::from_slice(&result.stdout)?;
    assert_eq!(parsed["status"], "completed");
    assert!(!tmp_store.exists());
    assert!(!home_dispatch_root(&env.home).exists());

    Ok(())
}

#[test]
fn existing_files_under_old_store_paths_are_unchanged() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = STORE_PATH_LOCK.lock().expect("store path mutex poisoned");
    let env = MockEnv::new("store-sentinels")?;
    let tmp_store = tmp_dispatch_root();
    let home_store = home_dispatch_root(&env.home);
    std::fs::create_dir_all(&tmp_store)?;
    std::fs::create_dir_all(&home_store)?;
    let tmp_sentinel = tmp_store.join("sentinel.bin");
    let home_sentinel = home_store.join("sentinel.bin");
    let tmp_bytes = b"tmp sentinel bytes\n";
    let home_bytes = b"home sentinel bytes\n";
    std::fs::write(&tmp_sentinel, tmp_bytes)?;
    std::fs::write(&home_sentinel, home_bytes)?;

    let result = env
        .dispatch_command()
        .args([
            "--json",
            "--prompt",
            "review",
            "--cwd",
            env.home.to_str().expect("utf-8 home path"),
        ])
        .env("CODEX_MOCK_MODE", "success")
        .output()?;

    assert!(result.status.success(), "stderr: {:?}", result.stderr);
    assert_eq!(std::fs::read(&tmp_sentinel)?, tmp_bytes);
    assert_eq!(std::fs::read(&home_sentinel)?, home_bytes);

    Ok(())
}

#[test]
fn missing_engine_binary_emits_failed_json_and_cli_error_exit(
) -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("missing-engine")?;
    let empty_bin = env.home.join("empty-bin");
    std::fs::create_dir_all(&empty_bin)?;

    let result = StdCommand::new(env!("CARGO_BIN_EXE_darkfactory-dispatch"))
        .env("HOME", &env.home)
        .env("PATH", empty_bin)
        .args([
            "--json",
            "--prompt",
            "review",
            "--cwd",
            env.home.to_str().expect("utf-8 home path"),
        ])
        .output()?;

    assert_eq!(result.status.code(), Some(2));
    let lines = String::from_utf8(result.stdout)?;
    assert_eq!(lines.lines().count(), 1, "{lines}");
    let parsed: Value = serde_json::from_str(lines.trim())?;
    assert_eq!(parsed["status"], "failed");
    assert_eq!(parsed["error"]["code"], "startup_failed");
    assert!(String::from_utf8(result.stderr)?.contains("codex binary not found"));

    Ok(())
}

#[test]
fn retained_flags_are_accepted_and_output_is_reported() -> Result<(), Box<dyn std::error::Error>> {
    let env = MockEnv::new("retained-flags")?;
    let output = env.home.join("artifact.md");
    let log = env.home.join("dispatch.log");

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
            "--log",
            log.to_str().expect("utf-8 log path"),
            "--grace",
            "2",
            "--heartbeat",
            "1",
        ])
        .env("CODEX_MOCK_MODE", "success")
        .output()?;

    assert!(result.status.success(), "stderr: {:?}", result.stderr);
    let parsed: Value = serde_json::from_slice(&result.stdout)?;
    assert_eq!(parsed["status"], "completed");
    assert_eq!(parsed["output"], output.display().to_string());
    assert_eq!(parsed["output_exists"], true);
    assert_eq!(std::fs::read_to_string(&output)?, "mock output\n");
    assert!(std::fs::read_to_string(log)?.contains("thread.started"));

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

fn tmp_dispatch_root() -> PathBuf {
    PathBuf::from("/tmp/darkfactory-dispatch")
}

fn home_dispatch_root(home: &Path) -> PathBuf {
    home.join(".darkfactory").join("dispatches")
}
