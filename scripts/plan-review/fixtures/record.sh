#!/usr/bin/env bash
# shell: bash
# Records the output of the python version of validate-review-report.sh for every invocation in invocations.txt.
# It runs the version at the pinned commit inside a container with python and PyYAML pinned (see Dockerfile), from this
# folder with relative paths, and writes expected/<name>.{stdout,stderr,exit}. Needs docker and git.
# Usage: scripts/plan-review/fixtures/record.sh
set -euo pipefail

PINNED_COMMIT="562cd973"
IMAGE="tekhne-pyyaml-recorder:6.0.2"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO="$(git -C "$HERE" rev-parse --show-toplevel)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

docker build -q -t "$IMAGE" "$HERE" >/dev/null
git -C "$REPO" archive "$PINNED_COMMIT" skills/project-mgmt/plan-review | tar -x -C "$WORK"
SKILL="$WORK/skills/project-mgmt/plan-review"

rm -rf "$HERE/expected"
mkdir -p "$HERE/expected"
while IFS='|' read -r name args; do
  [ -n "$name" ] || continue
  set +e
  # shellcheck disable=SC2086 # args are a space-separated list of fixture paths, split on purpose
  docker run --rm -v "$SKILL:/skill:ro" -v "$HERE:/work:ro" -w /work "$IMAGE" \
    bash /skill/scripts/validate-review-report.sh $args \
    >"$HERE/expected/$name.stdout" 2>"$HERE/expected/$name.stderr"
  echo "$?" >"$HERE/expected/$name.exit"
  set -e
done <"$HERE/invocations.txt"
echo "recorded $(find "$HERE/expected" -name '*.exit' | wc -l | tr -d ' ') invocations"
