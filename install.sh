#!/bin/sh
DARKFACTORY="$HOME/.darkfactory"
SKILLS_DIR="$DARKFACTORY/skills"
CLAUDE_SKILLS="$HOME/.claude/skills"
AGENTS_SKILLS="$HOME/.agents/skills"
CODEX_SKILLS="$HOME/.codex/skills"

mkdir -p "$CLAUDE_SKILLS" "$AGENTS_SKILLS" "$CODEX_SKILLS"

if [ ! -d "$SKILLS_DIR" ]; then
    echo "Error: $SKILLS_DIR not found" >&2
    exit 1
fi

count=0
for skill_dir in "$SKILLS_DIR"/*/; do
    [ -d "$skill_dir" ] || continue
    name=$(basename "$skill_dir")
    rm -f "$CLAUDE_SKILLS/$name" "$AGENTS_SKILLS/$name" "$CODEX_SKILLS/$name"
    ln -sf "${skill_dir%/}" "$CLAUDE_SKILLS/$name"
    ln -sf "$CLAUDE_SKILLS/$name" "$AGENTS_SKILLS/$name"
    ln -sf "$CLAUDE_SKILLS/$name" "$CODEX_SKILLS/$name"
    echo "linked: $name"
    count=$((count + 1))
done

if [ "$count" -eq 0 ]; then
    echo "Error: no skills found in $SKILLS_DIR" >&2
    exit 1
fi

echo "Done. $count skills installed."
