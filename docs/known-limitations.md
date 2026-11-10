# Known limitations

## Process cleanup is not containment

Timeout signals target the worker's process group. Descendants that create their
own groups or sessions can escape those signals.

On Linux, parent-death signals target direct processes. They do not guarantee
termination of every descendant in the group if the dispatcher or its parent
dies abruptly. This remains tracked in
[#223](https://github.com/stuartbowness/darkfactory/issues/223).

On macOS, the parent watcher runs inside the dispatcher and watches the
orchestrator. If the dispatcher itself is killed, its watcher cannot continue.
There are also races around parent exit and watcher registration. Do not rely
on this tool as a sandbox or a guarantee that no orphan can survive.

The invalid-log-path startup orphan described in
[#224](https://github.com/stuartbowness/darkfactory/issues/224) is fixed in this
release preparation: log setup happens before spawn, with a regression test.
Other unexpected I/O or event-emission errors after spawn are not a comprehensive
cancellation protocol. Use an external process supervisor for stronger lifecycle
guarantees.

## Output and telemetry

Worker events are observations from the CLI, not independently verified
filesystem changes. Token fields depend on the worker's event schema.
A heartbeat means the dispatcher is alive, not that a model is progressing.

A successful dispatch can recover its requested output from the final model
message. Check the handoff and run project validation before treating a task as
complete. Logs and prompts can contain source, personal data, or secrets; the
dispatcher does not redact them.

## Workflow and compatibility

The full mission is designed for Claude Code's skills, subagents, and plan mode.
Other clients may discover the Markdown but do not necessarily provide the same
execution semantics. Model identifiers in the defaults are operator choices;
select models available to your account.

The offline demo and CI do not call a real model. They test the adapter mechanics,
not current Codex event compatibility or full-mission quality. The opt-in live
smoke test is separate and consumes authenticated model usage.

## Repository history

Release preparation removes generated mission artifacts from the current tree.
It does not erase prior commits, tags, branches, issues, or PRs. Changing the
repository's visibility exposes that historical material too. The release audit
records the scope of automated scanning; a clean secret scan is not a claim that
every historical discussion is suitable for publication.
