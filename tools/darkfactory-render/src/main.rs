//! `darkfactory-render` CLI entry point.
//!
//! A thin shell: it collects the process arguments (dropping the program name)
//! and the real stdout/stderr handles, hands them to [`cli::run`], and exits
//! with the code it returns. All logic lives in [`darkfactory_render::cli`] so
//! it can be unit-tested in-process.
use std::io::{self, Write};
use std::process::ExitCode;

use darkfactory_render::cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();

    let code = cli::run(&args, &mut out, &mut err);

    let _ = out.flush();
    let _ = err.flush();

    // Exit codes are small non-negative integers; `as u8` is the documented
    // process exit byte.
    ExitCode::from(code as u8)
}
