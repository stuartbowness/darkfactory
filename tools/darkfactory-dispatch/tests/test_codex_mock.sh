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
    echo '{"type":"item.completed","item":{"type":"agent_message","text":"mock response"}}'
    echo '{"type":"turn.completed","usage":{"input_tokens":1,"output_tokens":2,"reasoning_tokens":3}}'
    echo '{still not json'
    write_output
    ;;
  malformed_only)
    echo 'not json'
    echo '{still not json'
    write_output
    ;;
  failed)
    echo '{"type":"thread.started","thread_id":"fail-test"}'
    echo '{"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":5,"reasoning_tokens":0}}'
    write_output
    exit 1
    ;;
  no_output)
    echo '{"type":"thread.started","thread_id":"no-output"}'
    echo '{"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":5,"reasoning_tokens":0}}'
    ;;
  response_no_output)
    echo '{"type":"thread.started","thread_id":"resp-no-output"}'
    echo '{"type":"item.completed","item":{"type":"agent_message","text":"recovered analysis content"}}'
    echo '{"type":"turn.completed","usage":{"input_tokens":1,"output_tokens":2,"reasoning_tokens":3}}'
    ;;
  empty_response_no_output)
    echo '{"type":"thread.started","thread_id":"empty-resp"}'
    echo '{"type":"item.completed","item":{"type":"agent_message","text":""}}'
    echo '{"type":"turn.completed","usage":{"input_tokens":1,"output_tokens":2,"reasoning_tokens":3}}'
    ;;
  special_chars)
    echo '{"type":"thread.started","thread_id":"special-chars"}'
    echo '{"type":"item.started","item":{"type":"function_call","command":"grep -n \"foo\\bar\" baz | jq .x"}}'
    echo '{"type":"turn.completed","usage":{"input_tokens":1,"output_tokens":2,"reasoning_tokens":3}}'
    write_output
    ;;
  term_clean)
    trap 'exit 0' TERM
    sleep 300
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
  delayed_success_5)
    echo '{"type":"thread.started","thread_id":"slow-456"}'
    sleep 5
    echo '{"type":"turn.completed","usage":{"input_tokens":4,"output_tokens":5,"reasoning_tokens":6}}'
    write_output
    ;;
  *)
    echo "unknown CODEX_MOCK_MODE: $mode" >&2
    exit 64
    ;;
esac
