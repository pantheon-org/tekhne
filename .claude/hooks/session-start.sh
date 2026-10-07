#!/bin/bash
# Installs tooling and dependencies for Claude Code cloud sessions.
#
# Preferred path: mise, which provides the toolchain pinned in mise.toml.
# mise.run and GitHub releases are blocked by the default cloud network
# policy, so mise itself is installed from npm. Its tool backends (aqua,
# core) still need github.com / api.github.com, so `mise install` fails
# unless the environment's network policy allows GitHub. In that case the
# hook falls back to the bun / node / cargo that ship in the container.
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "${CLAUDE_PROJECT_DIR:-$(pwd)}"

MISE_OK=0
if ! command -v mise >/dev/null 2>&1; then
  npm install -g mise >/dev/null 2>&1 || true
fi

if command -v mise >/dev/null 2>&1; then
  mise trust -q . || true
  if mise install -q; then
    MISE_OK=1
  else
    echo "mise install failed (GitHub likely blocked); falling back to container tools" >&2
  fi
fi

if [ "$MISE_OK" = "1" ]; then
  # Put the pinned tools on PATH for the rest of the session.
  if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
    echo 'export PATH="$HOME/.local/share/mise/shims:$PATH"' >> "$CLAUDE_ENV_FILE"
  fi
  export PATH="$HOME/.local/share/mise/shims:$PATH"
  # Runs `prepare`, which installs the hk git hooks.
  bun install
else
  # Skip `prepare`: it runs `hk install`, and hk is not available here.
  bun install --ignore-scripts
  RUMDL_VERSION="$(sed -n 's/^rumdl = "\(.*\)"/\1/p' mise.toml | head -1)"
  if ! command -v rumdl >/dev/null 2>&1; then
    cargo install rumdl --locked ${RUMDL_VERSION:+--version "$RUMDL_VERSION"}
  fi
fi
