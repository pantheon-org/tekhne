#!/bin/bash
# Installs the toolchain pinned in mise.toml and the JS dependencies for
# Claude Code cloud sessions. mise is the only install path: if it cannot
# install the tools, the hook fails instead of falling back.
#
# mise.run and the GitHub API are not reachable from the default cloud
# network policy, so mise itself comes from npm and the tools are installed
# from mise.lock (MISE_LOCKED=1): the lockfile carries each tool's version,
# download URL and checksum, so no GitHub API lookups are needed. A tool
# missing from mise.lock fails the install. Refresh it with `mise lock`
# from a machine with normal GitHub access whenever mise.toml changes.
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "${CLAUDE_PROJECT_DIR:-$(pwd)}"

if ! command -v mise >/dev/null 2>&1; then
  npm install -g mise
fi

export MISE_LOCKED=1
mise trust -q .
mise install -q

# Put the pinned tools on PATH for the rest of the session.
export PATH="$HOME/.local/share/mise/shims:$PATH"
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo "export PATH=\"\$HOME/.local/share/mise/shims:\$PATH\"" >> "$CLAUDE_ENV_FILE"
fi

# Runs `prepare`, which installs the hk git hooks.
bun install
