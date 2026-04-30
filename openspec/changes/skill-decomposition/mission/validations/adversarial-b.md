# Adversarial Review B

## Task 1 -- [task-1] move mission skills to skills/ at repo root
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | nit | (commit) | All 13 files are pure renames (0 insertions shown in stat), meaning no content changes were made. The contract requires a regular file, which is confirmed (git ls-tree shows 100644 blobs). No issues. |

## Task 1a -- [task-1a] update mission SKILL.md skill references to new names
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `skills/mission/SKILL.md` | Only 3 of 4 substitutions were made in this commit: `/grill-me`→`/grill`, `/ubiquitous-language`→`/language`, `/write-a-prd`→`/prd`. The `/darkfactory-init`→`/mission-init` substitution was NOT made here — it was added in the follow-up fix commit as a new line rather than a substitution. This means the original contract assertion "only the four substitutions" was not satisfied in a single atomic commit. The final state is correct, but the commit history shows `/darkfactory-init` was not present in SKILL.md at the base SHA to be substituted — it was added fresh. The contract assertion about "no content changes other than the four substitutions" is met by the combined state but the approach differed from spec intent. |

## Task 1a-fix -- [task-1a] fix: add /mission-init call to Phase 0
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | nit | `skills/mission/SKILL.md` | The `/mission-init` call is inserted BEFORE "Enter plan mode immediately" at line 45, but the phase heading is "Enter Plan Mode." Running an external skill before entering plan mode creates an ordering edge case: if `/mission-init` fails mid-session and the user retries, plan mode hasn't been entered yet. The gate checklist correctly puts `/mission-init` as item 1, so functionally this is acceptable, but the prose ordering is slightly awkward. Not a blocking issue. |

## Task 2 -- [task-2] fork grill, prd, language skills into skills/
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `skills/grill/SKILL.md` | Initial commit created grill, prd, and language WITHOUT `user-invokable: true`. The fix commit (2ad413d) adds it in the next commit. The contract assertion requires `user-invokable: true`, so the assertion would have failed against this commit alone. Mitigated by the fix commit, but the worker missed a contract requirement on first pass. |
| 2 | nit | (all three) | The source skills (`grill-me`, `write-a-prd`, `ubiquitous-language`) are not present in the darkfactory repo at base SHA — they come from an external repo (`layercodedev/skills`). The contract assertion "body content matches behavior of source skill" cannot be verified from this repo alone. This is an untestable assertion for any CI run against this repo. |

## Task 2-fix -- [task-2] fix: add user-invokable: true to grill, prd, language frontmatter
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | nit | `skills/grill/SKILL.md`, `skills/prd/SKILL.md`, `skills/language/SKILL.md` | The diff shows `user-invokable: true` inserted BEFORE the closing `---` delimiter. The final files are correctly structured (verified by reading current state), so no issue with the actual YAML structure. The diff view is misleading but the file is fine. |

## Task 3 -- [task-3] fork tdd and test-quality skills, update scrutiny-validator ref
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | nit | `skills/mission/prompts/scrutiny-validator.md` | After this commit, scrutiny-validator.md references `/security-review` (the old name) in the delegation pattern example on line 248 of the mid-commit state. The C4 cleanup commit later fixes this to `/security`. Not a final-state bug but shows the task-3 worker made an incomplete rename. |

## Task 4 -- [task-4] fork security skill into skills/
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `skills/security/SKILL.md` | The frontmatter contains both `user-invokable: true` (hyphen) and `user_invokable: true` (underscore) — a duplicate key with different separators. This is a copy artifact from the source `security-review` skill. YAML parsers treat these as two distinct keys, so neither overrides the other; the behavior depends on which key the harness reads. One should be removed. |
| 2 | nit | `skills/security/SKILL.md` | The frontmatter also contains `globs: []` — a stale field from the source skill, not present in other Darkfactory skill frontmatters. Harmless but inconsistent. |

## Task 5 -- [task-5] write mission-init skill
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | critical | `skills/mission-init/SKILL.md` | The skill's bash script (line 39) sets `expected="$HOME/.darkfactory/skills/$skill"` with NO trailing slash. But `install.sh` creates symlinks via `ln -sf "$skill_dir" "$target"` where `$skill_dir` is `$SKILLS_DIR/<name>/` (glob expansion of `*/` always includes a trailing slash). `readlink` then returns the value with a trailing slash. Verified in the live environment: all 12 skills report `MISMATCH` — every skill shows `[!] wrong target` when `/mission-init` runs. The skill will report all 12 skills as broken even on a correct installation. |
| 2 | warning | `skills/mission-init/SKILL.md` | The skill uses bare `readlink` without `-f` / `-n` flags. On macOS, `readlink` (BSD) does not follow chains and returns the literal symlink value, which here includes the trailing slash. On Linux, behavior is similar (also returns the literal value). The mismatch is caused by `install.sh`, but the fix needs coordination between both files. |

## Task 6 -- [task-6] write adversarial skill
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `skills/adversarial/SKILL.md` | The dispatch command block uses a single-quoted heredoc (`<< 'PROMPT'`) which suppresses all shell variable expansion. The placeholders `<codex-report-path>` and `<diff content>` inside the heredoc are literal strings — shell will not substitute them. An LLM agent following this skill verbatim would write a prompt file containing the placeholder text rather than actual values. This is an instruction-text convention, not runnable code, but the single-quoted delimiter is misleading and likely to cause errors if an agent generates this block and runs it directly. |
| 2 | nit | `skills/adversarial/SKILL.md` | The mission path detection logic (`openspec/changes/*/mission/`) uses a glob that would match multiple missions if more than one change is active. No disambiguation logic is provided. Low risk since most repos have at most one active mission. |

## Task 7 -- [task-7] write dispatch skill
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `skills/dispatch/SKILL.md` | Same heredoc issue as task-6: `cat > "$PROMPT_PATH" << 'TASK'` with single-quoted delimiter. The placeholder `<task description>` inside the heredoc will NOT be substituted. An agent running this block verbatim would write the literal string `<task description>` as the Codex prompt. |

## Task 8 -- [task-8] create install.sh
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | critical | `install.sh` | `ln -sf "$skill_dir" "$target"` where `$skill_dir` comes from `"$SKILLS_DIR"/*/` glob expansion — the glob always appends a trailing slash to directory matches. The symlink therefore points to `.darkfactory/skills/mission/` (trailing slash). The contract assertion (Task 8) requires `readlink` to return a path "ending in `~/.darkfactory/skills/mission`". The actual value ends in `mission/`, not `mission`. This will cause the contract execution check to fail. Fix: strip the trailing slash with `ln -sf "${skill_dir%/}" "$target"`. |
| 2 | nit | `install.sh` | The script uses `rm -f "$target"` before every `ln -sf` for idempotency. This is correct behavior but means install.sh silently replaces ANY existing symlink in `~/.claude/skills/` whose name matches a darkfactory skill, including symlinks that the user may have pointing elsewhere. No warning is given before overwriting. |

## Task 8-fix -- [task-8] guard empty skills dir and POSIX glob edge case
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | nit | `install.sh` | The `[ -d "$skill_dir" ] || continue` guard is correct for the POSIX glob case where no matches produce a literal `*/` entry. However, if `$SKILLS_DIR` contains only files (no subdirs), the loop executes once with `skill_dir = "$SKILLS_DIR"/*/` (literal), `[ -d ... ]` fails, `continue` fires, and `count` stays 0. This is the intended behavior and it works correctly. |

## Task 9 -- [task-9] update README skills table and external skills section
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `README.md` | The First-time install section uses three commands: `git clone ...`, `cd ~/.darkfactory`, `./install.sh`. The contract (Task C4) requires "two commands: `git clone` + `~/.darkfactory/install.sh`". The `cd` is a third command. A user can run `~/.darkfactory/install.sh` directly without cd. This is a minor divergence from the contract requirement. |
| 2 | nit | `README.md` | Line 257 still contains the phrase "separate skill" (`/mission-status` is described as "a separate skill, not a flag on `/mission`"). The contract check `grep -c "separate skill" UBIQUITOUS_LANGUAGE.md` passes because it targets UBIQUITOUS_LANGUAGE.md (which is clean), not README.md. The README occurrence is about `mission-status`, not `test-quality`, so it is semantically correct. No contract violation, but worth noting. |

## Task OpenSpec -- Add skill-decomposition OpenSpec artifacts and coverage audit
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task OpenSpec2 -- Add mission directory, contract, and coverage audit for skill-decomposition
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task C4 -- [task-c4] fix stale doc references
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `~/.claude/skills/darkfactory-init` (filesystem, not repo file) | This commit deletes `.claude/skills/darkfactory-init/SKILL.md` from the repo. Any user who had the old `darkfactory-init` skill symlinked via `~/.claude/skills/darkfactory-init -> ~/.darkfactory/.claude/skills/darkfactory-init` now has a broken symlink. `install.sh` does not clean up this stale symlink because it only creates the 12 new named symlinks. The user is left with a broken `/darkfactory-init` that points to a deleted file. Confirmed broken in the live environment. |
| 2 | nit | `openspec/changes/skill-decomposition/mission/progress.md` | This commit adds `progress.md` with 90 lines as a side effect. Committing progress state in the same commit as code changes mixes task work with orchestration bookkeeping. Not a correctness issue. |

## Task C2 -- [task-c2] cleanup old skill refs and remove dead skill directories
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | warning | `.codex/skills/` (filesystem) | The handoff documents that `.codex/skills/` could not be removed from disk (protected untracked files). The directory still exists on disk with 4 subdirectories. `git ls-files .codex/skills/` returns no output so the git-tracking contract assertion passes. However, any tool that scans `~/.darkfactory` for skills by directory listing (rather than by git) would encounter these stale untracked directories. This is an environment-specific issue and the handoff correctly acknowledges it. |

## Cross-task
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | critical | `install.sh` + `skills/mission-init/SKILL.md` | The trailing-slash mismatch is a cross-file bug: `install.sh` stores symlink targets WITH a trailing slash; `mission-init` compares against targets WITHOUT a trailing slash. These two files were written independently and no integration test was run. Every `/mission-init` call on a correctly installed system will report all 12 skills as having the wrong target. The fix is in `install.sh`: change `ln -sf "$skill_dir"` to `ln -sf "${skill_dir%/}"`. |
| 2 | warning | `skills/security/SKILL.md` | The `user_invokable: true` (underscore) leftover alongside `user-invokable: true` (hyphen) is a copy artifact. No other skill has both or uses the underscore form. If the harness checks `user-invokable` specifically, the underscore key is silently ignored. If any tooling checks `user_invokable`, the hyphen key is silently ignored. Should be cleaned to a single `user-invokable: true`. |
| 3 | warning | `~/.claude/skills/darkfactory-init` | `install.sh` only creates new symlinks for the 12 owned skills — it has no cleanup step to remove obsolete darkfactory-owned symlinks (like the now-deleted `darkfactory-init`). After C4 deletes the target, the old symlink in `~/.claude/skills/` is left broken indefinitely. `mission-init` does not check for or warn about broken non-owned symlinks, so this goes unreported. |
| 4 | nit | `skills/mission-resume/SKILL.md`, `skills/mission-status/SKILL.md`, `skills/mission/SKILL.md` | These three user-facing skills lack `user-invokable: true` in their frontmatter. The contract only required it for grill, prd, and language (Task 2), so this is not a contract violation. But the convention in all other new skills is to include this field, and these are directly user-invoked. |

## Summary

4 findings at critical or warning severity. The biggest risk is the trailing-slash mismatch between `install.sh` (which stores symlinks as `skills/mission/` with a trailing slash) and `mission-init` (which expects `skills/mission` without). This bug causes every `/mission-init` call on a correctly installed system to report all 12 skills as broken, defeating the purpose of the skill. The fix is a one-character change in `install.sh`: `"${skill_dir%/}"` instead of `"$skill_dir"`. The `security/SKILL.md` duplicate frontmatter key (`user-invokable` vs `user_invokable`) is a cosmetic bug with low runtime impact. The broken `darkfactory-init` symlink left on disk after C4 is a user-experience issue but does not affect mission operation. The README install section uses 3 commands instead of the contracted 2. Do not ship the `install.sh`/`mission-init` pairing as-is; the trailing-slash fix is a one-line change and should be resolved before the first real user runs `/mission-init`.
