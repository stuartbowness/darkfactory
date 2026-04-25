# Task 3.1 User-Test Validation

**Commit:** b596e77d1722732a4397a3241dd2c1bb9ac4c371
**Validated:** 2026-04-23

## Assertions

- [x] `README.md` contains "Bundled with Darkfactory" in the `/opsx:archive` row
- [x] `README.md` does NOT contain "Installed by `openspec init`" in the `/opsx:archive` row

## Evidence

The commit changed one line in README.md:

```diff
-| `/opsx:archive` | Installed by `openspec init` | Post-merge: moves the change's planning files into the OpenSpec archive directory |
+| `/opsx:archive` | Bundled with Darkfactory | Post-merge: moves the change's planning files into the OpenSpec archive directory |
```

**Assertion 1 (presence):** `grep 'opsx:archive' README.md | grep 'Bundled with Darkfactory'` matched the row. PASS.

**Assertion 2 (absence):** `grep 'opsx:archive' README.md | grep 'Installed by \`openspec init\`'` returned no output (exit code 1). PASS.

## Result

PASS -- both contract assertions satisfied.
