# Darkfactory

**Coding-agent workflows with a process supervisor you can inspect and test.**

Darkfactory combines a Rust supervisor for Codex workers with a set of skills for
planning, building, reviewing, and recovering software work. The supervisor owns
process lifecycle and structured results. The skills guide an agent through the
engineering workflow.

Built by [Stuart Bowness](https://github.com/stuartbowness), with AI coding
assistance and credited upstream material. [MIT licensed](LICENSE); see
[third-party notices](THIRD_PARTY_NOTICES.md).

## Why this exists

Delegating a coding task creates two different problems: deciding what good work
looks like, and knowing what happened to the worker process.

A worker can hang, exit without a useful result, or emit partial output. A
plausible implementation can still miss the requested behaviour. Darkfactory
addresses those problems at different layers:

| Layer | Responsibility | Implementation |
| --- | --- | --- |
| Dispatcher | Process groups, two-stage timeouts, heartbeats, event parsing, result reporting | Rust + Tokio |
| Mission workflow | Clarification, briefs, validation criteria, review, bounded fix attempts | Markdown skills interpreted by a coding agent |
| Operator | Scope, credentials, approvals, final review and merge | You |

This is a developer tool, not an unattended production service. A completed
dispatch means the worker process met the dispatch completion rules; it does
not mean its code is correct.

## Try the supervisor without an API key

Requirements: **Linux or macOS**, Rust stable, Python 3.11+, and Bash. Cargo needs
network access on its first build to download dependencies; the demo itself makes
no model calls.

```sh
git clone git@github.com:stuartbowness/darkfactory.git
cd darkfactory
python3 examples/dispatch-demo.py
```

The demo runs the real dispatcher against a local fixture named `codex`. It
exercises success, worker failure, and timeout. It does not replace your installed
Codex, install skills, or change your agent settings.

```text
Offline fixture demo: no model calls; usage values are synthetic.
success: status=completed, exit=0, output_exists=true
failed: status=failed, exit=1, output_exists=true
sleep: status=timed_out, exit=1, output_exists=false
All three process-lifecycle checks passed.
```

The failed worker deliberately writes output before exiting unsuccessfully.
The dispatcher still reports failure: an output file alone is not success.

## For engineers reviewing the project

Start with [the architecture](ARCH.md), then follow these boundaries:

- [`engine.rs`](tools/darkfactory-dispatch/src/engine.rs) coordinates dispatch,
  consumes worker events, enforces deadlines, and constructs the result.
- [`supervisor.rs`](tools/darkfactory-dispatch/src/supervisor.rs) owns process
  groups, signals, and child exit observation.
- [The process tests](tools/darkfactory-dispatch/tests/engine.rs) use real fixture
  subprocesses to exercise timeouts, output recovery, malformed events, and failure.
- [The mission skill](skills/mission/SKILL.md) describes the agent-operated workflow;
  [the contract template](skills/mission/templates/contract.md) defines observable
  acceptance criteria.
- [Installer tests](tests/test_install.py) verify conflict detection and
  preservation of existing user configuration.

The [dispatcher reference](tools/darkfactory-dispatch/README.md) documents the CLI,
events, result schema, and limits. [Contributing](CONTRIBUTING.md) lists the checks.

## Install the workflow

The full mission workflow is designed for Claude Code, with Codex as the default
external worker. It also needs OpenSpec, GitHub CLI, jq, and the target project's
build/test tools. Authenticate the clients separately. Model availability and
CLI compatibility depend on your environment.

```sh
# Review the proposed links before installing.
sh install.sh --dry-run
sh install.sh
make -C tools/darkfactory-dispatch install
```

The installer links skills into `~/.claude/skills`, `~/.codex/skills`, and
`~/.agents/skills`, plus Claude agent definitions into `~/.claude/agents`.
It checks all destinations first, preserves matching links, and stops on a
conflicting file, directory, or unrelated symlink. It does not overwrite another
skill collection. `--prefix /absolute/path` supports a disposable installation.

The full mission relies on Claude Code's subagent and plan-mode behaviour;
symlinking skills into another client does not establish equivalent support.
The installer does not install dependencies or initialize a project.

Restart your agent client and use `/mission-init` in a target repository to check
prerequisites. Review `config.yaml` and `agents/darkfactory-claude.md` for the
model choices. See [the workflow guide](docs/workflow.md) for configuration,
individual skills, and recovery.

## Use a mission

```text
/mission --basic Fix the CSV export escaping bug
/mission Add an authenticated API endpoint with integration tests
/mission-status
/mission-resume
```

Basic mode keeps the build/validate loop in one conversation. Full mode prepares
planning artifacts and a validation contract, dispatches one worker at a time,
runs reviewers, and caps repeated fix attempts before returning to the operator.
The workflow can create commits and draft PRs; run it only in a repository where
those actions are intended.

## Limits worth understanding

- **Prompt separation is not access control.** Workers are not given the
  validation contract in their briefs, but they share the repository filesystem.
- **Heartbeats report the supervisor's activity.** They are not proof that a model
  is making useful progress.
- **Process groups are not a sandbox.** Codex permissions are a separate boundary.
  Descendants can escape a group; parent-death cleanup has platform-specific
  limitations documented in [known limitations](docs/known-limitations.md).
- **The dispatcher is synchronous-only.** There is no async job store, dashboard,
  distributed queue, or durable process resumption. Mission recovery uses files
  and agent judgment.
- **Offline checks cover mechanics.** They do not benchmark model quality or
  validate a complete live mission. Live runs can consume paid model usage.

The current tree contains source, reusable skills, and focused documentation.
Generated mission records were removed from the working tree during release
preparation; existing Git history was preserved.
