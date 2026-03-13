#!/usr/bin/env bash
set -euo pipefail

TILES_DIR=".tessl/tiles"
OPENCODE_SKILLS_DIR=".agents/skills/tessl"
MCP_JSON=".mcp.json"
OPENCODE_JSON="opencode.json"

for arg in "$@"; do
  case "$arg" in
    --opencode-skills) OPENCODE_SKILLS_DIR=".opencode/skills/tessl" ;;
  esac
done

mkdir -p "$OPENCODE_SKILLS_DIR"

if [ -f "$MCP_JSON" ]; then
  mcp_servers=$(jq '.mcpServers | to_entries | map({key: .key, value: {type: "local", command: ([.value.command] + (.value.args // []))}}) | from_entries' "$MCP_JSON")

  if [ -f "$OPENCODE_JSON" ]; then
    existing=$(cat "$OPENCODE_JSON")
  else
    existing='{"$schema":"https://opencode.ai/config.json"}'
  fi

  updated=$(echo "$existing" | jq --argjson mcp "$mcp_servers" '."$schema" = "https://opencode.ai/config.json" | .mcp = $mcp')

  if [ "$updated" != "$existing" ]; then
    echo "$updated" > "$OPENCODE_JSON"
    echo "updated: $OPENCODE_JSON with mcp servers from $MCP_JSON"
  else
    echo "skip: $OPENCODE_JSON mcp section already up to date"
  fi
fi

declare -A managed_links=()

while IFS= read -r tile_json; do
  tile_dir="$(dirname "$tile_json")"

  tile_name=$(jq -r '.name // ""' "$tile_json" 2>/dev/null || true)
  tile_slug="${tile_name/\//_}"
  skill_names=$(jq -r '.skills // {} | keys[]' "$tile_json" 2>/dev/null || true)

  for skill_name in $skill_names; do
    target="$(pwd)/$tile_dir"
    link="$OPENCODE_SKILLS_DIR/${tile_slug}_${skill_name}"
    managed_links["$link"]=1

    if [ -L "$link" ]; then
      existing="$(readlink "$link")"
      if [ "$existing" = "$target" ]; then
        echo "skip: $link -> $target (already correct)"
        continue
      fi
      echo "update: $link -> $target (was $existing)"
      rm "$link"
    elif [ -e "$link" ]; then
      echo "warning: $link exists and is not a symlink, skipping"
      continue
    else
      echo "create: $link -> $target"
    fi

    ln -s "$target" "$link"
  done
done < <(find "$TILES_DIR" -name "tile.json" | sort)

while IFS= read -r stale; do
  if [ -z "${managed_links["$stale"]+_}" ]; then
    echo "remove: $stale (stale)"
    rm "$stale"
  fi
done < <(find "$OPENCODE_SKILLS_DIR" -maxdepth 1 -type l -name '*__*')
