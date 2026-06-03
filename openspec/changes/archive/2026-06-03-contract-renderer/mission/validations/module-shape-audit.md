# Module Shape Re-check: contract-renderer

Audit date: 2026-06-03 (Task 9, closing: module-shape). Performed by the orchestrator (an audit comparing design.md — which validators do not see — against the committed crate at HEAD after Tasks 1-8). Compares design.md's Module Inventory and Type Catalog to what `tools/darkfactory-render` actually exposes. Resolution per item: "no drift", "design.md updated" (deliberate), or "refactored".

## Method
- Actual surface gathered from the committed crate: `pub mod` declarations, `pub struct`/`pub enum`, and `pub fn` across `src/`.
- Design surface read from `openspec/changes/contract-renderer/design.md` Module Inventory + Type Catalog.

## Module Inventory findings
| Design module | Present in code? | Drift | Resolution |
|---|---|---|---|
| `core::types` | yes (`src/core/types.rs`) | none | no drift |
| `core::parse` | yes (`src/core/parse.rs`) | Design listed 3 public fns; code has a 4th, `validate_closing_kinds` (the `UnknownClosingKind` cross-check). | design.md UPDATED — added `validate_closing_kinds` to the interface and noted the placement (D5 envisaged the check at render time; co-locating with the parse-side validators is the as-built choice, called by the renderer). Code not refactored: correct + tested. |
| `core::render` | yes (`src/core/render.rs`) | Design listed a single `render(model)`; code exposes `build_model` AND `render`. | design.md UPDATED — documented the `build_model` (assemble + fail-loud validation) / `render` (pure projection) split (testability + clean fail-loud boundary). Code not refactored. |
| `core::shadow` | yes (`src/core/shadow.rs`) | none (`compare(rendered, existing) -> Diff` exactly as designed) | no drift |
| `cli` | yes (`src/cli.rs` lib module + thin `src/main.rs` bin) | Design said "cli (bin)"; code puts the logic in a `cli` lib module (with `run(args, out, err) -> i32`) called by a thin `main.rs`, for unit-testability. | design.md UPDATED — noted the lib-module + thin-bin shape; same concern boundary (file IO, exit codes, stderr, arg parsing), no domain logic. |

No module was added, removed, or renamed. The five designed modules all exist with matching names and the intended concern boundaries.

## Type Catalog findings
All 9 designed Type Catalog types exist in `core::types` with matching names:
| Type | Present? | Drift |
|---|---|---|
| `AcceptanceEntry` | yes (`core::types`) | none |
| `VerificationMethod` | yes (`core::types`) | none |
| `SpecReference` | yes (`core::types`) | none |
| `ScenarioStatus` | yes (`core::types`) | none |
| `RequirementScenario` | yes (`core::types`) | none |
| `ClosingTaskFragment` | yes (`core::types`) | none |
| `TaskList` | yes (`core::types`) | none |
| `RenderedContract` | yes (`core::types`); gained `warnings: Vec<String>` + `to_markdown()`/`warnings()` in Task 4 | minor (additive; the warnings surface) — recorded in design (render row) |
| `RenderError` | yes (`core::types`) | none. (`EmptyStatement` carries a `String` payload as of Task 2 — additive, recorded) |

Types present in code but NOT originally in the Type Catalog table:
| Type | Owning module | Assessment | Resolution |
|---|---|---|---|
| `ClosingTaskKind` | `core::types` | Sub-element of `ClosingTaskFragment` (name + checks). Reasonable helper, not a hidden domain primitive. | design.md UPDATED — added to the catalog as a sub-element |
| `Task` | `core::types` | Element of `TaskList` (number, name, optional closing kind). | design.md UPDATED — added to the catalog as an element |
| `ContractModel` | `core::render` | The intermediate assembled/validated/ordered model `build_model` produces; design narrative already referenced "the typed model". Fields private (opaque). | design.md UPDATED — added to the catalog |
| `Diff` | `core::shadow` | Already named in the design Module Inventory (`compare ... -> Diff`); was just absent from the catalog table. | design.md UPDATED — added to the catalog |

## Summary
- Module structure: NO drift (5/5 modules match by name and boundary).
- Type Catalog: all 9 designed types present with matching names; no type was renamed or removed.
- Interface widening found and reconciled (deliberate, design.md updated, code not refactored): `core::parse` +`validate_closing_kinds`; `core::render` +`build_model`; `cli` as a lib module + thin bin; 4 catalog additions (`ClosingTaskKind`, `Task`, `ContractModel`, `Diff`).
- No unintended drift requiring a code refactor. All deviations were the well-understood, tested as-built shape; design.md now matches the code.
