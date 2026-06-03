## ADDED Requirements
### Requirement: README reflects the domain-vocabulary changes
The README SHALL reflect the Domain Types section in worker briefs, the domain type boundary clarification in the design template, and the Type Catalog name verification in the module shape re-check.

This capability has no source in the archived change's `specs/`: archived Task 3 ("Update README to reflect changes") carried feature-task checks with no spec scenario behind them. Design decision D6 requires every feature-task check to trace to a scenario, so this scenario is authored during migration to host Task 3's three README checks. The check statements are reproduced verbatim from the archived contract.

#### Scenario: README documents the new domain-vocabulary surfaces
<!-- spec-ref: 1 -->
- **WHEN** a reader opens README.md after the domain-vocabulary change ships
- **THEN** the README documents the Domain Types section in worker briefs, the domain type boundary clarification in the design template, and the Type Catalog name verification in the module shape re-check

**Acceptance:**
- [task:3] [output] `README.md` mentions the Domain Types section in worker briefs
- [task:3] [output] `README.md` mentions the domain type boundary clarification in the design template
- [task:3] [output] `README.md` mentions the Type Catalog name verification in the module shape re-check
