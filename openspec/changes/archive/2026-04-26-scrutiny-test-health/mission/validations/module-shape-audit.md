# Module Shape Audit

## scrutiny-validator.md
- **Design said:** Deterministic Pre-passes section before check sequence, new checks 12-14 after check 11, concern separation between data gathering and evaluation.
- **Actual:** Pre-passes at lines 9-107, existing checks 1-11 at lines 109-194, new checks 12-14 at lines 196-256, output format at lines 258-301. Structure matches design exactly.
- **Drift:** None. The prompt is 301 lines (design predicted ~250). The extra 51 lines come from the detailed per-language coverage tool instructions in check 13, which are more thorough than anticipated but well-structured.

## test-quality-review skill
- **Design said:** SKILL.md with 5 evaluation dimensions, simple markdown output, follows existing skill format.
- **Actual:** SKILL.md with frontmatter (name, description), 5 numbered dimensions with per-language patterns, structured markdown output with P0-P3 severity, self-gating, explicit "what this skill does NOT do" section.
- **Drift:** None. The skill added a useful "What This Skill Does NOT Do" section not in the design -- this is a narrowing of scope (making boundaries clearer), not a widening.

## darkfactory-init
- **Design said:** Step 4 gains conditional coverage tool checks, Step 5 gains Coverage tools section.
- **Actual:** Step 2 adds /test-quality-review to supporting skills check. Step 4 adds language detection and per-language coverage tool check. Step 5 adds Coverage tools section with pass/fail/skip display.
- **Drift:** None. All changes match design.
