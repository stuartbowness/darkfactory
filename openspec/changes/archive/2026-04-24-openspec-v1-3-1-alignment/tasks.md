## 1. Fix darkfactory-init skill

- [x] 1.1 Add safe Bash pattern implementation note to darkfactory-init/SKILL.md (for file-existence and CLI-version checks) and add `--force` flag to the `openspec init` command

## 2. Regenerate openspec skills from v1.3.1

- [x] 2.1 Run `openspec update --force` from ~/.darkfactory to regenerate `.claude/skills/openspec-*/SKILL.md`, then copy the regenerated files to `.codex/skills/openspec-*/SKILL.md`

## 3. Update documentation

- [x] 3.1 Update README.md skill table: change `/opsx:archive` source from "Installed by `openspec init`" to "Bundled with Darkfactory"

## 4. Closing tasks

- [x] 4.1 Smoke test: verify `openspec update --force` produces files with `generatedBy: "1.3.1"`, and verify the darkfactory-init skill content includes the safe Bash pattern note and `--force` flag
- [x] 4.2 Cleanup: grep for any remaining `generatedBy: "1.2.0"` or `"Installed by openspec init"` references across the repo and remove them
