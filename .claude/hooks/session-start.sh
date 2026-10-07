#!/bin/bash
# Installs dependencies and linters for Claude Code cloud sessions.
# mise (mise.run / GitHub releases) is not reachable from the cloud network
# policy, so bun, node and cargo come from the container and rumdl is built
# with cargo at the version pinned in mise.toml.
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Skip the `prepare` script: it runs `hk install`, and git hooks are not
# needed in a cloud session.
bun install --ignore-scripts

RUMDL_VERSION="$(sed -n 's/^rumdl = "\(.*\)"/\1/p' mise.toml | head -1)"
if ! command -v rumdl >/dev/null 2>&1; then
  cargo install rumdl --locked ${RUMDL_VERSION:+--version "$RUMDL_VERSION"}
fi
