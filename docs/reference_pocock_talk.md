---
name: pocock-skills-talk-reference
description: Reference material from Matt Pocock's "Code Skills" talk -- design alignment, ubiquitous language, TDD, deep modules, and strategic AI development patterns
type: reference
originSessionId: d0be7010-3858-4b92-8153-32045af7ab9b
---
Source: Matt Pocock talk at AI Engineer Europe (April 2026). Skills repo: github.com/mattpocock/skills

## Core Ideas

**The communication barrier:** Developer and AI don't share a "design concept" (Frederick Brooks' term from The Design of Design). The invisible shared theory of what you're building needs to be made explicit before coding starts.

**Six failure modes and skills to address them:**

1. **Misalignment** -- AI doesn't understand what to build
   - Skill: `/grill-me` -- adversarial interview (40-100 questions) to reach shared understanding before creating any assets. Better than plan mode which rushes to create.

2. **Talking across purposes** -- AI too verbose, wrong terminology
   - Skill: `/ubiquitous-language` -- DDD-inspired glossary scan. Creates UBIQUITOUS_LANGUAGE.md with canonical terms, aliases to avoid, relationships. Improves AI thinking traces and implementation alignment.

3. **AI outruns its headlights** -- builds too much before testing
   - Skill: TDD -- forces small steps (write test, make pass, refactor). Rate of feedback = speed limit.

4. **Bad code architecture** -- hard to test, hard for AI to explore
   - Skill: Improve codebase architecture -- wrap related code in deep modules (John Ousterhout). Large functionality behind simple interfaces. Test at the interface level.

5. **Brain overload** -- developer can't keep up with AI output
   - Solution: Treat deep modules as "gray boxes" -- design the interface, delegate implementation to AI. Only review internals for critical paths (finance, auth).

6. **Strategic vs tactical** -- AI is a tactical programmer, you are the strategic one
   - Skill: `/prd-to-plan` -- include module changes and interface designs in PRDs. Think about system design every day (Kent Beck).

## Key Principle

"Code is not cheap." AI handles tactical programming. Human handles strategic design -- module boundaries, interfaces, system architecture. Master software fundamentals to guide AI effectively.
