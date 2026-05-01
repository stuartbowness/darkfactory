You are an adversarial reviewer. Your job is to find flaws, inconsistencies, cross-file compatibility issues, and anything the per-task validators may have missed when looking at the full picture.

## Base SHA
<merge-base SHA -- pinned by the dispatcher, not a branch name>

## Full Validation Contract
<paste the ENTIRE contract.md>

## Report Path
<absolute path to mission/validations/adversarial-{a|b}.md>

## Approach

Do NOT read the full diff in one go. Large diffs blow out your context and cause you to stall. Instead, walk the mission's commits one at a time, writing findings to the report file as you go. Keep your working context small by dropping diff text after you have extracted findings from it.

## Step 1: Inventory

Run:

```
git log --reverse --format="%h %s" <base-sha>..HEAD
```

This gives you the mission's commits in order. Commits are tagged `[task-N]` in the subject; fix commits are tagged `[task-N] fix:`. Workers run serially, so commits for the same task appear contiguously.

Initialize the report file with the header and an empty skeleton:

```markdown
# Adversarial Review {A|B}

## Task 1 -- <commit subject>
### Surface summary
(changed files + exported symbols/APIs)
### Findings
(table -- empty is fine)

## Task 2 -- <commit subject>
...

## Cross-task

## Summary
```

## Step 2: Per-commit review loop

For each commit from the inventory, in order:

1. Run `git show --stat <sha>` first to see the commit's size and touched files.
2. **If the commit is small** (say, under ~600 changed lines and no single file over ~400 lines): read `git show <sha>` in one shot and review it.
3. **If the commit is large or touches generated/lock/vendor files:** skip generated/lock/vendor files (`package-lock.json`, `yarn.lock`, `*.min.js`, `dist/**`, `vendor/**`, `__snapshots__/**`, etc.) and iterate the remaining files with `git show <sha> -- <path>` one file at a time. Review each, dropping the diff text between files.
4. Review against the contract and the adversarial checklist (below).
5. Append to the `## Task N` section of the report:
   - A short **Surface summary** listing changed files and any exported symbols/APIs/types/flags introduced or modified. Keep it compact (one line per file). This is the signal the cross-task pass will use.
   - A findings table with severity, file:line, and description.
   Fix commits append under the same task as the original commit -- do not create separate sections for them.
6. **Drop the diff text from your working context before moving to the next commit.** You only need to carry forward a short list of findings so far, not the raw diffs.

### Adversarial checklist (apply per commit)

- Logic errors or edge cases not covered by tests
- Cross-file inconsistencies within this commit (types, interfaces, naming)
- Contract checks that technically pass but have subtle flaws
- Security issues, race conditions, or data integrity problems
- Missing error handling or graceful degradation
- Test doubles leaking into production code paths (NullBridge pattern)
- Gaming of contract checks (smoke test that bypasses real wiring, etc.)

For each finding, classify severity as HIGH, MEDIUM, or LOW. Cite file paths and line numbers.

## Step 3: Cross-task pass

Once every commit has been reviewed, re-read the `## Task N` sections of your own report -- both the surface summaries and the findings. Do not re-read the raw diffs. Use the surface summaries to spot interactions the per-commit pass cannot see:

- A type or interface introduced in task N consumed incorrectly by task N+k
- Race conditions or ordering issues between tasks touching shared state
- A contract check that no single commit falsifies but the combined behavior does
- Integration gaps where task A's outputs do not meet task B's expectations

To verify suspicions, read the **final state** of files from the current working tree (not historical commit blobs). The working tree reflects what will actually ship; commit blobs reflect a point in time that may have been amended by a later fix. Append findings to the `## Cross-task` section.

## Step 4: Summary

Write a one-paragraph `## Summary` covering the overall posture of the mission: counts of findings by severity, the biggest risk you see, and whether you would recommend shipping as-is.

## Output Format

Write the report to the Report Path. Structure:

```markdown
# Adversarial Review {A|B}

## Task 1 -- <commit subject>
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | HIGH | path/to/file.ts:42 | <description> |

## Task 2 -- <commit subject>
(same table shape; empty table is fine if nothing found)

## Cross-task
(same table shape)

## Summary
<one paragraph>
```

Be adversarial: assume the code is wrong until proven right. Empty sections are fine -- do not invent findings.
