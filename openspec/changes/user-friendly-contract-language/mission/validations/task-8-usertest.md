# Task 8 User-Test Validation

## Verdict: PASS

## Test

Read README.md end-to-end (307 lines) and verified terminology from a user's perspective:

1. **No "assertion" in contract context.** The word "assertion" does not appear anywhere in README.md. Contract items are consistently called "checks."

2. **"Check" vs "gate" distinction is clear.** A reader encountering the README would understand:
   - A "check" is something the contract says the code should do (e.g., "contract check" on line 103)
   - A "gate" is a validator step (e.g., "mandatory gate" on line 77, "not an automated gate" on line 119)

3. **Documentation files are consistent.** All docs under `docs/` (provenance.md, design-decisions.md, reference_factory_talk.md, reference_pocock_talk.md) contain zero instances of "assertion" in any context. The UBIQUITOUS_LANGUAGE.md resolved-ambiguities section (line 183) correctly documents the three-way split between gate/check/assertion.

4. **Commit [task-8] 3256b83 message says "Documentation: verified consistent"** -- this aligns with the observed state. The task-8 worker confirmed consistency rather than needing to make changes, which is a valid outcome given that task-3 (99037c9) already renamed assertions to checks across prompts, templates, and docs.
