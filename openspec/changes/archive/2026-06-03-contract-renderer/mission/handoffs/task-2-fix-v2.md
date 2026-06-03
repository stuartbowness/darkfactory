# Handoff: Task 2 - Spec and acceptance parsing (Fix attempt 2)

- Task: Task 2 - Spec and acceptance parsing (Fix attempt 2)
- Commit SHA: 8c103c641eda8ca6a3242928c17dfcffd1d22d95

## Approach

Two targeted fail-loud fixes, nothing else touched. Followed `/tdd`
red->green->refactor: wrote the new failing parse tests first (compile-failed
because `EmptyStatement` was still a unit variant), then made them pass.

### Fix 1 - fail-loud openspec-validate test

`tests/openspec_validate.rs`: replaced the conditional early `return` (which made
the test report `ok` without verifying anything whenever `openspec` was missing)
with an `assert!(openspec_available(), ...)`. The validate body (build temp
project, run `openspec validate widget --type spec`, assert exit 0) is unchanged.
`openspec` is a required pipeline tool (1.3.1 is installed here), so a missing
tool is now a loud failure, not a silent pass. Verified by running the test
binary with `PATH=/usr/bin:/bin` (no openspec): it panics with the new message.
With openspec present it passes.

### Fix 2 - name the offender in the empty-statement error, and stop mislabelling malformed entries

`src/core/types.rs`: changed `RenderError::EmptyStatement` from a unit variant to
`EmptyStatement(String)`, message
`"acceptance statement is empty for scenario {0}"`. No new variants added.

`src/core/parse.rs`:
- Threaded the scenario identifier (`capability/N`, via the existing
  `scenario_identifier` helper, falling back to the title) through
  `parse_acceptance` into `parse_acceptance_entry`, so a genuinely empty or
  whitespace-only statement now yields `EmptyStatement("widget/N")`.
- Split the structurally-malformed cases away from the empty case. A missing
  `[task:N]` bracket, a non-numeric task number, or a missing `[<method>]`
  bracket no longer maps to `EmptyStatement`. These now fail loud as
  `UnknownVerificationMethod(<verbatim entry line>)`: the line is non-empty, so
  labelling it "empty" was misleading; the parser could not extract a valid
  verification method from it, and the payload shows the author the exact
  offending line. This reuses an existing variant (no new variant), as required.
  The genuine unknown-method case (a present-but-unrecognised method token) still
  maps to `UnknownVerificationMethod` as before.

Tests added (all green):
- `empty_statement_fails_loud_naming_the_offender` - empty statement -> `EmptyStatement("widget/1")`.
- `whitespace_only_statement_fails_loud_naming_the_offender` - whitespace-only statement -> `EmptyStatement("widget/5")`, and asserts the message contains `widget/5`.
- `structurally_malformed_entry_is_not_reported_as_empty_statement` - non-numeric task link -> not `EmptyStatement(_)`, equals `UnknownVerificationMethod(<line>)`.
- `entry_missing_method_bracket_is_not_reported_as_empty_statement` - missing method bracket -> not `EmptyStatement(_)`, equals `UnknownVerificationMethod(<line>)`.

## Commands run

- `cargo test --lib` (RED, before types change): exit non-zero, 4 compile errors (expected: `EmptyStatement` not yet a tuple variant).
- `cargo test --lib` (after fixes): exit 0, 25 passed.
- `cargo test` (full suite): exit 0, 27 passed across 4 suites.
- `PATH=/usr/bin:/bin ./target/debug/deps/openspec_validate-* acceptance_carrying_spec_passes_openspec_validate --exact`: test FAILED (panics with the required-openspec message) - confirms fail-loud when openspec is absent.
- `make check`: exit 0 (cargo fmt --check clean, cargo clippy --all-targets -D warnings clean, cargo test 27 passed).

## Concerns

- Choice of variant for the malformed-entry case: the brief said use an existing
  variant (no new ones) and be accurate. `RenderError` has no dedicated
  "malformed entry" / "malformed task link" variant, so I routed all structural
  breakage to `UnknownVerificationMethod` carrying the verbatim line. This is
  accurate for the missing/unknown-method case and defensible for the malformed
  `[task:N]` case (the parser cannot reach a valid method), but the message word
  "verification method" is a slightly loose fit for a purely task-bracket
  failure. If Stage 2 adds a malformed-entry variant, these two task-bracket
  cases should move to it.
