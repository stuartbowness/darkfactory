# Spec Delta Check

## Result: CLEAN

## Coverage Verification

| # | PRD Requirement | Audit Status | Verified | Notes |
|---|----------------|-------------|----------|-------|
| 1 | US1: Single clone + one script to be fully set up | COVERED | YES | `skills/` directory exists with 12 skill dirs. `install.sh` uses a dynamic `for skill_dir in "$SKILLS_DIR"/*/` loop that creates `ln -sf` symlinks for every skill dir found; idempotent via `rm -f` before each `ln -sf`. |
| 2 | US2: Run /grill without starting a mission | COVERED | YES | `skills/grill/SKILL.md` exists as a regular file (forked), `user-invokable: true` in frontmatter. Standalone interview content; no dependency on mission system. |
| 3 | US3: Run /adversarial on any branch | COVERED | YES | `skills/adversarial/SKILL.md` exists. Frontmatter description: "dual-agent adversarial code review." Accepts no-arg (`git diff HEAD`), PR number (`gh pr diff <N>`), or file path (`git diff HEAD -- <path>`). Codex dispatched async in background; Claude dispatched via Agent tool concurrently. |
| 4 | US4: Run /dispatch to send a codex task | COVERED | YES | `skills/dispatch/SKILL.md` exists. Wraps `darkfactory-dispatch` exclusively (explicit "Do not call the Codex CLI directly"). Parses JSON result; presents `completed` / `failed` outcomes clearly. |
| 5 | US5: Run /plan for standalone OpenSpec artifacts | SKIPPED | N/A | Intentionally excluded per audit (no spec, no task). |
| 6 | US6: Run /prd to file a PRD as a GitHub issue | COVERED | YES | `skills/prd/SKILL.md` exists as a regular file (forked), `user-invokable: true`. Final step instructs submitting the PRD as a GitHub issue. |
| 7 | US7: Run /language to update the glossary | COVERED | YES | `skills/language/SKILL.md` exists as a regular file (forked), `user-invokable: true`. Writes/updates `UBIQUITOUS_LANGUAGE.md` in working directory. |
| 8 | US8: Run /tdd for test-driven development | COVERED | YES | `skills/tdd/SKILL.md` exists as a regular file (forked), `user-invokable: true`. 5 companion files present: `tests.md`, `mocking.md`, `deep-modules.md`, `interface-design.md`, `refactoring.md`. SKILL.md references them by relative path. |
| 9 | US9: Run /security for a security review | COVERED | YES | `skills/security/SKILL.md` exists as a regular file (forked), `user-invokable: true`. No "toyo" sections present (verified via grep). Contains full multi-section security review workflow. |
| 10 | US10: /mission calls /grill, /language, /prd by new names | APPROVED-DEVIATION | YES (deviated behavior) | `skills/mission/SKILL.md` calls `/grill` (line 74), `/language` (line 95), `/prd` (line 112) — the new bare names — not the old prefixed names. Deviated behavior implemented as approved. See APPROVED-DEVIATION summary below. |
| 11 | US11: No .codex/skills/ duplicates | COVERED | YES | `git ls-files .codex/skills/` returns empty (0 tracked files). The `.codex/skills/` directory contains only 4 OpenSpec CLI subdirs (`openspec-apply-change`, `openspec-archive-change`, `openspec-explore`, `openspec-propose`) which are out-of-scope managed assets, not skill duplicates. |
| 12 | US12: /mission-init verifies symlinks from install.sh | COVERED | YES | `skills/mission-init/SKILL.md` Step 1 checks all 12 skills by name. For each, it verifies: symlink exists, target exists, `readlink` equals `$HOME/.darkfactory/skills/<name>`. Reports wrong-target and broken states. Instructs user to run `~/.darkfactory/install.sh` if any check fails. |
| 13 | Decision: bare names (no prefix) | COVERED | YES | All 12 skill directories use bare names: `adversarial`, `dispatch`, `grill`, `language`, `mission`, `mission-init`, `mission-resume`, `mission-status`, `prd`, `security`, `tdd`, `test-quality`. Mission-family retains `mission-` prefix as specified. |
| 14 | Decision: skills/ at repo root | COVERED | YES | `~/.darkfactory/skills/` is the canonical location; `install.sh` sets `SKILLS_DIR="$DARKFACTORY/skills"` and symlinks from there. |
| 15 | Decision: install.sh symlinks per skill | COVERED | YES | `install.sh` uses `ln -sf` per skill dir in a loop. Idempotent (`rm -f` before each link). POSIX-portable: no GNU-only flags found (verified via grep for `sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, `grep -P`). |
| 16 | Decision: mission SKILL.md stays monolithic | COVERED | YES | Single `skills/mission/SKILL.md` file (828 lines). All phases inline. No sub-skill files or delegation to separate mission sub-skills beyond calling `/grill`, `/language`, `/prd`, `/mission-init` (user-facing skill invocations, not sub-skill decomposition). |
| 17 | Decision: templates/prompts nested under skills/mission/ | COVERED | YES | `skills/mission/templates/` contains 6 files; `skills/mission/prompts/` contains 4 files. Both subdirectories present. |
| 18 | Decision: fork external skills (not symlink) | COVERED | YES | `grill`, `prd`, `language`, `tdd`, `security` are all regular files (verified). None are symlinks. Adversarial and dispatch are new originals (not forks), also regular files. |
| 19 | Decision: OpenSpec skills left alone | COVERED (out of scope) | N/A | Intentionally excluded per audit. OpenSpec CLI manages its own skills. No check required. |

## Discrepancies

None.

## APPROVED-DEVIATION Summary

| # | PRD Requirement | Classification | Deviated Behavior |
|---|----------------|---------------|-------------------|
| 10 | US10: /mission calls /grill, /language, /prd by new names | Substitution | Original spec preserved `mission/SKILL.md` unchanged (still referencing old names). Actual implementation updated the references to the new bare names (`/grill`, `/language`, `/prd`). The mission SKILL.md now calls these skills by their post-decomposition names, not the old prefixed names. Deviated behavior is implemented: lines 74, 95, 112 of `skills/mission/SKILL.md` confirm the updated references. |
