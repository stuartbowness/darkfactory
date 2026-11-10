# Architecture

Darkfactory has two execution layers: a deterministic Rust process supervisor and
an agent-interpreted engineering workflow. Keeping them separate makes the
runtime testable without treating prompt instructions as runtime guarantees.

## Runtime: one synchronous dispatch

```text
CLI arguments → DispatchSpec → engine::dispatch
                                  │
                            SupervisorBuilder
                                  │
                  codex exec in a new process group
                         │                 │
                    stdout NDJSON       stderr
                         └──────┬──────────┘
                         event tracker + log
                                │
                 DispatchResult JSON / exit status
```

| Module | Owns |
| --- | --- |
| `cli.rs` | Argument validation, defaults, result formatting, exit-code mapping |
| `engine.rs` | Dispatch coordination, event parsing, timeout loop, output recovery |
| `supervisor.rs` | Spawning, process groups, signalling, child exit observation |
| `event.rs` | Structured event emission and heartbeat formatting |
| `types.rs` | Dispatch specifications, outcomes, errors, usage and activity records |

The engine validates the log destination before spawning. It passes arguments as
an argument vector, closes child stdin, and reads stdout/stderr separately.
In JSON mode, stdout contains the final result; diagnostics and live events go
to stderr.

A soft deadline sends SIGTERM to the worker's process group. A later hard
deadline sends SIGKILL. A worker that exits after the soft deadline is still
reported as timed out. The stdout drain after child exit is bounded because a
descendant may keep the pipe open.

The worker's final message can recover a missing requested output file. Existing
output is not overwritten by that fallback. Neither an output file nor an exit
code proves that the generated software satisfies its task.

There is no background dispatch store. Callers retain the result, logs, and any
mission records they need. [Known limitations](docs/known-limitations.md) describes
parent-death and error-path cleanup boundaries.

## Workflow: orchestrator, worker, reviewers

The full mission skill guides a Claude Code session through:

```text
clarify → shared terminology → proposal/specs/design/tasks
       → validation contract → worker → scrutiny → user-test
                                  ↑                    │
                                  └── bounded fixes ───┘
       → whole-project checks → draft PR → independent review
```

The orchestrator plans, presents review gates, dispatches one implementation task
at a time, and tracks results. A worker receives a brief and produces a commit
and handoff. Reviewers receive validation criteria and inspect the result.

These are instructions interpreted by agents. They are not a compiled workflow
engine: adherence, timing, and recovery depend on the host client and model.
The circuit breaker is a workflow rule, not a Rust-enforced retry counter.

### Context separation

Worker briefs omit the validation contract. Reviewer prompts omit the proposal
and design. The aim is to reduce shared assumptions and checklist gaming.

All roles may still read the repository. This is selective prompt context,
**not filesystem isolation or an enforced information barrier**. Independent
prompts also do not guarantee independent model errors.

### File-backed recovery

```text
openspec/changes/<change>/
  proposal.md, specs/, design.md, tasks.md
  coverage-audit.md, contract.md, reviews/
  mission/
    config.yaml, progress.md
    briefs/, handoffs/, validations/, issues/, logs/
```

`mission-resume` reads these artifacts to infer the next step and checks whether
implementation and validation are complete. It does not resurrect a dead
process, provide transactional writes, or guarantee exactly-once execution.
External side effects must be checked before repeating work.

### Basic mode

`mission/basic/SKILL.md` keeps planning, implementation, and checks in one agent
session. It is suited to a bounded task. It does not provide independent review
or the full mission's context separation. Its failure path can commit partial
work and report failed checks; a commit is not a passing release.

## Installation and configuration

The installer creates direct symlinks into three clients' skill directories and
Claude's agent directory. Existing matching links, including legacy chained
links, are left intact. Conflicts are detected before installation; failures
from filesystem operations are propagated. Installation is not transactional
against concurrent filesystem changes.

`--prefix` makes installation testable without modifying the user's home.
Dependencies and authentication are deliberately outside the installer.

The global `config.yaml` holds the default Codex model used by skills. Mission
configuration assigns role engines. `agents/darkfactory-claude.md` configures
Claude subagents. The standalone dispatcher also has its own CLI default; pass
`--model` explicitly when you want reproducible model selection.

## Verification boundaries

Rust checks exercise real process fixtures, signals, parsing, output recovery,
and CLI results. Installer tests use disposable directories. Lint-audit tests use
synthetic TypeScript projects. CI covers Linux and macOS; it does not call live
models or execute a complete mission.

The repository release tag versions the overall project. The internal crate
version remains 1.0.0 by convention and does not identify the repository release.

See [design decisions](docs/design-decisions.md) for the main tradeoffs and
[provenance](docs/provenance.md) for upstream influences.
