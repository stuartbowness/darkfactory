#!/bin/sh
# Run this once when upgrading from the pre-skill-decomposition setup.
# Removes old ~/.claude/skills symlinks for skills that have been renamed
# or are now owned by this repo, then re-runs install.sh.

CLAUDE_SKILLS="$HOME/.claude/skills"

OLD_SKILLS="
darkfactory-init
grill-me
write-a-prd
ubiquitous-language
test-quality-review
security-review
"

removed=0
for name in $OLD_SKILLS; do
    target="$CLAUDE_SKILLS/$name"
    if [ -L "$target" ]; then
        rm "$target"
        echo "removed: $target"
        removed=$((removed + 1))
    fi
done

if [ "$removed" -eq 0 ]; then
    echo "Nothing to remove (already clean)."
else
    echo "Removed $removed old symlink(s)."
fi

echo ""
echo "Running install.sh..."
"$(dirname "$0")/install.sh"

echo ""
echo "Manual step: if you have a local clone of layercodedev/skills, you can now delete it."
echo "  Find it with:"
echo "    find ~ -name '.git' -maxdepth 5 -exec sh -c 'git -C \"\$(dirname {})\" remote -v 2>/dev/null | grep -q layercodedev/skills && echo \"\$(dirname {})\"' \; 2>/dev/null"
echo "  Then: rm -rf <path-to-clone>"
