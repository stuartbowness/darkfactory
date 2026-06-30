use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};

use nix::libc;
use thiserror::Error;
use tokio::process::{ChildStderr, ChildStdout, Command};
use tokio::sync::watch;

#[derive(Debug, Error, Clone)]
pub(crate) enum SupervisorError {
    #[error("spawn process `{program}`: {message}")]
    Spawn { program: String, message: String },

    #[error("child process did not expose a pid after spawn")]
    MissingPid,

    #[error("child stdout was not piped or was already taken")]
    StdoutUnavailable,

    #[error("child stderr was not piped or was already taken")]
    StderrUnavailable,

    #[error("wait for child process: {0}")]
    Wait(String),

    #[error("wait channel closed before child exit was recorded")]
    WaitChannelClosed,

    #[error("send signal {signal} to process group {pgid}: {message}")]
    Signal {
        signal: i32,
        pgid: i32,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExitResult {
    pub(crate) code: Option<i32>,
    pub(crate) signal: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum StdinMode {
    Null,
}

#[derive(Debug)]
pub(crate) struct SupervisorBuilder {
    binary: OsString,
    args: Vec<OsString>,
    cwd: Option<PathBuf>,
    env: Vec<(OsString, OsString)>,
    stdin: StdinMode,
    parent_death_watch: bool,
}

impl SupervisorBuilder {
    pub(crate) fn new(binary: impl AsRef<OsStr>) -> Self {
        Self {
            binary: binary.as_ref().to_os_string(),
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
            stdin: StdinMode::Null,
            parent_death_watch: true,
        }
    }

    pub(crate) fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|arg| arg.as_ref().to_os_string()));
        self
    }

    pub(crate) fn current_dir(mut self, cwd: impl AsRef<Path>) -> Self {
        self.cwd = Some(cwd.as_ref().to_path_buf());
        self
    }

    pub(crate) fn env(mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.env
            .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
        self
    }

    pub(crate) fn stdin(mut self, stdin: StdinMode) -> Self {
        self.stdin = stdin;
        self
    }

    pub(crate) fn parent_death_watch(mut self, enabled: bool) -> Self {
        self.parent_death_watch = enabled;
        self
    }

    pub(crate) fn spawn(self) -> Result<RunHandle, SupervisorError> {
        let program = self.binary.to_string_lossy().into_owned();
        let mut command = Command::new(&self.binary);
        command.args(&self.args);
        command.stdin(match self.stdin {
            StdinMode::Null => Stdio::null(),
        });
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        if let Some(cwd) = self.cwd {
            command.current_dir(cwd);
        }
        for (key, value) in self.env {
            command.env(key, value);
        }

        // SAFETY: This runs in the child process after fork and before exec.
        // Only async-signal-safe libc calls are made here.
        let _parent_death_watch = self.parent_death_watch;
        unsafe {
            command.pre_exec(move || {
                if libc::setpgid(0, 0) != 0 {
                    return Err(io::Error::last_os_error());
                }

                #[cfg(target_os = "linux")]
                {
                    if _parent_death_watch
                        && libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
                    {
                        return Err(io::Error::last_os_error());
                    }
                }

                Ok(())
            });
        }

        let mut child = command.spawn().map_err(|error| SupervisorError::Spawn {
            program,
            message: error.to_string(),
        })?;

        let pid = child.id().ok_or(SupervisorError::MissingPid)?;
        let pgid = process_group_id(pid);
        let stdout = child
            .stdout
            .take()
            .ok_or(SupervisorError::StdoutUnavailable)?;
        let stderr = child
            .stderr
            .take()
            .ok_or(SupervisorError::StderrUnavailable)?;

        if self.parent_death_watch {
            watch_parent_death(pgid);
        }

        let (wait_tx, wait_rx) = watch::channel(None);
        tokio::spawn(async move {
            let result = child
                .wait()
                .await
                .map(|status| ExitResult {
                    code: status.code(),
                    signal: status.signal(),
                })
                .map_err(|error| SupervisorError::Wait(error.to_string()));
            let _ = wait_tx.send(Some(result));
        });

        Ok(RunHandle {
            inner: Arc::new(RunState {
                pgid,
                stdout: Mutex::new(Some(stdout)),
                stderr: Mutex::new(Some(stderr)),
                wait_rx,
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RunHandle {
    inner: Arc<RunState>,
}

#[derive(Debug)]
struct RunState {
    pgid: i32,
    stdout: Mutex<Option<ChildStdout>>,
    stderr: Mutex<Option<ChildStderr>>,
    wait_rx: watch::Receiver<Option<Result<ExitResult, SupervisorError>>>,
}

impl RunHandle {
    pub(crate) fn stdout(&self) -> Result<ChildStdout, SupervisorError> {
        self.inner
            .stdout
            .lock()
            .expect("stdout mutex poisoned")
            .take()
            .ok_or(SupervisorError::StdoutUnavailable)
    }

    pub(crate) fn stderr(&self) -> Result<ChildStderr, SupervisorError> {
        self.inner
            .stderr
            .lock()
            .expect("stderr mutex poisoned")
            .take()
            .ok_or(SupervisorError::StderrUnavailable)
    }

    pub(crate) async fn wait(&self) -> Result<ExitResult, SupervisorError> {
        let mut rx = self.inner.wait_rx.clone();
        loop {
            let result = { rx.borrow().clone() };
            if let Some(result) = result {
                return result;
            }
            rx.changed()
                .await
                .map_err(|_| SupervisorError::WaitChannelClosed)?;
        }
    }

    pub(crate) fn kill_soft(&self) -> Result<(), SupervisorError> {
        signal_group(self.inner.pgid, libc::SIGTERM)
    }

    pub(crate) fn kill(&self) -> Result<(), SupervisorError> {
        signal_group(self.inner.pgid, libc::SIGKILL)
    }
}

fn process_group_id(pid: u32) -> i32 {
    // SAFETY: getpgid is called with a live child pid returned by spawn.
    let pgid = unsafe { libc::getpgid(pid as libc::pid_t) };
    if pgid > 0 {
        pgid
    } else {
        pid as i32
    }
}

fn signal_group(pgid: i32, signal: i32) -> Result<(), SupervisorError> {
    if pgid <= 0 {
        return Ok(());
    }

    // SAFETY: Negative pid targets the process group. ESRCH is harmless here
    // because it means the group has already exited.
    let rc = unsafe { libc::kill(-pgid, signal) };
    if rc == 0 {
        return Ok(());
    }

    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        return Ok(());
    }

    Err(SupervisorError::Signal {
        signal,
        pgid,
        message: error.to_string(),
    })
}

#[cfg(target_os = "macos")]
pub(crate) fn watch_parent_death(pgid: i32) {
    let parent_pid = unsafe { libc::getppid() };
    if parent_pid <= 1 {
        return;
    }

    let _ = std::thread::Builder::new()
        .name("parent-death-watch".to_string())
        .spawn(move || {
            // SAFETY: kqueue returns a process-local descriptor or -1 on failure.
            let kq = unsafe { libc::kqueue() };
            if kq < 0 {
                return;
            }

            let change = libc::kevent {
                ident: parent_pid as libc::uintptr_t,
                filter: libc::EVFILT_PROC,
                flags: (libc::EV_ADD | libc::EV_ONESHOT) as libc::c_ushort,
                fflags: libc::NOTE_EXIT,
                data: 0,
                udata: std::ptr::null_mut(),
            };
            let mut event = libc::kevent {
                ident: 0,
                filter: 0,
                flags: 0,
                fflags: 0,
                data: 0,
                udata: std::ptr::null_mut(),
            };

            // SAFETY: Pointers refer to valid kevent structs for the duration of
            // this blocking call. A null timeout means wait until parent exit.
            let n = unsafe { libc::kevent(kq, &change, 1, &mut event, 1, std::ptr::null()) };
            // SAFETY: kq is a descriptor returned by kqueue above.
            let _ = unsafe { libc::close(kq) };

            if n > 0 {
                let _ = signal_group(pgid, libc::SIGKILL);
            }
        });
}

#[cfg(target_os = "linux")]
pub(crate) fn watch_parent_death(_pgid: i32) {
    // SAFETY: prctl with PR_SET_PDEATHSIG only changes the current process'
    // parent-death signal. Errors are best-effort and intentionally ignored.
    let _ = unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) };
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub(crate) fn watch_parent_death(_pgid: i32) {}
