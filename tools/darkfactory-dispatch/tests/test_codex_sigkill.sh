#!/bin/bash
set -euo pipefail

if [[ -n "${CODEX_MOCK_PID_FILE:-}" ]]; then
  echo "$$" > "${CODEX_MOCK_PID_FILE}"
fi

trap '' TERM
echo '{"type":"thread.started","thread_id":"sigkill-test"}'
sleep 300
