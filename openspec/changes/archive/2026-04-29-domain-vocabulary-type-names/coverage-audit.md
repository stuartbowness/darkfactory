# Spec Coverage Audit

## Coverage Table
| # | PRD Requirement | Spec Requirement | Task(s) | Status |
|---|----------------|-----------------|---------|--------|
| 1 | Orchestrator brief has template section listing exact Type Catalog names | brief-domain-types/1: Brief for task that produces domain types | Task 1 | COVERED |
| 2 | Worker sees exact domain type names to use | brief-domain-types/1, brief-domain-types/2: Brief for task that consumes domain types | Task 1 | COVERED |
| 3 | Module shape re-check verifies Type Catalog names | type-name-audit/1, type-name-audit/2 | Task 2 | COVERED |
| 4 | Clear definition of domain type vs helper type | domain-type-boundary/1 | Task 2 | COVERED |
| 5 | Type Catalog definition states glossary relationship | domain-type-boundary/2 | Phase 2 (pre-build) | COVERED |
| 6 | Relationships section documents full naming chain | domain-type-boundary/3 | Phase 2 (pre-build) | COVERED |
| 7 | Type-name assertions do not belong in contracts | no spec (grill decision: leave contract unchanged) | -- | APPROVED-DEVIATION |

## Gaps
None.

## Deviations Requiring Approval
| # | PRD Requirement | Classification | What spec says | What we're doing instead | Prerequisites met? | Impact | Human Decision |
|---|----------------|---------------|----------------|-------------------------|-------------------|--------|----------------|
| 7 | Type-name assertions do not belong in contracts | Interpretation | No spec covers this requirement | The contract template and Phase 5 are already behavioral by design. No explicit prohibition is added. The grill decision was that "don't do X" instructions are noise when the positive instructions are clear. | -- | Future orchestrators could theoretically add type-name assertions to contracts, but the behavioral framing makes this unlikely. | Approved (grill session) |
