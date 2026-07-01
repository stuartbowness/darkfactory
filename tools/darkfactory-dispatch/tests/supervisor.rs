#[expect(
    dead_code,
    reason = "integration tests exercise this module boundary directly"
)]
#[path = "../src/supervisor.rs"]
mod supervisor;

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

use nix::libc;
use supervisor::SupervisorBuilder;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::test(flavor = "multi_thread")]
async fn spawn_nonexistent_binary_returns_error() -> Result<(), Box<dyn std::error::Error>> {
    let result = SupervisorBuilder::new("nonexistent-binary-xyz").spawn();

    assert!(result.is_err());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn kill_terminates_the_whole_process_group() -> Result<(), Box<dyn std::error::Error>> {
    let script = "sleep 300 & bg=$!; sleep 300 & fg=$!; echo $$ $bg $fg; wait";
    let handle = SupervisorBuilder::new("sh").args(["-c", script]).spawn()?;

    let mut lines = BufReader::new(handle.stdout()?).lines();
    let line = tokio::time::timeout(Duration::from_secs(2), lines.next_line())
        .await??
        .ok_or("child did not print process ids")?;
    let pids = parse_pids(&line)?;
    assert_eq!(pids.len(), 3);

    handle.kill()?;
    let result = handle.wait().await?;

    assert_eq!(result.signal, Some(libc::SIGKILL));
    for pid in pids {
        assert_process_exits(pid).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn kill_soft_sends_sigterm() -> Result<(), Box<dyn std::error::Error>> {
    let handle = SupervisorBuilder::new("sleep").args(["300"]).spawn()?;

    handle.kill_soft()?;
    let result = handle.wait().await?;

    assert_eq!(result.signal, Some(libc::SIGTERM));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn normal_exit_reports_code_without_signal() -> Result<(), Box<dyn std::error::Error>> {
    let handle = SupervisorBuilder::new("true").spawn()?;
    let result = handle.wait().await?;

    assert_eq!(result.code, Some(0));
    assert_eq!(result.signal, None);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn wait_reports_sigterm_and_sigkill() -> Result<(), Box<dyn std::error::Error>> {
    let term_handle = SupervisorBuilder::new("sleep").args(["300"]).spawn()?;
    term_handle.kill_soft()?;
    let term_result = term_handle.wait().await?;
    assert_eq!(term_result.signal, Some(libc::SIGTERM));
    assert_eq!(term_handle.wait().await?, term_result);

    let kill_handle = SupervisorBuilder::new("sleep").args(["300"]).spawn()?;
    kill_handle.kill()?;
    let kill_result = kill_handle.wait().await?;
    assert_eq!(kill_result.signal, Some(libc::SIGKILL));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn two_phase_timeout_escalates_from_sigterm_to_sigkill(
) -> Result<(), Box<dyn std::error::Error>> {
    let script = "trap '' TERM; echo $$; sleep 300";
    let handle = SupervisorBuilder::new("sh").args(["-c", script]).spawn()?;

    let mut lines = BufReader::new(handle.stdout()?).lines();
    let pid = tokio::time::timeout(Duration::from_secs(2), lines.next_line())
        .await??
        .ok_or("child did not print pid")?
        .parse::<i32>()?;

    handle.kill_soft()?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(process_exists(pid));

    handle.kill()?;
    let result = handle.wait().await?;

    assert_eq!(result.signal, Some(libc::SIGKILL));
    assert_process_exits(pid).await?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[tokio::test(flavor = "multi_thread")]
async fn parent_death_terminates_supervised_child() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = temp_test_dir("parent-death")?;
    let pid_file = tmp.join("pid");
    let status = Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "parent_death_helper_spawns_supervised_child",
            "--ignored",
            "--nocapture",
        ])
        .env("DARKFACTORY_PARENT_DEATH_PID_FILE", &pid_file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    assert!(status.success());
    let pid = wait_for_pid_file(&pid_file).await?;
    assert_process_stops(pid).await?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn parent_death_helper_spawns_supervised_child() -> Result<(), Box<dyn std::error::Error>> {
    let Some(pid_file) = std::env::var_os("DARKFACTORY_PARENT_DEATH_PID_FILE") else {
        return Ok(());
    };
    let pid_file = PathBuf::from(pid_file);
    let pid_path = pid_file.to_str().ok_or("pid file path is not utf-8")?;
    let _handle = SupervisorBuilder::new("sh")
        .args(["-c", "echo $$ > \"$1\"; exec sleep 300", "sh", pid_path])
        .spawn()?;

    let _pid = wait_for_pid_file(&pid_file).await?;
    std::process::exit(0);
}

fn parse_pids(line: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    line.split_whitespace()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn temp_test_dir(name: &str) -> io::Result<PathBuf> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "darkfactory-supervisor-{}-{name}-{nanos}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

async fn wait_for_pid_file(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        match std::fs::read_to_string(path) {
            Ok(text) => return text.trim().parse::<i32>().map_err(Into::into),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        if Instant::now() >= deadline {
            return Err(format!("pid file was not written: {}", path.display()).into());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn assert_process_exits(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if !process_exists(pid) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!("process {pid} still exists").into());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[cfg(target_os = "linux")]
async fn assert_process_stops(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if !process_is_running(pid) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!("process {pid} is still running").into());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn process_exists(pid: i32) -> bool {
    // SAFETY: kill(pid, 0) does not send a signal. It only checks whether the
    // process exists and whether this process can signal it.
    let rc = unsafe { libc::kill(pid, 0) };
    if rc == 0 {
        return true;
    }

    io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

#[cfg(target_os = "linux")]
fn process_is_running(pid: i32) -> bool {
    if !process_exists(pid) {
        return false;
    }

    let Ok(stat) = std::fs::read_to_string(format!("/proc/{pid}/stat")) else {
        return true;
    };
    let Some((_, after_name)) = stat.rsplit_once(") ") else {
        return true;
    };
    !after_name.starts_with('Z')
}
