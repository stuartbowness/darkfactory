# Contract: contract-renderer

Validation checks grouped by task. Workers never see this file. Each check is observable from outside the code. Spec references use `capability/N` (scenario order within each spec file).

Verification method tags: `[execution]` (run a command/test), `[output]` (inspect a file/output), `[inspection]` (read code).

## Task 1: Crate skeleton, domain types, RenderError, Makefile
<!-- Specs: acceptance-field/6, acceptance-field/7, acceptance-field/13, acceptance-field/14 -->
- [ ] [execution] From `tools/darkfactory-render`, `make check` (cargo fmt --check, clippy --all-targets -D warnings, cargo test) exits 0.
- [ ] [execution] A unit test shows parsing `[execution]`, `[output]`, `[inspection]` yields the three VerificationMethod variants, and an unrecognized tag returns a `RenderError`.
- [ ] [execution] A unit test shows an unrecognized scenario status returns a `RenderError`, and an omitted status is treated as non-deviated.
- [ ] [output] `tools/darkfactory-render/Cargo.toml` declares `version = "1.0.0"`.

## Task 2: Spec and acceptance parsing
<!-- Specs: acceptance-field/1, acceptance-field/2, acceptance-field/3, acceptance-field/4, acceptance-field/5, acceptance-field/8, acceptance-field/10 -->
- [ ] [execution] Parsing a scenario with one acceptance entry exposes its statement, verification-method tag, and task link, with the Spec Reference inherited from the scenario.
- [ ] [execution] Parsing a scenario with multiple acceptance entries yields independent entries that all share the scenario's Spec Reference.
- [ ] [execution] A scenario that omits `acceptance` parses successfully and contributes no check.
- [ ] [execution] A present-but-empty `acceptance` list, and an empty/whitespace-only check statement, each return a `RenderError` naming the offending scenario/entry.
- [ ] [execution] A scenario that has acceptance but no `spec-ref` returns a `MissingSpecRef` error; two scenarios sharing a Spec Reference return a `DuplicateSpecReference` error.
- [ ] [execution] Running `openspec validate` on a fixture spec file that carries an acceptance block exits 0 (acceptance syntax is validate-safe).

## Task 3: Task-list and closing-task fragment parsing
<!-- Specs: foundational parsing — produces the closing-task fragment source for contract-render/6 (verified rendered in Task 4) and the task keys for contract-render/8 -->
- [ ] [execution] Parsing a tasks.md fixture extracts each task's number, name, and `Closing: <kind>` field where present.
- [ ] [execution] Parsing `templates/closing-task-checks.md` yields the closing-task kinds mapped to their checks.
- [ ] [output] `templates/closing-task-checks.md` exists and declares kinds `smoke-test`, `cleanup`, `module-shape`, `claude-arch`, and `documentation`.
- [ ] [execution] A task whose `Closing: <kind>` is absent from the fragment returns an `UnknownClosingKind` error.

## Task 4: Contract render
<!-- Specs: contract-render/1, contract-render/2, contract-render/3, contract-render/4, contract-render/5, contract-render/6, contract-render/7, contract-render/8, contract-render/9, contract-render/10, contract-render/11, contract-render/12, contract-render/13, contract-render/14, contract-render/15, contract-render/16 -->
- [ ] [execution] Rendering places each spec-derived check under the `## Task N` section named by its acceptance entry's task link, formatted `- [ ] [<method>] <statement>`.
- [ ] [execution] The rendered check text equals the acceptance statement verbatim (no rewording).
- [ ] [execution] Checks within a task render in Spec Reference order (capability, then N), with ties broken by source order.
- [ ] [execution] Each task emits exactly one `<!-- Specs: ... -->` comment whose references are de-duplicated and ordered.
- [ ] [execution] A check whose scenario has `status: deviated` is marked `(DEVIATED)` in its task's Specs comment.
- [ ] [execution] Closing-task (operational) checks render under their closing tasks and carry no `<!-- Specs -->` comment.
- [ ] [execution] An acceptance entry linking to a closing task returns an `AcceptanceLinksClosingTask` error.
- [ ] [execution] Rendering the same input twice produces byte-identical output.
- [ ] [execution] Task sections render in task-list order.
- [ ] [execution] A dangling task link, a feature task with zero checks, and a duplicate Spec Reference each return the corresponding `RenderError` rather than a partial contract.
- [ ] [output] The rendered contract uses `## Task N: <name>` sections, `<!-- Specs: capability/N -->` comments, and `- [ ] [execution|output|inspection] <statement>` checks (existing-format fidelity).
- [ ] [execution] A task whose `[inspection]` checks exceed 20% of its checks produces a non-fatal warning; a task at or under 20% produces none.

## Task 5: Shadow compare and CLI
<!-- Specs: shadow-comparison/1, shadow-comparison/2, shadow-comparison/3, shadow-comparison/4, contract-render/15 -->
- [ ] [execution] `darkfactory-render render` over a fixture writes a contract in the expected format and exits 0.
- [ ] [execution] `darkfactory-render shadow <existing-contract>` outputs a diff and leaves the existing `contract.md` byte-unchanged.
- [ ] [execution] When the rendered and existing contracts are identical, `shadow` reports no differences.
- [ ] [execution] A `RenderError` makes the CLI exit non-zero with a message on stderr; an inspection-cap breach prints a warning to stderr and exits 0.
- [ ] [execution] A `shadow` run modifies no mission-skill or pipeline files (verified by checking the working tree is unchanged except for any explicit output path).

## Task 6: Stable-ref migration and archived-fixture integration test
<!-- Specs: acceptance-field/9, acceptance-field/11, shadow-comparison/2 -->
- [ ] [execution] An integration test renders the migrated archived fixture and asserts the result is equivalent to the hand-authored contract (same checks under the same tasks).
- [ ] [execution] A test shows that inserting a new scenario does not change the Spec References of existing scenarios.
- [ ] [execution] A test shows a migrated position-numbered capability yields stable Spec References that resolve to the same scenarios as before.

## Task 7: Smoke test
<!-- Specs: (operational closing task) -->
- [ ] [execution] `tests/smoketest.sh` builds the binary, runs `render` on a fixture and asserts a contract is emitted, runs `shadow` and asserts the existing contract is unchanged, and exits 0.

## Task 8: Cleanup
<!-- Specs: (operational closing task) -->
- [ ] [execution] `cargo clippy --all-targets -- -D warnings` exits 0 (no dead-code or unused warnings).
- [ ] [execution] `grep -rn` over `tools/darkfactory-render/src` finds no `todo!()`, `unimplemented!()`, or `#[allow(dead_code)]` left behind.

## Task 9: Module shape re-check
<!-- Specs: (operational closing task) -->
- [ ] [output] `mission/validations/module-shape-audit.md` exists and records, per design.md module and Type Catalog type, whether drift was found and what was done.

## Task 10: Update ARCH.md for the new tool
<!-- Specs: (operational closing task) -->
- [ ] [output] `ARCH.md` contains a section describing the `darkfactory-render` tool.
- [ ] [execution] `git diff` shows the root `CLAUDE.md` was not modified by this mission.

## Task 11: Documentation
<!-- Specs: shadow-comparison/5 -->
- [ ] [output] `tools/darkfactory-render/README.md` exists and documents the `render` and `shadow` usage and the acceptance authoring syntax.
- [ ] [output] The documentation describes the cutover gate criteria and a rollback procedure (regenerate the hand-authored contract, restore the prior validator flow).
- [ ] [output] The repo `README.md` mentions the `darkfactory-render` tool.
- [ ] [execution] At least one command shown in the tool README, when run against the built binary, produces the output pasted in the docs.
