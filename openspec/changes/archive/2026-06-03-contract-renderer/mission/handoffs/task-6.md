# Handoff: Task 6 — Stable-ref migration + archived-fixture equivalence test

## Task
Migrate the archived change `2026-04-29-domain-vocabulary-type-names` into a test fixture under the crate (stable spec-ref tokens + authored acceptance reproducing its contract checks, with a new scenario for the D6 README case), then add an integration equivalence test plus two migration-invariant tests. No core-logic changes; no new dependency.

## Commit SHA
`7b1d9e44fa5bc82b8e0dce12c122138708d7b743`
6 files changed, 547 insertions(+). All staged files are under `tools/darkfactory-render/`; zero `openspec/` files in the commit.

## Approach

### Migrated fixture (`tools/darkfactory-render/tests/fixtures/migrated/`)
Four capability specs + one tasks.md. The standing `templates/closing-task-checks.md` is reused as-is (not over-fit).

- `specs/brief-domain-types/spec.md` — 4 scenarios copied verbatim, spec-ref 1-4 in document order. Hosts archived Task 1's 5 checks (all `[task:1]`, verbatim from the archived contract):
  - ref 1 (produces): the "## Domain Types after ## Key Terms" `[output]` check + the "one-line instruction to use exact names" `[output]` check.
  - ref 2 (consumes): the "placeholder for Type Catalog entries from Produces and Consumes" `[output]` check.
  - ref 3 (no domain types): the "None for this task" fallback `[output]` check.
  - ref 4 (separate from Key Terms): the "separate from Key Terms (not merged)" `[inspection]` check.
- `specs/domain-type-boundary/spec.md` — all 3 archived scenarios copied verbatim to keep the archived spec faithful. Only the first carries `spec-ref: 1` + acceptance (archived Task 2's design-template `[output]` check, `[task:2]`); scenarios 2-3 are acceptance-less (they parse as no-entry scenarios), matching that the archived contract referenced only `domain-type-boundary/1`.
- `specs/type-name-audit/spec.md` — 2 scenarios copied verbatim, spec-ref 1-2. ref 1 hosts archived Task 2's SKILL.md "verify every domain type exists with matching name" `[output]` check; ref 2 hosts the "drift resolved by updating design.md or refactoring code" `[output]` check. Both `[task:2]`.
- `specs/readme-updates/spec.md` — NEW capability authored for the D6 case (see below).
- `tasks.md` — the archived 8-task list reproduced (same numbers, names, order). Tasks 1-3 are feature tasks (no Closing field). Tasks 4-8 carry `Closing:` fields: 4=smoke-test, 5=cleanup, 6=module-shape, 7=claude-arch, 8=documentation (kind names match the standing fragment's five kinds).

### Mapping archived checks -> scenarios
Task 1 lists `brief-domain-types/1-4` and has 5 checks (4 `[output]` + 1 `[inspection]`); each check was attached to the scenario whose subject it describes (refs 1 carries 2 checks, refs 2/3/4 one each). Task 2 lists `domain-type-boundary/1, type-name-audit/1-2` and has 3 `[output]` checks, one per listed reference. Every archived feature-task check is reproduced exactly once, verbatim, with its original method tag.

### D6 — Task 3 (README) has no spec source
Archived Task 3 ("Update README to reflect changes") carried 3 `[output]` checks with no scenario behind them. Per design D6, I authored a new `readme-updates` capability with one scenario (`spec-ref: 1`) hosting all 3 README checks, statements verbatim, each `[task:3]`. Task 3 stays a distinct feature task in tasks.md (NOT folded into the documentation closing task). Consequence: under v2 Task 3 now carries a Specs comment (`<!-- Specs: readme-updates/1 -->`) where the archived contract had none — this is the intended D6 outcome (every feature-task check traces to a scenario), and the equivalence test asserts it explicitly rather than treating it as a deviation.

### Equivalence-test assertions (`tests/migrated_contract.rs`)
`migrated_fixture_renders_equivalent_to_the_archived_contract` runs the full pipeline (parse_specs per capability -> parse_tasks -> parse_fragment -> build_model -> render) and asserts validator-outcome parity, NOT byte-identity:
- Same 8 `## Task N: <name>` sections, same names, in the same order (asserted via strictly-increasing heading positions).
- FEATURE tasks 1-3: each renders the archived per-task checks VERBATIM (full `- [ ] [method] statement` line match) and carries the expected Specs comment: Task 1 `brief-domain-types/1-4`, Task 2 `domain-type-boundary/1, type-name-audit/1, type-name-audit/2` (alphabetical capability ordering matches the archived contract's order), Task 3 `readme-updates/1`.
- CLOSING tasks 4-8: each is asserted STRUCTURALLY — present, carries NO Specs comment, and renders the standing fragment's generic checks for its kind (asserted by reparsing the fragment and requiring each fragment check to appear). Deliberately NOT compared byte-for-byte to the archived mission-specific closing checks (v2 standardizes closing checks via the fragment). A long comment in the test states this equivalence definition.

### Two migration-invariant tests
- `inserting_a_scenario_does_not_change_existing_spec_references` (check 2): parses `brief-domain-types`, records refs 1-4, inserts a NEW scenario with `spec-ref: 5` in the MIDDLE of the file (before the ref-3 scenario), reparses, and asserts the pre-existing refs are unchanged and ref 3 resolves to the identical scenario despite the position shift. Proves the D4 stable-ID invariant (ref is the token, not the position).
- `migrated_tokens_resolve_to_the_same_scenarios_as_positional_refs` (check 3): for each `capability/N` the archived contract referenced, asserts the migrated `spec-ref: N` token sits under the SAME scenario heading the archived positional Nth scenario had (verified against the scenario's archived title within its heading block), the token resolves to a parsed scenario, and that scenario's acceptance entries all link to the archived task that referenced it (Task 1 for brief-domain-types, Task 2 for the other two). Proves positional->token migration preserves resolution.

### Migration helper
None. No core logic changed and no helper was needed; the existing pipeline parsed the authored fixture directly.

## Commands run + exit codes
- `cargo test --test migrated_contract` -> 3 passed, exit 0
- `cargo fmt` (applied formatting to the new test file) -> exit 0
- `make check` (fmt --check + clippy --all-targets -D warnings + test) -> exit 0
- `cargo test` (final) -> 76 passed across 8 suites (73 baseline + 3 new), exit 0
- `git rev-parse HEAD` -> 7b1d9e44fa5bc82b8e0dce12c122138708d7b743

## Concerns
- None blocking. Every archived feature-task check was reproduced verbatim, exactly once.
- Note on Task 3 Specs comment: it differs from the archived contract (which had no Specs comment on Task 3). This is the intended D6 outcome, asserted explicitly. If a downstream consumer expected byte-identity with the archived Task 3 header, that expectation is incompatible with D6 by design.
- `domain-type-boundary` scenarios 2-3 were copied verbatim but left acceptance-less (they parse as no-entry scenarios) because the archived contract never referenced them. This keeps the migrated spec faithful to the archived spec without inventing checks the archived contract did not have.
