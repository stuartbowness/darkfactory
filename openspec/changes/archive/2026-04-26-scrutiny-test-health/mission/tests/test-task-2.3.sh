#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/.claude/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

check7="$(
  awk '/^7\. \*\*Code standards check:/{in_check=1} /^8\. \*\*Code simplify:/{in_check=0} in_check {print}' "$prompt"
)"

contains_in_check7() {
  local needle="$1"
  grep -Fq "$needle" <<<"$check7" || fail "check 7 missing: $needle"
}

contains_in_check7 'Before reviewing code standards, read the `Error handling smells` named fact from the deterministic pre-pass.'
contains_in_check7 'Use the smell count, changed production line count, per-KLOC density, and locations in the Code standards result.'
contains_in_check7 'Under 5 per KLOC: PASS for the error handling smell density component.'
contains_in_check7 '5-10 per KLOC: WARNING for the error handling smell density component.'
contains_in_check7 'Over 10 per KLOC: FAIL for the error handling smell density component.'
contains_in_check7 'Standalone test files are excluded from the smell count by the pre-pass; confirm that exclusion and do not double-count test files.'
contains_in_check7 'Examine each flagged smell location and report file:line references with specific remediation guidance.'
contains_in_check7 'Replace `.unwrap()` at store.rs:42 with `.context("failed to open connection pool")?` or `.expect("pool initialized in main")`.'

printf 'PASS: scrutiny validator code standards error handling smell density instructions are present\n'
