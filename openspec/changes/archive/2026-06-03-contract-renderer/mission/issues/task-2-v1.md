# Issue: Task 2 - Spec and acceptance parsing (Attempt 1)

## Failed Gates
- Gate 13 (silent skip detection): FAIL
- Gate 11 (contract checks): FAIL — one check not fully met (empty/whitespace statement error)

## Observed Symptoms
- `tests/openspec_validate.rs:38-44`: the test `acceptance_carrying_spec_passes_openspec_validate` returns early (prints to stderr via `eprintln!`, asserts nothing) when the `openspec` binary is not on PATH. A returned-early Rust `#[test]` reports as `ok`/passed, so in any environment lacking `openspec` this test passes while verifying nothing. The accompanying comment claims it "skips loudly via the test name," which is false — Rust's default harness has no skip status; a returned-early test is indistinguishable from a real pass. The task brief explicitly required failing loud or `#[ignore]`, not a silent return.
- The error returned when an acceptance entry's statement is empty or whitespace-only does not identify WHICH scenario or entry caused it: `RenderError::EmptyStatement` is a unit variant with no payload, unlike `EmptyAcceptanceList(String)` which carries the `capability/N` identifier. There is also no dedicated test for a whitespace-only statement (the existing test uses a genuinely-empty trailing token).

## Validator Recommendations
- Make the openspec-validate test require `openspec` and fail loud (panic/assert) when it is absent — `openspec` is a documented repo dependency that is present in this environment (openspec 1.3.1). Do not return silently.
- Give the empty-statement error an identifier payload naming the offending scenario/entry (e.g. `EmptyStatement(String)` carrying `capability/N` and/or the entry text), supply it at the call site, and add tests for BOTH an empty and a whitespace-only statement asserting the identifier appears in the error. Ensure a structurally malformed (but non-empty) entry line is not reported as "empty statement" (that would be misleading). Do not add new `RenderError` variants beyond what is needed for this fix.
