# Design decisions

## Separate process supervision from agent instructions

The dispatcher is executable Rust with subprocess tests. Planning and review are
Markdown workflows interpreted by an agent. This avoids duplicating a host
client's conversational machinery, but the workflow's guarantees are weaker
than the supervisor's tested mechanics.

## Prefer a synchronous dispatcher

The current CLI performs one dispatch and returns its result. The earlier async
store and status/wait/result commands were removed. A synchronous interface
reduces lifecycle state and makes failure handling easier to inspect. A caller
that needs background execution must supervise that invocation itself.

## Use process groups for timeout signals

The worker enters a new Unix process group so timeout signals can reach its
ordinary child processes. This is useful resource cleanup, not containment:
processes can leave the group, and abrupt parent/dispatcher death is a separate
problem. See [known limitations](known-limitations.md).

## Validate log setup before starting work

An invalid log destination used to fail after spawning the worker, potentially
leaving it running outside the dispatch loop. Log setup now occurs before spawn.
A regression fixture verifies that an unusable log path produces no worker PID
or output.

## Keep stdout machine-readable

The final JSON result goes to stdout; events and diagnostics go to stderr. This
lets callers parse the result without guessing which line is the final one.
Log capture and event output remain separate concerns.

## Limit shared assumptions during review

Workers receive briefs; reviewers receive acceptance criteria. Separating their
prompt context encourages reviewers to inspect observable behaviour rather than
repeat design intent. The files are not access-restricted, so this is a review
technique rather than an enforced information wall.

## Keep state inspectable

Mission artifacts are files that an operator can read and an agent can revisit.
This supports recovery after a conversation ends, but it is not a transactional
workflow store. Repeated external actions require explicit state checks.

## Preserve user configuration during installation

Symlinks allow skill updates without copying files into several clients.
Preflight checks refuse conflicts and leave existing links to this checkout
unchanged. Switching from another checkout is an explicit operator decision;
the installer does not silently replace that collection.
