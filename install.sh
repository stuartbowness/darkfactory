#!/bin/sh
# Resolve repo root from script location, not a hardcoded path.
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"
DARKFACTORY="$SCRIPT_DIR"
SKILLS_DIR="$DARKFACTORY/skills"
AGENT_DEFS_DIR="$DARKFACTORY/agents"
CLAUDE_SKILLS="$HOME/.claude/skills"
AGENTS_SKILLS="$HOME/.agents/skills"
CODEX_SKILLS="$HOME/.codex/skills"
CLAUDE_AGENTS="$HOME/.claude/agents"

mkdir -p "$CLAUDE_SKILLS" "$AGENTS_SKILLS" "$CODEX_SKILLS" "$CLAUDE_AGENTS"

if [ ! -d "$SKILLS_DIR" ]; then
    echo "Error: $SKILLS_DIR not found" >&2
    exit 1
fi

if [ ! -d "$AGENT_DEFS_DIR" ]; then
    echo "Error: $AGENT_DEFS_DIR not found" >&2
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

# Agent definitions pin the model and reasoning effort for Claude-engine mission roles.
# Claude Code only reads them from ~/.claude/agents, so they install separately.
agent_count=0
for agent_file in "$AGENT_DEFS_DIR"/*.md; do
    [ -f "$agent_file" ] || continue
    name=$(basename "$agent_file")
    rm -f "$CLAUDE_AGENTS/$name"
    ln -sf "$agent_file" "$CLAUDE_AGENTS/$name"
    echo "linked agent: $name"
    agent_count=$((agent_count + 1))
done

if [ "$agent_count" -eq 0 ]; then
    echo "Error: no agent definitions found in $AGENT_DEFS_DIR" >&2
    exit 1
fi

echo "Done. $count skills and $agent_count agents installed."
