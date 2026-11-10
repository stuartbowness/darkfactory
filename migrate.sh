#!/bin/sh
# Compatibility entry point: migration no longer removes existing user skills.
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)"
echo "Migration uses the conflict-safe installer; old skill names are left intact."
exec sh "$SCRIPT_DIR/install.sh" "$@"
