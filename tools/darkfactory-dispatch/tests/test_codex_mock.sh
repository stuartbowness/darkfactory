#!/usr/bin/env bash
set -euo pipefail

output=""
while (($#)); do
  if [[ "$1" == "-o" && $# -ge 2 ]]; then
    output="$2"
    shift 2
    continue
  fi
  shift
done

mode="${CODEX_MOCK_MODE:-success}"
if [[ -n "${CODEX_MOCK_PID_FILE:-}" ]]; then
  echo "$$" > "${CODEX_MOCK_PID_FILE}"
fi

write_output() {
  if [[ -n "$output" ]]; then
    mkdir -p "$(dirname "$output")"
    echo "mock output" > "$output"
  fi
}

case "$mode" in
  success)
    echo '{"type":"thread.started","thread_id":"test-123"}'
    echo '{"type":"item.started","item":{"type":"tool_use","name":"shell"}}'
    echo '{"type":"item.started","item":{"type":"function_call","command":"cargo test"}}'
    echo '{"type":"item.completed","item":{"type":"function_call","command":"cargo test"}}'
    echo '{"type":"item.completed","item":{"type":"file_change","file_path":"src/main.rs"}}'
    echo '{"type":"item.completed","item":{"type":"file_read","file_path":"README.md"}}'
    echo '{"type":"item.completed","item":{"type":"tool_use","name":"shell"}}'
    echo '{"type":"turn.completed","usage":{"input_tokens":100,"output_tokens":50,"reasoning_tokens":20}}'
    echo "mock stderr line" >&2
    write_output
    ;;
  malformed)
    echo 'not json'
    echo '{"type":"turn.completed","usage":{"input_tokens":1,"output_tokens":2,"reasoning_tokens":3}}'
    echo '{still not json'
    write_output
    ;;
  sleep)
    sleep 300
    ;;
  delayed_success)
    echo '{"type":"thread.started","thread_id":"slow-123"}'
    sleep 3
    echo '{"type":"turn.completed","usage":{"input_tokens":4,"output_tokens":5,"reasoning_tokens":6}}'
    write_output
    ;;
  *)
    echo "unknown CODEX_MOCK_MODE: $mode" >&2
    exit 64
    ;;
esac
