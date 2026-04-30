# Task 4 Scrutiny Validation

**Commit:** 0195f82d883c2d7f09f8354d88944425b60a98ac
**Title:** [task-4.1] Add Phase 4.3a, task topology, and strategy-aware quality standard
**Files changed:** `skills/mission/SKILL.md`, `openspec/config.yaml`
**Verdict:** PASS (12/12)

## Assertion Results

- [x] **[output] SKILL.md contains a Phase 4.3a section between Phase 4.3 and Phase 4.4**
  Phase 4.3 ends at line ~200 with "If approved, proceed to Phase 4.3a." Phase 4.3a header is at line 202. Phase 4.4 header is at line 216. Correct ordering confirmed.

- [x] **[output] Phase 4.3a requires the orchestrator to present: decomposition approach (layered/vertical/hybrid), target task count range, and dependency rationale**
  All three are present as numbered items: (1) Decomposition approach with layered/vertical/hybrid options, (2) Target task count range with example, (3) Dependency ordering rationale with example.

- [x] **[output] Phase 4.3a requires human approval before tasks.md is generated**
  Line 212: "The human approves this strategy before tasks.md is generated. If rejected, revise and re-present."

- [x] **[output] Phase 4.3a includes an instruction to populate the task decomposition strategy anchor after approval**
  Line 214: "After approval, populate the task decomposition strategy anchor in the plan's Deterministic Anchors section."

- [x] **[output] SKILL.md Phase 4.4 contains a Task Topology subsection requiring four fields per task: Depends on, Consumes, Produces, Layers**
  Lines 230-240: "#### Task Topology" subsection with fixed markdown format containing all four fields: `Depends on`, `Consumes`, `Produces`, `Layers`.

- [x] **[output] Phase 4.4 includes the fixed markdown format for topology metadata matching design D5**
  The fenced code block (lines 233-238) shows the exact format:
  ```
  - [ ] Task N: <name>
    Depends on: <prior task numbers, or "none">
    Consumes: <surfaces from prior tasks -- types, interfaces, endpoints>
    Produces: <surfaces this task creates -- types, modules, interfaces, endpoints>
    Layers: <logical architectural layers touched, e.g. domain, transport, storage>
  ```

- [x] **[output] Phase 4.4 includes the >2 layers flag that applies only when the approved strategy is layered or hybrid (vertical strategy is exempt)**
  Line 242: "When the approved strategy (Phase 4.3a) is layered or hybrid and a task's Layers field lists more than 2 architectural layers, flag the task for human review... When the approved strategy is vertical, the layer count flag does not apply since vertical tasks are expected to cross all layers."

- [x] **[output] Phase 4.4 task quality standard says "each task must produce a concrete, testable output" rather than unconditionally requiring vertical slices**
  Line 222: "Each task must produce a concrete, testable output." The old wording ("Each task must be a vertical slice") is removed.

- [x] **[output] Phase 4.4 task quality standard references Phase 4.3a for the approved strategy and permits layered tasks when the strategy is layered or hybrid**
  Line 222: "When the approved strategy (Phase 4.3a) is vertical, tasks cross all layers. When the strategy is layered or hybrid, tasks may be scoped to a single layer, provided each task still has a clear done state and a test surface."

- [x] **[output] `openspec/config.yaml` `rules.tasks` uses strategy-aware wording instead of unconditionally requiring vertical slices**
  config.yaml line: "Each task must produce a concrete, testable output. When the approved strategy is vertical, tasks cross all layers. When layered or hybrid, tasks may be scoped to a single layer with a clear done state and test surface." Replaces the old "Each task should be a vertical slice" rule.

- [x] **[output] SKILL.md Phase Index includes 4.3a**
  Line 29 of SKILL.md: `4.3a. **Task Strategy Approval** -- approve decomposition approach before generating tasks`. Positioned correctly after line 28 (Phase 4 entry) and before line 30 (Phase 4a entry).

- [x] **[execution] `grep -c "4.3a" skills/mission/SKILL.md` returns at least 2**
  Result: 6 (Phase Index entry, Phase 4.3 anchor reference, proceed-to link, section header, and two references in Phase 4.4).

- [x] **[execution] `grep -c "Depends on:" skills/mission/SKILL.md` returns at least 1**
  Result: 1 (in the Task Topology fixed format block).

## Phase Positioning Verification

The Phase Index order is: 4 -> 4.3a -> 4a -> 5 -> 6 -> 7 -> 8 -> 9. Phase 4.3a is correctly positioned as a sub-phase of Phase 4, after the main Phase 4 entry and before Phase 4a.

In the document body: Phase 4.3 (line 166) -> Phase 4.3a (line 202) -> Phase 4.4 (line 216). The flow links are correct: Phase 4.3 says "proceed to Phase 4.3a", Phase 4.3a says "proceed to Phase 4.4".

## Checks Run

- Read full diff via `git show`
- Read Phase Index and phase headers with line numbers
- Read Phase 4.3a body text and Phase 4.4 body text
- Read config.yaml rules.tasks
- Ran `grep -c "4.3a"` (result: 6, passes >= 2)
- Ran `grep -c "Depends on:"` (result: 1, passes >= 1)
- Verified phase ordering in both index and body

## Checks Skipped

- Build, test, lint (markdown-only change, per instructions)
