#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/.claude/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

contains() {
  local needle="$1"
  grep -Fq "$needle" "$prompt" || fail "missing: $needle"
}

contains "3. **Tests:** Run the project's test command. Capture stdout/stderr and the exit code. After the command finishes, fill the \`Test runner\` named fact by parsing the captured summary line as described in the deterministic pre-pass."
contains "Report the Tests row using structured counts from \`Test runner\`, for example \`tests | FAIL | 49 passed, 46 failed, 2 ignored (51.5% pass rate)\` or \`tests | PASS | 234 passed, 0 failed, 0 ignored (100% pass rate)\`."
contains "If the test command exits non-zero, mark Tests as FAIL."
contains "If tests fail, group failures that share the same error message or failure signature."
contains "46 failures in 1 group: 'connection refused to 127.0.0.1:5432' -- infrastructure-dependent tests running without infrastructure."
contains "If the test command exits 0 but fewer than 90% of discovered tests actually ran, keep Tests as PASS and add a WARNING note: \`Only N of M tests actually ran. The rest were skipped/ignored.\`"
contains "Actual run rate is \`(passed + failed) / (passed + failed + skipped_or_ignored) * 100\`."
contains "| Tests | PASS/FAIL | \`<passed> passed, <failed> failed, <skipped_or_ignored> skipped/ignored (<pass_rate>% pass rate)\`"

printf 'PASS: scrutiny validator structured test reporting instructions are present\n'
