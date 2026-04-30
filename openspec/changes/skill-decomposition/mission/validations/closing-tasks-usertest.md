# User-Test Validation: Closing Tasks (C2, C3, C4)

## Result: PASS

## Test Strategy
Shell assertions on the current HEAD state of /Users/stuart/.darkfactory (branch: mission/skill-decomposition). Commits validated:
- `ab1b35c` — [task-c4] fix stale doc references (covers C2 and C4 work)
- `5f1b120` — [task-c2] handoff

All assertions were executed via direct shell commands. No manual inspection required.

## Tests Run

| # | Test | Method | Result |
|---|------|--------|--------|
| 1 | `git ls-files .claude/skills/` → no output | Shell | PASS |
| 2 | `git ls-files .codex/skills/` → no output | Shell | PASS |
| 3 | `grep -rE "grill-me\|write-a-prd\|ubiquitous-language\|darkfactory-init\|test-quality-review" skills/` → no output | Shell | PASS |
| 4 | `.claude/commands/opsx/*.md` files still exist | Shell | PASS |
| 5 | No `~/.darkfactory-skills/` path refs in `skills/` | Shell | PASS |
| 6 | `ls skills/` lists exactly 12 dirs | Shell | PASS |
| 7 | All 12 skills have SKILL.md with correct `name:` frontmatter | Shell | PASS |
| 8 | `skills/mission/templates/` has 6 required files | Shell | PASS |
| 9 | `skills/mission/prompts/` has 4 required files | Shell | PASS |
| 10 | `skills/tdd/` has SKILL.md + 5 companions (6 total) | Shell | PASS |
| 11 | `mission/validations/module-shape-audit.md` exists | Shell | PASS |
| 12 | `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` → 0 | Shell | PASS |
| 13 | `grep -iE "darkfactory-init\|grill-me\|write-a-prd\|ubiquitous-language\|test-quality-review\|security-review" README.md` → no output | Shell | PASS |
| 14 | README skills table contains mission-init, grill, prd, language, tdd, test-quality, security, adversarial, dispatch | Shell | PASS |
| 15 | `grep -c "install.sh" README.md` ≥ 1 | Shell | PASS |

## Contract Assertions

| # | Task | Assertion | Result | Evidence |
|---|------|-----------|--------|----------|
| 1 | C2 | `.claude/skills/` has no tracked files | PASS | `git ls-files` returned empty output |
| 2 | C2 | `.codex/skills/` has no tracked files | PASS | `git ls-files` returned empty output |
| 3 | C2 | No stale skill name refs in `skills/` | PASS | `grep -rE` exit code 1 (no matches) |
| 4 | C2 | `.claude/commands/opsx/` still has 4 `.md` files | PASS | `apply.md`, `archive.md`, `explore.md`, `propose.md` present |
| 5 | C2 | No `~/.darkfactory-skills/` path refs in `skills/` | PASS | `grep -r "darkfactory-skills"` exit code 1 (no matches) |
| 6 | C3 | `skills/` contains exactly 12 directories | PASS | `ls skills/` shows 12 dirs: adversarial, dispatch, grill, language, mission, mission-init, mission-resume, mission-status, prd, security, tdd, test-quality |
| 7 | C3 | All 12 skills have `SKILL.md` with `name:` frontmatter | PASS | All 12 checked; each has matching `name:` in frontmatter |
| 8 | C3 | `skills/mission/templates/` has 6 files | PASS | contract.md, coverage-audit.md, plan.md, pr-body.md, worker-brief.md, worker-fix-brief.md |
| 9 | C3 | `skills/mission/prompts/` has 4 files | PASS | adversarial-reviewer.md, scrutiny-validator.md, spec-delta-checker.md, usertest-validator.md |
| 10 | C3 | `skills/tdd/` has SKILL.md + 5 companions | PASS | SKILL.md + deep-modules.md, interface-design.md, mocking.md, refactoring.md, tests.md (6 total) |
| 11 | C3 | `mission/validations/module-shape-audit.md` exists | PASS | Found at `openspec/changes/skill-decomposition/mission/validations/module-shape-audit.md` (contract path is relative to the openspec change root) |
| 12 | C4 | `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` → 0 | PASS | exit code 1, 0 matches |
| 13 | C4 | No stale skill names in README.md | PASS | `grep -iE` exit code 1 (no matches for any of the 6 patterns) |
| 14 | C4 | README skills table contains all 9 required skill names | PASS | mission-init: 2, grill: 2, prd: 1, language: 4, tdd: 1, test-quality: 1, security: 1, adversarial: 4, dispatch: 10 |
| 15 | C4 | `grep -c "install.sh" README.md` ≥ 1 | PASS | 1 match (line 34: `./install.sh`) |

## Failed Assertions

None. All 15 assertions passed.

## Notes

- Assertion 11 path interpretation: the contract specifies `mission/validations/module-shape-audit.md`. The task brief (`task-c3.md`) makes explicit that this means `openspec/changes/skill-decomposition/mission/validations/module-shape-audit.md`. The file exists at that path.
- Commit `ab1b35c` removed `.claude/skills/` and `.codex/skills/` tracked files that previously existed (e.g., `darkfactory-init/SKILL.md`, `openspec-*` skill files). Current HEAD has no tracked files in either directory, satisfying C2 assertions 1 and 2.
