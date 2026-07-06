---
name: language
user-invokable: true
description: Extract a DDD-style ubiquitous language glossary from the current conversation, flagging ambiguities and proposing canonical terms. Saves to UBIQUITOUS_LANGUAGE.md. Use when user wants to define domain terms, build a glossary, harden terminology, create a ubiquitous language, or mentions "domain model" or "DDD".
---

# Ubiquitous Language

Extract and formalize domain terminology from the current conversation into a consistent glossary, saved to a local file.

## Process

1. **Scan the conversation** for domain-relevant nouns, verbs, and concepts
2. **Identify problems**:
   - Same word used for different concepts (ambiguity)
   - Different words used for the same concept (synonyms)
   - Vague or overloaded terms
3. **Propose a canonical glossary** with opinionated term choices
4. **Write to `UBIQUITOUS_LANGUAGE.md`** in the working directory using the format below
5. **Output a summary** inline in the conversation

## Output Format

Write a `UBIQUITOUS_LANGUAGE.md` file with this structure:

```md
# Ubiquitous Language

> Scope: ambiguity-prone domain terms only. Mechanics and how-it-works detail belong in the architecture doc; skill-specific vocabulary belongs in the owning skill. Reference those, don't duplicate them here.

## Order lifecycle

- **Order** -- A customer's request to purchase one or more items. *Avoid:* purchase, transaction
- **Invoice** -- A request for payment sent to a customer after delivery. *Avoid:* bill, payment request

## People

- **Customer** -- A person or organization that places orders. *Avoid:* client, buyer, account
- **User** -- An authentication identity in the system. *Avoid:* login, account

## Relationships

- An **Invoice** belongs to exactly one **Customer**
- An **Order** produces one or more **Invoices**

## Example dialogue

> **Dev:** "When a **Customer** places an **Order**, do we create the **Invoice** immediately?"
> **Domain expert:** "No — an **Invoice** is only generated once a **Fulfillment** is confirmed. A single **Order** can produce multiple **Invoices** if items ship in separate **Shipments**."
> **Dev:** "So if a **Shipment** is cancelled before dispatch, no **Invoice** exists for it?"
> **Domain expert:** "Exactly. The **Invoice** lifecycle is tied to the **Fulfillment**, not the **Order**."

## Flagged ambiguities

- "account" was used to mean both **Customer** and **User** — these are distinct concepts: a **Customer** places orders, while a **User** is an authentication identity that may or may not represent a **Customer**.
```

## Rules

- **Apply the inclusion bar.** A term earns an entry only if all four hold: (1) it names a domain concept, not a mechanism or process; (2) it is ambiguity-prone -- the same word is used for two things, or two words for one, so there is an alias worth flagging; (3) you can define it in one sentence as what it IS; (4) it is shared across more than one role or file. If it fails any of the four, leave it out.
- **Exclude mechanics and tooling.** Never add implementation mechanics (how a step works), tool internals, file or config names, gate-by-gate behavior, or coding rules. These belong in the architecture document (e.g. `ARCH.md`) or the owning skill -- point to them, do not define them here. A glossary that documents how the system works will go stale; keep it to vocabulary.
- **Be opinionated.** When multiple words exist for the same concept, pick the best one and list the others as aliases to avoid.
- **Flag conflicts explicitly.** If a term is used ambiguously in the conversation, call it out in the "Flagged ambiguities" section with a clear recommendation.
- **Only include terms relevant for domain experts.** Skip the names of modules or classes unless they have meaning in the domain language.
- **Keep definitions tight.** One sentence max. Define what it IS, not what it does.
- **One bullet per term.** Format each entry as `- **Term** -- definition. *Avoid:* alias1, alias2`. Aliases are lowercase except proper nouns and acronyms (QA, PRD, TypeScript). Drop the `*Avoid:*` clause when a term has no aliases worth flagging. Never use tables -- definitions hold prose and code (including `|`), which tables mangle.
- **Show relationships.** Use bold term names and express cardinality where obvious.
- **Only include domain terms.** Skip generic programming concepts (array, function, endpoint) unless they have domain-specific meaning.
- **Group terms into multiple sections** when natural clusters emerge (e.g. by subdomain, lifecycle, or actor). Each group gets its own heading and bullet list. If all terms belong to a single cohesive domain, one section is fine -- don't force groupings.
- **Write an example dialogue.** A short conversation (3-5 exchanges) between a dev and a domain expert that demonstrates how the terms interact naturally. The dialogue should clarify boundaries between related concepts and show terms being used precisely.

<example>

## Example dialogue

> **Dev:** "How do I test the **sync service** without Docker?"

> **Domain expert:** "Provide the **filesystem layer** instead of the **Docker layer**. It implements the same **Sandbox service** interface but uses a local directory as the **sandbox**."

> **Dev:** "So **sync-in** still creates a **bundle** and unpacks it?"

> **Domain expert:** "Exactly. The **sync service** doesn't know which layer it's talking to. It calls `exec` and `copyIn` — the **filesystem layer** just runs those as local shell commands."

</example>

## Re-running

When invoked again in the same conversation:

1. Read the existing `UBIQUITOUS_LANGUAGE.md`
2. Re-apply the inclusion bar to the whole file, not just the additions: add new terms that pass it, and **prune** any that have drifted into mechanics or moved into a skill. Re-running is as much about removing drift as adding terms -- the file should not grow monotonically.
3. Update definitions if understanding has evolved
4. Re-flag any new ambiguities
5. Rewrite the example dialogue to incorporate new terms
