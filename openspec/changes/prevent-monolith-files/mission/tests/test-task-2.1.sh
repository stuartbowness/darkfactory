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
    /^### Phase 4\.3: Generate and Review Design/ { in_phase = 1 }
    in_phase { print }
    /^### Phase 4\.4: Generate and Review Tasks/ { exit }
  ' "$skill"
)"

[[ -n "$phase" ]] || fail "missing Phase 4.3 section"

contains "#### Concern Separation Scan" "$phase"
contains "multi-task module" "$phase"
contains "human cannot approve" "$phase"
contains "named concerns" "$phase"
contains "do not prescribe specific filenames" "$phase"
contains "orchestrator drafts" "$phase"

design_line="$(awk 'index($0, "Generate `design.md`") { print NR; exit }' <<<"$phase")"
existing_line="$(awk 'index($0, "#### Existing Codebases") { print NR; exit }' <<<"$phase")"
concern_line="$(awk 'index($0, "#### Concern Separation Scan") { print NR; exit }' <<<"$phase")"
review_line="$(awk 'index($0, "Present it to the human with a recommendation") { print NR; exit }' <<<"$phase")"

[[ -n "$design_line" ]] || fail "missing design.md generation line"
[[ -n "$existing_line" ]] || fail "missing Existing Codebases section"
[[ -n "$concern_line" ]] || fail "missing Concern Separation Scan section"
[[ -n "$review_line" ]] || fail "missing Phase 4.3 review handoff"

(( design_line < concern_line )) || fail "concern scan must come after design.md generation"
(( existing_line < concern_line )) || fail "concern scan must come after Existing Codebases"
(( concern_line < review_line )) || fail "concern scan must come before review handoff"

printf 'PASS: Phase 4.3 includes concern separation scan gate before design review\n'
