# Worker Brief: Task 7 - Smoke test (closing task: smoke-test)

## Your Task

Write `tools/darkfactory-render/tests/smoketest.sh`: an end-to-end smoke test of the primary user flow against the REAL built binary, no internal mocks. It builds the tool, runs `render` over a fixture and asserts a contract is emitted in the expected format, runs `shadow` and asserts it leaves the existing contract unchanged, and exits 0 on success / non-zero on any failure.

The Makefile already has a `smoke` target (`smoke: ./tests/smoketest.sh`), so `make smoke` will run your script. Make the script executable (`chmod +x`).

## Requirements

1. **Portable POSIX shell.** Start with `#!/bin/sh` and `set -e`. The repo's scripts must run on BOTH Debian/Ubuntu (GNU coreutils) and macOS (BSD coreutils) — avoid GNU-only flags: use `mktemp -d /tmp/render-smoke.XXXXXX` (not `--suffix`), no `sed -i`, no `readlink -f`, no `grep -P`, no `stat -c`. Use a `trap cleanup EXIT` to remove the temp dir. Follow the structure of `tools/darkfactory-dispatch/tests/smoketest.sh` (a `check()` helper that increments PASS/FAIL and prints a line; a final `=== Results: N/M passed ===` summary; exit 1 if any FAIL).

2. **Build the binary.** Run `cargo build` and use `./target/debug/darkfactory-render` (the script runs from the crate root via `make smoke`). Fail loud (exit 2) if the build fails or the binary is missing.

3. **render check (primary flow).** Run:
   `./target/debug/darkfactory-render render --specs tests/fixtures/cli/specs --tasks tests/fixtures/cli/tasks.md --fragment tests/fixtures/cli/fragment.md`
   Capture stdout and the exit code. Assert: exit code 0; stdout contains the expected-format markers — a `# Contract` title, at least one `## Task ` section, at least one `<!-- Specs: ` comment, and at least one `- [ ] [` check line. (You may also render the migrated fixture `tests/fixtures/migrated/...` as a second case if useful.)

4. **shadow check (read-only).** The committed `tests/fixtures/cli/contract_identical.md` is a captured render of that same fixture. Run:
   `./target/debug/darkfactory-render shadow --specs tests/fixtures/cli/specs --tasks tests/fixtures/cli/tasks.md --fragment tests/fixtures/cli/fragment.md --existing tests/fixtures/cli/contract_identical.md`
   Assert: it leaves the existing contract file BYTE-UNCHANGED. Capture a checksum of `contract_identical.md` BEFORE and AFTER the shadow run (use `cksum` or `shasum` or `md5` — pick one available on both platforms; `cksum` is POSIX and present on both) and assert they are equal. Also assert the shadow exit code is 0 and stdout reports no differences (since the existing file is the render of the same inputs). Optionally also run shadow against `contract_different.md` and assert exit 1 + a non-empty diff, to exercise the difference path.

5. **Exit code.** Exit 0 only if every check passed; non-zero otherwise. Use exit 2 for environment/setup failures (build failed, binary missing).

## Project Context

(from proposal.md) This is the mandatory smoke-test closing task: one end-to-end test of the renderer's primary flow (render + shadow) against the real binary, proving the tool runs as an operator would invoke it. The shadow check also re-confirms the read-only guarantee that underpins the shadow/cutover mechanism.

## What Previous Tasks Built (the CLI you are testing)
- The `render`/`shadow` CLI (Task 5, commit d5e5a4f4; library + thin bin, stdlib args, no external deps): subcommands `render --specs <dir> --tasks <md> --fragment <md>` and `shadow --specs <dir> --tasks <md> --fragment <md> --existing <contract>`. Exit codes: 0 success / 1 shadow-differs / 2 RenderError-or-usage. `render` prints the contract to stdout, warnings to stderr; `shadow` is strictly read-only.
- Fixtures under `tools/darkfactory-render/tests/fixtures/cli/`: `specs/` (a `widget/spec.md`), `tasks.md`, `fragment.md`, `contract_identical.md` (the captured render of that fixture), `contract_different.md`. Also `tests/fixtures/migrated/`.
- The standing fragment: `templates/closing-task-checks.md`.
- `make check` (fmt+clippy-Dwarnings+test) is green (76 tests). Your shell script must not break it; `make check` does NOT run the smoke test (it is run via `make smoke`).

## Procedures

**Before writing the script**, invoke the `/tdd` skill (Skill tool). The deliverable here IS a test (the smoke script), so apply the spirit of /tdd: after writing it, confirm it actually PASSES against the working binary AND that it would FAIL if a check were violated (e.g. temporarily point the render at a bad fixture or assert against wrong text to confirm the check is real, then revert). Report that you verified the assertions are meaningful.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/make/git in the foreground.
3. Create the script with the Write tool at `/Users/home/.darkfactory-2-build/tools/darkfactory-render/tests/smoketest.sh`, then `chmod +x` it.

### Verify
- Run `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make smoke` and confirm it prints all PASS and exits 0. Paste the output + exit code into the handoff.
- Run `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` and confirm it is still green (the script file should not affect fmt/clippy/test). Paste the result.
- Confirm `contract_identical.md` is unchanged after running the smoke test (`git status` clean for fixtures).

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-7] end-to-end smoke test (render + read-only shadow)"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-7.md`
Format: Task; Commit SHA; Approach (what the script checks, how it asserts byte-unchanged, how you confirmed the checks are real); Commands run + exit codes (incl. `make smoke` and `make check`); Concerns.

## Final message
Reply with the commit SHA, a summary of the smoke test's checks, confirmation `make smoke` passes (exit 0) and `make check` is still green, and that you verified the assertions are meaningful (would fail if violated).
