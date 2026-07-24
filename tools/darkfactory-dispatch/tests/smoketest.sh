#!/bin/sh
# smoketest.sh -- live-codex end-to-end tests for darkfactory-dispatch
# Requires: darkfactory-dispatch on PATH, codex CLI authenticated
# Run: ./tests/smoketest.sh (or: make smoke)
set -e

DISPATCH="darkfactory-dispatch"
TMPDIR=$(mktemp -d /tmp/dispatch-smoke.XXXXXX)
PASS=0
FAIL=0

cleanup() {
    rm -rf "$TMPDIR"
}
trap cleanup EXIT

# Extract a JSON field from a file using grep (avoids jq control-char issues)
json_field() {
    grep -o "\"$1\":\"[^\"]*\"" "$2" 2>/dev/null | head -1 | cut -d'"' -f4
}
json_bool() {
    grep -o "\"$1\":[a-z]*" "$2" 2>/dev/null | head -1 | cut -d: -f2
}

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

echo "=== darkfactory-dispatch smoke tests ==="
echo ""

# Verify binary exists
command -v "$DISPATCH" >/dev/null 2>&1 || {
    echo "ERROR: $DISPATCH not found on PATH. Run: make install"
    exit 2
}

# Verify codex is available
command -v codex >/dev/null 2>&1 || {
    echo "ERROR: codex not found on PATH. Install: npm install -g @openai/codex"
    exit 2
}

echo "Binary: $(command -v "$DISPATCH")"
echo "Version: $("$DISPATCH" --version)"
echo ""

# --- Test 1: Simple task (write a file) ---
echo "Test 1: Simple task"
"$DISPATCH" \
    --model gpt-5.6-terra \
    --effort low \
    --timeout 60 \
    --sandbox workspace-write \
    --prompt "Create a file called $TMPDIR/smoke-output.txt containing the text 'smoke test passed'. Use echo or printf." \
    --output "$TMPDIR/smoke-output.txt" \
    --json > "$TMPDIR/result1.json" 2>/dev/null || true

check "dispatch completes" [ "$(json_field status "$TMPDIR/result1.json")" = "completed" ]
check "output file exists" [ "$(json_bool output_exists "$TMPDIR/result1.json")" = "true" ]
check "output file has content" [ -s "$TMPDIR/smoke-output.txt" ]

echo ""

# --- Test 2: Timeout (child killed) ---
echo "Test 2: Timeout handling"
"$DISPATCH" \
    --model gpt-5.6-terra \
    --effort high \
    --timeout 10 \
    --grace 5 \
    --sandbox workspace-write \
    --prompt "Run the command: sleep 300. Wait for it to complete. Do not skip or cancel." \
    --json > "$TMPDIR/result2.json" 2>/dev/null || true

check "dispatch times out" [ "$(json_field status "$TMPDIR/result2.json")" = "timed_out" ]
check "error is retryable" [ "$(json_bool retryable "$TMPDIR/result2.json")" = "true" ]

echo ""

# --- Test 3: Prompt file ---
echo "Test 3: Prompt file"
printf 'Create a file called %s/prompt-test.txt containing "prompt file works".\n' "$TMPDIR" > "$TMPDIR/prompt.txt"

"$DISPATCH" \
    --model gpt-5.6-terra \
    --effort low \
    --timeout 60 \
    --sandbox workspace-write \
    --prompt-file "$TMPDIR/prompt.txt" \
    --output "$TMPDIR/prompt-test.txt" \
    --json > "$TMPDIR/result3.json" 2>/dev/null || true

check "prompt-file dispatch completes" [ "$(json_field status "$TMPDIR/result3.json")" = "completed" ]

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
