# Design: contract-renderer

## Context

Stage 1 of EPIC #199. Today the orchestrator hand-authors `contract.md` in Phase 5, re-stating behaviors the specs already define, which drifts. This change adds an `acceptance` field to spec scenarios and a deterministic renderer that projects `contract.md` from it. The renderer runs in shadow mode; the live mission skill's Phase 5 is untouched until a later, gated cutover.

Constraints:
- Repo is Rust (`tools/darkfactory-dispatch`) plus Claude Code skills, markdown templates, and bash. No app/IDE.
- Shell must work on Debian/Ubuntu and macOS; avoid GNU-only flags (CLAUDE.md).
- Internal tools are pinned at v1.0.0; only the repo release tag is bumped.
- The rendered contract must match the existing format so validators and downstream phases consume it unchanged.

## Goals / Non-Goals

**Goals:**
- An `acceptance` schema on spec scenarios (Option B: tagged, task-linked, verbatim statement) with stable Spec References.
- A deterministic renderer that composes `contract.md` from acceptance (spec-derived) plus a standing closing-task fragment.
- A read-only shadow/compare mode and a documented cutover gate + rollback.
- Fail-loud parsing; byte-identical output; existing-format fidelity.

**Non-Goals:**
- Wiring the renderer into Phase 5 / cutover (later change, gated).
- Computed coverage (Stage 2), brief render (Stage 3), intent doc (Stage 4), gate redesign (Stage 5), spec-delta (Stage 6).
- The full deviation lifecycle (Stage 2); Stage 1 only reads `deviated` for the marker.
- Executable acceptance (Option C).

## Architecture Decision

The renderer reads spec files (with acceptance), a task list, and a closing-task fragment, and emits `contract.md`; it also offers a shadow/compare mode. This is a net-new tool in a Rust-plus-bash repo. The existing `tools/darkfactory-dispatch` is a deep, well-separated crate (core engine + thin CLI) and needs no refactoring; the new tool follows its structure.

| Candidate | Separation of concerns | Testability | Task independence | Module depth | Total | Recommendation |
|-----------|------------------------|-------------|-------------------|--------------|-------|----------------|
| A: new Rust crate `tools/darkfactory-render` (core lib + thin CLI) | 3 | 3 | 3 | 3 | 12 | Recommended |
| B: extend `darkfactory-dispatch` with a `render` subcommand | 1 | 2 | 2 | 2 | 7 | Not recommended |
| C: Python/bash script under `scripts/` or `skills/mission/` | 2 | 2 | 2 | 2 | 8 | Not recommended |

### Scoring rationale

- **A (recommended, 12):** A pure core library (parse → typed model → render) with a thin CLI gives each responsibility one home, makes the core testable on fixtures with no external deps, lets each task touch one module, and hides all parsing/formatting behind a small `render()` surface. Matches the house style.
- **B (7):** Folding a document renderer into the process supervisor mixes two unrelated concerns (subprocess supervision vs markdown projection) in one crate, lowering separation and depth; tasks would share a build with dispatch.
- **C (8):** A script avoids a crate but adds a new runtime (Python) or fragile markdown parsing (bash) to a Rust-plus-bash repo, weakens the byte-identical-determinism and fail-loud guarantees a typed compiled tool gives, and complicates cross-platform behavior.

No recommended-candidate dimension scores 0. Recommendation: **Candidate A** — a new crate `tools/darkfactory-render` with a `core` library and a thin CLI binary, built/installed/tested like `darkfactory-dispatch` (Makefile, `~/.local/bin`, `tests/`).

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `core::types` | narrow (type definitions + `RenderError`) | nothing (data + error enum) | One home for the domain vocabulary every other module shares |
| `core::parse` | narrow (`parse_specs`, `parse_tasks`, `parse_fragment` → typed model or `RenderError`) | all markdown parsing, the acceptance-block grammar, spec-ref and status extraction, task numbers and `Closing:` kinds | Isolates fragile text parsing so render logic works on typed data only |
| `core::render` | narrow (`render(model) -> Result<RenderedContract, RenderError>`; `RenderedContract::to_markdown() -> String`) | grouping by task, Spec Reference comment grammar (dedupe/order/DEVIATED), check formatting, inspection-cap warning, closing-kind→task join | The deterministic projection lives behind one call; this is the deep core |
| `core::shadow` | narrow (`compare(rendered, existing) -> Diff`) | diffing strategy | Keeps the read-only comparison separate from rendering |
| `cli` (bin) | moderate (subcommands `render`, `shadow`; flags for paths) | file IO, exit codes, stderr warnings | Thin transport over the pure core; no domain logic |

## Type Catalog

| Type name | Purpose | Owning module |
|-----------|---------|---------------|
| `AcceptanceEntry` | One acceptance entry: verbatim statement, method, task link (Spec Reference inherited from its scenario) | `core::types` |
| `VerificationMethod` | Enum: execution / output / inspection | `core::types` |
| `SpecReference` | A `capability/N` scenario identifier | `core::types` |
| `ScenarioStatus` | Enum: proposed / approved / built / verified / deviated | `core::types` |
| `RequirementScenario` | A parsed scenario: its Spec Reference, status, and acceptance entries | `core::types` |
| `ClosingTaskFragment` | The standing source of operational checks; declares which tasks are closing tasks | `core::types` |
| `TaskList` | The ordered tasks (from tasks.md), each a feature or closing task | `core::types` |
| `RenderedContract` | The output contract document model | `core::types` |
| `RenderError` | The fail-loud error enum | `core::types` |

Type names align with `UBIQUITOUS_LANGUAGE.md`: `AcceptanceEntry`↔Acceptance (field), `VerificationMethod`↔Verification method, `SpecReference`↔Spec Reference, `ScenarioStatus`'s `deviated`↔Deviated status.

## Error Strategy

- **Error type:** one `RenderError` enum in `core::types` using `thiserror` (as `darkfactory-dispatch` does). Variants map to the spec's fail-loud cases: `UnknownVerificationMethod`, `EmptyStatement`, `EmptyAcceptanceList`, `MissingSpecRef`, `DuplicateSpecReference`, `UnknownStatus`, `DanglingTaskLink`, `AcceptanceLinksClosingTask`, `FeatureTaskNoChecks`, `UnknownClosingKind`.
- **Propagation:** `core::parse` and `core::render` return `Result<_, RenderError>`. The core never prints and never falls back silently — it returns the typed error.
- **Translation:** the `cli` binary catches `RenderError`, prints a single actionable message to stderr, and exits non-zero. Parse/render are the only throwers; the CLI is the only translator.
- **Warnings vs errors:** the inspection-cap breach is a non-fatal warning (stderr, exit 0). Everything in the fail-loud list is a hard error (non-zero exit).

## Integration Points

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| `cli` | `core::parse` | `parse_specs/parse_tasks/parse_fragment` | outbound |
| `cli` | `core::render` | `render(model)` then `RenderedContract::to_markdown()` | outbound |
| `cli` (shadow) | `core::shadow` | `compare(rendered, existing)` | outbound |
| `core::render` | `core::types` | constructs `RenderedContract` | internal |
| `cli` | filesystem | reads `specs/**/*.md`, `tasks.md`, closing-task fragment, existing `contract.md` | inbound |

No integration with the live mission skill in Stage 1. A later change adds a Phase 5 → `cli render` call.

## Test Strategy

- **Framework/runner:** Rust's built-in test harness (`cargo test`) plus a shell smoke test, mirroring `darkfactory-dispatch`.
- **Location:** unit tests colocated in `core` modules (pure parse/render/error paths); integration tests in `tests/`; a `tests/smoketest.sh` for the CLI.
- **Unit vs integration:** `core::parse` and `core::render` get extensive unit tests (no external deps — they operate on in-memory strings and typed models). Integration tests exercise the CLI end-to-end against fixtures.
- **Primary fixture:** the archived change `openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/` (real `specs/` + hand-authored `contract.md`, 8 tasks). Migrate it: author acceptance on the scenarios behind each spec-derived check, model the mandatory operational checks via the fragment, and author a scenario for any feature-task check that had no spec source (per D6). Then render and assert the result is equivalent to the hand-authored contract — same checks under the same tasks (validator-outcome parity) — not necessarily byte-identical to the un-migrated original.
- **No real external dependencies:** all fixture-based and deterministic; no network or DB.
- **Critical paths to cover:** acceptance parsing incl. fail-loud cases; grouping/ordering; Spec Reference comment grammar (dedupe/order/DEVIATED); deterministic byte-identical output; inspection-cap warning; the closing-task fragment composition; and that `openspec validate` still passes on a spec that carries an acceptance block.

## Decisions

- **D1 — Language/placement.** New Rust crate `tools/darkfactory-render` (core lib + thin CLI), pinned v1.0.0, built/installed/tested like `darkfactory-dispatch`. Alternatives: extend dispatch (rejected — mixes concerns); Python/bash script (rejected — new runtime / fragile parsing / weaker determinism).
- **D2 — Canonical task key.** Tasks are keyed by their number N from `## Task N: <name>` (rendered contract) and `Task N:` (tasks.md). Acceptance entries link with `[task:N]`; an `N` absent from tasks.md is `DanglingTaskLink`. This single rule matches both acceptance entries and closing tasks — there is no name-based matching.
- **D3 — Acceptance authoring syntax.** Acceptance lives in a block under each `#### Scenario:`, after the WHEN/THEN bullets: a `**Acceptance:**` line followed by entries of the form `- [task:N] [<method>] <statement>`. The block is extra content `openspec validate` ignores; a Stage 1 test asserts validate still passes.
- **D4 — Stable Spec Reference and status annotations.** Directly under each scenario heading, two HTML comments: `<!-- spec-ref: N -->` (N an integer unique within the capability, assigned once, never renumbered; Spec Reference = `<capability>/<N>`, inherited by the scenario's acceptance entries) and optional `<!-- status: <value> -->` (one of proposed/approved/built/verified/deviated; omitted = non-deviated; unrecognized = `UnknownStatus`). A scenario with acceptance but no `spec-ref` is `MissingSpecRef`; two scenarios sharing N is `DuplicateSpecReference`. Positional numbering rejected — not stable across insertion.
- **D5 — Closing-task fragment and task kinds.** A standing file `templates/closing-task-checks.md` declares the mandatory closing-task KINDS (smoke-test, cleanup, module-shape, claude-arch, documentation) with their `[<method>] <statement>` checks. In tasks.md each closing task carries a `Closing: <kind>` topology field; a task is a closing task iff it carries a fragment-known kind, and every other task is a feature task. The renderer joins each `Closing: <kind>` task to the fragment's checks for that kind. A `Closing:` kind absent from the fragment is `UnknownClosingKind`; an acceptance entry linking to a closing task is `AcceptanceLinksClosingTask`.
- **D6 — Every feature-task check traces to a scenario.** There is no third check source: feature-task checks come from spec `acceptance`, operational checks from the fragment. A feature task with zero acceptance checks is `FeatureTaskNoChecks`. Migration consequence: where a v1 contract had a feature-task check with no spec source (e.g. a bespoke "update README" task), the migration authors a scenario for it, or models it as a documentation closing task if it is genuinely operational. The fixture parity test therefore asserts the rendered contract is equivalent — same checks under the same tasks / same validator outcomes — not byte-identical to the un-migrated original.
- **D7 — Deterministic ordering.** Task sections in task-list order; checks within a task by Spec Reference (capability, then N); ties (multiple acceptance entries sharing one scenario's Spec Reference) broken by source order — scenario file order, then entry order within the scenario.
- **D8 — Shadow comparison.** Line-based diff between the rendered and existing `contract.md`; the text diff is diagnostic only. Cutover remains a manual, gated action.

## Risks / Trade-offs

- **Acceptance authoring moves the judgment, not removes it** → mitigation: Codex reviews acceptance at source; blind quality re-rating in the cutover gate (EPIC #199).
- **Markdown parsing is fragile** → mitigation: typed parse boundary, fail-loud on every malformed case, extensive fixture tests, and a validate-compatibility test.
- **Stable-ref migration of archived contracts** → mitigation: an explicit migration step + a fixture test proving references resolve to the same scenarios after migration.
- **Closing-task fragment can drift from the mission's actual closing tasks** → mitigation: tasks.md tags each closing task with a fragment-known `Closing:` kind; an unknown kind fails loud (`UnknownClosingKind`), and a test asserts the fixture's closing-task checks reproduce from the fragment.
- **A v1 feature-task check may have no spec source** (e.g. a bespoke README task) → mitigation: D6 — migration authors a scenario for it or models it as a documentation closing task; the parity test is equivalence, not byte-identity.
- **Determinism regressions** → mitigation: a byte-identical repeat-run test and pinned ordering (task-list order, then Spec Reference, then source order).

## Migration Plan

1. Build the tool in the worktree; no change to the live mission skill.
2. Operators run `darkfactory-render shadow` against real missions and inspect diffs.
3. A later change wires Phase 5 to `darkfactory-render render` once the cutover gate passes (validator-outcome parity across diverse missions, separate-session authoring, a seeded failure + a real defect caught).
4. Rollback: regenerate the hand-authored contract and restore the prior Phase 5 flow; the tool is additive and removable without affecting v1.

## Open Questions

- Exact byte-level acceptance syntax may be refined by the implementer within the D2 shape, provided the parse tests and the validate-compatibility test pass.
- Whether `spec-ref` should eventually live in the scenario heading rather than a comment; for Stage 1 the comment form is chosen for validate-safety.
