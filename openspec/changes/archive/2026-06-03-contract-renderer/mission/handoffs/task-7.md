# Handoff: Task 7 - Smoke test (closing task: smoke-test)

## Task
Write `tools/darkfactory-render/tests/smoketest.sh`: a portable POSIX-sh
end-to-end smoke test that builds the binary, runs `render` over a fixture and
asserts a contract is emitted in the expected format, runs `shadow` and asserts
the existing contract is byte-unchanged, and exits 0 on success / non-zero on
any failure. The Makefile's existing `smoke` target runs it.

## Commit SHA
`31c5f15c4cc40c16cd3b057b8a2fc2ca682ad5c5`
Single file changed: `tools/darkfactory-render/tests/smoketest.sh` (+134).
No `openspec/changes/` files staged.

## Approach

### What the script checks (10 checks, 3 groups)
- Setup: `cargo build`, fail loud with exit 2 if build fails or the binary
  (`./target/debug/darkfactory-render`) is missing/not executable.
- Test 1 — render (primary flow): runs
  `render --specs tests/fixtures/cli/specs --tasks tests/fixtures/cli/tasks.md
  --fragment tests/fixtures/cli/fragment.md`, captures stdout + exit code, and
  asserts: exit 0; stdout contains `# Contract`, `## Task `, `<!-- Specs: `, and
  `- [ ] [`.
- Test 2 — shadow read-only / byte-unchanged: `cksum`s
  `contract_identical.md` BEFORE, runs `shadow ... --existing
  contract_identical.md`, `cksum`s AFTER, and asserts: exit 0; stdout reports
  `no differences`; BEFORE checksum == AFTER checksum (byte-unchanged).
- Test 3 — shadow difference path: runs shadow against `contract_different.md`
  and asserts exit 1 + a non-empty diff on stdout.
- Summary: prints `=== Results: N/M passed ===`; exit 1 if any check failed,
  else exit 0.

### How byte-unchanged is asserted
POSIX `cksum` of the existing contract file is captured into `BEFORE` before the
shadow run and `AFTER` afterwards (using only the CRC + size fields via
`cut -d' ' -f1,2`). The check is `[ "$BEFORE" = "$AFTER" ]`. `cksum` is POSIX
and present on both Debian/Ubuntu and macOS, so no GNU/BSD divergence.

### Portability
`#!/bin/sh`, `set -e`, `trap cleanup EXIT` removing a
`mktemp -d /tmp/render-smoke.XXXXXX` temp dir. No GNU-only flags. One BSD gotcha
fixed: the `- [ ] [` substring starts with `-`, so the grep helper uses
`grep -qF -e "$pattern"` to stop BSD grep parsing it as an option.

### How I confirmed the checks are real (TDD verify-then-revert)
I copied the script and ran two mutations, then reverted:
- Mutation 1: changed the render assertion to a marker that does not exist
  (`# NONEXISTENT_MARKER`). Result: `render stdout has '# Contract' title`
  FAILED, suite 9/10, exit 1. Confirms the render-format check is real.
- Mutation 2: injected a write to the existing contract file between the BEFORE
  and AFTER checksums (simulating a non-read-only shadow). Result: `existing
  contract is byte-unchanged` FAILED, suite 9/10, exit 1. Confirms the
  byte-unchanged check actually detects a change. Fixture restored with
  `git checkout`; `git status` clean for fixtures afterward.
After reverting, the unmodified script passes 10/10 and exits 0.

## Commands run + exit codes
- `cargo build` -> exit 0.
- `make smoke` -> `=== Results: 10/10 passed === ALL PASSED`, exit 0.
- `make check` (fmt + clippy -Dwarnings + test) -> exit 0, no warnings.
- `cargo test --quiet` -> 76 passed (8 suites), exit 0.
- Mutation 1 (bad render marker) -> 9/10, exit 1 (expected).
- Mutation 2 (file modified mid-run) -> 9/10, exit 1 (expected).
- `git status --short` -> fixtures clean; only `tools/darkfactory-render/
  tests/smoketest.sh` was new under tools/ (now committed).

## Concerns
None. `make check` does not run the smoke test (smoke is a separate target), so
adding the script does not affect fmt/clippy/test. The `cksum` CRC+size pair is
sufficient to detect any byte change for this single-file read-only assertion.
