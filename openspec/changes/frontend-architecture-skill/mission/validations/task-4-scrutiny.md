# Scrutiny Validation: Task 4

Commit: cb16105
Files: skills/frontend/PATTERNS.md, skills/frontend/TESTING.md

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | PATTERNS.md exists with >= 2 deep component code blocks | PASS | Block 1 (lines 13-18): FilesDrawerProps interface. Block 2 (lines 29-33): BrowserPane useMemo derive-don't-sync. Both under "Deep Component Examples" heading. |
| 2 | PATTERNS.md has >= 2 allowed-shallow code blocks | PASS | Block 3 (lines 46-62): Button design system leaf. Block 4 (lines 73-93): Card + CardHeader compound component. Both under "Allowed-Shallow Examples" heading. |
| 3 | PATTERNS.md has >= 1 hook with derive-don't-sync | PASS | Block 5 (lines 105-126): useChatScroll with isAtBottomPosition pure function, under "Hook Pattern - Derive Don't Sync" heading. |
| 4 | PATTERNS.md has >= 1 anti-pattern with before/after or don't/do structure | PASS | Block 6 (lines 134-144) labeled "BAD: syncing a pure derivation via useEffect". Block 7 (lines 151-154) labeled "GOOD: pure function called on demand". Clear don't-do-this / do-this structure. |
| 5 | TESTING.md exists and contains "role" or "label" | PASS | Line 17: `getByRole("button", ...)`. Line 18: `getByLabelText(/file name/i)`. |
| 6 | TESTING.md contains "userEvent" or "user event" or "fire events" | PASS | Line 25: "Use `userEvent` for real interaction." Line 28: `userEvent.setup()`. |
| 7 | TESTING.md contains "prop plumbing" or "internal state" or "implementation" | PASS | Line 44: "Do not test prop plumbing." Line 48: "Do not test internal state names or values." Line 49: "State is an implementation detail." |
| 8 | TESTING.md contains "snapshot" | PASS | Line 52: "Do not use snapshot tests for layout." |
| 9 | TESTING.md contains "/tdd" | PASS | Line 56: "Complex hook logic belongs in separate `/tdd` tasks" |

## Overall Result

PASS

All 9 contract checks satisfied. Both files exist, contain the required code blocks in the correct categories, and TESTING.md covers all required topics.
