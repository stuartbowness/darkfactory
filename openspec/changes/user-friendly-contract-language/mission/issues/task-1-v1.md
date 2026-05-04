# Issue: Task 1 - Rename scrutiny validator steps (Attempt 1)

## Failed Checks
- Check 2: The scrutiny validator prompt report template labels each step as "Gate N" (e.g., "Gate 1: Test presence", "Gate 10: Contract checks")

## Observed Symptoms
The report template table in scrutiny-validator.md (lines 273-286) uses bare descriptive names without gate numbering:
```
| Test presence | PASS/FAIL | ... |
| Build | PASS/FAIL | ... |
```
Expected:
```
| Gate 1: Test presence | PASS/FAIL | ... |
| Gate 2: Build | PASS/FAIL | ... |
```

## Validator Recommendations
Update the report template table rows to prefix each with its gate number (Gate 1 through Gate 14).
