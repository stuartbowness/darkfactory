#[expect(
    dead_code,
    reason = "integration tests exercise this module boundary directly"
)]
#[path = "../src/supervisor.rs"]
mod supervisor;

use std::io;
use std::time::{Duration, Instant};

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
async fn graceful_stop_kills_after_grace_when_sigterm_is_ignored(
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = SupervisorBuilder::new("python3")
        .args([
            "-c",
            "import signal, time; signal.signal(signal.SIGTERM, signal.SIG_IGN); print('ready', flush=True); time.sleep(300)",
        ])
        .spawn()?;
    let mut lines = BufReader::new(handle.stdout()?).lines();
    let ready = tokio::time::timeout(Duration::from_secs(2), lines.next_line())
        .await??
        .ok_or("child did not become ready")?;
    assert_eq!(ready, "ready");

    let started = Instant::now();
    let result = handle.graceful_stop(1).await?;
    let elapsed = started.elapsed();

    assert_eq!(result.signal, Some(libc::SIGKILL));
    assert!(elapsed >= Duration::from_secs(1));
    assert!(elapsed < Duration::from_secs(3));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn kill_soft_sends_sigterm() -> Result<(), Box<dyn std::error::Error>> {
    let handle = SupervisorBuilder::new("sleep").arg("300").spawn()?;

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
    let term_handle = SupervisorBuilder::new("sleep").arg("300").spawn()?;
    let term_result = term_handle.graceful_stop(10).await?;
    assert_eq!(term_result.signal, Some(libc::SIGTERM));
    assert_eq!(term_handle.wait().await?, term_result);

    let kill_handle = SupervisorBuilder::new("sleep").arg("300").spawn()?;
    kill_handle.kill()?;
    let kill_result = kill_handle.wait().await?;
    assert_eq!(kill_result.signal, Some(libc::SIGKILL));

    Ok(())
}

fn parse_pids(line: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    line.split_whitespace()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
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

fn process_exists(pid: i32) -> bool {
    // SAFETY: kill(pid, 0) does not send a signal. It only checks whether the
    // process exists and whether this process can signal it.
    let rc = unsafe { libc::kill(pid, 0) };
    if rc == 0 {
        return true;
    }

    io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}
