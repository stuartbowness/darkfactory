# Worker Brief: Task 6 - Stable-ref migration and archived-fixture integration test

## Your Task

Prove the renderer reproduces a REAL hand-authored contract. Migrate the archived change `2026-04-29-domain-vocabulary-type-names` into a test fixture under the crate: assign stable Spec References to its scenarios, author acceptance reproducing its spec-derived checks, author a scenario for the one feature-task check group that has no spec source (D6), then add an integration test that renders the migrated fixture and asserts equivalence to the archived hand-authored contract. Plus two migration-invariant tests.

## The archived fixture (the migration source + equivalence target)

Located at `openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/`. You MAY (and must) read its `specs/`, `tasks.md`, and `contract.md` — that archived `contract.md` is INPUT DATA for this migration (the thing you reproduce), NOT this mission's validation contract. (Do NOT read `openspec/changes/contract-renderer/contract.md` — that one is off-limits.)

Its shape:
- 3 capabilities, each `specs/<capability>/spec.md` with `#### Scenario: <title>` headings and `- **WHEN** ... - **THEN** ...` bullets:
  - `brief-domain-types` — 4 scenarios (→ contract refs brief-domain-types/1-4)
  - `domain-type-boundary` — (→ domain-type-boundary/1)
  - `type-name-audit` — (→ type-name-audit/1, type-name-audit/2)
- `contract.md` has 8 tasks:
  - Task 1 (feature) — Specs brief-domain-types/1-4; 5 checks (4 `[output]` + 1 `[inspection]`).
  - Task 2 (feature) — Specs domain-type-boundary/1, type-name-audit/1-2; 3 `[output]` checks.
  - Task 3 (feature) "Update README to reflect changes" — **NO Specs comment / NO spec source**; 3 `[output]` checks. This is the D6 case.
  - Task 4 Smoke test, Task 5 Cleanup, Task 6 Module shape re-check, Task 7 CLAUDE.md+ARCH.md, Task 8 Documentation — these are the 5 mandatory CLOSING tasks (kinds: smoke-test, cleanup, module-shape, claude-arch, documentation).

## Migration procedure

Create the migrated fixture UNDER THE CRATE (do NOT modify the archived change itself — copy what you need): `tools/darkfactory-render/tests/fixtures/migrated/` with `specs/<capability>/spec.md`, `tasks.md`, and reuse the standing `tools/darkfactory-render/templates/closing-task-checks.md` as the fragment.

1. **Copy the 3 capability spec files** into the migrated fixture. Under each `#### Scenario:` heading add a stable `<!-- spec-ref: N -->` token. Number them so the Spec References match the archived contract's references exactly (e.g. `brief-domain-types` scenarios get spec-ref 1,2,3,4 → `brief-domain-types/1..4`; `domain-type-boundary` → 1; `type-name-audit` → 1,2). The numbering is in document order here, but the TOKEN is what makes it stable (see invariant test 2).
2. **Author acceptance** under each scenario: a `**Acceptance:**` block with `- [task:N] [<method>] <statement>` entries reproducing the archived contract's checks for that scenario's Spec Reference. Carry each check statement VERBATIM from the archived contract, with the same `[<method>]` tag and the `[task:N]` linking to the archived task number (Task 1 checks → `[task:1]`, Task 2 → `[task:2]`). Map each archived check to the scenario whose Spec Reference the contract lists for that task — where a task lists multiple refs and has more/fewer checks than scenarios, use judgment to attach each check to the most relevant scenario (a scenario may carry multiple acceptance entries; every archived feature-task check must be reproduced exactly once).
3. **D6 — Task 3 (README) has no spec source.** Author a NEW capability (e.g. `specs/readme-updates/spec.md`) with scenario(s) carrying acceptance for the 3 README `[output]` checks, each `[task:3]`, statements verbatim from the archived contract. (Keep Task 3 a feature task — do not fold it into the documentation closing task; the archived contract has Task 3 AND Task 8 distinctly.)
4. **Migrated tasks.md:** reproduce the archived 8-task list (same numbers, same names, same order). Tag tasks 4-8 with their `Closing: <kind>` field (4=smoke-test, 5=cleanup, 6=module-shape, 7=claude-arch, 8=documentation). Tasks 1-3 are feature tasks (no Closing field).
5. **Fragment:** use the existing standing `templates/closing-task-checks.md` as-is. Do NOT over-fit it to this fixture — it is generic across missions. The closing tasks will render the fragment's GENERIC operational checks, which deliberately differ from the archived mission-specific closing checks (this is the v2 design: closing checks are standardized).

## The integration equivalence test (contract check 1)

Add an integration test (in `tests/`) that runs the full pipeline on the migrated fixture (`parse_specs` per capability + `parse_tasks` + `parse_fragment` → `build_model` → `render`) and asserts equivalence to the archived hand-authored contract. Equivalence here is **"same checks under the same tasks (validator-outcome parity), NOT byte-identity"** (design D6 / Test Strategy). Concretely assert:
- The rendered contract has the SAME 8 task sections, with the same `## Task N: <name>` names, in the same order.
- For the FEATURE tasks (1, 2, 3): the rendered checks reproduce the archived contract's checks for those tasks — same statements (verbatim), same method tags, and the feature tasks carry the expected `<!-- Specs: ... -->` comment with the right refs. This is the strong parity claim (the substantive per-mission checks are faithfully reproduced from authored acceptance).
- For the CLOSING tasks (4-8): each renders its kind's operational checks FROM THE FRAGMENT and carries NO `<!-- Specs -->` comment. Assert the closing tasks are present with their fragment checks — do NOT assert byte-equality to the archived mission-specific closing checks (they intentionally differ).
Write a short comment in the test explaining this equivalence definition so a reader understands why closing tasks are compared structurally, not textually.

## The two migration-invariant tests (contract checks 2 and 3)

- **Inserting a new scenario does not change existing Spec References (check 2).** Take a migrated capability spec, parse it, record the existing scenarios' Spec References. Insert a NEW scenario (with a new, higher spec-ref N) in the MIDDLE of the file, re-parse, and assert the pre-existing scenarios' Spec References are UNCHANGED (because the ref is the explicit `spec-ref: N` token, not a position). This proves the D4 stable-ID invariant.
- **A migrated position-numbered capability yields stable Spec References that resolve to the same scenarios as before (check 3).** Show that the migrated spec-ref token N for each scenario equals the archived contract's positional reference N, and that parsing the migrated spec yields, for each `<capability>/N`, the SAME scenario (same title / WHEN-THEN content) the positional reference pointed at in the archived spec. This proves the migration from positional numbering to stable tokens preserves resolution.

## Project Context

(from proposal.md) Stage 1 must ship with a migration path for archived contracts with position-numbered references, and prove the renderer is trustworthy via a fixture that renders to an equivalent contract. This task is that proof: it migrates a real past mission and shows the renderer reproduces its substantive (spec-derived) checks under the same task structure, with closing-task checks standardized via the fragment.

## Architecture

(from design.md) Primary fixture = this archived change (real specs + hand-authored contract, 8 tasks). Decision D6: every feature-task check traces to a scenario; where a v1 contract had a feature-task check with no spec source, the migration authors a scenario for it (Task 3 README here). The parity test asserts equivalence (same checks under same tasks / same validator outcomes), not byte-identity. Risk mitigation: an explicit migration step + a fixture test proving references resolve to the same scenarios after migration (checks 2, 3).

## Domain Types / pipeline (from prior tasks)

Use the existing pipeline; you are NOT changing core logic, only authoring fixture content + tests (a migration helper is fine if useful):
- `core::parse::{parse_specs(capability, md), parse_tasks(md), parse_fragment(md)}`
- `core::render::{build_model(&[RequirementScenario], &TaskList, &ClosingTaskFragment), render(&ContractModel)}`
- `RenderedContract::to_markdown()` / `::warnings()`
- Acceptance grammar: `<!-- spec-ref: N -->` + optional `<!-- status: ... -->` under `#### Scenario:`; `**Acceptance:**` then `- [task:N] [<method>] <statement>`.

## What Previous Tasks Built (commits)
- Tasks 1-5 complete: feb7b283 (types), 8c103c64 (parse_specs), bf06a9d0 (parse_tasks/parse_fragment/validate_closing_kinds + the standing fragment), a12d37f1 (build_model/render), d5e5a4f4 (shadow + render/shadow CLI). `make check` = fmt + clippy -D warnings + test; currently 73 tests, all green. No external deps beyond `thiserror`.

## Procedures

**Before writing any code/fixtures**, invoke the `/tdd` skill (Skill tool). Write the invariant tests (checks 2, 3) red-first where practical; the equivalence test is naturally written after the fixture exists, but author the fixture and the test together and confirm it renders equivalent. Mandatory.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (or the worktree root). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/git in the foreground.
3. Create/edit files with the Edit/Write tools using ABSOLUTE paths under `/Users/home/.darkfactory-2-build/tools/darkfactory-render/`. Read the archived fixture with the Read tool (absolute paths under `openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/`).
4. Do NOT modify the archived change. Do NOT read `openspec/changes/contract-renderer/contract.md`.

### Build / test
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && make check` must pass clean. Add NO new dependency. Paste commands + exit codes into the handoff.

### Commit
One commit. Stage ONLY files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render && \
  git commit -m "[task-6] stable-ref migration + archived-fixture equivalence test"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-6.md`
Format: Task; Commit SHA; Approach (how you mapped archived checks→scenarios, how you handled the D6 README case, your equivalence-test assertions, any migration helper); Commands run + exit codes; Concerns (esp. any feature-task check you could not reproduce exactly).

## Final message
Reply with the commit SHA, a summary of the migrated fixture (capabilities, scenarios, how the D6 README case was handled), how the equivalence test asserts parity (feature vs closing), confirmation the two invariant tests pass, and the `make check` result.
