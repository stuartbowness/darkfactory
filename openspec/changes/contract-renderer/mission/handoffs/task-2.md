# Handoff: Task 2 - Spec and acceptance parsing

- **Task:** Task 2 - Spec and acceptance parsing (`core::parse::parse_specs`)
- **Commit SHA:** `fcb6500f519c71f816ad7a5e14d5b601c26411e5`

## Approach

Added the `core::parse` module (new `src/core/parse.rs`, declared `pub mod parse;`
in `src/lib.rs`'s `core` block). It does no file IO; it parses in-memory strings
into typed `core::types` values and fails loud with a `RenderError` on every
malformed case. Followed `/tdd` red -> green -> refactor: each fail-loud case is
its own test.

Chose the single-capability signature from the brief:

```rust
pub fn parse_specs(
    capability: &str,
    spec_markdown: &str,
) -> Result<Vec<RequirementScenario>, RenderError>
```

Capability is an input (the spec folder name), so the parser never invents it.
Duplicate-spec-ref detection is scoped within the one capability via a `HashSet<u32>`.

Grammar parsed (design D3/D4):
- Scenario starts at `#### Scenario: <title>`.
- Directly under the heading, optional `<!-- spec-ref: N -->` and
  `<!-- status: <value> -->` comments. Status goes through
  `ScenarioStatus::parse_optional` (omitted = `None`, unknown = `UnknownStatus`).
- After WHEN/THEN bullets, an optional `**Acceptance:**` block lists
  `- [task:N] [<method>] <statement>` entries. This parser strips the brackets
  and passes the bare method word to `VerificationMethod::parse`. Statement is
  carried verbatim (only surrounding whitespace trimmed).
- Each entry inherits the scenario's `SpecReference` (stored on the
  `RequirementScenario`, not the entry).

Fail-loud cases (each has a unit test):
- Empty/whitespace statement -> `EmptyStatement`.
- Present-but-empty `**Acceptance:**` block -> `EmptyAcceptanceList("widget/N")`.
- Acceptance present but no `spec-ref` -> `MissingSpecRef`.
- Two scenarios sharing N within the capability -> `DuplicateSpecReference("widget/N")`.
- Unknown method tag -> `UnknownVerificationMethod` (from Task 1's parser).
- Unknown status -> `UnknownStatus` (from Task 1's parser).
- Acceptance-less scenario parses fine, contributes no entries, and does NOT
  require a spec-ref (two tests cover this).

Error-routing note: `RenderError` (owned by Task 1) has no "malformed entry"
or "malformed spec-ref" variant. A non-numeric `spec-ref` surfaces as
`MissingSpecRef` (no valid ref either way). A structurally broken entry line
(missing `[task:N]`/`[<method>]` brackets, or a non-numeric task number)
surfaces as `EmptyStatement` (the line yields no usable check statement). Both
are documented at the call sites. No new `RenderError` variants were added.

## openspec-validate-compatibility test

Added `tests/openspec_validate.rs` with a committed fixture
`tests/fixtures/acceptance_spec.md` (a minimal valid OpenSpec spec carrying a
two-entry acceptance block). The test stands up a temp OpenSpec project
(`openspec/config.yaml` + `openspec/specs/widget/spec.md`), runs
`openspec validate widget --type spec`, and asserts exit 0. It RAN (was not
ignored) in this environment; confirmed it took the validate path, not the skip
path. A second test round-trips the same fixture through `parse_specs`, proving
the authored acceptance is both validate-safe and parseable.

The test spawns `openspec` from PATH. If `openspec` is genuinely absent it
prints a skip note and returns rather than failing on an env limitation; it is
NOT `#[ignore]`'d, so it runs whenever the binary is present (it was, here).

## Commands run

- `cargo test --lib parse` -> exit 0 (RED then GREEN per cycle)
- `cargo test --test openspec_validate` -> exit 0 (2 passed; validate ran, not skipped)
- `cargo fmt` (applied formatting) then `cargo fmt --check` -> exit 0
- `cargo clippy --all-targets -- -D warnings` -> exit 0 (no issues)
- `make check` (= fmt + lint + test) -> exit 0: 22 lib tests + 2 integration tests pass, zero fmt diffs, zero clippy warnings
- `git add tools/darkfactory-render && git commit ...` -> 4 files changed, 605 insertions
- Verified the commit stages ZERO files under `openspec/changes/`

## Concerns

- The openspec-validate test depends on `openspec` being on PATH for the
  spawned cargo-test process. It is present here and the test ran clean. In an
  environment without `openspec`, the test self-skips with an `eprintln!` note
  instead of failing (env limitation, not a code defect). If the mission wants
  a hard failure when openspec is missing, flip the early-return to a panic.
- `parse_acceptance` treats a blank line or a new `###`/`####` heading as the
  end of the acceptance block, and ignores non-`- ` lines inside it. This keeps
  trailing prose from breaking parsing, matching D3's "extra content" framing.
