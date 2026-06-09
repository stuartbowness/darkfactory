# Handoff: Task 1 - Scenario model and parser retention

- **Task**: Task 1 - Scenario model and parser retention
- **Commit SHA**: cafa105aa3aa0f4a818b77cc9e7488ff484ec883

## Approach

Followed the `/tdd` red-green workflow.

1. **Model** (`src/core/types.rs`): Added two fields to `RequirementScenario`:
   - `title: String` — the text after `#### Scenario:`.
   - `behavior: Vec<String>` — the verbatim, ordered WHEN/THEN/AND bullets that
     precede `**Acceptance:**`, with HTML comments and blank lines excluded.
   Changed `spec_ref` from `SpecReference` to `Option<SpecReference>`; it is
   `None` only for an acceptance-less scenario with no `<!-- spec-ref: N -->`.

2. **Parser** (`src/core/parse.rs`): `parse_scenario` now always returns a
   `RequirementScenario` (no longer `Option`), so `parse_specs` retains every
   scenario, including acceptance-less ones (previously dropped). It captures
   the title and the behavior bullets in source order via a new
   `is_behavior_bullet` helper that matches `- **WHEN**`/`- **THEN**`/`- **AND**`.
   Duplicate-spec-ref detection in `parse_specs` now runs only when the scenario
   carries `Some(spec_ref)`. The fail-loud rules are unchanged: an
   acceptance-carrying scenario with no spec-ref still returns `MissingSpecRef`,
   and `EmptyAcceptanceList` / `EmptyStatement` / `UnknownStatus` /
   `UnknownVerificationMethod` / `DuplicateSpecReference` are preserved.

3. **Strict path** (`src/core/render.rs`): `placed_entries` skips a scenario
   whose `spec_ref` is `None` (acceptance-less, no entries), and binds the inner
   `&SpecReference` for the rest. No output change — `build_model`/`render` only
   ever process acceptance-carrying scenarios, which always carry `Some`.
   `src/core/shadow.rs` operates purely on strings and needed no change.

### Tests
- Added `acceptance_less_scenario_is_retained_with_title_and_behavior` (the
  brief's required retention test): proves an acceptance-less scenario is kept
  with its title and behavior captured, `spec_ref == None`, and no entries.
- Added `behavior_text_includes_and_excludes_comments_and_stops_at_acceptance`:
  proves `AND` is captured, comments/blanks excluded, acceptance bullets
  excluded, and an acceptance-carrying scenario carries `Some(spec_ref)`.
- Replaced the two old parse tests that asserted acceptance-less scenarios are
  DROPPED (`...parses_and_contributes_no_entries`,
  `...does_not_require_a_spec_ref`) — that dropped behavior is exactly what
  Task 1 reverses; the new retention test supersedes both.
- Mechanically adapted `Option<SpecReference>` read sites in two integration
  tests (`tests/migrated_contract.rs`, `tests/openspec_validate.rs`). No
  assertion was weakened; only `Option` unwrapping/filtering was added.

## Commands run
- `make check` (baseline, before changes): exit 0.
- `cargo test --lib acceptance_less_scenario_is_retained` (RED, pre-impl): fields
  did not exist, compile error as expected.
- `cargo fmt`: exit 0 (applied formatting twice during the cycle).
- `make check` (final, fmt + clippy `-D warnings` + cargo test): **exit 0**, all
  60 lib tests plus all integration suites pass.
- `make smoke` (CLI end-to-end render + shadow byte-identity): exit 0, 10/10
  passed.

## Concerns
- **Render/shadow tests required attention, but none were weakened.** The render
  test helper `scenario()` (in `#[cfg(test)]`) had to gain `title`/`behavior`
  and wrap `spec_ref` in `Some` to compile; every `#[test]` body and its
  byte-exact output assertion is unchanged. Shadow tests were untouched.
- **`domain-type-boundary/spec.md` fixture has 4 scenarios but only 1 acceptance
  block.** Under the new retention rule, `migrated_scenarios()` now returns 3
  extra acceptance-less scenarios (`spec_ref == None`). These contribute zero
  checks (render skips `None`), so the rendered contract stays byte-identical and
  the migration-equivalence test still passes. Downstream coverage (Task 2) will
  now see those 3 previously-invisible scenarios — that is the intended effect.
- The integration-test read sites use `.as_ref().unwrap()` / `filter_map` on
  `Option<SpecReference>` against fixtures known to be acceptance-carrying. If a
  future fixture adds an acceptance-less scenario to one of those specific
  capabilities, the `inserting_...` / `openspec_validate` reads would need the
  same filtering already applied to the `find`/`map` closures (they currently
  filter rather than unwrap where mixing is possible).
