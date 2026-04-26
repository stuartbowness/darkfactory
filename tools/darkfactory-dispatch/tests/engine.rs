#[path = "../src/engine.rs"]
mod engine;
#[path = "../src/event.rs"]
mod event;
#[expect(
    dead_code,
    reason = "integration tests exercise selected supervisor APIs indirectly"
)]
#[path = "../src/supervisor.rs"]
mod supervisor;
#[path = "../src/types.rs"]
mod types;

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use event::{Emitter, StreamMode};
use nix::libc;
use serde_json::Value;
use tokio::sync::Mutex as AsyncMutex;
use types::{DispatchError, DispatchSpec, DispatchStatus};

static TEST_ID: AtomicU64 = AtomicU64::new(1);
static PATH_LOCK: LazyLock<AsyncMutex<()>> = LazyLock::new(|| AsyncMutex::new(()));

#[derive(Clone)]
struct SharedBuffer {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner
            .lock()
            .expect("shared buffer mutex poisoned")
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn successful_dispatch_tracks_tokens_and_activity() -> Result<(), Box<dyn std::error::Error>>
{
    let _path_guard = TestPath::install("success").await?;
    let tmp = temp_test_dir("success")?;
    let output = tmp.join("result.md");
    let log = tmp.join("codex.log");
    let result = engine::dispatch(
        spec(&tmp, &output, Some(log.clone()), 10, 0),
        "dispatch-success".to_string(),
        emitter("dispatch-success").0,
    )
    .await?;

    assert_eq!(result.status, DispatchStatus::Completed);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.output_exists);
    assert_eq!(result.tokens.input, 100);
    assert_eq!(result.tokens.output, 50);
    assert_eq!(result.tokens.reasoning, 20);
    assert_eq!(result.activity.tool_calls, 1);
    assert_eq!(result.activity.commands_run, vec!["cargo test"]);
    assert_eq!(result.activity.files_changed, vec!["src/main.rs"]);
    assert_eq!(result.activity.files_read, vec!["README.md"]);

    let log_text = std::fs::read_to_string(log)?;
    assert!(log_text.contains("thread.started"));
    assert!(log_text.contains("mock stderr line"));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn timeout_dispatch_stops_child_process() -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::install("sleep").await?;
    let tmp = temp_test_dir("timeout")?;
    let output = tmp.join("result.md");
    let pid_file = tmp.join("pid");
    std::env::set_var("CODEX_MOCK_PID_FILE", &pid_file);

    let result = engine::dispatch(
        spec(&tmp, &output, None, 1, 1),
        "dispatch-timeout".to_string(),
        emitter("dispatch-timeout").0,
    )
    .await?;

    std::env::remove_var("CODEX_MOCK_PID_FILE");
    assert_eq!(result.status, DispatchStatus::TimedOut);
    assert!(result.error.as_ref().is_some_and(|error| error.retryable));

    let pid = std::fs::read_to_string(pid_file)?.trim().parse::<i32>()?;
    assert_process_exits(pid).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn heartbeat_events_are_emitted_during_running_dispatch(
) -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::install("delayed_success").await?;
    let tmp = temp_test_dir("heartbeat")?;
    let output = tmp.join("result.md");
    let (emitter, buffer) = emitter("dispatch-heartbeat");

    let result = engine::dispatch(
        spec(&tmp, &output, None, 10, 0),
        "dispatch-heartbeat".to_string(),
        emitter,
    )
    .await?;

    assert_eq!(result.status, DispatchStatus::Completed);
    let events = buffer_events(&buffer)?;
    let heartbeats = events
        .iter()
        .filter(|event| event["type"] == "heartbeat")
        .collect::<Vec<_>>();
    assert!(heartbeats.len() >= 2, "events: {events:?}");
    assert!(heartbeats
        .iter()
        .all(|event| event["interval_s"].as_u64() == Some(1)));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn malformed_ndjson_is_counted_and_logged() -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::install("malformed").await?;
    let tmp = temp_test_dir("malformed")?;
    let output = tmp.join("result.md");
    let log = tmp.join("codex.log");
    let result = engine::dispatch(
        spec(&tmp, &output, Some(log.clone()), 10, 0),
        "dispatch-malformed".to_string(),
        emitter("dispatch-malformed").0,
    )
    .await?;

    assert_eq!(result.status, DispatchStatus::Completed);
    assert_eq!(result.activity.parse_errors, 2);
    let log_text = std::fs::read_to_string(log)?;
    assert!(log_text.contains("not json"));
    assert!(log_text.contains("{still not json"));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn missing_codex_binary_returns_spawn_error() -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::missing_binary().await?;
    let tmp = temp_test_dir("missing-binary")?;
    let output = tmp.join("result.md");

    let result = engine::dispatch(
        spec(&tmp, &output, None, 10, 0),
        "dispatch-missing-binary".to_string(),
        emitter("dispatch-missing-binary").0,
    )
    .await;

    assert!(matches!(
        result,
        Err(DispatchError::BinaryNotFound | DispatchError::SpawnFailed(_))
    ));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_child_reports_failed_status_and_exit_code() -> Result<(), Box<dyn std::error::Error>>
{
    let _path_guard = TestPath::install("failed").await?;
    let tmp = temp_test_dir("failed-child")?;
    let output = tmp.join("result.md");

    let result = engine::dispatch(
        spec(&tmp, &output, None, 10, 0),
        "dispatch-failed-child".to_string(),
        emitter("dispatch-failed-child").0,
    )
    .await?;

    assert_eq!(result.status, DispatchStatus::Failed);
    assert_eq!(result.exit_code, Some(1));
    assert_eq!(result.signal, None);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn missing_output_file_reports_failed_result() -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::install("no_output").await?;
    let tmp = temp_test_dir("missing-output")?;
    let output = tmp.join("nonexistent-output.md");

    let result = engine::dispatch(
        spec(&tmp, &output, None, 10, 0),
        "dispatch-missing-output".to_string(),
        emitter("dispatch-missing-output").0,
    )
    .await?;

    assert_eq!(result.status, DispatchStatus::Failed);
    assert!(!result.output_exists);
    assert_eq!(
        result.error.as_ref().map(|error| error.code.as_str()),
        Some("missing_output")
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn soft_timeout_reports_timed_out_after_sigterm_exit(
) -> Result<(), Box<dyn std::error::Error>> {
    let _path_guard = TestPath::install("term_clean").await?;
    let tmp = temp_test_dir("term-clean")?;
    let output = tmp.join("result.md");
    let pid_file = tmp.join("pid");
    std::env::set_var("CODEX_MOCK_PID_FILE", &pid_file);

    let result = engine::dispatch(
        spec(&tmp, &output, None, 1, 5),
        "dispatch-term-clean".to_string(),
        emitter("dispatch-term-clean").0,
    )
    .await?;

    std::env::remove_var("CODEX_MOCK_PID_FILE");
    assert_eq!(result.status, DispatchStatus::TimedOut);

    let pid = std::fs::read_to_string(pid_file)?.trim().parse::<i32>()?;
    assert_process_exits(pid).await?;

    Ok(())
}

fn emitter(dispatch_id: &str) -> (Arc<Emitter>, Arc<Mutex<Vec<u8>>>) {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let emitter = Emitter::with_writer(
        dispatch_id,
        StreamMode::Verbose,
        SharedBuffer {
            inner: Arc::clone(&buffer),
        },
    );
    (Arc::new(emitter), buffer)
}

fn buffer_events(buffer: &Arc<Mutex<Vec<u8>>>) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let bytes = buffer.lock().expect("shared buffer mutex poisoned").clone();
    let text = String::from_utf8(bytes)?;
    text.lines()
        .map(|line| serde_json::from_str::<Value>(line).map_err(Into::into))
        .collect()
}

fn spec(
    cwd: &Path,
    output: &Path,
    log: Option<PathBuf>,
    timeout_sec: u64,
    grace_sec: u64,
) -> DispatchSpec {
    DispatchSpec {
        model: "gpt-5".to_string(),
        effort: Some("low".to_string()),
        timeout_sec,
        grace_sec,
        sandbox: "workspace-write".to_string(),
        configs: vec!["notify=false".to_string()],
        prompt: "run the test task".to_string(),
        output: Some(output.to_path_buf()),
        cwd: cwd.to_path_buf(),
        log,
        store_dir: None,
    }
}

fn temp_test_dir(name: &str) -> io::Result<PathBuf> {
    let id = TEST_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "darkfactory-dispatch-test-{}-{name}-{id}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

struct TestPath {
    _guard: tokio::sync::MutexGuard<'static, ()>,
    previous_path: Option<String>,
    previous_mode: Option<String>,
}

impl TestPath {
    async fn install(mode: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let guard = PATH_LOCK.lock().await;
        let bin_dir = temp_test_dir("bin")?;
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_codex_mock.sh");
        let target = bin_dir.join("codex");
        std::fs::copy(source, &target)?;

        let previous_path = std::env::var("PATH").ok();
        let previous_mode = std::env::var("CODEX_MOCK_MODE").ok();
        let path = previous_path.clone().unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{path}", bin_dir.display()));
        std::env::set_var("CODEX_MOCK_MODE", mode);

        Ok(Self {
            _guard: guard,
            previous_path,
            previous_mode,
        })
    }

    async fn missing_binary() -> Result<Self, Box<dyn std::error::Error>> {
        let guard = PATH_LOCK.lock().await;
        let bin_dir = temp_test_dir("empty-bin")?;

        let previous_path = std::env::var("PATH").ok();
        let previous_mode = std::env::var("CODEX_MOCK_MODE").ok();
        std::env::set_var("PATH", bin_dir);
        std::env::remove_var("CODEX_MOCK_MODE");

        Ok(Self {
            _guard: guard,
            previous_path,
            previous_mode,
        })
    }
}

impl Drop for TestPath {
    fn drop(&mut self) {
        match &self.previous_path {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
        match &self.previous_mode {
            Some(mode) => std::env::set_var("CODEX_MOCK_MODE", mode),
            None => std::env::remove_var("CODEX_MOCK_MODE"),
        }
        std::env::remove_var("CODEX_MOCK_PID_FILE");
    }
}

async fn assert_process_exits(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..40 {
        if !process_exists(pid) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    Err(format!("process {pid} still exists").into())
}

fn process_exists(pid: i32) -> bool {
    let rc = unsafe { libc::kill(pid, 0) };
    if rc == 0 {
        return true;
    }

    io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}
