## Why

The orchestrator re-authors `contract.md` in Phase 5 by restating behaviors the spec scenarios already define; because that is an LLM regenerating the same facts, the contract drifts from the specs and the pipeline carries reconciliation machinery (Phase 4a, Phase 9) to clean up after it. This change makes the contract a deterministic projection of a single authored source, removing duplicated-artifact drift. It is Stage 1 of EPIC #199.

## What Changes

- Add `acceptance` to OpenSpec spec scenarios as **a list of one or more entries**; each entry is a verbatim falsifiable check statement plus a **Verification method** tag, a `task` link, and a **Spec Reference** (`capability/N`). A scenario also carries a `status`.
- Give each scenario a **stable Spec Reference** with these invariants (concrete mechanism in design): inserting a scenario does not renumber siblings; duplicate Spec References fail loudly; and archived contracts with position-numbered references have a migration path.
- Add a standing **closing-task check fragment** as the authored source for the mandatory operational checks (smoke test, cleanup, module-shape, CLAUDE/ARCH, documentation) that do not trace to any spec scenario.
- Add a deterministic **renderer** (a new tool) that composes `contract.md` from two sources — spec `acceptance` grouped by task, plus the closing-task fragment — in the existing contract format, carrying check statements verbatim and never inventing checks.
- Add a **shadow/compare mode** to the renderer (render and diff against an existing hand-authored contract without replacing it) and document the cutover gate and rollback procedure.
- **Scope guard (not in Stage 1):** the live mission skill's Phase 5 is **not** modified to call the renderer. Cutover — wiring the renderer into the pipeline and deleting hand-authoring — is gated on validator-outcome parity across real missions and lands in a later change. v1 keeps shipping unchanged.

## Capabilities

### New Capabilities
- `acceptance-field`: the per-scenario `acceptance` source — a list of entries (schema, verification-method tag, task link, verbatim statement), the scenario `status` (Stage 1 reads `deviated` for the `(DEVIATED)` marker; the full deviation lifecycle is out of scope), and the stable Spec Reference identity it depends on.
- `contract-render`: the deterministic renderer that composes `contract.md` from acceptance entries (grouped by task) and the closing-task fragment — format fidelity, Spec Reference comment grammar, fail-loud semantics, byte-identical determinism, the inspection-cap warning, and the `(DEVIATED)` marker.
- `shadow-comparison`: the renderer's read-only shadow/compare mode that renders a contract and diffs it against an existing hand-authored one without modifying it, supporting the cutover gate.

### Modified Capabilities
<!-- None. The contract is hand-authored today with no baseline spec describing its production; this change introduces net-new capabilities and does not alter existing requirements in openspec/specs/. -->

## Impact

- **New tool** (leaning Rust under `tools/`, following `darkfactory-dispatch`; final language/placement decided in design.md). Built, tested, and installed like the existing dispatch tool.
- **New spec-authoring convention**: scenarios gain an `acceptance` block and a stable Spec Reference; the spec template and authoring docs change.
- **New artifact**: the standing closing-task check fragment (lives under `templates/`).
- **No change to the running mission skill in Stage 1**: the orchestrator's Phase 5 hand-authoring is untouched until the gated cutover, so daily missions are unaffected.
- **Downstream**: a later change wires Phase 5 to the renderer and removes hand-authoring once the cutover gate passes.
