# Darkfactory

A lightweight multi-agent mission system built from Claude Code primitives.

## What this repo is

This repo contains the Darkfactory skills (the product) and their build history (archived). The skills get installed globally to `~/.claude/skills/` via `/darkfactory-init` and then work in any project.

## Project structure

- `.claude/skills/` -- the skills (mission, mission-resume, mission-status, darkfactory-init)
- `docs/design-decisions.md` -- every architectural choice with rationale
- `UBIQUITOUS_LANGUAGE.md` -- shared terminology (read this first)
- `openspec/` -- clean workspace for any future changes
- `archive/` -- build history (ignored by .claudeignore, don't read unless asked)

## How to work on this repo

- Edit skills directly. They're markdown prompt files, not code.
- Update `docs/design-decisions.md` when making architectural changes.
- Update `UBIQUITOUS_LANGUAGE.md` if new terms emerge.
- Commit directly to main. No branches or PRs needed for this repo.

## Terminology

Use terms from UBIQUITOUS_LANGUAGE.md precisely. Key terms: Mission, Orchestrator, Worker, Validator, Contract, Assertion, Brief, Handoff, Monitor, Engine, Plan mode, Fix loop, Circuit breaker.

## Dependencies

External skills are forked to `layercodedev/skills` and pinned to commit `651eab0` for supply chain security. Don't reference `mattpocock/skills` directly in any install or fetch path.

## Do not

- Add runtime checks or code -- this is a skills-only repo
- Read archive/ unless specifically asked (it's ignored for a reason)
- Reference v1 skills (they're archived, v2 is canonical)
- Suggest multiple workflow paths -- `/mission` is the single entry point for all work
