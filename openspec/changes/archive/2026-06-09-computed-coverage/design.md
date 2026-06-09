# Design: computed-coverage

## Architecture Decision

This is an additive extension of the existing single-crate `tools/darkfactory-render`, not an architecture change, so no scored candidates are required (per the design gate). The crate's architecture stays as is and computed coverage slots into it.

**Current architecture.** `darkfactory-render` is one Rust crate with a `core` library and a thin CLI:
- `core::parse` — turns spec/task/fragment markdown into domain types (`parse_specs`, `parse_tasks`, `parse_fragment`), fail-loud via `RenderError`.
- `core::render` — `build_model` groups acceptance entries by task and `render` projects the contract; strict (errors on dangling/closing-task links).
- `core::shadow` — line-diffs a rendered contract against an existing one.
- `core::types` — domain types plus the `RenderError` enum.
- `cli` / `main` — argument parsing and subcommand dispatch (`render`, `shadow`), mapping results to stdout and exit codes.

The pipeline shape is `parse -> analyze -> render`, deterministic and fail-loud. Computed coverage adds a second, *tolerant* analysis beside the strict render path, reusing the same parsed model.

**Key decision — one extended parsed model, not a second parser.** Coverage needs three things render does not: every scenario (including acceptance-less ones, which `parse_specs` currently drops), each scenario's title and behavior text, and tolerance of a missing Spec Reference. The chosen approach extends the single parsed model:
- `parse_specs` retains *every* scenario and gains its title and behavior text.
- `RequirementScenario.spec_ref` becomes `Option<SpecReference>` (None only for an acceptance-less scenario with no `<!-- spec-ref: N -->`).
- `render`'s `build_model` is unchanged in behavior: it still processes only acceptance-carrying scenarios, and the parser still enforces that acceptance implies a Spec Reference, so those scenarios always carry `Some(spec_ref)`. Render output is therefore byte-identical for every input (spec requirement: existing render/shadow unchanged).

Alternative considered — a separate coverage-only scenario type and parse path sharing the low-level line helpers. Rejected: it leaves render literally untouched but creates two scenario representations that can drift, violating the "one parser, two modes cannot disagree" goal. The extended-model choice is DRY and keeps a single source of truth; its only cost is mechanically threading an `Option` through render where the value is always `Some`.

**Tolerant analysis is separate from strict render.** Coverage does NOT call `build_model`. The same condition that `build_model` treats as a fatal error (a dangling link, a link to a closing task) coverage classifies as a `GapKind`. This is the tolerance boundary: parse-level malformed input still fails loud (exit 2); only link *resolution* is tolerant.

## Module Inventory

- `core::coverage` (new) — **Narrow.** Public entry points: `analyze(scenarios, tasks) -> CoverageReport` and `render_report(&CoverageReport) -> String`. Hides gap classification, the fixed gap-kind ordering, gap ordering (capability then document position), and the canonical markdown formatting. Boundary: the tolerant coverage view is a distinct responsibility from strict contract rendering.
- `core::parse` (modified) — **Moderate.** `parse_specs` extended to retain all scenarios plus title and behavior text (acceptance-carrying scenarios still require a Spec Reference; only acceptance-less ones may have `spec_ref: None`). `parse_tasks` is left permissive — duplicate-task-number rejection lives in the coverage CLI path, NOT here, so `render`/`shadow` keep accepting exactly what they accept today. Hides markdown grammar. Boundary unchanged: one parser feeds both render and coverage.
- `core::types` (modified) — **Moderate.** Adds `CoverageGap`, `GapKind`, `CoverageReport`; extends `RequirementScenario` (title, behavior, `spec_ref: Option`); adds `RenderError::DuplicateTaskNumber`. Hides nothing — it is the shared vocabulary.
- `cli` (modified) — **Moderate.** Adds the `coverage` subcommand and its args (`--specs`, `--tasks`), maps `CoverageReport` to stdout and the `0`/`3`/`2` exit code. Hides filesystem I/O and exit-code policy.
- `core::render`, `core::shadow` (unchanged behaviorally) — render adapts to the `Option<SpecReference>` field with identical output.
- `main` (unchanged) — subcommand dispatch.

## Type Catalog

- `CoverageGap` — one uncovered Scenario: its identifier, title, behavior text, and gap kinds. Owning module: `core::types`.
- `GapKind` — enum of the three uncovered reasons (no acceptance, dangling task link, closing-task link). Owning module: `core::types`.
- `CoverageReport` — the computed Gap report model: the ordered gaps plus the total/covered/gaps counts. Owning module: `core::types`.
- `RequirementScenario` (extended) — a spec scenario; now also carries its title, behavior text, its `capability` (the spec directory name, set for every scenario so an acceptance-less scenario without a Spec Reference still reports under its real capability), and an optional Spec Reference. Owning module: `core::types`.

## Error Strategy

- **One error type.** Malformed input keeps using the existing fail-loud `RenderError` (unknown verification method, empty statement, missing/duplicate Spec Reference, unknown status, malformed task link). One new variant, `DuplicateTaskNumber`, is added — but raised in the coverage CLI path after `parse_tasks`, NOT inside the shared `parse_tasks`, so `render`/`shadow` keep accepting whatever they accept today.
- **Tolerant boundary.** `core::coverage::analyze` is infallible — it never returns an error. A dangling link, a closing-task link, omitted acceptance, or an *acceptance-less* scenario's missing Spec Reference become data in the `CoverageReport` (a `GapKind` or a title-based identity), not errors. An *acceptance-carrying* scenario without a Spec Reference is NOT tolerated — `parse_specs` rejects it with `MissingSpecRef` (exit `2`) exactly as today, before `analyze` runs. `build_model`'s strict errors for the same link conditions are unchanged and untouched.
- **Propagation.** The parse layer (`parse_specs`, `parse_tasks`) throws `RenderError`. The coverage CLI path additionally runs a duplicate-task-number check (raising `DuplicateTaskNumber`) before analysis. On any such error `cli` exits `2`. When parsing and the duplicate check succeed, `cli` calls the infallible `analyze`, then maps the report: non-empty gaps exit `3`, empty exit `0`.

## Integration Points

- `main` -> `cli::run` — internal (subcommand dispatch).
- `cli` (coverage path) -> `core::parse::parse_specs` — outbound (reads `--specs`).
- `cli` (coverage path) -> `core::parse::parse_tasks` — outbound (reads `--tasks`).
- `cli` (coverage path) -> `core::coverage::analyze` — internal (produces `CoverageReport`).
- `cli` (coverage path) -> `core::coverage::render_report` — internal (markdown to stdout).
- `core::coverage::analyze` -> `core::types` (`CoverageReport`, `CoverageGap`, `GapKind`) — internal.

## Test Strategy

- **Framework/runner.** Rust built-in `cargo test`. Unit tests colocated in `core::coverage`; integration tests in a new `tests/coverage.rs`, matching the crate's existing `tests/` layout.
- **Unit tests** (coverage analysis): gap classification per kind, the partial-link "not a gap" case, gap-kind set + fixed order, gap ordering (capability then document position), and `render_report` determinism.
- **Integration / fixture tests** (the trust mechanism): one fixture per gap kind (no acceptance, dangling link, closing-task link), a no-gap fixture, the canonical-format byte-identical case from the spec example, and the real archived Stage 1 change (`openspec/changes/archive/2026-06-03-contract-renderer/`) as a near-zero-gap fixture. CLI tests assert exit `0`/`3`/`2`, including malformed-input cases.
- **Regression.** The existing `render`/`shadow` tests, `make check`, and `make smoke` must stay green — this is the guard that the `spec_ref` change and parser retention did not alter render behavior.
- **External deps.** None — filesystem fixtures only, no network or database. `cargo-tarpaulin` is not installed, so the scrutiny diff-coverage gate reports coverage unavailable, same as Stage 1 (non-blocking).
