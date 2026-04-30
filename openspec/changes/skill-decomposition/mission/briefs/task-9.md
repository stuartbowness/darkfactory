# Worker Brief: Task 9 - Update README.md and UBIQUITOUS_LANGUAGE.md

## Your Task

Make four targeted edits to `README.md` and one check on `UBIQUITOUS_LANGUAGE.md` at `/Users/stuart/.darkfactory/`.

## Context

Skills that were previously external (installed separately from `~/.darkfactory-skills/`) are now bundled in this repo under `skills/`. Their names have changed. `install.sh` (added in the previous task) now handles symlinking. `README.md` still references the old skill names and old install flow in several places. `UBIQUITOUS_LANGUAGE.md` already uses "owned skill" correctly — no changes needed there.

## Exact Changes Required

### Change 1: Replace `/darkfactory-init` in the post-install instructions

Find this block (lines ~49-53):

```
Restart Claude Code, then from any project run:

```
/darkfactory-init
```

This checks your entire setup and tells you exactly what's missing. It verifies core skills, supporting skills (including `/test-quality-review` for LLM-based test quality analysis), CLI tools (codex, gh, openspec), coverage tools for your project's language (cargo-tarpaulin for Rust, jest/c8 for TypeScript, pytest-cov for Python), and initializes OpenSpec in the current project if needed.
```

Replace it with:

```
Restart Claude Code, then from any project run:

```
/mission-init
```

This checks your entire setup and tells you exactly what's missing. It verifies all 12 skills are symlinked, CLI tools (codex, gh, openspec, darkfactory-dispatch, jq), coverage tools for your project's language (cargo-tarpaulin for Rust, jest/c8 for TypeScript, pytest-cov for Python), and initializes OpenSpec in the current project if needed.
```

### Change 2: Replace the 4-row skills table with the full 12-skill table

Find this skills table under the `## Skills` section:

```markdown
| Skill | Purpose |
|-------|---------|
| `/mission` | Start an autonomous mission from scratch. Walks you through planning, dispatches workers, validates, and delivers a draft PR. |
| `/mission-resume` | Resume a mission that was interrupted (session crash, closed terminal). Detects exactly where it left off -- including tasks that were partially validated or committed but not yet checked -- and continues through to the draft PR and code review. |
| `/mission-status` | Read-only progress check. Open a second terminal while a mission runs to see which tasks are done, in progress, or queued. |
| `/darkfactory-init` | Check your project setup. Reports what's installed, what's missing, and gives you the exact commands to fix it. |
```

Replace it with:

```markdown
| Skill | Purpose |
|-------|---------|
| `/mission` | Start an autonomous mission from scratch. Walks you through planning, dispatches workers, validates, and delivers a draft PR. |
| `/mission-resume` | Resume a mission that was interrupted (session crash, closed terminal). Detects exactly where it left off and continues through to the draft PR and code review. |
| `/mission-status` | Read-only progress check. Open a second terminal while a mission runs to see which tasks are done, in progress, or queued. |
| `/mission-init` | Check your project setup. Verifies all 12 skill symlinks, CLI tools, and coverage tools. Runs `openspec init` if needed. |
| `/grill` | Alignment interview: asks pointed questions until the goals, constraints, and edge cases are clear. Used in the mission planning phase. |
| `/prd` | Turns an interview into a structured PRD filed as a GitHub issue. Used in the mission planning phase. |
| `/language` | Creates a shared glossary (ubiquitous language) so you, the orchestrator, and all workers use the same terms. Used in the mission planning phase. |
| `/tdd` | Test-driven development guidance. Workers invoke this during the build phase. Companion files cover deep modules, interfaces, mocking, refactoring, and test patterns. |
| `/test-quality` | LLM-based semantic review of test effectiveness. The scrutiny validator runs this as a subagent after every worker commit. |
| `/security` | Security review. The scrutiny validator runs this when the diff touches auth, secrets, user input, or network/filesystem boundaries. |
| `/adversarial` | Dual-agent code review: dispatches a Codex reviewer and a Claude reviewer with identical input, then surfaces disagreements. |
| `/dispatch` | Run a coding task via `darkfactory-dispatch`. Accepts a task description inline or prompts for one. Presents structured outcome. |
```

### Change 3: Replace the "Required external skills" section

Find the entire section from the heading through the table and the two closing paragraphs:

```markdown
### Required external skills

The skills above are bundled in this repo. But `/mission` also calls other skills during execution. Some are built into Claude Code, some need to be installed separately. The table below shows each skill, where it comes from, and when it's used.

| Skill | Source | When it's used |
|-------|--------|---------|
| `/grill-me` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/grill-me) | Planning phase: the alignment interview that figures out what you actually want |
| `/ubiquitous-language` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/ubiquitous-language) | Planning phase: creates a shared glossary so everyone uses the same terms |
| `/write-a-prd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/write-a-prd) | Planning phase: turns the interview into a structured PRD |
| `/tdd` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/tdd) | Build phase: workers use test-driven development |
| `/review` | Built-in Claude Code (required) | Scrutiny validator: code review step |
| `/simplify` | Built-in Claude Code (required) | Scrutiny validator: complexity check step |
| `/security-review` | Your global Claude skills (conditional) | Scrutiny validator: security audit, only runs when the diff touches auth, secrets, user input, network boundaries, or filesystem trust boundaries |
| `/test-quality-review` | [layercodedev/skills](https://github.com/layercodedev/skills/tree/main/test-quality-review) | Scrutiny validator: LLM-based semantic review of test effectiveness (assertion quality, implementation coupling, untested branches, edge cases) |
| `/test-app` | Your project's Claude skills (optional) | User-test validator: browser-based testing |
| `/demo-video` | Your project's Claude skills (optional) | Records a demo of frontend changes after validation passes |
| `openspec` CLI | Installed by `npm install -g openspec` | Planning phase: creates the structured artifacts (proposal, specs, design, tasks) |
| `/opsx:archive` | Bundled with Darkfactory | Post-merge: moves the change's planning files into the OpenSpec archive directory |

`/review` and `/simplify` are mandatory -- the scrutiny validator always runs them. `/security-review` only runs when relevant. `/test-app` and `/demo-video` are optional and project-specific -- you write these yourself for your project if you want browser testing or demo recordings.
```

Replace it with:

```markdown
### Other skills used by `/mission`

All Darkfactory skills are bundled in this repo. `/mission` also calls two built-in Claude Code skills and accepts two optional project-specific skills:

| Skill | Source | When it's used |
|-------|--------|---------|
| `/review` | Built-in Claude Code (required) | Scrutiny validator: code review step |
| `/simplify` | Built-in Claude Code (required) | Scrutiny validator: complexity check step |
| `/test-app` | Your project's Claude skills (optional) | User-test validator: browser-based testing |
| `/demo-video` | Your project's Claude skills (optional) | Records a demo of frontend changes after validation passes |
| `openspec` CLI | Installed by `npm install -g openspec` | Planning phase: creates the structured artifacts (proposal, specs, design, tasks) |
| `/opsx:archive` | Bundled with Darkfactory | Post-merge: moves the change's planning files into the OpenSpec archive directory |

`/review` and `/simplify` are mandatory -- the scrutiny validator always runs them. `/test-app` and `/demo-video` are optional and project-specific -- you write these yourself for your project if you want browser testing or demo recordings.
```

### Change 4: UBIQUITOUS_LANGUAGE.md — verify no changes needed

Run `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md`. If it returns 0, no change needed — just confirm in your handoff. If it returns non-zero, change "separate skill" to "owned skill" in the test-quality term definition.

## Acceptance Criteria

- `grep -c "ln -sf" README.md` returns 0
- `grep -iE "darkfactory-init|grill-me|write-a-prd|ubiquitous-language|test-quality-review|security-review" README.md` returns no output
- README skills table contains all of: mission-init, grill, prd, language, tdd, test-quality, security, adversarial, dispatch
- README first-time install section contains `install.sh`
- `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` returns 0

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/ (SKILL.md + templates/ + prompts/)
- Created: skills/mission-resume/SKILL.md, skills/mission-status/SKILL.md

### Task 1a: Update mission SKILL.md skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — /grill-me→/grill, /write-a-prd→/prd, /ubiquitous-language→/language; /mission-init call added to Phase 0

### Task 2: Fork grill, prd, language (commits 66500d39 + 2ad413df)
- Created: skills/grill/SKILL.md (name: grill), skills/prd/SKILL.md (name: prd), skills/language/SKILL.md (name: language)

### Task 3: Fork tdd and test-quality (commit 8c12e873)
- Created: skills/tdd/SKILL.md (name: tdd) + 5 companions, skills/test-quality/SKILL.md (name: test-quality)

### Task 4: Fork security (commit c06149050)
- Created: skills/security/SKILL.md (name: security)

### Task 5: Write mission-init (commit 4a3ed785)
- Created: skills/mission-init/SKILL.md (name: mission-init) — replaces /darkfactory-init

### Task 6: Write adversarial (commit 1ac6f1c7)
- Created: skills/adversarial/SKILL.md (name: adversarial)

### Task 7: Write dispatch (commit b639abe1)
- Created: skills/dispatch/SKILL.md (name: dispatch)

### Task 8: Create install.sh (commits 7b1a9b29 + 2d4e351)
- Created: install.sh (executable, #!/bin/sh, loops skills/*/, ln -sf, POSIX-portable)
- Task 8 worker also updated README.md first-time install section to use git clone + ./install.sh

## Procedures

1. Make the four changes described above to README.md
2. Verify UBIQUITOUS_LANGUAGE.md needs no changes (grep check)
3. Commit in a single commit with message: `[task-9] update README skills table and external skills section`
4. Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-9.md`

## Handoff Report Format

- Task: Task 9 - Update README.md and UBIQUITOUS_LANGUAGE.md
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and exit codes>
- Concerns: <anything uncertain>
