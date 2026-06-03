#!/bin/sh
# smoketest.sh -- end-to-end smoke test for darkfactory-render.
#
# Exercises the primary operator flow against the REAL built binary (no mocks):
#   1. `render` over a fixture -> a contract in the expected format on stdout.
#   2. `shadow` over the same fixture vs the captured render -> no differences,
#      and the existing contract file is left BYTE-UNCHANGED (the read-only
#      guarantee that underpins the shadow/cutover mechanism).
#   3. `shadow` vs a deliberately different contract -> exit 1 + a non-empty diff.
#
# Portable POSIX sh: runs on both Debian/Ubuntu (GNU) and macOS (BSD). No
# GNU-only flags (no `sed -i`, `readlink -f`, `grep -P`, `stat -c`); `cksum` is
# POSIX and present on both platforms.
#
# Run from the crate root: ./tests/smoketest.sh  (or: make smoke)
# Exit: 0 if every check passes; 1 if any check fails; 2 on setup failure
# (build failed or binary missing).
set -e

BIN="./target/debug/darkfactory-render"
SPECS="tests/fixtures/cli/specs"
TASKS="tests/fixtures/cli/tasks.md"
FRAGMENT="tests/fixtures/cli/fragment.md"
EXISTING_IDENTICAL="tests/fixtures/cli/contract_identical.md"
EXISTING_DIFFERENT="tests/fixtures/cli/contract_different.md"

TMPDIR=$(mktemp -d /tmp/render-smoke.XXXXXX)
PASS=0
FAIL=0

cleanup() {
    rm -rf "$TMPDIR"
}
trap cleanup EXIT

check() {
    TEST_NAME="$1"
    shift
    if "$@"; then
        PASS=$((PASS + 1))
        printf "  PASS  %s\n" "$TEST_NAME"
    else
        FAIL=$((FAIL + 1))
        printf "  FAIL  %s\n" "$TEST_NAME"
    fi
}

# POSIX `cksum` of a file: first whitespace-delimited field (CRC + size, which
# is enough to detect any byte change). Portable across GNU and BSD.
checksum() {
    cksum "$1" | cut -d' ' -f1,2
}

contains() {
    # contains <file> <fixed-substring>
    # `-e <pattern>` so substrings starting with `-` (e.g. "- [ ] [") are not
    # parsed as grep options on BSD grep.
    grep -qF -e "$2" "$1"
}

echo "=== darkfactory-render smoke tests ==="
echo ""

# --- Setup: build the binary (fail loud) ---
echo "Building binary..."
cargo build || {
    echo "ERROR: cargo build failed" >&2
    exit 2
}
[ -x "$BIN" ] || {
    echo "ERROR: binary not found or not executable: $BIN" >&2
    exit 2
}
echo "Binary: $BIN"
echo ""

# --- Test 1: render (primary flow) ---
echo "Test 1: render emits a contract in the expected format"
RENDER_OUT="$TMPDIR/render.out"
RENDER_ERR="$TMPDIR/render.err"
RENDER_CODE=0
"$BIN" render --specs "$SPECS" --tasks "$TASKS" --fragment "$FRAGMENT" \
    >"$RENDER_OUT" 2>"$RENDER_ERR" || RENDER_CODE=$?

check "render exits 0" [ "$RENDER_CODE" -eq 0 ]
check "render stdout has '# Contract' title" contains "$RENDER_OUT" "# Contract"
check "render stdout has a '## Task ' section" contains "$RENDER_OUT" "## Task "
check "render stdout has a '<!-- Specs: ' comment" contains "$RENDER_OUT" "<!-- Specs: "
check "render stdout has a '- [ ] [' check line" contains "$RENDER_OUT" "- [ ] ["

echo ""

# --- Test 2: shadow vs identical contract (read-only, byte-unchanged) ---
echo "Test 2: shadow leaves the existing contract byte-unchanged"
BEFORE=$(checksum "$EXISTING_IDENTICAL")
SHADOW_OUT="$TMPDIR/shadow.out"
SHADOW_ERR="$TMPDIR/shadow.err"
SHADOW_CODE=0
"$BIN" shadow --specs "$SPECS" --tasks "$TASKS" --fragment "$FRAGMENT" \
    --existing "$EXISTING_IDENTICAL" \
    >"$SHADOW_OUT" 2>"$SHADOW_ERR" || SHADOW_CODE=$?
AFTER=$(checksum "$EXISTING_IDENTICAL")

check "shadow (identical) exits 0" [ "$SHADOW_CODE" -eq 0 ]
check "shadow reports 'no differences'" contains "$SHADOW_OUT" "no differences"
check "existing contract is byte-unchanged" [ "$BEFORE" = "$AFTER" ]

echo ""

# --- Test 3: shadow vs different contract (difference path) ---
echo "Test 3: shadow flags a differing contract"
DIFF_OUT="$TMPDIR/diff.out"
DIFF_ERR="$TMPDIR/diff.err"
DIFF_CODE=0
"$BIN" shadow --specs "$SPECS" --tasks "$TASKS" --fragment "$FRAGMENT" \
    --existing "$EXISTING_DIFFERENT" \
    >"$DIFF_OUT" 2>"$DIFF_ERR" || DIFF_CODE=$?

check "shadow (different) exits 1" [ "$DIFF_CODE" -eq 1 ]
check "shadow (different) emits a non-empty diff" [ -s "$DIFF_OUT" ]

echo ""

# --- Summary ---
TOTAL=$((PASS + FAIL))
echo "=== Results: $PASS/$TOTAL passed ==="

if [ "$FAIL" -gt 0 ]; then
    echo "FAILED"
    exit 1
else
    echo "ALL PASSED"
    exit 0
fi
