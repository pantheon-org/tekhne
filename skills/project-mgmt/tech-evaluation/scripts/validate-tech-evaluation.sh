#!/usr/bin/env bash
# shell: bash
# Validates a generated tech-evaluation YAML against its schema. Runs validate-tech-evaluation.ts on bun.
# Usage: validate-tech-evaluation.sh <file> [<file> ...]
# Needs bun 1.3.10 or later (the CI version; earlier versions read some YAML differently). No python, no PyYAML.
set -euo pipefail

MIN_MAJOR=1
MIN_MINOR=3
MIN_PATCH=10
NEEDED="needs bun ${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH} or later"

HERE="$(cd "${BASH_SOURCE[0]%/*}" && pwd -P)"

if ! command -v bun >/dev/null 2>&1; then
  echo "validate-tech-evaluation.sh ${NEEDED}, and bun is not on PATH." >&2
  exit 1
fi

# The version without a pre-release or build part (1.4.0-canary.1+abc gives 1.4.0), compared as numbers.
version="$(bun --version 2>/dev/null || true)"
core="${version%%[-+]*}"
IFS=. read -r major minor patch _ <<<"$core"
supported=0
if [[ "$major" =~ ^[0-9]+$ && "$minor" =~ ^[0-9]+$ && "$patch" =~ ^[0-9]+$ ]]; then
  if ((major > MIN_MAJOR || (major == MIN_MAJOR && (minor > MIN_MINOR || (minor == MIN_MINOR && patch >= MIN_PATCH))))); then
    supported=1
  fi
fi
if [ "$supported" -ne 1 ]; then
  echo "validate-tech-evaluation.sh ${NEEDED}; found '${version}'." >&2
  exit 1
fi

# --no-install: a script that imports a package must fail here, not fetch it from npm.
exec bun --no-install "${HERE}/validate-tech-evaluation.ts" "$@"
