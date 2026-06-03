# Task 9 Validation (independent user-test): module-shape re-check

**Result: PASS**

Validator: independent reviewer (user-test format). Judged against committed code at worktree HEAD `5350696` (Task 8), the audit, and the reconciled design.md. The audit and design.md were read directly because this task is explicitly about reconciling design vs code.

## Contract check

- [output] `mission/validations/module-shape-audit.md` exists and records, per design.md module and Type Catalog type, whether drift was found and what was done. **PASS** — the audit has a Module Inventory findings table (5/5 modules), a Type Catalog findings table (9/9 designed types), an added-types table (4 types), each row carrying a drift assessment and a resolution.

## What I verified

| Check | Method | Result |
|---|---|---|
| Worktree HEAD is Task 8 `5350696` | `git rev-parse HEAD` | PASS — `5350696619...` |
| Crate dir clean at start and end | `git status --porcelain tools/darkfactory-render` | PASS — empty |
| Audit covers every Module Inventory module | Read audit vs design.md | PASS — core::types, core::parse, core::render, core::shadow, cli all present, each with drift + resolution |
| Audit covers every Type Catalog type | Read audit vs design.md | PASS — all 9 designed types assessed; 4 added types assessed |
| `core::parse` exposes a 4th pub fn `validate_closing_kinds` | `grep pub fn` | PASS — parse.rs:148, beyond parse_specs(49)/parse_tasks(84)/parse_fragment(123) |
| `validate_closing_kinds` called by the renderer | `grep validate_closing_kinds` | PASS — render.rs:94 calls it inside `build_model` |
| `core::render` exposes BOTH `build_model` and `render` | `grep pub fn` | PASS — render.rs:88 `build_model`, render.rs:127 `render` |
| `build_model` then `render` called by CLI | `grep build_model` in cli.rs | PASS — cli.rs:150 `build_model(...)` then `render` |
| 9 catalog types exist in `core::types` | `grep pub struct/enum` | PASS — all 9 in types.rs (lines 15-193) |
| Added type `ClosingTaskKind` in core::types | grep | PASS — types.rs:135 |
| Added type `Task` in core::types | grep | PASS — types.rs:144 |
| Added type `ContractModel` in core::render, fields private | Read render.rs:39-43 | PASS — `pub struct ContractModel { sections: Vec<TaskSection> }` (field private) |
| Added type `Diff` in core::shadow with `Identical`/`Different(text)` | Read shadow.rs:24-29 | PASS — `Identical`, `Different(String)` |
| `RenderedContract` gained `warnings: Vec<String>` + `to_markdown()`/`warnings()` | Read types.rs:167-185 | PASS — additive, matches audit |
| `RenderError::EmptyStatement` carries a `String` payload | Read types.rs:197 | PASS — `EmptyStatement(String)` |
| cli is a lib module + thin `main.rs` bin with `run(args, out, err) -> i32` | Read main.rs, grep | PASS — cli.rs:53 `pub fn run<O,E>(...) -> i32`; main.rs delegates to `cli::run` |
| pub mod decls match design (cli + core::{parse,render,shadow,types}) | Read lib.rs:12-17 | PASS |
| design.md reconciliation: validate_closing_kinds documented | Read design.md:50 | PASS |
| design.md reconciliation: build_model/render split documented | Read design.md:51 | PASS |
| design.md reconciliation: cli lib-module + thin-bin note | Read design.md:53 | PASS |
| design.md reconciliation: 4 added types in Type Catalog | Read design.md:68-71 | PASS |
| No unaccounted public module/type/fn | Full grep of pub surface vs audit | PASS (see note) |

## Real public surface gathered (against which the audit was checked)

- pub mod: `cli`, `core::{parse, render, shadow, types}` (lib.rs:12-17).
- pub struct/enum: types.rs — `VerificationMethod`, `ScenarioStatus`, `SpecReference`, `AcceptanceEntry`, `RequirementScenario`, `ClosingTaskFragment`, `ClosingTaskKind`, `Task`, `TaskList`, `RenderedContract`, `RenderError`; render.rs — `ContractModel`; shadow.rs — `Diff`.
- pub fn: parse.rs — `parse_specs`, `parse_tasks`, `parse_fragment`, `validate_closing_kinds`; render.rs — `build_model`, `render`; shadow.rs — `compare`; cli.rs — `run`; types.rs (associated methods, not module free fns) — `VerificationMethod::parse`, `ScenarioStatus::{parse, parse_optional, is_deviated}`, `RenderedContract::{to_markdown, warnings}`.
- pub const: cli.rs — `EXIT_USAGE`, `EXIT_RENDER_ERROR`, `EXIT_DIFFERENT`; shadow.rs — `NEWLINE_ONLY_MESSAGE`.

Every module-level item maps to an audit entry. The remaining items (the enum `parse`/`is_deviated` associated methods, the two `RenderedContract` accessors, the CLI exit-code consts, and `NEWLINE_ONLY_MESSAGE`) are intra-type sub-elements of catalog types / the cli module, not new modules or new domain primitives, so the audit's module/type-level granularity legitimately covers them. No hidden module, domain type, or top-level free fn is unaccounted for.

## Accuracy findings

Every specific claim in the audit matched the code. No claim contradicted by the code:

- `core::parse` 4th fn `validate_closing_kinds` — confirmed, and it is co-located in parse and invoked by `build_model` (render.rs:94), exactly as the audit and the reconciled design.md (D5 note) state.
- `core::render` `build_model` + `render` split — confirmed; `ContractModel` fields are private (opaque), matching the audit.
- 9 catalog types all present in `core::types` with matching names — confirmed.
- 4 added types (`ClosingTaskKind`, `Task` in core::types; `ContractModel` in core::render; `Diff` in core::shadow) — all confirmed at the claimed locations.
- cli lib-module + thin bin with `run(...) -> i32` — confirmed.

## design.md consistency findings

design.md's Module Inventory (lines 47-53) and Type Catalog (lines 57-71) were updated to match the code. The four "design.md updated" claims in the audit are all reflected in design.md: `validate_closing_kinds` in the parse interface with the D5 placement note (line 50); the `build_model`/`render` split (line 51); the cli lib-module + thin-bin shape (line 53); and the four added types in the Type Catalog rows (lines 68-71, each tagged "added Task 9"). No mismatch between the audit's "design.md updated" claims and design.md's actual content.

Note (non-blocking): design.md's Integration Points table (lines 84-92) still lists the old single-step `render(model)` / `parse_specs/parse_tasks/parse_fragment` interface and does not mention `build_model` or `validate_closing_kinds`. The Module Inventory + Type Catalog (the sections this task reconciles) are fully consistent; the Integration Points table is a narrative cross-reference outside the two sections under audit, so it does not affect the result.

## Failures

None.

## Conclusion

The audit exists, is complete (every design.md module and every Type Catalog type assessed, plus the 4 added types), and is accurate against the committed code. design.md's Module Inventory and Type Catalog are consistently reconciled. No unaccounted public surface. Worktree crate clean at end. **PASS.**
