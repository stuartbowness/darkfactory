# Darkfactory

Darkfactory is an agent skill system for running autonomous software missions. Skills orchestrate Codex and Claude workers to build, validate, and ship code.

## Repository Map

- `ARCH.md` — Top-level architecture. Read first. Update when you change architecture.
- `skills/` — Agent skills, each with a SKILL.md. Symlinked to ~/.claude/skills/ and ~/.codex/skills/ by install.sh
- `agents/` — Agent definitions pinning model and reasoning effort for Claude-engine mission roles. Symlinked to ~/.claude/agents/ by install.sh
- `skills/mission/` — Mission orchestrator: the main build loop (grill, plan, dispatch, validate, fix, merge)
- `skills/frontend/` — React/TypeScript component workflow with visual verification via chrome-devtools-mcp
- `skills/dispatch/` — Thin wrapper around darkfactory-dispatch for user-facing task dispatch
- `skills/artifact-review/` — Dispatches planning artifacts to Codex for review
- `tools/darkfactory-dispatch/` — Rust process supervisor for Codex workers (timeout, heartbeat, structured JSON)
- `docs/` — Workflow guide, design decisions, known limitations, and provenance
- `examples/dispatch-demo.py` — Credential-free process lifecycle demo
- `tests/test_install.py` — Installer tests with disposable prefixes
- `LICENSE` and `THIRD_PARTY_NOTICES.md` — MIT license and upstream attribution
- `openspec/` — OpenSpec change management (proposals, specs, designs, tasks, contracts)
- `install.sh` — Symlinks all skills to ~/.claude/skills/, ~/.codex/skills/, ~/.agents/skills/

## OpenSpec

Planning artifacts follow the OpenSpec change lifecycle: propose -> spec -> design -> tasks -> contract -> build. Local changes live in `openspec/changes/<change-name>/` (ignored by Git). Curated public examples belong in `examples/`. Run `openspec --help` for CLI usage.

## Rules

- When dispatching work to Codex, always use `darkfactory-dispatch`, never call the Codex CLI directly.
- After making any changes to skills, init, or project configuration, update README.md to reflect the change.
- After making architectural changes, update ARCH.md to reflect the change.
- Shell scripts in skills must work on both Debian/Ubuntu (GNU coreutils) and macOS (BSD coreutils). Avoid GNU-only flags (`sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, `grep -P`). Use POSIX-portable forms instead (e.g., `mktemp /tmp/prefix.XXXXXX`, not `mktemp --suffix=prefix`).

## Versioning

The darkfactory repo has one version: the GitHub release tag (v1.4.x). Internal tools like `darkfactory-dispatch` are pinned at v1.0.0 in their Cargo.toml and never bumped -- the repo release is the only version that matters. When creating a release, increment the repo tag only.

## darkfactory-dispatch

Use `darkfactory-dispatch` to dispatch coding tasks to Codex with process supervision (timeout, heartbeat, process group isolation, structured JSON results). Run `darkfactory-dispatch --help` for full usage and examples.

## Release checks

Run the commands in CONTRIBUTING.md. Do not install into the real home during
tests; use `install.sh --prefix` with a disposable directory. Preserve user
configuration on conflicts. Keep generated mission records and secrets out of
the public tree. Context separation is prompt policy, not access control.
