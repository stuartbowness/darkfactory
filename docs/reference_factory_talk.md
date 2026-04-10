---
name: factory-missions-talk-reference
description: Reference material from Factory's "Assembling Agent Teams" talk at AI Engineer Europe -- core architecture details for the Missions multi-agent system
type: reference
originSessionId: d0be7010-3858-4b92-8153-32045af7ab9b
---
Source: Talk by Factory's core agent harness lead at AI Engineer Europe (April 2026). Slide images in `/Users/stuart/Downloads/talkimages/` (13 JPEGs). Full transcript and summary provided by Stuart in the initial conversation.

## Key Architecture Details

**Five multi-agent strategies (taxonomy):**
1. Delegation -- parent spawns sub-agents
2. Creator-Verifier -- separate agents for building vs checking
3. Direct Communication -- agents talk without coordinator (fragile)
4. Negotiation -- agents coordinate over shared resources
5. Broadcast -- one agent sends info to many

**Missions combine 4 of 5** (delegation, creator-verifier, broadcast, negotiation) into a single workflow.

**Three-role architecture:**
- **Orchestrators** -- plan features/milestones, ask clarifying questions, produce a "validation contract" (defines done before coding starts, can be hundreds of assertions)
- **Workers** -- implement one feature at a time with clean context, commit to git, structured handoff report (what was done, what's undone, commands/exit codes, issues, procedure compliance)
- **Validators** -- two types:
  - *Scrutiny Validator* -- test suite, type checking, lint, dedicated code review agents per feature
  - *User Testing Validator* -- QA engine that spawns the app, interacts via computer use, fills forms, clicks buttons, checks flows end-to-end

**Execution model:**
- Serial feature execution (one worker at a time) to avoid conflicts
- Parallelization only for read-only ops (code search, API research) and within validators
- Correctness compounds over multi-day runs

**Model strategy:**
- Model-agnostic: different LLMs for planning (slow reasoning), implementation (fast code fluency), validation (precise instruction following)
- Structure compensates for non-frontier models
- Almost all orchestration logic is in ~700 lines of prompts/skills, not hard-coded

**Longest mission:** 16 days. Results: ~50% test coverage lines, ~90% code coverage. Most time spent on user-testing validation, not code generation.

**Designed not to be obsolete:** when a better model drops, the system just gets smarter with no code changes needed.
