# Darkfactory 2: Requirement-as-Atom Planning

**Status:** Proposal / north-star design. Not implemented. The v1 pipeline in `skills/mission/SKILL.md` still ships missions; this describes where the planning pipeline should go, and why.

## Motivation

Damien flagged that mission planning duplicates what is in the PRD into later phases. Walking the pipeline (grill -> PRD -> proposal -> specs -> design -> tasks -> coverage audit -> contract) confirmed it. The same information is hand-authored at several altitudes, and two documents both claim to be the "source of truth" (the PRD at one point, the specs at another).

The root cause: the pipeline conflates *documents* with *information*. Several documents are different views of the same underlying facts, but each is written by hand, so the facts drift. The coverage audit (Phase 4a) and the Phase 9 spec-delta check exist mainly to reconcile those drifting copies.

## First principles

Strip the pipeline down to the information it actually has to produce.

Three things the human must approve:

- **Intent** -- what and why
- **Design** -- the architecture and shape
- **Task plan** -- the ordered slices

One thing the machine needs:

- **Behavior** -- the precise, testable required behaviors

Two things that should be computed, not authored:

- **Coverage** -- did every requirement reach a task? (a join)
- **Contract** -- the per-task checks (a view of behavior, re-sliced by task)

In v1, all of these are separate hand-written documents. Intent is authored three times (grill notes, PRD, proposal). Behavior is authored twice (specs, then re-sliced into the contract). Coverage and contract are authored even though both are derivable.

## The model: the requirement is the atom

Make the requirement the single unit of truth. Everything else is a projection of the requirement set.

Each requirement is one record:

- `id` -- stable identifier
- `behavior` -- what the builder is told (the WHAT, no test specifics)
- `acceptance` -- the observable check that proves it (the validator's view)
- `task` -- the task(s) that satisfy it
- `status` -- proposed / approved / built / verified / deviated

From that set the system projects:

- **Worker brief** -- the `behavior` field, filtered to the task being built.
- **Contract (validator input)** -- the `acceptance` field, grouped by task. Generated, not authored.
- **Coverage** -- any requirement with no `task` link. Computed.
- **Spec-delta check** -- any requirement whose `acceptance` no longer matches the committed code.

This preserves the one property that justified separate spec and contract documents in v1: the worker still never sees the checks. We are not merging the builder and validator views. We are merging the *source* and projecting two views from it. That distinction is what v1 misses.

### v1 vs v2

| Need | v1 (authored) | v2 (source + projection) |
|---|---|---|
| Intent | grill notes + PRD + proposal | one intent doc |
| Behavior | specs | `requirement.behavior` |
| Verification | `contract.md` (authored) | `requirement.acceptance`, projected per task |
| Coverage | `coverage-audit.md` (authored table) | computed gap report |
| Architecture | `design.md` | `design.md` (unchanged) |
| Slices | `tasks.md` | `tasks.md`, referencing requirement IDs |

What stays: `design.md` (the "how," genuinely distinct) and `tasks.md` (the slices, now pointing at requirement IDs). The grill stays too -- it is the process that produces intent, not an artifact.

What collapses: proposal becomes a section of the intent doc; specs + contract become one requirement set with two projected views; the coverage audit becomes a computed gap report; the spec-delta check shrinks to an acceptance-vs-code diff.

## Generated does not mean trusted: the review model

Generation removes one class of error -- drift, where copies diverge. It does nothing for the other class -- judgment: a check too weak to falsify the behavior, an acceptance criterion that passes trivially, a task linked to a requirement it cannot satisfy. Codex catches those today, and they are orthogonal to whether an artifact was typed or projected.

So every artifact, generated or not, still passes Codex review before the human sees it. The loop per artifact:

```
generate -> Codex review -> auto-fix at source -> regenerate -> human review
```

The rule that makes generated-plus-reviewed coherent: **Codex's fixes land on the source, then the view regenerates. Never patch a generated file.** If Codex flags a contract check, the fix goes to that requirement's `acceptance` (or its `task` link, or the intent), and the contract re-projects. Patching the generated contract directly re-creates the drift the model just removed. This is cleaner than v1, where contract fixes are patched locally and the spec-delta check exists to reconcile them later. Route fixes to source and most of that reconciliation disappears.

A circuit breaker escalates to the human if auto-fix cannot satisfy Codex after a couple of rounds.

### What Codex uniquely catches

Beyond what a mechanical projection can see:

- **Contract** -- checks that do not actually falsify the behavior, vague "appropriate error" assertions, missing observability, a check that would pass even if the feature did not exist.
- **Coverage** -- the mechanical join only finds *missing* task links. Codex finds *wrong* ones: a requirement mapped to a task that cannot satisfy it, two conflicting requirements piled on one task, a "covered" item that needs work no task does.

## Non-goals and caveats

- **Review gates are a separate dial from document count.** Some v1 artifacts exist so the human can approve at that point, not because their content is unique. How many gates to keep is a product decision driven by how costly a wrong build is, decided independently of this consolidation.
- **This is a core-skill redesign with migration cost.** The v1 pipeline ships missions today. This is a direction, not a rip-and-replace.
- **OpenSpec is the substrate.** v1 already traces contract checks to spec scenarios by `capability/N`. The model leans on that: it makes the trace a first-class link and generates the contract from it, instead of hand-authoring both.

## Migration path

Cheapest first step that tests the whole thesis: stop hand-authoring `contract.md`; generate it from the spec scenarios it already references by `capability/N`. If drift drops and the spec-delta check goes quiet, the requirement-as-atom model is validated before committing to the larger change. From there:

1. Add an `acceptance` view to spec scenarios; generate the contract from it.
2. Make task -> requirement links first-class; derive coverage instead of authoring it.
3. Fold the proposal's narrative into a single intent doc; keep only the capability index as structure.
4. Move the Codex-review-then-auto-fix gate onto the generated artifacts, routing fixes to source.

## Open questions

- Storage shape for a requirement record in an OpenSpec/markdown world: structured scenarios per capability spec, vs a single requirements file.
- Whether the worker should ever see acceptance values (anti-gaming vs clarity).
- How v1's APPROVED-DEVIATION flow maps onto requirement `status`.
- Whether `design.md` should carry requirement links too (module -> requirement traceability).
