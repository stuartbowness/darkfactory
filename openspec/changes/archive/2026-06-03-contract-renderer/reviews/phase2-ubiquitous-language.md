# Review: UBIQUITOUS_LANGUAGE.md (Darkfactory 2 additions)

Phase: 2
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 7 (2 HIGH, 4 MEDIUM, 1 LOW) — all applied at source

Codex (gpt-5.5) reviewed the new "Requirement-as-atom planning (Darkfactory 2)" section, the new relationships, the example dialogue, and the four resolved ambiguities. All seven findings were applied at source before human approval:

- [HIGH] Task-link ownership was duplicated on both the requirement record and each acceptance entry → fixed so each acceptance owns its `task` link and the record-level link is the computed union (prevents reintroducing drift inside the single source).
- [HIGH] Cutover-gate definition was thinner than EPIC #199 → added the explicit mission-shape set and the "real missed/weak requirement caught" criterion.
- [MEDIUM] Reused the existing v1 term **Spec Reference** instead of inventing "spec-scenario id"; removed the transient "Option B" label from the durable glossary.
- [MEDIUM] Added a **Verification method** term ([execution]/[output]/[inspection] + 20% inspection cap), previously referenced but undefined.
- [MEDIUM] Made shadow mode explicitly diagnostic: text diffs are for inspection, parity is judged by validator outcomes.
- [MEDIUM] Added `temporary/permanent` to the deviation sub-field.
- [LOW] Added `check`, `contract check` to the acceptance avoid-list.

Human approved the glossary after fixes.
