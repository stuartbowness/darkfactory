#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

contains() {
  local needle="$1"
  local haystack="$2"
  grep -Fq "$needle" <<<"$haystack" || fail "missing: $needle"
}

check6="$(
  awk '
    /^6\. \*\*Module depth review:\*\*/ { in_check = 1 }
    in_check { print }
    /^7\. \*\*Code standards:\*\*/ { exit }
  ' "$prompt"
)"

[[ -n "$check6" ]] || fail "missing Check 6 module depth review"

contains "600" "$check6"
contains "WARNING" "$check6"
contains "non-blocking" "$check6"
contains "1000" "$check6"
contains "hard-fail" "$check6"

grep -Fq "900" "$prompt" && fail "900-line threshold should be removed"
grep -Fq "| Module depth | PASS/WARN/FAIL |" "$prompt" || fail "Module depth output row must allow WARN"

printf 'PASS: scrutiny validator Check 6 uses graduated file size thresholds\n'
