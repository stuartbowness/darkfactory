#!/bin/sh
DARKFACTORY="$HOME/.darkfactory"
SKILLS_DIR="$DARKFACTORY/skills"
CLAUDE_SKILLS="$HOME/.claude/skills"

mkdir -p "$CLAUDE_SKILLS"

if [ ! -d "$SKILLS_DIR" ]; then
    echo "Error: $SKILLS_DIR not found" >&2
    exit 1
fi

count=0
for skill_dir in "$SKILLS_DIR"/*/; do
    [ -d "$skill_dir" ] || continue
    name=$(basename "$skill_dir")
    target="$CLAUDE_SKILLS/$name"
    rm -f "$target"
    ln -sf "${skill_dir%/}" "$target"
    echo "linked: $target -> ${skill_dir%/}"
    count=$((count + 1))
done

if [ "$count" -eq 0 ]; then
    echo "Error: no skills found in $SKILLS_DIR" >&2
    exit 1
fi

echo "Done. $count skills installed."
