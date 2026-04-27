# Contract: skill-decomposition

---

## Task 1: Move mission, mission-resume, mission-status
<!-- PRD Requirements: #16 (monolithic SKILL.md), #17 (templates/prompts nested), #14 (skills/ at repo root) -->

- [ ] [output] `skills/mission/SKILL.md` exists and is a regular file (not a symlink)
- [ ] [output] `skills/mission/templates/` directory exists and contains files (at minimum: plan.md, coverage-audit.md, contract.md, pr-body.md, worker-brief.md, worker-fix-brief.md)
- [ ] [output] `skills/mission/prompts/` directory exists and contains files (at minimum: scrutiny-validator.md, usertest-validator.md, adversarial-reviewer.md, spec-delta-checker.md)
- [ ] [output] `skills/mission-resume/SKILL.md` exists and the frontmatter `name` field is `mission-resume`
- [ ] [output] `skills/mission-status/SKILL.md` exists and the frontmatter `name` field is `mission-status`
- [ ] [output] The commit diff shows `.claude/skills/mission/`, `.claude/skills/mission-resume/`, and `.claude/skills/mission-status/` removed from git tracking
- [ ] [inspection] `skills/mission/SKILL.md` contains all phases 1-9 in a single file (no phase delegated to an external file via sub-skill invocations)

---

## Task 1a: Update mission SKILL.md skill references
<!-- PRD Requirements: #10 (US10: /mission calls renamed skills by new names) -- APPROVED-DEVIATION -->

- [ ] [output] `grep -c "/grill-me" skills/mission/SKILL.md` returns `0`
- [ ] [output] `grep -c "/write-a-prd" skills/mission/SKILL.md` returns `0`
- [ ] [output] `grep -c "/ubiquitous-language" skills/mission/SKILL.md` returns `0`
- [ ] [output] `grep -c "/darkfactory-init" skills/mission/SKILL.md` returns `0`
- [ ] [output] `grep -c "/grill" skills/mission/SKILL.md` returns at least `1` (the replacement reference)
- [ ] [output] `grep -c "/mission-init" skills/mission/SKILL.md` returns at least `1`
- [ ] [inspection] No content changes in `skills/mission/SKILL.md` other than the four substitutions above; all phases, rules, and behavior are intact

---

## Task 2: Fork grill, language, and prd skills
<!-- PRD Requirements: #2 (US2: /grill), #6 (US6: /prd), #7 (US7: /language), #13 (bare names), #18 (fork not symlink) -->

- [ ] [output] `skills/grill/SKILL.md` exists as a regular file (not a symlink) and `grep "^name:" skills/grill/SKILL.md` returns `name: grill`
- [ ] [output] `skills/prd/SKILL.md` exists as a regular file and `grep "^name:" skills/prd/SKILL.md` returns `name: prd`
- [ ] [output] `skills/language/SKILL.md` exists as a regular file and `grep "^name:" skills/language/SKILL.md` returns `name: language`
- [ ] [output] `grep "user-invokable: true" skills/grill/SKILL.md skills/prd/SKILL.md skills/language/SKILL.md` returns three matching lines
- [ ] [inspection] Each skill's body content matches the behavior of its source skill (grill-me, write-a-prd, ubiquitous-language respectively); the only change from the source is the frontmatter name field

---

## Task 3: Fork tdd and test-quality skills
<!-- PRD Requirements: #8 (US8: /tdd), #18 (fork), mission-orchestration spec (scrutiny-validator ref updated) -->

- [ ] [output] `skills/tdd/SKILL.md` exists as a regular file and `grep "^name:" skills/tdd/SKILL.md` returns `name: tdd`
- [ ] [output] All 5 companion files exist: `skills/tdd/deep-modules.md`, `skills/tdd/interface-design.md`, `skills/tdd/mocking.md`, `skills/tdd/refactoring.md`, `skills/tdd/tests.md`
- [ ] [output] `skills/test-quality/SKILL.md` exists as a regular file and `grep "^name:" skills/test-quality/SKILL.md` returns `name: test-quality`
- [ ] [output] `grep -c "/test-quality-review" skills/mission/prompts/scrutiny-validator.md` returns `0`
- [ ] [output] `grep -c "/test-quality" skills/mission/prompts/scrutiny-validator.md` returns at least `1`

---

## Task 4: Fork security skill
<!-- PRD Requirements: #9 (US9: /security), #18 (fork), design: strip toyo sections -->

- [ ] [output] `skills/security/SKILL.md` exists as a regular file and `grep "^name:" skills/security/SKILL.md` returns `name: security`
- [ ] [output] `grep -ic "toyo" skills/security/SKILL.md` returns `0` (no toyo-specific references)
- [ ] [output] `grep -icE "injection|auth|secret|vulnerabilit|OWASP|XSS|SQL" skills/security/SKILL.md` returns at least `1` (general security logic present)

---

## Task 5: Write mission-init skill
<!-- PRD Requirements: #12 (US12: /mission-init verifies symlinks) -->

- [ ] [output] `skills/mission-init/SKILL.md` exists and `grep "^name:" skills/mission-init/SKILL.md` returns `name: mission-init`
- [ ] [output] `grep -cE "grill-me|darkfactory-init|test-quality-review" skills/mission-init/SKILL.md` returns `0` (old names not referenced)
- [ ] [output] `grep -c "install.sh" skills/mission-init/SKILL.md` returns at least `1` (suggests install.sh as fix)
- [ ] [output] `grep -cE "darkfactory-dispatch|codex|openspec|jq" skills/mission-init/SKILL.md` returns at least `1` (CLI tool checks documented)
- [ ] [inspection] Skill verifies `~/.claude/skills/<name>` → `~/.darkfactory/skills/<name>` symlinks for all 12 owned skills; runs `openspec init --tools claude,codex --force` when `openspec/config.yaml` absent

---

## Task 6: Write adversarial skill
<!-- PRD Requirements: #3 (US3: /adversarial on any branch) -->

- [ ] [output] `skills/adversarial/SKILL.md` exists and `grep "^name:" skills/adversarial/SKILL.md` returns `name: adversarial`
- [ ] [output] `grep -c "gh pr diff" skills/adversarial/SKILL.md` returns at least `1` (PR number input mode documented)
- [ ] [output] `grep -c "darkfactory-dispatch" skills/adversarial/SKILL.md` returns at least `1` (codex reviewer via dispatch)
- [ ] [output] `grep -c "adversarial-codex.md" skills/adversarial/SKILL.md` returns at least `1` (codex report path documented)
- [ ] [output] `grep -icE "critical|warning|nit" skills/adversarial/SKILL.md` returns at least `1` (severity grouping documented)
- [ ] [inspection] Skill dispatches Codex reviewer via `darkfactory-dispatch --async` before launching Claude reviewer via Agent tool; both receive identical input; disagreements between the two reports are surfaced after both complete

---

## Task 7: Write dispatch skill
<!-- PRD Requirements: #4 (US4: /dispatch standalone) -->

- [ ] [output] `skills/dispatch/SKILL.md` exists and `grep "^name:" skills/dispatch/SKILL.md` returns `name: dispatch`
- [ ] [output] `grep -c "darkfactory-dispatch" skills/dispatch/SKILL.md` returns at least `1` (uses dispatch binary)
- [ ] [output] `grep -c "codex exec" skills/dispatch/SKILL.md` returns `0` (does not call codex exec directly)
- [ ] [output] `grep -c "\-\-json" skills/dispatch/SKILL.md` returns at least `1` (structured output documented)
- [ ] [output] `grep -icE "fail|timed_out|error" skills/dispatch/SKILL.md` returns at least `1` (failure path documented)
- [ ] [inspection] Skill prompts for task description when no arguments given; presents result summary (status, duration, files written, output path) after dispatch completes

---

## Task 8: Create install.sh
<!-- PRD Requirements: #1 (US1: single clone + one script), #15 (install.sh symlinks per skill), idempotent, POSIX-portable -->

- [ ] [execution] `test -f install.sh && test -x install.sh` exits 0
- [ ] [output] First line of `install.sh` is `#!/bin/sh`
- [ ] [execution] After running `install.sh`, all 12 symlinks exist and resolve: `readlink ~/.claude/skills/mission` returns a path ending in `~/.darkfactory/skills/mission` (and similarly for grill, language, prd, adversarial, dispatch, tdd, test-quality, security, mission-resume, mission-status, mission-init)
- [ ] [execution] Running `install.sh` a second time exits 0 with all 12 symlinks still resolving correctly (idempotency)
- [ ] [output] `grep -E "sed -i|date -d|stat -c|mktemp --suffix|readlink -f|grep -P" install.sh` returns no output (no GNU-only flags)
- [ ] [output] Script output includes one line per skill linked and a final summary line with the total count

---

## Task 9: Update README.md and UBIQUITOUS_LANGUAGE.md
<!-- PRD Requirements: #1 (US1: install process documented), install-script spec (README sections updated) -->

- [ ] [output] `grep -c "ln -sf" README.md` returns `0` (no manual symlink commands in README)
- [ ] [output] README.md first-time install section contains `install.sh` as the single install command
- [ ] [output] `grep -iE "darkfactory-init|grill-me|write-a-prd|ubiquitous-language|test-quality-review|security-review" README.md` returns no output (no legacy skill names)
- [ ] [output] README.md skills table contains entries for current skill names (at minimum: mission-init, grill, prd, language, tdd, test-quality, security, adversarial, dispatch)
- [ ] [output] `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` returns `0` (test-quality no longer described as separate)

---

## Task C1: Smoke test
<!-- PRD Requirements: Testing decision: run /mission-init after install.sh -->

- [ ] [execution] `~/.darkfactory/install.sh` exits 0
- [ ] [execution] All 12 symlinks exist under `~/.claude/skills/`: mission, grill, language, prd, adversarial, dispatch, tdd, test-quality, security, mission-resume, mission-status, mission-init
- [ ] [execution] Each symlink resolves to its `~/.darkfactory/skills/<name>` counterpart (run `readlink ~/.claude/skills/<name>` for each)
- [ ] [output] Handoff documents the output of running `skills/mission-init/SKILL.md` manually (or equivalent verification) and confirms skill symlinks are reported correctly

---

## Task C2: Cleanup
<!-- PRD Requirements: #11 (US11: no .codex/skills/ duplicates), skill-layout spec (legacy dirs deleted) -->

- [ ] [output] `git ls-files .claude/skills/` returns no output (directory fully removed from git)
- [ ] [output] `git ls-files .codex/skills/` returns no output (directory fully removed from git)
- [ ] [output] `grep -r "/grill-me\|/write-a-prd\|/ubiquitous-language\|/darkfactory-init\|/test-quality-review" skills/` returns no output

---

## Task C3: Module shape re-check
<!-- PRD Requirements: #14 (skills/ at repo root with correct structure) -->

- [ ] [output] `ls skills/` lists exactly 12 directories matching the canonical list: adversarial, dispatch, grill, language, mission, mission-init, mission-resume, mission-status, prd, security, tdd, test-quality
- [ ] [output] `ls skills/mission/templates/` and `ls skills/mission/prompts/` each contain at least the files from the original `.claude/skills/mission/` structure
- [ ] [output] `ls skills/tdd/` shows SKILL.md plus all 5 companion files
- [ ] [output] `mission/validations/module-shape-audit.md` exists and documents the shape check result

---

## Task C4: Documentation
<!-- PRD Requirements: #1 (install documented), install-script spec (README complete) -->

- [ ] [output] README.md "First-time install" section shows `git clone` + `~/.darkfactory/install.sh` as the complete setup (two commands)
- [ ] [output] `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` returns `0`
- [ ] [output] README.md does not list grill, prd, language, tdd, test-quality, or security as "required external" from another repo
