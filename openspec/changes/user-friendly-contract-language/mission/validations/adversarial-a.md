Wrote the adversarial report to [adversarial-a.md](/Users/stuart/.darkfactory/openspec/changes/user-friendly-contract-language/mission/validations/adversarial-a.md).

Key findings:
- HIGH: `UBIQUITOUS_LANGUAGE.md` contains required glossary fixes only in the uncommitted worktree, while `HEAD` still has old assertion/check meanings.
- LOW: task-5 and task-6 handoffs point at rebased commits not contained in the current branch.
- LOW: source/provenance docs may have replaced source wording with project terminology.

Checks run: commit-by-commit `git show --stat` and `git show`, final-state `rg` sweeps, `git diff`, `git show HEAD:UBIQUITOUS_LANGUAGE.md`, and report file verification.