#!/usr/bin/env bash
# Ensure unmatched globs expand to nothing instead of remaining as literals.
shopt -s nullglob
#!/usr/bin/env bash
# Ensure unmatched globs expand to nothing instead of remaining as literals.
set -euo pipefail
shopt -s nullglob

# Resolve repo root relative to this script, so it works regardless of CWD.
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

TILES_DIR=".tessl/tiles"
OPENCODE_SKILLS_DIR=".agents/skills"
MCP_JSON=".mcp.json"
OPENCODE_JSON="opencode.json"

for arg in "$@"; do
  case "$arg" in
    --opencode-skills) OPENCODE_SKILLS_DIR=".opencode/skills" ;;
  esac
done

mkdir -p "$OPENCODE_SKILLS_DIR"

for link in "$OPENCODE_SKILLS_DIR"/*; do
  if [ -L "$link" ] && [ ! -e "$link" ]; then
    echo "unlink: $link (broken symlink)"
    rm "$link"
  fi
done

if [ -f "$MCP_JSON" ]; then
  mcp_servers=$(jq '.mcpServers | to_entries | map({key: .key, value: {type: "local", command: ([.value.command] + (.value.args // []))}}) | from_entries' "$MCP_JSON")

  if [ -f "$OPENCODE_JSON" ]; then
    existing=$(cat "$OPENCODE_JSON")
  else
    # shellcheck disable=SC2016
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

for tile_json in "$TILES_DIR"/*/*/tile.json; do
  tile_dir="$(dirname "$tile_json")"

  while IFS= read -r skill_name; do
    target="$REPO_ROOT/$tile_dir"
    link="$OPENCODE_SKILLS_DIR/tessl__$skill_name"

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
  done < <(jq -r '.skills | keys[]' "$tile_json")
done
