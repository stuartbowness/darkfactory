# Worker Brief: Task 2 - Spec and acceptance parsing

## Your Task

Add the first real parser to the crate: a new `core::parse` module with `parse_specs`, which turns spec-scenario markdown (carrying an authored acceptance block) into a list of `RequirementScenario` values, failing loudly on every malformed case. Also add a test proving the acceptance syntax is `openspec validate`-safe.

Concretely, produce:

1. **`core::parse` module** (new file `src/core/parse.rs`, declared `pub mod parse;` in `src/lib.rs`'s `core` module). It hides all markdown parsing; it returns typed `core::types` values or a `RenderError`. It does NO file IO — per the project's test strategy, `core::parse` operates on in-memory strings so it is unit-testable with no external dependencies. (Reading files from disk is the CLI's job, added in Task 5.)

2. **`parse_specs`** — parses one capability's spec markdown into its scenarios. Because a Spec Reference is `capability/N`, the capability name is an INPUT (the spec folder name), not something the parser invents. Recommended signature:
   `pub fn parse_specs(capability: &str, spec_markdown: &str) -> Result<Vec<RequirementScenario>, RenderError>`
   If you prefer a multi-capability signature (e.g. accepting `&[(capability, content)]`), that is acceptable, provided: core stays IO-free, and duplicate-Spec-Reference detection is scoped WITHIN a capability. Keep the interface narrow.

3. **The acceptance authoring grammar** (design decisions D3/D4 — parse exactly this):
   - A scenario begins at a `#### Scenario: <title>` heading (the standard OpenSpec scenario heading — look at the existing `openspec/changes/contract-renderer/specs/*/spec.md` files for the heading style and WHEN/THEN bullet style).
   - Directly under the scenario heading, two optional HTML comments may appear:
     - `<!-- spec-ref: N -->` where N is an integer. The scenario's Spec Reference is `<capability>/<N>` (build a `SpecReference { capability, number: N }`).
     - `<!-- status: <value> -->` where `<value>` is one of `proposed|approved|built|verified|deviated`. Parse it with `ScenarioStatus::parse_optional` (omitted = `None` = non-deviated; present-but-unrecognized = `RenderError::UnknownStatus`).
   - After the scenario's WHEN/THEN bullets, an acceptance block may appear: a line `**Acceptance:**` followed by one or more entries, each on its own line in the form:
     `- [task:N] [<method>] <statement>`
     - `[task:N]` → the integer task link N (store in `AcceptanceEntry::task_link`).
     - `[<method>]` → strip the brackets and pass the bare word to `VerificationMethod::parse` (so `[execution]` → `VerificationMethod::Execution`; an unrecognized method → `RenderError::UnknownVerificationMethod`). The bracket stripping is THIS parser's job (Task 1's `VerificationMethod::parse` deliberately takes the bare word).
     - `<statement>` → the remaining text, carried VERBATIM into `AcceptanceEntry::statement` (you may trim surrounding whitespace, but do not reword — the renderer emits this text exactly). 
   - Every acceptance entry in a scenario INHERITS that scenario's `SpecReference` (it is not stored on the entry; it lives on the `RequirementScenario`).

4. **Fail-loud schema errors** (these are the behaviors to test — use `/tdd`):
   - An empty or whitespace-only statement in an entry → `RenderError::EmptyStatement`.
   - A `**Acceptance:**` block that is present but has zero entries → `RenderError::EmptyAcceptanceList(<scenario identifier>)` (use a human-readable scenario identifier, e.g. the title or the `capability/N`).
   - A scenario that HAS an acceptance block but NO `spec-ref` comment → `RenderError::MissingSpecRef`.
   - Two scenarios within the same capability sharing the same `spec-ref` N → `RenderError::DuplicateSpecReference(<the capability/N>)`.
   - Unknown method tag and unknown status come for free from the Task 1 parsers — cover them too.
   - A scenario that OMITS acceptance entirely parses successfully and contributes no entries (do NOT require `spec-ref` for an acceptance-less scenario). This is a key behavior: only scenarios WITH acceptance require a spec-ref.

5. **openspec-validate-compatibility test** — add a test asserting that `openspec validate` exits 0 on a spec file that carries an acceptance block. Author a minimal valid OpenSpec fixture spec (the acceptance block must be extra content that `openspec validate` ignores). The `openspec` CLI is installed in this repo (run `openspec --help` to see usage). If you cannot make `openspec validate` run in the test environment after a genuine attempt, do NOT silently skip it — instead make the test fail loudly OR mark it `#[ignore]` with a clear reason in your handoff (an env limitation, not a code defect). Prefer a working executable test.

## Project Context

(from proposal.md) This change makes `contract.md` a deterministic projection of authored acceptance instead of LLM-restated prose. Spec scenarios gain an `acceptance` field (a list of entries: a verbatim falsifiable check statement + a verification-method tag + a task link + a stable Spec Reference), plus a scenario `status`. A deterministic renderer (later tasks) composes the contract from these. Your task parses that authored acceptance into the typed model the renderer will consume. Parsing is fragile, so it must be fail-loud with a typed error on every malformed case.

## Architecture

(from design.md) Module inventory: `core::parse` has a narrow interface (`parse_specs`, `parse_tasks`, `parse_fragment` → typed model or `RenderError`) and hides all markdown parsing, the acceptance-block grammar, and spec-ref/status extraction — so the render logic works on typed data only. You are building the `parse_specs` slice plus the shared module. Error strategy: `core::parse` returns `Result<_, RenderError>`; it never prints and never falls back silently. Test strategy: `core::parse` gets extensive unit tests operating on in-memory strings (no external deps); plus a test asserting `openspec validate` still passes on a spec carrying an acceptance block. Relevant decisions: D3 (acceptance block syntax), D4 (stable Spec Reference + status comments, MissingSpecRef / DuplicateSpecReference rules).

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Acceptance (field)** → modeled as `AcceptanceEntry`; a list under a scenario; carries method, task link; inherits the scenario's Spec Reference.
- **Verification method** → `VerificationMethod` (execution/output/inspection).
- **Spec Reference** → `SpecReference` = `capability/N`; capability is the spec folder name; N is a stable integer unique within the capability.
- **Requirement record / scenario** → `RequirementScenario` (spec_ref, status, acceptance entries).
- **Deviated status** → a `ScenarioStatus` variant; Stage 1 only reads it later for the `(DEVIATED)` marker.

## Domain Types

All already defined in `core::types` (Task 1) — import and use them, do not redefine:
- `RequirementScenario { spec_ref: SpecReference, status: Option<ScenarioStatus>, acceptance: Vec<AcceptanceEntry> }` — your output element.
- `AcceptanceEntry { statement: String, method: VerificationMethod, task_link: u32 }` — Spec Reference is NOT on the entry (inherited from the scenario).
- `SpecReference { capability: String, number: u32 }`.
- `VerificationMethod` with `VerificationMethod::parse(token: &str) -> Result<Self, RenderError>` (bare word).
- `ScenarioStatus` with `ScenarioStatus::parse_optional(Option<&str>) -> Result<Option<Self>, RenderError>`.
- `RenderError` — use the existing variants: `EmptyStatement`, `EmptyAcceptanceList(String)`, `MissingSpecRef`, `DuplicateSpecReference(String)`, `UnknownVerificationMethod(String)`, `UnknownStatus(String)`.

## What Previous Tasks Built

### Task 1: Crate skeleton + core::types (commit feb7b283)
- Crate at `tools/darkfactory-render` (standalone package, version 1.0.0, dep `thiserror = "2"`). Makefile targets include `make check` (= fmt + clippy -D warnings + test). `make check` must stay green.
- Library root `src/lib.rs` declares `pub mod core { pub mod types; }`. Add `pub mod parse;` to that `core` block for this task. All domain types live in `crate::core::types`. `src/main.rs` is a stub (real CLI is Task 5 — do not build it here).
- Public API you will consume (all `pub` in `core::types`):
  - `enum VerificationMethod { Execution, Output, Inspection }`; `fn parse(token: &str) -> Result<Self, RenderError>`.
  - `enum ScenarioStatus { Proposed, Approved, Built, Verified, Deviated }`; `fn parse(&str) -> Result<Self, RenderError>`; `fn parse_optional(Option<&str>) -> Result<Option<Self>, RenderError>`; `fn is_deviated(self) -> bool`.
  - `struct SpecReference { capability: String, number: u32 }`.
  - `struct AcceptanceEntry { statement: String, method: VerificationMethod, task_link: u32 }`.
  - `struct RequirementScenario { spec_ref: SpecReference, status: Option<ScenarioStatus>, acceptance: Vec<AcceptanceEntry> }`.
  - `enum RenderError` (thiserror) with variants listed above plus others reserved for later tasks (`DanglingTaskLink(u32)`, `AcceptanceLinksClosingTask(u32)`, `FeatureTaskNoChecks(u32)`, `UnknownClosingKind(String)`).

## Downstream Consumers

- Task 4 (`core::build_model` / `core::render`) consumes the `RequirementScenario` list from `parse_specs` to group checks by task link and render the contract. Minimum integration check: a `RequirementScenario` with two `AcceptanceEntry` values (task links 2 and 3) round-trips so the renderer can place each check under its task.
- Task 6 (migration) parses the migrated archived fixture's specs through `parse_specs`.

## Procedures

**Before writing any code**, invoke the `/tdd` skill (use the Skill tool) and follow its red → green → refactor workflow. This is mandatory. Each fail-loud case above is a red test first.

### ENVIRONMENT — non-negotiable
You are working in a git worktree at `/Users/home/.darkfactory-2-build` (branch `mission/darkfactory-2`). The shell's cwd RESETS to `/Users/home/.darkfactory` after every Bash call, and you can only WRITE into the worktree when cwd is inside it. Therefore:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or `cd /Users/home/.darkfactory-2-build`). Never assume cwd persisted. Use absolute paths.
2. Do NOT run Bash commands in the background; run `cargo`/`git`/`openspec` in the foreground.
3. Create and edit source files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`.

### Build / test
- Verify clean before committing: `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` (zero fmt diffs, zero clippy `-D warnings`, all tests pass). Paste the commands and exit codes into your handoff.

### Commit
Make exactly one commit. Stage ONLY the crate files you changed under `tools/darkfactory-render/`. Do NOT run `git add -A`/`git add .`, and never stage anything under `openspec/changes/contract-renderer/` (mission bookkeeping owned by the orchestrator). Use:
```
cd /Users/home/.darkfactory-2-build && \
  git add tools/darkfactory-render && \
  git commit -m "[task-2] spec and acceptance parsing (core::parse::parse_specs)"
```
Then capture the full SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.
(Note: if your openspec-validate test needs a fixture spec file, put it UNDER `tools/darkfactory-render/` — e.g. `tools/darkfactory-render/tests/fixtures/` — so it is part of the crate and the commit, NOT under `openspec/changes/`.)

### Handoff
Write your handoff (do NOT commit it) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-2.md`

Format:
- Task: Task 2 - Spec and acceptance parsing
- Commit SHA: <full SHA>
- Approach: <what you did and why; note the parse_specs signature you chose>
- Commands run: <commands and exit codes, including make check and the openspec validate test>
- Concerns: <anything uncertain, especially if the openspec-validate test had to be #[ignore]'d and why>

## Final message
Reply with the commit SHA, the `parse_specs` signature, the `make check` result, and whether the openspec-validate test runs or was ignored (with reason).
