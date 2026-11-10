#!/bin/sh
# Link this checkout without replacing user content.
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)"
INSTALL_ROOT="$HOME"
DRY_RUN=false
usage() {
    echo "Usage: sh install.sh [--prefix /absolute/root] [--dry-run]"
    echo "Conflicting files, directories, and links are never replaced."
}
while [ "$#" -gt 0 ]; do
    case "$1" in
        --prefix)
            [ "$#" -ge 2 ] || { usage >&2; exit 2; }
            INSTALL_ROOT=$2; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --help|-h) usage; exit 0 ;;
        *) usage >&2; exit 2 ;;
    esac
done
case "$INSTALL_ROOT" in
    /*) ;;
    *) echo "Error: --prefix must be an absolute path." >&2; exit 2 ;;
esac
SKILLS_DIR="$SCRIPT_DIR/skills"
AGENT_DEFS_DIR="$SCRIPT_DIR/agents"
[ -d "$SKILLS_DIR" ] && [ -d "$AGENT_DEFS_DIR" ] || {
    echo "Error: this checkout must contain skills/ and agents/." >&2; exit 1
}
skill_count=0
agent_count=0
for source in "$SKILLS_DIR"/*/; do
    [ -d "$source" ] || continue
    [ -f "$source/SKILL.md" ] || {
        echo "Error: missing SKILL.md in $source" >&2; exit 1
    }
    skill_count=$((skill_count + 1))
done
for source in "$AGENT_DEFS_DIR"/*.md; do
    [ -f "$source" ] || continue
    agent_count=$((agent_count + 1))
done
[ "$skill_count" -gt 0 ] && [ "$agent_count" -gt 0 ] || {
    echo "Error: no skills or agent definitions found." >&2; exit 1
}
# Accept links into this checkout, including the older .claude/skills chain.
same_link() {
    [ -L "$2" ] || return 1
    [ "$(readlink "$2")" = "$1" ] && return 0
    if [ -d "$1" ] && [ -d "$2" ]; then
        [ "$(CDPATH= cd -- "$2" && pwd -P)" = "$1" ] && return 0
    fi
    return 1
}
preflight() {
    if [ -e "$2" ] || [ -L "$2" ]; then
        same_link "$1" "$2" || {
            echo "Conflict: $2 is not a link to this checkout." >&2
            echo "Move it aside deliberately, or use its existing checkout." >&2
            exit 1
        }
    fi
}
install_link() {
    if same_link "$1" "$2"; then
        echo "unchanged: $2"
    elif [ "$DRY_RUN" = true ]; then
        echo "would link: $2 -> $1"
    else
        # No -f: never force replacement of an existing link or file.
        ln -s "$1" "$2"
        echo "linked: $2"
    fi
}
visit_links() {
    operation=$1
    for source in "$SKILLS_DIR"/*/; do
        source=${source%/}
        name=${source##*/}
        for client in .claude .codex .agents; do
            "$operation" "$source" "$INSTALL_ROOT/$client/skills/$name"
        done
    done
    for source in "$AGENT_DEFS_DIR"/*.md; do
        name=${source##*/}
        "$operation" "$source" "$INSTALL_ROOT/.claude/agents/$name"
    done
}
# Preflight every destination before creating anything.
for client in .claude .codex .agents; do
    for dir in "$INSTALL_ROOT/$client" "$INSTALL_ROOT/$client/skills"; do
        if { [ -e "$dir" ] || [ -L "$dir" ]; } && [ ! -d "$dir" ]; then
            echo "Conflict: $dir is not a directory." >&2; exit 1
        fi
    done
done
dir="$INSTALL_ROOT/.claude/agents"
if { [ -e "$dir" ] || [ -L "$dir" ]; } && [ ! -d "$dir" ]; then
    echo "Conflict: $dir is not a directory." >&2; exit 1
fi
visit_links preflight
if [ "$DRY_RUN" = false ]; then
    mkdir -p "$INSTALL_ROOT/.claude/skills" "$INSTALL_ROOT/.claude/agents" \
        "$INSTALL_ROOT/.codex/skills" "$INSTALL_ROOT/.agents/skills"
fi
visit_links install_link
echo "Done. $skill_count skills and $agent_count agents checked (dry-run: $DRY_RUN)."
