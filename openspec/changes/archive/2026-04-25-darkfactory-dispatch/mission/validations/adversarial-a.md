I completed the adversarial review and wrote the full 203-line report to [adversarial-a.md](/tmp/adversarial-a.md).

I could not write to the requested path because the sandbox blocks writes outside `/Users/stuart/.darkfactory/tools/darkfactory-dispatch`; both `apply_patch` and direct shell redirection failed for `/Users/stuart/.darkfactory/openspec/changes/darkfactory-dispatch/mission/validations/adversarial-a.md`.

Key findings in the report:

- [engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs:99): timeout is not truly two-phase and can run an extra grace window.
- [engine.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/engine.rs:78): stdout NDJSON can be dropped when child exit wins the select race.
- [main.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/main.rs:45): `--async` does not detach.
- [cli.rs](/Users/stuart/.darkfactory/tools/darkfactory-dispatch/src/cli.rs:361): lifecycle commands can follow a reused PID.
- [SKILL.md](/Users/stuart/.darkfactory/.claude/skills/mission/SKILL.md:484): mission dispatch leaks dispatcher stderr outside the task log and loses structured failure details.

Verification run: `cargo test` passed.