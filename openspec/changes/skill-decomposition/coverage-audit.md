# Spec Coverage Audit: skill-decomposition

PRD: https://github.com/layercodedev/darkfactory/issues/103

## Coverage Table

| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | US1: Single clone + one script to be fully set up | skill-layout: skills/ directory exists; install-script: install.sh exists, creates all 12 symlinks | Tasks 1, 8, 9 | COVERED |
| 2 | US2: Run /grill without starting a mission | skill-layout: grill skill forked from grill-me, standalone invokable | Task 2 | COVERED |
| 3 | US3: Run /adversarial on any branch | adversarial-review: adversarial skill exists, accepts diff/PR/file target, dual-agent parallel | Task 6 | COVERED |
| 4 | US4: Run /dispatch to send a codex task | dispatch-skill: dispatch skill exists, wraps darkfactory-dispatch, presents result | Task 7 | COVERED |
| 5 | US5: Run /plan for standalone OpenSpec artifacts | (no spec exists) | -- | SKIPPED |
| 6 | US6: Run /prd to file a PRD as a GitHub issue | skill-layout: prd skill forked from write-a-prd, standalone invokable | Task 2 | COVERED |
| 7 | US7: Run /language to update the glossary | skill-layout: language skill forked from ubiquitous-language, standalone invokable | Task 2 | COVERED |
| 8 | US8: Run /tdd for test-driven development | skill-layout: tdd skill forked with 5 companion files, standalone invokable | Task 3 | COVERED |
| 9 | US9: Run /security for a security review | skill-layout: security skill forked from security-review (toyo sections stripped), standalone invokable | Task 4 | COVERED |
| 10 | US10: /mission calls /grill, /language, /prd by new names | mission-orchestration: SKILL.md preserved unchanged (original design) vs PRD: refs updated to new names | Tasks 1, 1a | APPROVED-DEVIATION |
| 11 | US11: No .codex/skills/ duplicates | skill-layout: .codex/skills/ directory deleted, git ls-files .codex/skills/ returns empty | C2 | COVERED |
| 12 | US12: /mission-init verifies symlinks from install.sh | setup-verification: checks ~/.claude/skills/<name> → ~/.darkfactory/skills/<name> for all 12 skills | Task 5 | COVERED |
| 13 | Decision: bare names (no prefix) | skill-layout: 12 skills named without prefix except mission-family | Tasks 1–9 | COVERED |
| 14 | Decision: skills/ at repo root | skill-layout: ~/.darkfactory/skills/ is the canonical location | Task 1 | COVERED |
| 15 | Decision: install.sh symlinks per skill | install-script: ln -sf for each, idempotent, POSIX-portable | Task 8 | COVERED |
| 16 | Decision: mission SKILL.md stays monolithic | mission-orchestration: all phases in single file, no sub-skill delegation | Task 1 | COVERED |
| 17 | Decision: templates/prompts nested under skills/mission/ | mission-orchestration: templates/ and prompts/ subdirs preserved | Task 1 | COVERED |
| 18 | Decision: fork external skills (not symlink) | skill-layout: 6 skills forked as regular files | Tasks 2, 3, 4 | COVERED |
| 19 | Decision: OpenSpec skills left alone | (no spec; managed by openspec CLI / user dotfiles) | -- | COVERED (out of scope) |

## Gaps

| # | PRD Requirement | AI Recommendation | Human Decision |
|---|----------------|-------------------|----------------|
| 5 | US5: /plan standalone OpenSpec skill | Skip — user prefers planning logic to stay inside /mission; opsx:propose already fills this need | **SKIPPED** |

## Deviations Requiring Approval

| # | PRD Requirement | Classification | What PRD says | What we're doing instead | Prerequisites met? | Impact | Human Decision |
|---|----------------|---------------|---------------|--------------------------|-------------------|--------|----------------|
| 10 | US10: /mission calls renamed skills by new names | Substitution | "Mission skill references updated: /grill-me→/grill, /write-a-prd→/prd, /ubiquitous-language→/language throughout SKILL.md" | Task 1a updates all four references in SKILL.md: /grill-me→/grill, /write-a-prd→/prd, /ubiquitous-language→/language, /darkfactory-init→/mission-init | All covered: Task 1 moves SKILL.md first; Task 1a updates refs in the moved file | After this mission /mission will invoke the bare names, which resolve to skills/ via install.sh symlinks | **APPROVED-DEVIATION** (Task 1a added to implement as PRD intended) |

## Notes

- Task 1a was added to tasks.md to resolve Deviation #10. design.md updated accordingly.
- Source paths for forked skills: tasks.md references `~/.darkfactory-skills/`
  which does not exist. Actual sources are in `~/Code/dotfiles/.claude/skills/layercode-skills/`
  (grill-me, write-a-prd, ubiquitous-language, tdd, test-quality-review) and
  `~/Code/dotfiles/.claude/skills/security-review/`. Worker briefs will use these correct paths.
