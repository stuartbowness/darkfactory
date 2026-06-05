//! The `darkfactory-render` command-line transport (design: thin CLI).
//!
//! This is the only layer that touches the filesystem, prints, and chooses exit
//! codes. It owns NO domain logic: it reads the specs directory, the tasks file,
//! the fragment, and (for `shadow`) the existing contract, hands their text to
//! the pure `core` pipeline (`parse_* -> build_model -> render`, then `compare`
//! for shadow), and translates the result to stdout / stderr / an exit code.
//!
//! It is the sole translator of [`RenderError`](crate::core::types::RenderError)
//! to stderr and a non-zero exit; `core` never prints. `shadow` is strictly
//! read-only: it reads files and reports a diff, and never writes or modifies
//! anything (no auto-cutover).
//!
//! Argument parsing is stdlib-only (`std::env::args`); the crate adds no CLI
//! dependency.

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::core::coverage::{analyze, check_duplicate_tasks, render_report};
use crate::core::parse::{parse_fragment, parse_specs, parse_tasks};
use crate::core::render::{build_model, render};
use crate::core::shadow::{compare, Diff};
use crate::core::types::{RenderError, RenderedContract};

/// Exit code for a usage error (bad arguments or unknown subcommand).
pub const EXIT_USAGE: i32 = 2;
/// Exit code for a `RenderError` (an IO or pipeline failure).
///
/// Shared by `render` and `shadow` so a render failure is distinguishable from
/// `shadow`'s "differences found" exit (`EXIT_DIFFERENT`).
pub const EXIT_RENDER_ERROR: i32 = 2;
/// Exit code for `shadow` when the rendered contract differs from the existing
/// one. Distinct from `EXIT_RENDER_ERROR` so the cutover gate can script it.
pub const EXIT_DIFFERENT: i32 = 1;
/// Exit code for `coverage` when the report contains one or more gaps.
///
/// Distinct from [`EXIT_USAGE`]/[`EXIT_RENDER_ERROR`] (`2`) and `shadow`'s
/// [`EXIT_DIFFERENT`] (`1`) so the orchestrator can mechanically tell a clean
/// audit (`0`) from gaps found (`3`) from a tool error (`2`). A gaps-found run
/// is a tolerant, non-failing outcome: the report still reaches stdout.
pub const EXIT_GAPS: i32 = 3;

const USAGE: &str = "\
usage:
  darkfactory-render render --specs <dir> --tasks <md> --fragment <md>
  darkfactory-render shadow --specs <dir> --tasks <md> --fragment <md> --existing <contract_md>
  darkfactory-render coverage --specs <dir> --tasks <md>";

/// Run the CLI against `args` (the process arguments WITHOUT the program name),
/// writing to `out` (stdout) and `err` (stderr), and return the process exit
/// code.
///
/// Splitting the streams out as parameters keeps the whole CLI testable in
/// process: a caller can pass buffers and assert on them. `main` passes the real
/// stdout/stderr locks.
///
/// Exit codes: success is 0; a usage error is [`EXIT_USAGE`]; a
/// [`RenderError`] (including a failed file read) is [`EXIT_RENDER_ERROR`];
/// `shadow` returns [`EXIT_DIFFERENT`] when the contracts differ.
pub fn run<O: Write, E: Write>(args: &[String], out: &mut O, err: &mut E) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        let _ = writeln!(err, "error: missing subcommand\n{USAGE}");
        return EXIT_USAGE;
    };

    match command.as_str() {
        "render" => run_render(rest, out, err),
        "shadow" => run_shadow(rest, out, err),
        "coverage" => run_coverage(rest, out, err),
        other => {
            let _ = writeln!(err, "error: unknown subcommand `{other}`\n{USAGE}");
            EXIT_USAGE
        }
    }
}

/// The `render` subcommand: render the contract to stdout, warnings to stderr.
fn run_render<O: Write, E: Write>(args: &[String], out: &mut O, err: &mut E) -> i32 {
    let parsed = match RenderArgs::parse(args) {
        Ok(parsed) => parsed,
        Err(message) => {
            let _ = writeln!(err, "error: {message}\n{USAGE}");
            return EXIT_USAGE;
        }
    };

    let rendered = match render_pipeline(&parsed.specs, &parsed.tasks, &parsed.fragment) {
        Ok(rendered) => rendered,
        Err(message) => {
            let _ = writeln!(err, "error: {message}");
            return EXIT_RENDER_ERROR;
        }
    };

    emit_warnings(&rendered, err);
    let _ = write!(out, "{}", rendered.to_markdown());
    0
}

/// The `shadow` subcommand: render, diff against the existing contract, and
/// report the diff WITHOUT modifying any file.
fn run_shadow<O: Write, E: Write>(args: &[String], out: &mut O, err: &mut E) -> i32 {
    let parsed = match ShadowArgs::parse(args) {
        Ok(parsed) => parsed,
        Err(message) => {
            let _ = writeln!(err, "error: {message}\n{USAGE}");
            return EXIT_USAGE;
        }
    };

    let rendered = match render_pipeline(&parsed.specs, &parsed.tasks, &parsed.fragment) {
        Ok(rendered) => rendered,
        Err(message) => {
            let _ = writeln!(err, "error: {message}");
            return EXIT_RENDER_ERROR;
        }
    };

    // Read-only: this is the only filesystem touch in shadow, and it is a read.
    let existing = match read_to_string(&parsed.existing) {
        Ok(existing) => existing,
        Err(message) => {
            let _ = writeln!(err, "error: {message}");
            return EXIT_RENDER_ERROR;
        }
    };

    emit_warnings(&rendered, err);

    match compare(&rendered.to_markdown(), &existing) {
        Diff::Identical => {
            let _ = writeln!(out, "no differences");
            0
        }
        Diff::Different(diff) => {
            let _ = writeln!(out, "{diff}");
            EXIT_DIFFERENT
        }
    }
}

/// The `coverage` subcommand: run the tolerant coverage pipeline and print the
/// Gap report to stdout.
///
/// Exit codes: `0` when the report has no gaps, [`EXIT_GAPS`] (`3`) when it has
/// one or more, and [`EXIT_USAGE`]/[`EXIT_RENDER_ERROR`] (`2`) for a usage or
/// parse/IO error. A dangling acceptance link is a gap (exit `3`), never a parse
/// error: `analyze` is tolerant where `render` fails loud.
fn run_coverage<O: Write, E: Write>(args: &[String], out: &mut O, err: &mut E) -> i32 {
    let parsed = match CoverageArgs::parse(args) {
        Ok(parsed) => parsed,
        Err(message) => {
            let _ = writeln!(err, "error: {message}\n{USAGE}");
            return EXIT_USAGE;
        }
    };

    let report = match coverage_pipeline(&parsed.specs, &parsed.tasks) {
        Ok(report) => report,
        Err(message) => {
            let _ = writeln!(err, "error: {message}");
            return EXIT_RENDER_ERROR;
        }
    };

    let has_gaps = !report.gaps.is_empty();
    let _ = write!(out, "{}", render_report(&report));
    if has_gaps {
        EXIT_GAPS
    } else {
        0
    }
}

/// Read the specs directory and tasks file, then run the tolerant coverage
/// pipeline: `parse_specs`/`parse_tasks` -> `check_duplicate_tasks` ->
/// `analyze`.
///
/// All file IO lives here; the returned `String` on the error path is the
/// actionable message the CLI prints. A duplicate task number is a fail-loud
/// input error, mapped to the parse-error exit; a dangling acceptance link is
/// not — `analyze` records it as a gap.
fn coverage_pipeline(
    specs_dir: &str,
    tasks_path: &str,
) -> Result<crate::core::types::CoverageReport, String> {
    let scenarios = read_scenarios(specs_dir)?;
    let tasks_md = read_to_string(tasks_path)?;
    let tasks = parse_tasks(&tasks_md).map_err(render_error_message)?;
    check_duplicate_tasks(&tasks).map_err(render_error_message)?;
    Ok(analyze(&scenarios, &tasks))
}

/// Read every `<specs_dir>/<capability>/spec.md`, parse the tasks and fragment,
/// and run `build_model` then `render`.
///
/// All file IO lives here; the returned `String` on the error path is the
/// actionable message the CLI prints. A failed read or a malformed document
/// surfaces as `Err`, never a partial render.
fn render_pipeline(
    specs_dir: &str,
    tasks_path: &str,
    fragment_path: &str,
) -> Result<RenderedContract, String> {
    let scenarios = read_scenarios(specs_dir)?;
    let tasks_md = read_to_string(tasks_path)?;
    let tasks = parse_tasks(&tasks_md).map_err(render_error_message)?;
    let fragment_md = read_to_string(fragment_path)?;
    let fragment = parse_fragment(&fragment_md).map_err(render_error_message)?;
    let model = build_model(&scenarios, &tasks, &fragment).map_err(render_error_message)?;
    render(&model).map_err(render_error_message)
}

/// Collect scenarios from every capability subdirectory of `specs_dir`.
///
/// Each immediate subdirectory `<capability>/` whose `spec.md` exists is parsed
/// with `parse_specs(<capability>, content)`; the capability is the directory
/// name. Subdirectories are visited in sorted name order so the collected
/// scenario list (and therefore the rendered contract) is deterministic across
/// filesystems. A subdirectory without a `spec.md` is skipped.
fn read_scenarios(specs_dir: &str) -> Result<Vec<crate::core::types::RequirementScenario>, String> {
    let entries =
        fs::read_dir(specs_dir).map_err(|e| format!("cannot read specs dir `{specs_dir}`: {e}"))?;

    let mut capabilities: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("cannot read specs dir `{specs_dir}`: {e}"))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(capability) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if path.join("spec.md").is_file() {
            capabilities.push(capability.to_string());
        }
    }
    capabilities.sort();

    let mut scenarios = Vec::new();
    for capability in capabilities {
        let spec_path = Path::new(specs_dir).join(&capability).join("spec.md");
        let content = read_to_string(spec_path.to_string_lossy().as_ref())?;
        let parsed = parse_specs(&capability, &content).map_err(render_error_message)?;
        scenarios.extend(parsed);
    }
    Ok(scenarios)
}

/// Read a file to a string, mapping an IO failure to an actionable message.
fn read_to_string(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read `{path}`: {e}"))
}

/// Format a [`RenderError`] as the single actionable line the CLI prints.
fn render_error_message(error: RenderError) -> String {
    error.to_string()
}

/// Write each render warning to stderr, one per line, prefixed `warning:`.
fn emit_warnings<E: Write>(rendered: &RenderedContract, err: &mut E) {
    for warning in rendered.warnings() {
        let _ = writeln!(err, "warning: {warning}");
    }
}

/// The parsed flags for `render`.
struct RenderArgs {
    specs: String,
    tasks: String,
    fragment: String,
}

impl RenderArgs {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut flags = Flags::parse(args)?;
        let parsed = Self {
            specs: flags.take("--specs")?,
            tasks: flags.take("--tasks")?,
            fragment: flags.take("--fragment")?,
        };
        flags.finish()?;
        Ok(parsed)
    }
}

/// The parsed flags for `shadow` (the `render` flags plus `--existing`).
struct ShadowArgs {
    specs: String,
    tasks: String,
    fragment: String,
    existing: String,
}

impl ShadowArgs {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut flags = Flags::parse(args)?;
        let parsed = Self {
            specs: flags.take("--specs")?,
            tasks: flags.take("--tasks")?,
            fragment: flags.take("--fragment")?,
            existing: flags.take("--existing")?,
        };
        flags.finish()?;
        Ok(parsed)
    }
}

/// The parsed flags for `coverage` (just `--specs` and `--tasks`; no fragment
/// or existing contract, since coverage neither renders nor diffs).
struct CoverageArgs {
    specs: String,
    tasks: String,
}

impl CoverageArgs {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut flags = Flags::parse(args)?;
        let parsed = Self {
            specs: flags.take("--specs")?,
            tasks: flags.take("--tasks")?,
        };
        flags.finish()?;
        Ok(parsed)
    }
}

/// A minimal `--flag value` parser over the argument slice.
///
/// Collects `--flag value` pairs into a list; [`take`](Flags::take) consumes a
/// required flag by name and [`finish`](Flags::finish) fails loud on any leftover
/// (unknown or duplicate) flag. Stdlib only — no `clap`.
struct Flags {
    pairs: Vec<(String, String)>,
}

impl Flags {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut pairs = Vec::new();
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if !arg.starts_with("--") {
                return Err(format!("unexpected argument `{arg}`"));
            }
            let value = iter
                .next()
                .ok_or_else(|| format!("flag `{arg}` needs a value"))?;
            pairs.push((arg.clone(), value.clone()));
        }
        Ok(Self { pairs })
    }

    /// Consume the value of a required flag, removing it from the pool.
    fn take(&mut self, name: &str) -> Result<String, String> {
        let position = self
            .pairs
            .iter()
            .position(|(flag, _)| flag == name)
            .ok_or_else(|| format!("missing required flag `{name}`"))?;
        Ok(self.pairs.remove(position).1)
    }

    /// Fail loud if any flag was left unconsumed (unknown or duplicate).
    fn finish(self) -> Result<(), String> {
        match self.pairs.first() {
            None => Ok(()),
            Some((flag, _)) => Err(format!("unexpected flag `{flag}`")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_subcommand_is_a_usage_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run(&["wat".to_string()], &mut out, &mut err);
        assert_eq!(code, EXIT_USAGE);
        let stderr = String::from_utf8(err).unwrap();
        assert!(
            stderr.contains("unknown subcommand `wat`"),
            "stderr:\n{stderr}"
        );
        assert!(stderr.contains("usage:"), "usage must be shown:\n{stderr}");
    }

    #[test]
    fn no_subcommand_is_a_usage_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run(&[], &mut out, &mut err);
        assert_eq!(code, EXIT_USAGE);
        assert!(String::from_utf8(err)
            .unwrap()
            .contains("missing subcommand"));
    }

    #[test]
    fn render_missing_a_required_flag_is_a_usage_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let args = [
            "render".to_string(),
            "--specs".to_string(),
            "/x".to_string(),
            "--tasks".to_string(),
            "/y".to_string(),
            // --fragment omitted
        ];
        let code = run(&args, &mut out, &mut err);
        assert_eq!(code, EXIT_USAGE);
        assert!(String::from_utf8(err)
            .unwrap()
            .contains("missing required flag `--fragment`"));
    }

    #[test]
    fn an_unknown_flag_is_a_usage_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let args = [
            "render".to_string(),
            "--specs".to_string(),
            "/x".to_string(),
            "--tasks".to_string(),
            "/y".to_string(),
            "--fragment".to_string(),
            "/z".to_string(),
            "--bogus".to_string(),
            "v".to_string(),
        ];
        let code = run(&args, &mut out, &mut err);
        assert_eq!(code, EXIT_USAGE);
        assert!(String::from_utf8(err)
            .unwrap()
            .contains("unexpected flag `--bogus`"));
    }

    #[test]
    fn flags_parse_in_any_order() {
        let args = [
            "--tasks".to_string(),
            "t".to_string(),
            "--fragment".to_string(),
            "f".to_string(),
            "--specs".to_string(),
            "s".to_string(),
        ];
        let parsed = RenderArgs::parse(&args).expect("flags parse in any order");
        assert_eq!(parsed.specs, "s");
        assert_eq!(parsed.tasks, "t");
        assert_eq!(parsed.fragment, "f");
    }
}
