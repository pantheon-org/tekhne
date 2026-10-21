#!/usr/bin/env bash
# Check that the pinned pantheon-skill-auditor release is the same in all three
# places that name it:
#   - mise.toml                                       (the version pin)
#   - mise.lock                                       (version and linux-x64 checksum)
#   - .github/actions/install-skill-auditor/action.yml (VERSION and SHA256, used in CI)
#
# CI installs the auditor through the action, local hooks through mise, so a pin
# bumped in only one place would run different scorers in different places.
#
# Usage:
#   scripts/stored-audits/check-auditor-pin.sh [--root DIR]
#
# Exit: 0 = all three agree; 1 = they differ (each difference is printed);
#       2 = a pin could not be read.
set -euo pipefail

root=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --root) root="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
root="${root:-$(git rev-parse --show-toplevel)}"

tool='github:pantheon-org/pantheon-skill-auditor'
mise_toml="$root/mise.toml"
mise_lock="$root/mise.lock"
action="$root/.github/actions/install-skill-auditor/action.yml"

for f in "$mise_toml" "$mise_lock" "$action"; do
  [ -f "$f" ] || { echo "missing file: ${f#"$root"/}" >&2; exit 2; }
done

# mise.toml: "github:pantheon-org/pantheon-skill-auditor" = "0.3.1"
toml_version="$(sed -n "s|^\"$tool\" = \"\\(.*\\)\"\$|\\1|p" "$mise_toml" | head -1)"

# mise.lock: the version in the tool's table, and the checksum in its linux-x64 table.
lock_version="$(awk -v t="[[tools.\"$tool\"]]" '
  $0 == t { in_tool = 1; next }
  in_tool && /^version = / { gsub(/version = |"/, ""); print; exit }
  in_tool && /^\[\[/ { exit }' "$mise_lock")"
lock_sha="$(awk -v t="[tools.\"$tool\".\"platforms.linux-x64\"]" '
  $0 == t { in_platform = 1; next }
  in_platform && /^checksum = / { gsub(/checksum = |"|sha256:/, ""); print; exit }
  in_platform && /^\[/ { exit }' "$mise_lock")"

# action.yml: VERSION: "0.3.1" and SHA256: "<hex>"
action_version="$(sed -n 's|^ *VERSION: "\(.*\)"$|\1|p' "$action" | head -1)"
action_sha="$(sed -n 's|^ *SHA256: "\(.*\)"$|\1|p' "$action" | head -1)"

for pair in "mise.toml version:$toml_version" "mise.lock version:$lock_version" \
  "mise.lock linux-x64 checksum:$lock_sha" "install action VERSION:$action_version" \
  "install action SHA256:$action_sha"; do
  if [ -z "${pair#*:}" ]; then
    echo "could not read: ${pair%%:*}" >&2
    exit 2
  fi
done

rc=0
if [ "$toml_version" != "$lock_version" ]; then
  echo "version differs: mise.toml has $toml_version, mise.lock has $lock_version" >&2
  rc=1
fi
if [ "$toml_version" != "$action_version" ]; then
  echo "version differs: mise.toml has $toml_version, the install action has $action_version" >&2
  rc=1
fi
if [ "$lock_sha" != "$action_sha" ]; then
  echo "checksum differs: mise.lock has $lock_sha, the install action has $action_sha" >&2
  rc=1
fi

if [ "$rc" -eq 0 ]; then
  echo "Auditor pin is consistent: $toml_version ($lock_sha)."
fi
exit "$rc"
