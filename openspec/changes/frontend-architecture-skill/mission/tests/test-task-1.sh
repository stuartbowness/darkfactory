#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
deep_components="$repo_root/skills/frontend/DEEP-COMPONENTS.md"
stack="$repo_root/skills/frontend/STACK.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

contains() {
  local file="$1"
  local needle="$2"
  grep -Fqi "$needle" "$file" || fail "$file missing: $needle"
}

[[ -f "$deep_components" ]] || fail "missing skills/frontend/DEEP-COMPONENTS.md"
[[ -f "$stack" ]] || fail "missing skills/frontend/STACK.md"

contains "$deep_components" "Small semantic interface"
contains "$deep_components" "How many independent domain concepts"
contains "$deep_components" "Derive don't sync"
contains "$deep_components" "Can this value be computed"
contains "$deep_components" "Composition over configuration"
contains "$deep_components" "Would this be clearer as compound components"
contains "$deep_components" "Collapse coordination"
contains "$deep_components" "Does the parent coordinate state"
contains "$deep_components" "design system"
contains "$deep_components" "route"
contains "$deep_components" "adapter"
contains "$deep_components" "headless"
contains "$deep_components" "deletion test"
contains "$deep_components" "If you deleted this component"

contains "$stack" "React 19"
contains "$stack" "Vite"
contains "$stack" "TailwindCSS 4"
contains "$stack" "Radix UI / ShadCN"
contains "$stack" "Vitest"
contains "$stack" "React Router"
contains "$stack" "barrel"
contains "$stack" "cn()"
contains "$stack" "components/ui/"
contains "$stack" "components/<domain>/"
contains "$stack" "semantic color tokens"
contains "$stack" "data-slot"

printf 'PASS: frontend foundation vocabulary files contain required references\n'
