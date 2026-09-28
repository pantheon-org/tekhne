#!/usr/bin/env bash
# shell: bash
# Prints every Claude Code MCP server, plugin, and hook-wired binary this machine
# currently has active, at both project and global scope. Intended as raw input
# for a manual or agent-driven diff against the Context Radar tool list: it does
# NOT interpret the data, just surfaces it, since matching tool names to Context
# Radar tool `id`s reliably needs judgement (e.g. a hook command path like
# ".../cargo-tokensave/latest/bin/tokensave" maps to the "tokensave" id).
set -euo pipefail

CLAUDE_HOME="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
PROJECT_DIR="${1:-$(pwd)}"

section() { printf '\n=== %s ===\n' "$1"; }

section "Project .mcp.json ($PROJECT_DIR/.mcp.json)"
[ -f "$PROJECT_DIR/.mcp.json" ] && node -e "console.log(Object.keys(require('$PROJECT_DIR/.mcp.json').mcpServers || {}).join('\n'))" || echo "(none)"

section "Global .mcp.json ($CLAUDE_HOME/.mcp.json)"
[ -f "$CLAUDE_HOME/.mcp.json" ] && node -e "console.log(Object.keys(require('$CLAUDE_HOME/.mcp.json').mcpServers || {}).join('\n'))" || echo "(none)"

section "Global plugins, scope=user ($CLAUDE_HOME/plugins/installed_plugins.json)"
if [ -f "$CLAUDE_HOME/plugins/installed_plugins.json" ]; then
  node -e "
    const p = require('$CLAUDE_HOME/plugins/installed_plugins.json').plugins || {};
    for (const [name, entries] of Object.entries(p)) {
      for (const e of entries) if (e.scope === 'user') console.log(name, e.version);
    }
  "
else
  echo "(none)"
fi

section "Global hook commands ($CLAUDE_HOME/settings.json)"
if [ -f "$CLAUDE_HOME/settings.json" ]; then
  node -e "
    const h = require('$CLAUDE_HOME/settings.json').hooks || {};
    for (const [event, entries] of Object.entries(h)) {
      for (const entry of entries) {
        for (const hook of entry.hooks || []) {
          console.log(event, '|', entry.matcher || '(any)', '->', hook.command);
        }
      }
    }
  "
else
  echo "(none)"
fi

section "Global permission-allowlisted MCP tool prefixes ($CLAUDE_HOME/settings.json)"
if [ -f "$CLAUDE_HOME/settings.json" ]; then
  node -e "
    const allow = (require('$CLAUDE_HOME/settings.json').permissions || {}).allow || [];
    const servers = new Set(
      allow.filter((p) => p.startsWith('mcp__')).map((p) => p.split('__')[1])
    );
    console.log([...servers].join('\n'));
  "
else
  echo "(none)"
fi

section "Rule files mandating a specific tool ($CLAUDE_HOME/rules/*.md)"
grep -lriE 'mandatory|MUST use|NEVER use.*when' "$CLAUDE_HOME"/rules/*.md 2>/dev/null || echo "(none)"
