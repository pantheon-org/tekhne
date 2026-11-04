#!/usr/bin/env bash
# shell: bash
# Records the output of the python version of a skill's validator for every invocation in a fixtures folder.
# Usage: scripts/validator-recording/record.sh <skill folder under skills/project-mgmt> <script name> <fixtures folder>
#   e.g. scripts/validator-recording/record.sh plan-create validate-plan-scaffold scripts/plan-create/fixtures
# It runs the script at the pinned commit inside the container of the Dockerfile beside this file, from the fixtures
# folder with relative paths, and writes expected/<name>.{stdout,stderr,exit}. Needs docker and git.
set -euo pipefail

PINNED_COMMIT="9fdd876c"
IMAGE="tekhne-validator-recorder:pyyaml-6.0.2"
SKILL_NAME="${1:?skill folder}"
SCRIPT_NAME="${2:?script name}"
FIXTURES_ARG="${3:?fixtures folder}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO="$(git -C "$HERE" rev-parse --show-toplevel)"
FIXTURES="$(cd "$REPO/$FIXTURES_ARG" && pwd -P)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

docker build -q -t "$IMAGE" "$HERE" >/dev/null
git -C "$REPO" archive "$PINNED_COMMIT" "skills/project-mgmt/$SKILL_NAME" | tar -x -C "$WORK"
SKILL="$WORK/skills/project-mgmt/$SKILL_NAME"

rm -rf "$FIXTURES/expected"
mkdir -p "$FIXTURES/expected"
while IFS='|' read -r name args; do
  [ -n "$name" ] || continue
  set +e
  # shellcheck disable=SC2086 # args are a space-separated list of fixture paths, split on purpose
  docker run --rm -v "$SKILL:/skill:ro" -v "$FIXTURES:/work:ro" -w /work "$IMAGE" \
    bash "/skill/scripts/$SCRIPT_NAME.sh" $args \
    >"$FIXTURES/expected/$name.stdout" 2>"$FIXTURES/expected/$name.stderr"
  echo "$?" >"$FIXTURES/expected/$name.exit"
  set -e
done <"$FIXTURES/invocations.txt"
echo "recorded $(find "$FIXTURES/expected" -name '*.exit' | wc -l | tr -d ' ') invocations"
