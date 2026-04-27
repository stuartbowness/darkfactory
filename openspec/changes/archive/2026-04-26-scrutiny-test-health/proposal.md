## Why

Darkfactory missions produce code that passes the scrutiny validator but scores lower on test quality than human-iterated code. The ironbench benchmark showed the Darkfactory-produced irontodo-v2 scored -2 on test quality (49.5% of tests either panicked or silently pretended to pass), -1 on architecture (a 1,940-line monolith grew undetected), -1 on robustness (15.0 unwrap/KLOC), and -1 on maintainability. The scrutiny validator's test presence gate only checks "were tests added?" -- it doesn't catch silent skips, low coverage, meaningless assertions, oversized files, or sloppy error handling. No industry consensus exists on measuring AI-generated test quality (confirmed via primary source research), so we implement practical checks grounded in the specific benchmark gaps.

## What Changes

- **Enhance scrutiny check 3 (tests)** with test runner output parsing -- extract passed/failed/skipped counts, group failures by cause, WARNING when <90% of tests actually ran.
- **Enhance scrutiny check 6 (module depth)** with a file size pre-pass -- deterministic line count on touched files, injected into the LLM review when any source file exceeds 900 lines, cross-referenced with design.md concern boundaries.
- **Enhance scrutiny check 7 (code standards)** with error handling smell density -- deterministic grep for language-specific anti-patterns on changed production lines, graduated threshold (under 5/KLOC PASS, 5-10 WARNING, over 10 FAIL).
- **Add scrutiny check 12: silent skip detection** -- grep test files for patterns that report PASS while testing nothing. Any silent skip is a FAIL.
- **Add scrutiny check 13: diff coverage** -- run language-appropriate coverage tool on added lines in the task's diff. 80%+ PASS, 70-79% WARNING, below 70% FAIL.
- **Add scrutiny check 14: test quality review** -- invoke new `/test-quality-review` skill as subagent to evaluate assertion quality, implementation coupling, untested branches, behavioral changes without tests, and edge case awareness.
- **New `/test-quality-review` skill** in layercodedev/skills repo.
- **Update `darkfactory-init`** to check for coverage tool availability per detected project language.

## Capabilities

### New Capabilities
- `silent-skip-detection`: Deterministic detection of tests that report PASS while testing nothing, across Rust, TypeScript, and Python.
- `diff-coverage`: Per-task line coverage measurement on added lines using language-appropriate coverage tools.
- `test-quality-review`: LLM-based semantic review of test effectiveness, evaluating assertion quality, implementation coupling, branch coverage, and edge case awareness.

### Modified Capabilities
- `scrutiny-validation`: Enhance existing checks 3 (test runner output parsing), 6 (file size pre-pass), and 7 (error handling smell density). Add new checks 12, 13, 14. All checks self-gate based on diff contents.
- `darkfactory-init`: Add coverage tool dependency checks conditional on detected project language.

## Impact

- **Scrutiny validator prompt** (`prompts/scrutiny-validator.md`): primary file modified. All six capabilities are implemented as instructions within this prompt.
- **darkfactory-init skill** (`skills/darkfactory-init/SKILL.md`): add step for coverage tool verification.
- **layercodedev/skills repo**: new `test-quality-review/` skill directory with SKILL.md.
- **UBIQUITOUS_LANGUAGE.md**: already updated with new terms (silent skip, diff coverage, error handling smell, file size pre-pass, test quality review, tautological assertion, self-gating check).
- **README.md**: update to reflect new scrutiny checks and new skill.
- **No schema changes, no API changes, no new runtime dependencies** within Darkfactory itself. Coverage tools are external dependencies verified by darkfactory-init.
