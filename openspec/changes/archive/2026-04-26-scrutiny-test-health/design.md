## Context

The scrutiny validator is a single markdown prompt file at `.claude/skills/mission/prompts/scrutiny-validator.md`. It runs as a Claude subagent during Phase 8 of every mission. The prompt defines 11 checks that the LLM executes in sequence, mixing shell commands (build, test, lint) with LLM judgment (code review, module depth, code standards) and skill invocations (`/review`, `/simplify`, `/security-review`).

This design adds 3 new checks and enhances 3 existing checks, plus a new skill in a separate repo and an update to `darkfactory-init`. The changes are prompt engineering and skill authoring -- no runtime code, no binaries, no new infrastructure.

### Current state

- Scrutiny validator: 11 checks, ~108 lines of markdown prompt
- darkfactory-init: 5 steps checking skills, OpenSpec, and CLI dependencies
- layercodedev/skills: 15 skills, each with SKILL.md and optional reference files

### Constraints

- The scrutiny validator is a prompt, not code. "Implementation" means writing instructions the LLM follows. Testing means running missions that use the prompt.
- Two repos: darkfactory (prompt, init) and layercodedev/skills (new skill). Separate PRs.
- All checks must work across Rust, TypeScript/Node/Next.js, and Python without project-specific configuration.

## Goals / Non-Goals

**Goals:**
- Catch the specific test health problems the ironbench benchmark revealed (silent skips, low coverage, tautological assertions, monolith files, error handling smells)
- Make scrutiny checks deterministic where possible (grep, line count, coverage tool) and reserve LLM judgment for semantic analysis
- Keep the scrutiny validator as a single prompt file -- no external scripts or dependencies beyond existing coverage tools
- All checks self-gate based on diff contents so closing tasks skip naturally

**Non-Goals:**
- Mutation testing (deferred, too slow per-task)
- Multi-language coverage in mixed-language projects (primary language only)
- Benchmarking the impact (separate mission)
- Design-time concern separation in design.md (#88, separate issue)

## Modules

### Module 1: Scrutiny validator prompt
- **Interface:** Unchanged. The orchestrator spawns a Claude subagent with the prompt, filling in commit SHA and contract assertions. The subagent writes a report to `mission/validations/task-N-scrutiny.md`.
- **What changes internally:** The check sequence grows from 11 to 14. Three existing checks (3, 6, 7) gain deterministic pre-passes. Three new checks (12, 13, 14) are appended. The output table gains 3 new rows.
- **What it hides:** Language detection logic, grep patterns, coverage tool invocation, threshold evaluation, output parsing formats -- all internal to the prompt instructions.
- **Why this boundary:** The scrutiny validator is already the single place that defines validation behavior. Keeping everything in one prompt means one file to read, one place to update, and no interface changes for the orchestrator.

#### Internal concern separation

The prompt is getting longer (from ~108 to ~250 lines). To keep it navigable:
- Existing checks 1-11 stay in their current section
- A new `## Deterministic Pre-passes` section goes before the check sequence, defining the language detection, grep patterns, and coverage commands. The check sequence references these results.
- A new `## New Checks (12-14)` section follows check 11, keeping the numbering sequential.
- Each deterministic pre-pass is a self-contained block: "Run this command. Parse this output. Inject these facts into check N."

This avoids interleaving new instructions into existing check descriptions, which would make the prompt harder to maintain.

### Module 2: Test quality review skill
- **Interface:** Invoked by the scrutiny validator via the Agent tool, same as `/review` and `/simplify`. Receives the full diff as context. Writes markdown findings to stdout or a specified path.
- **What it hides:** The five evaluation dimensions (assertion quality, implementation coupling, branch coverage, behavioral changes, edge cases), language-specific tautological patterns, severity classification logic.
- **Why this boundary:** Separating this into its own skill keeps the scrutiny prompt from becoming a monolith (ironic given what we're building). The skill can be invoked independently outside missions. It follows the established pattern of `/review` and `/simplify`.

#### Skill file structure
```
test-quality-review/
  SKILL.md           -- main skill prompt with the 5 dimensions
```

No supplementary reference files needed. The language-specific patterns are short enough to inline in SKILL.md.

### Module 3: darkfactory-init skill
- **Interface:** Unchanged. User runs `/darkfactory-init`, gets a status report.
- **What changes internally:** Step 4 (CLI dependencies) gains a conditional section that detects the project language from build files and checks for the corresponding coverage tool. The status report in Step 5 gains a "Coverage tools" section.
- **What it hides:** Language detection logic, tool version check commands, install suggestions.
- **Why this boundary:** Init is already the place that checks dependencies. Adding coverage tools is a natural extension, not a new module.

### Module 4: Documentation updates
- **UBIQUITOUS_LANGUAGE.md:** Already updated with new terms (Phase 2).
- **README.md:** Update the scrutiny validator description to mention the 6 new capabilities, the new skill, and the init coverage check.

## Decisions

### Decision 1: Deterministic pre-passes injected into LLM checks (not separate checks)

File size and error handling smell density are computed deterministically (line count, grep) but evaluated by the LLM (module depth review, code standards review). Rather than adding them as separate pass/fail checks, we inject the deterministic data into the existing LLM checks.

**Why:** The LLM needs context to decide if a 950-line file is justified or if 6 unwraps/KLOC are acceptable. A separate deterministic check would either be too strict (false positives) or too lenient (missing context). Injecting the data lets the LLM make the judgment with hard numbers it can't overlook.

**Alternative considered:** Separate deterministic checks with their own thresholds. Rejected because it duplicates the judgment that module depth (check 6) and code standards (check 7) already make, and the irontodo benchmark failure was that the LLM missed the numbers, not that it made bad judgments when it saw them.

### Decision 2: Coverage runs for primary language only in multi-language projects

**Why:** Running two coverage tools per task doubles the time cost (2-6 minutes instead of 1-3). In practice, most tasks in mixed-language projects touch one language. The primary language is determined by which language has the most files in the diff.

**Alternative considered:** Per-file coverage using both tools. Rejected for now due to time cost. Can be upgraded later if multi-language coverage gaps appear in benchmarks.

### Decision 3: 70% hard fail for coverage (not 50%, not 80%)

**Why:** Aligns with Chromium's per-CL gate. Workers use `/tdd` which naturally produces high coverage on new code. 70% is generous enough to allow a few untested error paths but strict enough to catch tasks where large sections are completely untested.

**Alternative considered:** 50% (too lenient -- half the new code untested). 80% (too strict -- would fail tasks with a few hard-to-cover error paths and overwhelm the fix loop).

### Decision 4: New skill in layercodedev/skills repo (not inline in scrutiny prompt)

**Why:** The test quality review is a 5-dimension LLM evaluation. Inlining it would push the scrutiny prompt past 300 lines. The existing pattern (`/review`, `/simplify`, `/security-review`) delegates specialized LLM work to separate skills. The skill can also be used independently outside missions.

**Alternative considered:** Inline the evaluation dimensions directly in the scrutiny prompt. Rejected because it would make the prompt a monolith -- exactly the anti-pattern we're trying to detect.

### Decision 5: Prompt structure uses deterministic pre-pass section

**Why:** The scrutiny prompt will grow from ~108 to ~250 lines. Without structure, the LLM might skip or confuse checks. A dedicated "Deterministic Pre-passes" section runs all shell commands upfront, stores results in named variables, and the check sequence references those results. This mirrors how a developer would organize a script: gather data first, evaluate second.

**Alternative considered:** Interleave deterministic commands into each check's description. Rejected because it mixes fast/cheap commands with slow/expensive LLM evaluation, making the prompt harder to maintain and the execution order less predictable.

## Risks / Trade-offs

**[Coverage tool installation time]** `cargo install cargo-tarpaulin` compiles from source and takes 2-3 minutes on first install. → Mitigated by darkfactory-init checking upfront. The skip-with-warning fallback means missions don't fail if the tool is missing.

**[Prompt length growth]** The scrutiny prompt roughly doubles in size. Longer prompts risk the LLM losing focus on later checks. → Mitigated by the pre-pass/check separation structure, and by delegating the most complex check (test quality review) to a separate skill/subagent.

**[False positives in silent skip detection]** The grep patterns might match legitimate code patterns that look like silent skips but aren't (e.g., a Rust test that returns early after a conditional assertion, not to skip the test). → Mitigated by keeping the pattern set narrow and specific. The fix worker can address false positives by refactoring the flagged test to be clearer.

**[Coverage tool incompatibility]** Some projects may have custom test setups that don't work with standard coverage tools (e.g., custom test harnesses, workspace configurations). → Mitigated by the skip-with-warning fallback. If the coverage tool fails, the check skips rather than blocking the mission.

**[Two-repo coordination]** The scrutiny prompt references `/test-quality-review` which lives in a different repo. If the skill isn't installed, check 14 fails. → Mitigated by darkfactory-init adding `/test-quality-review` to its supporting skills check, and by the check self-gating (SKIP when the skill isn't available, with a warning).

## Open Questions

None. All decisions were resolved during the grill session (Phase 1).
