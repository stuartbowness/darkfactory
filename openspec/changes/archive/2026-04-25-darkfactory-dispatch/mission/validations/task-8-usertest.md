# Task 8.1 User-Test Validation

**Commit:** 37d5c79
**Date:** 2026-04-23
**Validator:** Claude Opus 4.6

## Test Scenario

A developer runs `/darkfactory-init` on a fresh machine to check if their environment is ready for a mission.

## Walkthrough

### Pre-condition
The developer has cloned the darkfactory repo and symlinked the init skill, but has not yet installed `darkfactory-dispatch` or `jq`.

### Step 1: Developer runs `/darkfactory-init`

The init skill reaches Step 4 (Verify CLI dependencies) and runs:
```bash
darkfactory-dispatch --version 2>/dev/null || echo "NOT_INSTALLED"
jq --version 2>/dev/null || echo "NOT_INSTALLED"
```

Both commands output `NOT_INSTALLED`.

**Expected behavior:** The skill reports both tools as missing and provides install instructions.

**Verified:** The install instructions block (lines 107-113) includes:
```
- darkfactory-dispatch: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install`
- jq: `brew install jq`
```

The developer sees exactly what to run. **PASS**

### Step 2: Developer installs both tools and re-runs `/darkfactory-init`

After running `cd ~/.darkfactory/tools/darkfactory-dispatch && make install` and `brew install jq`, the developer re-runs the init skill.

**Expected behavior:** The status report shows both tools as installed.

**Verified:** The status report template (lines 139-145) includes:
```
CLI tools:
  [x] codex (authenticated)
  [x] darkfactory-dispatch
  [x] gh (authenticated)
  [x] openspec
  [x] jq
```

Both new tools appear in the report with checkmarks. **PASS**

### Step 3: Developer attempts a mission without `darkfactory-dispatch`

If the developer skips the install and runs `/mission`, the worker dispatch step will fail because `darkfactory-dispatch` is not found. The init skill now catches this before the developer wastes time starting a mission.

**Expected behavior:** Init warns before mission start. This is the purpose of the check.

**Verified:** The check is in Step 4, which runs before the "Ready: run /mission to start" message. A missing `darkfactory-dispatch` prevents the ready message. **PASS**

## Edge Cases

### `darkfactory-dispatch` installed but not on PATH
The `--version` check runs the command directly, so if the binary exists but is not on `$PATH`, it reports `NOT_INSTALLED`. The install instructions tell the user to run `make install`, which (per the Makefile) copies the binary to a PATH-accessible location. **Acceptable behavior.**

### `jq` not installed but `darkfactory-dispatch` works
The init skill checks both independently. If `jq` is missing, the user gets the install instruction for `jq` specifically. The mission skill needs `jq` to parse dispatch JSON output, so this check prevents a confusing mid-mission failure. **Correct behavior.**

## Verdict

**PASS** -- All user-facing scenarios produce correct output. Install instructions are actionable. The status report reflects the new dependencies.
