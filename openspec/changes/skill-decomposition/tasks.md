# Tasks: skill-decomposition

## Feature Tasks

- [x] 1. Move mission skill, mission-resume, and mission-status
  Move `.claude/skills/mission/` (SKILL.md + templates/ + prompts/) to `skills/mission/`. Move `.claude/skills/mission-resume/` to `skills/mission-resume/`. Move `.claude/skills/mission-status/` to `skills/mission-status/`. Do not change any skill content in any of these three.

- [x] 1a. Update mission SKILL.md skill references
  In `skills/mission/SKILL.md`, replace all occurrences of old skill invocation names with new ones: `/grill-me` → `/grill`, `/write-a-prd` → `/prd`, `/ubiquitous-language` → `/language`, `/darkfactory-init` → `/mission-init`. No other content changes. This implements PRD user story 10 and was added during the coverage audit.

- [x] 2. Fork grill, language, and prd skills
  Copy `~/.darkfactory-skills/grill-me/SKILL.md` to `skills/grill/SKILL.md`, update frontmatter name to `grill`. Copy `~/.darkfactory-skills/ubiquitous-language/SKILL.md` to `skills/language/SKILL.md`, update frontmatter name to `language`. Copy `~/.darkfactory-skills/write-a-prd/SKILL.md` to `skills/prd/SKILL.md`, update frontmatter name to `prd`. Content otherwise unchanged.

- [x] 3. Fork tdd and test-quality skills
  Copy `~/.darkfactory-skills/tdd/SKILL.md` and all 5 companion files (`deep-modules.md`, `interface-design.md`, `mocking.md`, `refactoring.md`, `tests.md`) to `skills/tdd/`, update frontmatter name to `tdd`. Copy `~/.darkfactory-skills/test-quality-review/SKILL.md` to `skills/test-quality/SKILL.md`, update frontmatter name to `test-quality`. Also update the one reference in `skills/mission/prompts/scrutiny-validator.md`: replace `/test-quality-review` with `/test-quality` in check 14.

- [x] 4. Fork security skill
  Copy `~/.darkfactory-skills/security-review/SKILL.md` to `skills/security/SKILL.md`. Update frontmatter name to `security`. Remove any toyo-review-specific sections (references to toyo's review conventions or toyo-specific severity levels). Preserve all general security review logic.

- [x] 5. Write mission-init skill
  Write `skills/mission-init/SKILL.md`. This replaces `darkfactory-init` with symlink-based verification. The skill checks that each of the 12 skills has a symlink at `~/.claude/skills/<name>` pointing to `~/.darkfactory/skills/<name>`. On missing or broken symlinks it suggests running `install.sh`. Retains: CLI tool checks (codex, darkfactory-dispatch, gh, openspec, jq), coverage tool detection per project language (Rust/TypeScript/Python), and `openspec init --tools claude,codex --force` if `openspec/config.yaml` is absent.

- [x] 6. Write adversarial skill
  Write `skills/adversarial/SKILL.md`. Standalone dual-agent review skill: dispatches a Codex reviewer via `darkfactory-dispatch --async`, then launches a Claude reviewer via the Agent tool. Both receive identical input (current diff by default; PR diff via `gh pr diff <number>` if a PR number is given; a specific file if a path is given). Reports written to `mission/validations/adversarial-codex.md` and `adversarial-claude.md` (or temp paths outside a mission). After both complete, surfaces disagreements between the two reports.

- [x] 7. Write dispatch skill
  Write `skills/dispatch/SKILL.md`. Thin user-facing wrapper around the `darkfactory-dispatch` binary. Accepts a task description inline or prompts for one. Calls `darkfactory-dispatch` with a prompt file and `--json`, parses the structured result, and presents outcome (status, duration, files written, output path). Reports failure clearly; does not suppress it.

- [x] 8. Create install.sh
  Write `install.sh` at the repo root. POSIX-portable (`#!/bin/sh`), loops over `skills/*/` directories, creates `ln -sf` symlinks from `~/.claude/skills/<name>` to `~/.darkfactory/skills/<name>` for each. Idempotent. Prints one line per skill and a final count. Make the file executable.

- [x] 9. Update README.md and UBIQUITOUS_LANGUAGE.md
  Rewrite three README sections: (1) First-time install — replace the block of manual `ln -sf` commands with `git clone` + `~/.darkfactory/install.sh`. (2) Skills table — replace the 4-row legacy table with the full 12-skill table. (3) Required external skills section — skills previously listed as external (`grill-me`, `ubiquitous-language`, `write-a-prd`, `tdd`, `test-quality-review`, `security-review`) are now bundled; remove them from the external table, keep only `/review` and `/simplify` (built-in Claude Code) and `/test-app` and `/demo-video` (optional project-specific). Update UBIQUITOUS_LANGUAGE.md: change "separate skill" to "owned skill" in the test-quality term definition.

## Closing Tasks

- [ ] C1. Smoke test
  Run `~/.darkfactory/install.sh` and verify: (1) all 12 symlinks exist under `~/.claude/skills/` and resolve to the correct `~/.darkfactory/skills/<name>` targets; (2) running `/mission-init` in a project with a `Cargo.toml`, `package.json`, or `pyproject.toml` reports the expected coverage tool status.

- [x] C2. Cleanup
  Remove any dead code, leftover scaffold, or draft files introduced during the mission. Check that no skill file contains references to old names (`grill-me`, `write-a-prd`, `ubiquitous-language`, `darkfactory-init`, `test-quality-review`). Verify no skill references the external `~/.darkfactory-skills/` repo path.

- [ ] C3. Module shape re-check
  Audit the final `skills/` directory against `design.md`. Verify the 12 skill directories exist with correct names, each has a `SKILL.md` with valid frontmatter, `skills/mission/` has `templates/` and `prompts/` subdirectories with all original files, and `skills/tdd/` has all 5 companion files. Flag any drift from the design.

- [ ] C4. Documentation
  Verify `README.md` accurately describes the post-refactor install and skill set. Verify `UBIQUITOUS_LANGUAGE.md` no longer describes test-quality as a "separate skill". Update any other in-repo documentation that references the old skill names or old install process.
