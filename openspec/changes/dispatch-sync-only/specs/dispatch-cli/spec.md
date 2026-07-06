# dispatch-cli

## ADDED Requirements

### Requirement: Async flag and lifecycle subcommands are removed
The CLI SHALL NOT accept the `--async` flag or the `status`, `wait`, and `result` subcommands. Invoking any of them SHALL fail at the CLI parser with a non-zero exit and a usage error on stderr, emit no JSON result object on stdout, and create no files.

#### Scenario: --async is rejected
- **WHEN** `darkfactory-dispatch --async` is invoked with otherwise valid arguments
- **THEN** the process exits non-zero, prints a usage/error message to stderr, prints no JSON result to stdout, and creates no dispatch files

#### Scenario: lifecycle subcommands are rejected
- **WHEN** `darkfactory-dispatch status <id>`, `darkfactory-dispatch wait <id>`, or `darkfactory-dispatch result <id>` is invoked
- **THEN** the process exits non-zero with a usage/error message and produces no JSON result

### Requirement: Retained flags continue to work
The CLI SHALL accept `--model`, `--effort`, `--timeout`, `--sandbox`, `--config`, `--prompt-file`, `--prompt`, `--output`, `--cwd`, `--log`, `--json`, `--grace`, and `--heartbeat`.

#### Scenario: A normal synchronous dispatch runs
- **WHEN** a dispatch is invoked with the retained flags (including `--json`)
- **THEN** the worker runs under supervision and the dispatcher emits a single JSON result object

#### Scenario: --output writes the worker's output artifact to the given path
- **WHEN** a dispatch is invoked with `--output p`
- **THEN** the worker's output artifact is written to `p`, and the JSON result's `output` field is `p` with `output_exists` true

### Requirement: No automatic filesystem side effects
A dispatch SHALL NOT create any files under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/`. User-specified `--log` and `--output` paths are written as directed and are out of scope for this prohibition.

#### Scenario: A dispatch leaves no store or status artifacts
- **WHEN** a dispatch completes with no user-specified `--log`/`--output` path under those locations
- **THEN** no files are created under `/tmp/darkfactory-dispatch/` or `~/.darkfactory/dispatches/` by that run

#### Scenario: A planted control file cannot abort a dispatch
- **WHEN** a `control.json` containing an abort request is present under the old `/tmp` dispatch path while a dispatch runs
- **THEN** the dispatch ignores it and runs to normal completion (the control-file abort channel no longer exists)

### Requirement: Help text omits the removed surface
The CLI `--help` output SHALL NOT list the removed `--async` flag or the lifecycle subcommands named `status`, `wait`, and `result`. (Incidental prose about the JSON result object is acceptable.)

#### Scenario: Help lists no async surface
- **WHEN** `darkfactory-dispatch --help` is shown
- **THEN** the help's flag and subcommand listing includes no `--async` flag and no `status`/`wait`/`result` subcommands (checking the listed flags/subcommands, not arbitrary prose)
