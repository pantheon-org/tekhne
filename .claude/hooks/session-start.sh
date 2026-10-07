#!/bin/bash
# Installs the toolchain pinned in mise.toml and the JS dependencies for
# Claude Code cloud sessions. mise is the only install path: if it cannot
# install the tools, the hook fails instead of falling back.
#
# mise.run and GitHub releases are blocked by the default cloud network
# policy, so mise itself comes from npm. Its tool backends need
# github.com, api.github.com and objects.githubusercontent.com, so those
# hosts must be allowed in the environment's network policy, and GitHub
# access must be available to the session (or a GITHUB_TOKEN secret set).
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "${CLAUDE_PROJECT_DIR:-$(pwd)}"

if ! command -v mise >/dev/null 2>&1; then
  npm install -g mise
fi

mise trust -q .
mise install -q

# Put the pinned tools on PATH for the rest of the session.
export PATH="$HOME/.local/share/mise/shims:$PATH"
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo 'export PATH="$HOME/.local/share/mise/shims:$PATH"' >> "$CLAUDE_ENV_FILE"
fi

# Runs `prepare`, which installs the hk git hooks.
bun install
