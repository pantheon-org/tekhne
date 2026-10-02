#!/usr/bin/env bash
#
# check.sh
#
# Check that each named skill's latest stored audit still matches a fresh
# evaluation, and tell the author exactly how to fix it when it does not. Shared
# by the pre-push hook and CI so both give the same answer and the same advice.
#
# Usage:
#   scripts/stored-audits/check.sh [--root DIR] <skill>...
#   scripts/stored-audits/check.sh [--root DIR] --from-file FILE   one skill per line
#   scripts/stored-audits/check.sh [--root DIR] --all              every skill under skills/
#
# Skills are keys relative to skills/ (for example ci-cd/helm/validator).
# The auditor binary is $AUDITOR, default <root>/target/release/pantheon-skill-auditor.
#
# Exit: 0 = all current (or nothing to check); 1 = stale or missing audit(s);
#       2 = the check itself could not run.
set -uo pipefail

root=""
from_file=""
all=0
skills=()

while [ $# -gt 0 ]; do
  case "$1" in
    --root) root="$2"; shift 2 ;;
    --from-file) from_file="$2"; shift 2 ;;
    --all) all=1; shift ;;
    *) skills+=("$1"); shift ;;
  esac
done

root="${root:-$(git rev-parse --show-toplevel)}"
auditor="${AUDITOR:-$root/target/release/pantheon-skill-auditor}"

if [ "$all" -eq 1 ]; then
  while IFS= read -r skill_md; do
    dir="$(dirname "$skill_md")"
    skills+=("${dir#"$root"/skills/}")
  done < <(find "$root/skills" -name SKILL.md | sort)
fi

if [ -n "$from_file" ]; then
  while IFS= read -r line; do
    [ -n "$line" ] && skills+=("$line")
  done < "$from_file"
fi

if [ "${#skills[@]}" -eq 0 ]; then
  echo "No skills to check."
  exit 0
fi

if [ ! -x "$auditor" ]; then
  echo "Could not check stored audits: auditor not found at $auditor." >&2
  echo "Build it with: cargo build --release -p pantheon-skill-auditor" >&2
  exit 2
fi

err_file="$(mktemp)"
trap 'rm -f "$err_file"' EXIT

output="$("$auditor" check-stored "${skills[@]}" 2>"$err_file")"
rc=$?

case "$rc" in
  0)
    echo "Stored audits are current for ${#skills[@]} skill(s)."
    exit 0
    ;;
  1)
    echo "$output"
    stale="$(echo "$output" | awk '/^STALE /{ sub(/:$/, "", $2); print $2 }' | tr '\n' ' ')"
    echo
    echo "Stored audits are out of date. Refresh them, then commit the new .audits folders:"
    echo
    # Show the binary relative to the repo when it lives inside it, so the
    # advice carries no local path.
    echo "  ${auditor#"$root"/} batch ${stale% } --store"
    exit 1
    ;;
  *)
    echo "Could not check stored audits (auditor exit $rc)." >&2
    cat "$err_file" >&2
    exit 2
    ;;
esac
