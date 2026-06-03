## ADDED Requirements
### Requirement: Module shape re-check verifies Type Catalog names
The module shape re-check mandatory closing task SHALL verify that every domain type in the Type Catalog exists in the committed code with a matching name.

#### Scenario: All Type Catalog names present
<!-- spec-ref: 1 -->
- **WHEN** the module shape re-check reads design.md's Type Catalog and compares to the codebase
- **THEN** each domain type name from the Type Catalog has a corresponding type definition in the code with the same name, and the audit report notes "no type name drift"

**Acceptance:**
- [task:2] [output] `skills/mission/SKILL.md` module shape re-check closing task instructions mention verifying that every domain type in the Type Catalog exists in the committed code with a matching name

#### Scenario: Type name drift detected
<!-- spec-ref: 2 -->
- **WHEN** the module shape re-check finds a Type Catalog entry whose name does not match any type definition in the code
- **THEN** the audit report flags the drift with the expected name and the actual name found (or "missing"), and the worker either updates design.md to match the code (deliberate change) or refactors the code to match design.md (unintended drift)

**Acceptance:**
- [task:2] [output] The SKILL.md instruction specifies that drift is resolved by either updating design.md or refactoring the code, matching the existing module surface drift resolution pattern
