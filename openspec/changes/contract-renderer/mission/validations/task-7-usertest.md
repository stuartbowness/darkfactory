# User-Test Validation: Task 7

## Result: PASS

## Test Strategy

This is a closing task whose deliverable is the smoke test itself
(`tools/darkfactory-render/tests/smoketest.sh`, run via `make smoke`). The change
is a CLI/infrastructure artifact, so I verified it by RUNNING it against the real
built binary and by READING the script to confirm its assertions are real (not
always-pass). I also ran two non-destructive mutations in TEMP COPIES to prove
the key assertions actually fail when their expectations are violated. No
committed script or fixture was modified.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `make smoke` builds the binary and runs the full suite | Manual (real binary) | PASS |
| 2 | Smoke test exits 0 with 10/10 checks passing | Manual | PASS |
| 3 | Read `smoketest.sh` to confirm it builds, runs `render` + asserts contract markers, runs `shadow` + asserts byte-unchanged via cksum, exits 0 only on all-pass | Manual | PASS |
| 4 | Mutation A (temp copy): change render marker `# Contract` to a non-existent string | Manual | PASS (correctly FAILs, exit 1) |
| 5 | Mutation B (temp copy): append a byte to the existing-contract fixture before the AFTER cksum | Manual | PASS (correctly FAILs, exit 1) |
| 6 | Committed script + fixtures untouched; temp dir cleaned | Manual (cksum + ls) | PASS |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | [execution] `tests/smoketest.sh` builds the binary, runs `render` on a fixture and asserts a contract is emitted, runs `shadow` and asserts the existing contract is unchanged, and exits 0. | PASS | `make smoke` ran `cargo build` then the script, output `=== Results: 10/10 passed ===` / `ALL PASSED`, exit 0. See evidence below. |

### Evidence for Contract Check 1

`make smoke` output (exit 0):

```
=== darkfactory-render smoke tests ===
Building binary...
Binary: ./target/debug/darkfactory-render
Test 1: render emits a contract in the expected format
  PASS  render exits 0
  PASS  render stdout has '# Contract' title
  PASS  render stdout has a '## Task ' section
  PASS  render stdout has a '<!-- Specs: ' comment
  PASS  render stdout has a '- [ ] [' check line
Test 2: shadow leaves the existing contract byte-unchanged
  PASS  shadow (identical) exits 0
  PASS  shadow reports 'no differences'
  PASS  existing contract is byte-unchanged
Test 3: shadow flags a differing contract
  PASS  shadow (different) exits 1
  PASS  shadow (different) emits a non-empty diff
=== Results: 10/10 passed ===
ALL PASSED
```

Each clause of the contract check is satisfied:

- Builds the binary: `cargo build` runs in setup (line 66), exits 2 if the build
  fails or the binary is missing.
- Runs `render` on a fixture and asserts a contract is emitted: Test 1 runs
  `render --specs ... --tasks ... --fragment ...` and asserts the stdout contract
  contains `# Contract`, `## Task `, `<!-- Specs: `, and `- [ ] [`.
- Runs `shadow` and asserts the existing contract is unchanged: Test 2 captures a
  `cksum` of the existing contract BEFORE (line 95) and AFTER (line 102) the
  `shadow` run, asserts they are equal (byte-unchanged), asserts shadow exits 0,
  and asserts it reports "no differences". (Test 3 additionally exercises the
  difference path: a differing contract makes shadow exit 1 with a non-empty
  diff.)
- Exits 0 on all-pass: the summary exits 0 only when `FAIL == 0`, else exit 1
  (lines 128-134); setup failures exit 2.

The assertions are REAL, confirmed by mutation testing in temp copies (committed
files untouched):

- Mutation A: changing the asserted render marker `# Contract` to a string the
  output cannot contain turned that check to FAIL and the script exited 1
  (9/10 passed, FAILED). Proves the render contract-marker assertion is enforced.
- Mutation B: pointing the existing-contract fixture at a temp copy and appending
  one byte to it before the AFTER cksum turned "existing contract is
  byte-unchanged" to FAIL and the script exited 1 (7/10 passed, FAILED). It also
  flipped "shadow (identical) exits 0" and "shadow reports 'no differences'" to
  FAIL, which further proves `shadow` genuinely reads and diffs the existing
  contract rather than ignoring it.

Post-verification: committed `tests/smoketest.sh` and both fixtures have their
original cksums, the temp dir was removed, and the worktree shows only expected
mission-harness scaffolding (no changes to crate code or fixtures).

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A -- all checks pass.
