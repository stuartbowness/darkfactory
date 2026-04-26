#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "event emitter is consumed by the upcoming engine module"
    )
)]

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use chrono::{SecondsFormat, Utc};
use serde::Serialize;
use thiserror::Error;
use tokio::sync::watch;

const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 30;

#[derive(Debug, Error)]
pub(crate) enum EventError {
    #[error("serialize dispatch event: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("write dispatch event: {0}")]
    Write(#[from] io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DispatchEvent {
    #[serde(rename = "type")]
    pub(crate) event_type: String,
    pub(crate) dispatch_id: String,
    pub(crate) ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) elapsed_s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_activity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) interval_s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) engine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) message: Option<String>,
}

pub(crate) struct Emitter {
    dispatch_id: String,
    start: Instant,
    stream_mode: StreamMode,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StreamMode {
    Silent,
    Normal,
    Verbose,
}

#[derive(Debug, Clone)]
pub(crate) struct CancellationToken {
    tx: watch::Sender<bool>,
    rx: watch::Receiver<bool>,
}

impl CancellationToken {
    pub(crate) fn new() -> Self {
        let (tx, rx) = watch::channel(false);
        Self { tx, rx }
    }

    pub(crate) fn cancel(&self) {
        let _ = self.tx.send(true);
    }

    pub(crate) async fn cancelled(&self) {
        let mut rx = self.rx.clone();
        loop {
            if *rx.borrow() {
                return;
            }
            if rx.changed().await.is_err() {
                return;
            }
        }
    }
}

impl Emitter {
    pub(crate) fn new(dispatch_id: impl Into<String>, stream_mode: StreamMode) -> Self {
        Self::with_writer(dispatch_id, stream_mode, io::stderr())
    }

    fn with_writer(
        dispatch_id: impl Into<String>,
        stream_mode: StreamMode,
        writer: impl Write + Send + 'static,
    ) -> Self {
        Self {
            dispatch_id: dispatch_id.into(),
            start: Instant::now(),
            stream_mode,
            writer: Arc::new(Mutex::new(Box::new(writer))),
        }
    }

    pub(crate) fn emit_dispatch_start(
        &self,
        engine: impl Into<String>,
        model: impl Into<String>,
    ) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "dispatch_start".to_string(),
            engine: Some(engine.into()),
            model: Some(model.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_dispatch_end(
        &self,
        status: impl Into<String>,
        duration_ms: u64,
    ) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "dispatch_end".to_string(),
            status: Some(status.into()),
            duration_ms: Some(duration_ms),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_heartbeat(
        &self,
        last_activity: impl Into<String>,
    ) -> Result<(), EventError> {
        self.emit_heartbeat_with_interval(last_activity, DEFAULT_HEARTBEAT_INTERVAL_SECS)
    }

    pub(crate) fn emit_timeout_warning(&self) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "timeout_warning".to_string(),
            elapsed_s: Some(self.elapsed_s()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_tool_start(&self, tool: impl Into<String>) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "tool_start".to_string(),
            tool: Some(tool.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_tool_end(&self, tool: impl Into<String>) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "tool_end".to_string(),
            tool: Some(tool.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_file_write(&self, path: impl Into<String>) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "file_write".to_string(),
            file: Some(path.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_file_read(&self, path: impl Into<String>) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "file_read".to_string(),
            file: Some(path.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_command_run(&self, command: impl Into<String>) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "command_run".to_string(),
            command: Some(command.into()),
            ..self.base_event()
        })
    }

    pub(crate) fn emit_error(
        &self,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "error".to_string(),
            code: Some(code.into()),
            message: Some(message.into()),
            ..self.base_event()
        })
    }

    fn emit_heartbeat_with_interval(
        &self,
        last_activity: impl Into<String>,
        interval_s: u64,
    ) -> Result<(), EventError> {
        self.emit(DispatchEvent {
            event_type: "heartbeat".to_string(),
            elapsed_s: Some(self.elapsed_s()),
            last_activity: Some(last_activity.into()),
            interval_s: Some(interval_s),
            ..self.base_event()
        })
    }

    fn emit(&self, event: DispatchEvent) -> Result<(), EventError> {
        if !self.should_emit(&event.event_type) {
            return Ok(());
        }

        let mut line = serde_json::to_vec(&event)?;
        line.push(b'\n');

        let mut writer = self.writer.lock().expect("event writer mutex poisoned");
        writer.write_all(&line)?;
        writer.flush()?;

        Ok(())
    }

    fn should_emit(&self, event_type: &str) -> bool {
        match self.stream_mode {
            StreamMode::Silent => matches!(
                event_type,
                "dispatch_start" | "dispatch_end" | "timeout_warning" | "error"
            ),
            StreamMode::Normal | StreamMode::Verbose => true,
        }
    }

    fn base_event(&self) -> DispatchEvent {
        DispatchEvent {
            event_type: String::new(),
            dispatch_id: self.dispatch_id.clone(),
            ts: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            elapsed_s: None,
            last_activity: None,
            interval_s: None,
            engine: None,
            model: None,
            tool: None,
            file: None,
            command: None,
            status: None,
            duration_ms: None,
            code: None,
            message: None,
        }
    }

    fn elapsed_s(&self) -> u64 {
        self.start.elapsed().as_secs()
    }
}

pub(crate) fn heartbeat_ticker(
    emitter: Arc<Emitter>,
    interval_secs: u64,
    last_activity: Arc<Mutex<String>>,
    cancel: CancellationToken,
) {
    let interval_secs = interval_secs.max(1);
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        ticker.tick().await;
        loop {
            tokio::select! {
                () = cancel.cancelled() => break,
                _ = ticker.tick() => {
                    let activity = last_activity
                        .lock()
                        .expect("last activity mutex poisoned")
                        .clone();
                    let _ = emitter.emit_heartbeat_with_interval(activity, interval_secs);
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serde_json::Value;

    use super::*;

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

    fn emitter_with_buffer(stream_mode: StreamMode) -> (Emitter, Arc<Mutex<Vec<u8>>>) {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let emitter = Emitter::with_writer(
            "dispatch-123",
            stream_mode,
            SharedBuffer {
                inner: Arc::clone(&buffer),
            },
        );
        (emitter, buffer)
    }

    fn buffer_lines(buffer: &Arc<Mutex<Vec<u8>>>) -> Vec<Value> {
        let output =
            String::from_utf8(buffer.lock().expect("shared buffer mutex poisoned").clone())
                .expect("events are utf8");

        output
            .lines()
            .map(|line| serde_json::from_str(line).expect("event line is valid json"))
            .collect()
    }

    #[test]
    fn heartbeat_event_format() {
        let (emitter, buffer) = emitter_with_buffer(StreamMode::Normal);

        emitter
            .emit_heartbeat("running: cargo test")
            .expect("emit heartbeat");

        let events = buffer_lines(&buffer);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["type"], "heartbeat");
        assert_eq!(events[0]["dispatch_id"], "dispatch-123");
        assert!(events[0]["ts"].as_str().is_some());
        assert!(events[0]["elapsed_s"].as_u64().is_some());
        assert_eq!(events[0]["last_activity"], "running: cargo test");
        assert!(events[0]["interval_s"].as_u64().is_some());
    }

    #[test]
    fn stream_mode_filtering() {
        let (silent, silent_buffer) = emitter_with_buffer(StreamMode::Silent);
        silent.emit_tool_start("shell").expect("emit tool start");
        silent.emit_timeout_warning().expect("emit timeout warning");

        let silent_events = buffer_lines(&silent_buffer);
        assert_eq!(silent_events.len(), 1);
        assert_eq!(silent_events[0]["type"], "timeout_warning");

        let (normal, normal_buffer) = emitter_with_buffer(StreamMode::Normal);
        normal.emit_tool_start("shell").expect("emit tool start");
        normal.emit_timeout_warning().expect("emit timeout warning");

        let normal_events = buffer_lines(&normal_buffer);
        assert_eq!(
            normal_events
                .iter()
                .map(|event| event["type"].as_str().expect("type is a string"))
                .collect::<Vec<_>>(),
            vec!["tool_start", "timeout_warning"]
        );
    }

    #[test]
    fn event_type_coverage() {
        let (emitter, buffer) = emitter_with_buffer(StreamMode::Normal);

        emitter
            .emit_dispatch_start("codex", "gpt-5")
            .expect("emit dispatch start");
        emitter
            .emit_dispatch_end("completed", 123)
            .expect("emit dispatch end");
        emitter.emit_heartbeat("running").expect("emit heartbeat");
        emitter
            .emit_timeout_warning()
            .expect("emit timeout warning");
        emitter.emit_tool_start("shell").expect("emit tool start");
        emitter.emit_tool_end("shell").expect("emit tool end");
        emitter
            .emit_file_write("src/main.rs")
            .expect("emit file write");
        emitter.emit_file_read("README.md").expect("emit file read");
        emitter
            .emit_command_run("cargo test")
            .expect("emit command run");
        emitter
            .emit_error("process_failed", "child exited")
            .expect("emit error");

        let event_types = buffer_lines(&buffer)
            .iter()
            .map(|event| {
                event["type"]
                    .as_str()
                    .expect("type is a string")
                    .to_string()
            })
            .collect::<Vec<_>>();

        assert_eq!(
            event_types,
            vec![
                "dispatch_start",
                "dispatch_end",
                "heartbeat",
                "timeout_warning",
                "tool_start",
                "tool_end",
                "file_write",
                "file_read",
                "command_run",
                "error",
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn heartbeat_ticker_can_be_cancelled() {
        let (emitter, _buffer) = emitter_with_buffer(StreamMode::Verbose);
        let cancel = CancellationToken::new();
        let last_activity = Arc::new(Mutex::new("initializing".to_string()));

        heartbeat_ticker(
            Arc::new(emitter),
            3600,
            Arc::clone(&last_activity),
            cancel.clone(),
        );
        cancel.cancel();
        cancel.cancelled().await;

        let _stderr_emitter = Emitter::new("dispatch-456", StreamMode::Verbose);
    }
}
