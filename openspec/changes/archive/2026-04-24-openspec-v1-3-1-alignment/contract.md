# Contract

## Task 1.1: Fix darkfactory-init skill
<!-- PRD Requirements: #1, #2 -->
- [ ] [output] `.claude/skills/darkfactory-init/SKILL.md` contains a note instructing the agent to use `test -f` patterns (or equivalent always-exit-0 patterns) for file-existence checks
- [ ] [output] `.claude/skills/darkfactory-init/SKILL.md` contains a note instructing the agent to use `|| echo "NOT_INSTALLED"` patterns (or equivalent always-exit-0 patterns) for CLI version checks
- [ ] [output] `.claude/skills/darkfactory-init/SKILL.md` contains `openspec init --tools claude,codex --force` (with the --force flag)
- [ ] [output] `.claude/skills/darkfactory-init/SKILL.md` does NOT contain `openspec init --tools claude,codex` without the --force flag

## Task 2.1: Regenerate openspec skills from v1.3.1
<!-- PRD Requirements: #3, #4, #6 -->
- [ ] [output] `.claude/skills/openspec-propose/SKILL.md` contains `generatedBy: "1.3.1"`
- [ ] [output] `.claude/skills/openspec-apply-change/SKILL.md` contains `generatedBy: "1.3.1"`
- [ ] [output] `.claude/skills/openspec-archive-change/SKILL.md` contains `generatedBy: "1.3.1"`
- [ ] [output] `.claude/skills/openspec-explore/SKILL.md` contains `generatedBy: "1.3.1"`
- [ ] [execution] `grep -r 'generatedBy: "1.2.0"' .claude/skills/openspec-* .codex/skills/openspec-*` returns no matches
- [ ] [execution] `diff .claude/skills/openspec-propose/SKILL.md .codex/skills/openspec-propose/SKILL.md` returns no differences
- [ ] [execution] `diff .claude/skills/openspec-apply-change/SKILL.md .codex/skills/openspec-apply-change/SKILL.md` returns no differences
- [ ] [execution] `diff .claude/skills/openspec-archive-change/SKILL.md .codex/skills/openspec-archive-change/SKILL.md` returns no differences
- [ ] [execution] `diff .claude/skills/openspec-explore/SKILL.md .codex/skills/openspec-explore/SKILL.md` returns no differences

## Task 3.1: Update README skill table
<!-- PRD Requirements: #5 -->
- [ ] [output] `README.md` contains "Bundled with Darkfactory" in the `/opsx:archive` row
- [ ] [output] `README.md` does NOT contain "Installed by `openspec init`" in the `/opsx:archive` row

## Task 4.1: Smoke test
<!-- PRD Requirements: #3, #4 (verification) -->
- [ ] [execution] `grep 'generatedBy: "1.3.1"' .claude/skills/openspec-propose/SKILL.md .claude/skills/openspec-apply-change/SKILL.md .claude/skills/openspec-archive-change/SKILL.md .claude/skills/openspec-explore/SKILL.md` returns 4 matches
- [ ] [output] darkfactory-init SKILL.md contains both the safe Bash pattern note and the `--force` flag

## Task 4.2: Cleanup
<!-- PRD Requirements: all (final sweep) -->
- [ ] [execution] `grep -r 'generatedBy: "1.2.0"' .claude/skills/ .codex/skills/` returns no matches
- [ ] [execution] `grep -r 'Installed by .openspec init.' README.md` returns no matches
