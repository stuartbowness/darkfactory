#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
skill="$repo_root/skills/mission/SKILL.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

contains() {
  local needle="$1"
  local haystack="$2"
  grep -Fqi "$needle" <<<"$haystack" || fail "missing: $needle"
}

phase="$(
  awk '
    /^### Phase 4\.4: Generate and Review Tasks/ { in_phase = 1 }
    in_phase { print }
    /^### Phase 4a: Spec Coverage Audit/ { exit }
  ' "$skill"
)"

[[ -n "$phase" ]] || fail "missing Phase 4.4 section"

contains "#### Concern Separation Scan" "$phase"
contains "multi-task module" "$phase"
contains "human cannot approve" "$phase"
contains "named concerns" "$phase"
contains "do not prescribe specific filenames" "$phase"
contains "orchestrator drafts" "$phase"

closing_line="$(awk 'index($0, "#### Mandatory Closing Tasks") { print NR; exit }' <<<"$phase")"
concern_line="$(awk 'index($0, "#### Concern Separation Scan") { print NR; exit }' <<<"$phase")"
review_line="$(awk 'index($0, "Present the full task list") { print NR; exit }' <<<"$phase")"

[[ -n "$closing_line" ]] || fail "missing Mandatory Closing Tasks section"
[[ -n "$concern_line" ]] || fail "missing Concern Separation Scan section"
[[ -n "$review_line" ]] || fail "missing Phase 4.4 review handoff"

(( closing_line < concern_line )) || fail "concern scan must come after Mandatory Closing Tasks"
(( concern_line < review_line )) || fail "concern scan must come before review handoff"

printf 'PASS: Phase 4.4 includes concern separation scan gate before tasks review\n'
