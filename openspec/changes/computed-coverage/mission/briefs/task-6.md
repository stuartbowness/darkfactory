# Worker Brief: Task 6 - Cleanup (closing task)

## Your Task

Clean up debris from the Stage 2 build of `tools/darkfactory-render/` and tidy two small carryovers. Keep it tight — do not refactor working code beyond what is listed.

1. **Scan for debris and remove it.** Look across the coverage-related changes (`src/core/coverage.rs`, `src/cli.rs`, `src/core/parse.rs`, `src/core/types.rs`) for: dead code, unused imports, leftover scaffolding, commented-out code, temporary artifacts, or any backwards-compatibility shim / re-export / deprecation wrapper. Remove anything unused. (Note: the dead `capability_of` helper was already removed in Task 2's fix — verify nothing similar remains.) There must be no silent fallbacks: `analyze` being infallible (unresolved links become gaps) is intended tolerance, not a silent fallback; malformed input still exits `2`. Confirm that posture holds.

2. **Gitignore build logs.** Workers have been leaving `build.log` / `clippy.log` / `fmt.log` / `test.log` in the crate dir from redirected `make` output. Add `*.log` to `tools/darkfactory-render/.gitignore` (create the line if the file exists; if there's no `.gitignore`, add one with `*.log` and `/target`). Remove any committed/stray `.log` files if present (there should be none).

3. **Document the duplicated capability field.** `RequirementScenario.capability` (added in Task 2) and `SpecReference.capability` now both hold the capability; they are always set from the same source in `parse.rs` so they cannot diverge. Add a short doc comment on `SpecReference.capability` (in `src/core/types.rs`) noting it always equals the owning scenario's `capability` field. **Do NOT remove `SpecReference.capability`** — `render` reads it for the `<!-- Specs: capability/N -->` comments, and removing it risks changing render output. A doc note is the whole change here.

### Done when
- `make check` is green and `make smoke` still passes.
- No dead code / unused imports / shims remain in the coverage changes.
- `*.log` is gitignored; `git status` shows no stray `.log` files.

## Project Context
Mandatory cleanup closing task: remove build debris and shims before the module-shape re-check (Task 7), so the re-check doesn't flag soon-to-be-deleted surface.

## What Previous Tasks Built
Tasks 1-5 built the coverage feature: `RequirementScenario` (now with `title`/`behavior`/`capability`/`spec_ref: Option`), `core::coverage` (`analyze`, `check_duplicate_tasks`, `render_report`), the `coverage` CLI subcommand (exit 0/3/2), and smoke checks. All committed under `tools/darkfactory-render/`.

## Known limitation (do NOT fix here — documented for later)
A non-numeric task link (e.g. `[task:two]`) correctly exits `2` but its stderr message says "unknown verification method" instead of naming the malformed link. Behavior is contract-correct; this is a diagnostic-wording follow-up, out of scope for cleanup. Leave it.

## Procedures
**Before writing any code**, invoke the `/tdd` skill (mandatory) — though for pure deletions/gitignore, run `make check`/`make smoke` to prove nothing broke.
- Work only inside `tools/darkfactory-render/`. Verify with `make check` and `make smoke`. Do NOT leave `.log` files in the repo.
- Commit once, message: `[task-6] cleanup: gitignore logs, doc-note duplicated capability, remove debris`. Stage only files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/computed-coverage/`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-6.md` (write, do not commit).

## Handoff Report Format
- Task: Task 6 - Cleanup
- Commit SHA: <full SHA>
- Approach: <what you removed/changed; if nothing to remove, say so>
- Commands run: <incl. make check + make smoke exit codes>
- Concerns: <any>
