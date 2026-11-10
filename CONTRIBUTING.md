# Contributing

Start with [ARCH.md](ARCH.md) for the runtime and workflow boundaries. Keep
changes focused: explain a concrete failure or workflow problem, the resulting
behaviour, and how it was checked.

## Local verification

Requirements: Rust stable with rustfmt/Clippy, Python 3.11+, a POSIX shell, Bash,
and jq. The first Cargo build downloads dependencies; the checks do not need
model credentials, a paid API, or access to a real GitHub project.

```sh
python3 -m unittest discover -s tests -v
sh skills/lint-audit/tests/audit-sh.sh
sh skills/lint-audit/tests/validate-rules-json.sh
make -C tools/darkfactory-dispatch check
python3 examples/dispatch-demo.py
```

CI runs the same checks on Linux and macOS. The explicit ignored Linux test in
`tests/supervisor.rs` is a helper process invoked by its parent test, not omitted
product coverage. Counts differ between platforms and some module tests are
compiled into more than one test target.

## What to include in a change

- For process-lifecycle changes, use real subprocess fixtures and verify their
  observable exit, signals, and output. Avoid a test that merely mirrors the code.
- For installer changes, use `--prefix` with a disposable directory. Never run
  tests against the user's real home.
- For skill changes, update the relevant workflow documentation. Prompt
  instructions do not establish an enforced runtime guarantee.
- Keep logs, account details, customer data, and local mission output out of Git.
  Add small synthetic examples under `examples/` instead.
- Preserve upstream notices when adapting bundled skills.

The live smoke test (`make -C tools/darkfactory-dispatch smoke`) runs authenticated
Codex and may consume usage. It is opt-in and is not part of CI. A successful
offline run does not establish that a particular model or CLI version works.
