# Worker Brief: Task 11 - Documentation (closing task: documentation)

## Your Task

Write the tool's documentation and mention it in the repo README. Two files:
1. **Create** `tools/darkfactory-render/README.md` — usage of `render`/`shadow`, the acceptance authoring syntax, the closing-task fragment, AND the cutover-gate criteria + rollback procedure (required by the shadow-comparison spec).
2. **Update** the repo-root `README.md` to mention the `darkfactory-render` tool (repo CLAUDE.md rule).

CRITICAL (repo rule): any command shown in the docs MUST be executed against the BUILT binary and its ACTUAL output pasted — do not invent or paraphrase output. The validator will re-run a documented command and compare.

## Tool README content (`tools/darkfactory-render/README.md`)

Cover, in a clear structure:
- **What it is:** a deterministic tool that renders `contract.md` from authored sources (per-scenario `acceptance` + a standing closing-task fragment), removing duplicated-artifact drift. Stage 1; runs in read-only shadow mode; not wired into the live pipeline yet.
- **Build/install:** `make build` / `make release` / `make install` (to `~/.local/bin`), `make check`, `make smoke`.
- **`render` usage:** `darkfactory-render render --specs <specs_dir> --tasks <tasks.md> --fragment <fragment.md>`. Show a REAL invocation against the committed fixtures and PASTE the actual stdout. Recommended runnable example (run it, paste real output):
  `./target/debug/darkfactory-render render --specs tests/fixtures/cli/specs --tasks tests/fixtures/cli/tasks.md --fragment templates/closing-task-checks.md`
  Document that it prints the contract to stdout, warnings to stderr, exit 0; a `RenderError` → stderr + exit 2.
- **`shadow` usage:** `darkfactory-render shadow --specs ... --tasks ... --fragment ... --existing <contract.md>`. Show a real invocation + output (e.g. against `tests/fixtures/cli/contract_identical.md` → `no differences`, exit 0; and a differing one → diff, exit 1). State it is READ-ONLY (never modifies any file) and the exit codes (0 identical / 1 different / 2 error).
- **Acceptance authoring syntax** (the input the renderer reads): under each `#### Scenario:` heading, `<!-- spec-ref: N -->` (stable integer, unique per capability; Spec Reference = `<capability>/<N>`) and optional `<!-- status: proposed|approved|built|verified|deviated -->` (omitted = non-deviated; a `deviated` scenario's ref is marked `(DEVIATED)`). After the WHEN/THEN bullets, a `**Acceptance:**` line then entries `- [task:N] [<method>] <statement>` where `<method>` is `execution`/`output`/`inspection`. Entries inherit the scenario's Spec Reference; statements render verbatim.
- **Closing-task fragment** (`templates/closing-task-checks.md`): declares the mandatory closing-task KINDS (smoke-test, cleanup, module-shape, claude-arch, documentation) with their `[<method>] <statement>` checks; in tasks.md each closing task carries `Closing: <kind>`; the renderer joins each closing task to its kind's checks. An unknown kind → `UnknownClosingKind`.
- **Cutover gate + rollback** (REQUIRED by the shadow-comparison spec — `specs/shadow-comparison/spec.md` "Document the cutover gate and rollback"): 
  - Cutover gate criteria: validator-outcome parity across diverse mission shapes (small CLI, large multi-task, frontend, deviation-heavy); acceptance and the control (hand-authored) contract authored in separate sessions; a seeded failure caught PLUS at least one real missed-or-weak requirement caught; and a rollback path. Cutover is a deliberate, separate human action — the renderer never auto-cutovers.
  - Rollback procedure: regenerate the hand-authored contract (resume Phase 5 hand-authoring) and restore the prior validator flow; the tool is additive and removable without affecting v1.

## Repo README update (`README.md`, repo root)

Add a concise, accurate mention of `darkfactory-render` (the repo README currently mentions `darkfactory-dispatch` in Architecture and Prerequisites — mirror that placement/style). Describe it as the deterministic contract renderer that (in Stage 1) runs in read-only shadow mode and is not yet wired into the pipeline. Do not overstate. A short paragraph or a bullet near the existing darkfactory-dispatch mention is enough; you may also point to `tools/darkfactory-render/README.md` for details.

## Contract checks this task must satisfy (you don't see the contract)
- `tools/darkfactory-render/README.md` exists and documents `render` and `shadow` usage and the acceptance authoring syntax.
- The documentation describes the cutover gate criteria and a rollback procedure (regenerate the hand-authored contract, restore the prior validator flow).
- The repo `README.md` mentions the `darkfactory-render` tool.
- At least one command shown in the tool README, when run against the built binary, produces the output pasted in the docs.

## What Previous Tasks Built (the surface you are documenting)
- CLI (Task 5, d5e5a4f4): `render --specs/--tasks/--fragment`, `shadow --specs/--tasks/--fragment/--existing`; exit 0/1/2; render→stdout, warnings→stderr; shadow read-only. Built binary at `./target/debug/darkfactory-render` (or `make install`).
- Acceptance grammar (Task 2) + fragment (Task 3, `templates/closing-task-checks.md`, 5 kinds) + render format (Task 4) + migration (Task 6).
- Fixtures: `tests/fixtures/cli/{specs,tasks.md,fragment.md,contract_identical.md,contract_different.md}` (and `templates/closing-task-checks.md`). `make check` (76 tests) + `make smoke` (10/10) green.

## Procedures

**Before writing**, invoke the `/tdd` skill (Skill tool). Documentation task (exempt from the test-presence gate); apply /tdd's discipline: BUILD the binary and RUN every command you document, pasting the real output. Read `specs/shadow-comparison/spec.md` for the exact cutover/rollback requirement.

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render` (for the tool + binary) or `cd /Users/home/.darkfactory-2-build` (for the repo README). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run cargo/make/git in the foreground.
3. Create/edit files with Edit/Write using ABSOLUTE paths: `/Users/home/.darkfactory-2-build/tools/darkfactory-render/README.md` and `/Users/home/.darkfactory-2-build/README.md`.
4. Build first (`cargo build`), run the documented commands, paste the REAL output into the README.

### Verify
- `cd /Users/home/.darkfactory-2-build/tools/darkfactory-render && cargo build` then run each documented command and confirm the pasted output matches.
- `make check` (76 tests) and `make smoke` (10/10) still green (README files don't affect them, but confirm).
- Confirm both files: `ls tools/darkfactory-render/README.md` and `grep -c darkfactory-render README.md` (>0).
- Do NOT modify CLAUDE.md or ARCH.md (already done) or anything under `openspec/changes/`.

### Commit
One commit. Stage ONLY the two README files: `tools/darkfactory-render/README.md` and the repo-root `README.md`. Do NOT `git add -A`/`.`; do NOT stage `openspec/changes/`, CLAUDE.md, or ARCH.md.
```
cd /Users/home/.darkfactory-2-build && git add tools/darkfactory-render/README.md README.md && \
  git commit -m "[task-11] document darkfactory-render (tool README + repo README + cutover/rollback)"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-11.md`
Format: Task; Commit SHA; Approach (what the tool README covers, which commands you ran + pasted, where you mentioned it in the repo README); Commands run + exit codes; Concerns.

## Final message
Reply with the commit SHA, a summary of the tool README sections (incl. the cutover-gate + rollback), confirmation a documented command was run against the built binary with real output pasted, that the repo README mentions the tool, and that `make check`/`make smoke` are green.
