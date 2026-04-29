# Darkfactory
After making any changes to skills, init, or project configuration, update README.md to reflect the change.

Shell scripts in skills must work on both Debian/Ubuntu (GNU coreutils) and macOS (BSD coreutils). Avoid GNU-only flags (`sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, `grep -P`). Use POSIX-portable forms instead (e.g., `mktemp /tmp/prefix.XXXXXX`, not `mktemp --suffix=prefix`).

## darkfactory-dispatch

Use `darkfactory-dispatch` to dispatch coding tasks to Codex with process supervision (timeout, heartbeat, process group isolation, structured JSON results). Run `darkfactory-dispatch --help` for full usage and examples.
