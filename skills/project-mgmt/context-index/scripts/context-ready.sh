#!/usr/bin/env bash
# shell: bash
# context-ready.sh - list every ACTIVE .context/ file with zero open
# blockers, computed from the `blocked-by` frontmatter field that
# regenerate-context-index.sh carries into .context/index.yaml.
#
# Deliberately a query, not a gate: it only reads the index and never writes
# anything, so it is safe to run at any time with no side effects.
#
# A blocker is "open" when the referenced file is missing from the index
# entirely, or present with any status other than `done`. Resolution fails
# closed (an unresolvable blocker counts as still-blocking) rather than
# silently treating a bad reference as satisfied.
#
# `blocked-by` values are resolved the same way `related` already is:
# relative to the REFERENCING file's own directory, not the repo root.
#
# Usage:
#   context-ready.sh            # list active items with zero open blockers
#   context-ready.sh --blocked  # list active items that are NOT ready, with
#                                # each open blocker named
set -euo pipefail

MODE="ready"
if [ "${1:-}" = "--blocked" ]; then
  MODE="blocked"
elif [ "${1:-}" != "" ]; then
  echo "usage: context-ready.sh [--blocked]" >&2
  exit 2
fi

ROOT="$(git rev-parse --show-toplevel)"
INDEX="$ROOT/.context/index.yaml"

if [ ! -f "$INDEX" ]; then
  echo "NOTICE: $INDEX not found -- run regenerate-context-index.sh first" >&2
  exit 0
fi

# Freshness nudge (advisory only, same spirit as the follow-up skill's Rule
# 4): a stale index would silently under- or over-report open blockers. This
# script never regenerates the index itself -- it only reads it, matching
# every sibling script in this family.
ACTUAL_COUNT="$(find "$ROOT/.context" -name '*.md' | wc -l | tr -d ' ')"
HEADER_COUNT="$(sed -n 's/^# \([0-9]\{1,\}\) entries:.*/\1/p' "$INDEX" | head -1)"
if [ -n "$HEADER_COUNT" ] && [ "$HEADER_COUNT" != "$ACTUAL_COUNT" ]; then
  echo "WARNING: index reports $HEADER_COUNT entries but $ACTUAL_COUNT .md file(s) exist under .context/ -- it may be stale. Run regenerate-context-index.sh first." >&2
fi

python3 - "$INDEX" "$MODE" <<'PYEOF'
import re
import sys
from pathlib import Path
from posixpath import normpath

index_path = Path(sys.argv[1])
mode = sys.argv[2]
text = index_path.read_text()

# Tolerant line-based parse of the machine-generated index, the same
# approach check-plan-staleness.sh already uses -- no YAML library needed
# for a shape this script fully controls.
entries = []
current_section = None
current = {}
list_key = None  # tags | related | blocks | blocked-by | None


def flush():
    global current
    if current:
        entries.append((current_section, current))
        current = {}


for line in text.splitlines():
    section_match = re.match(r"^([a-z-]+):\s*$", line)
    if section_match:
        flush()
        current_section = section_match.group(1)
        list_key = None
        continue
    if current_section is None:
        continue
    m = re.match(r"^\s*- path: (.+)$", line)
    if m:
        flush()
        current = {"path": m.group(1).strip().strip('"')}
        list_key = None
        continue
    m = re.match(r"^\s*title: (.+)$", line)
    if m:
        current["title"] = m.group(1).strip().strip('"')
        continue
    m = re.match(r"^\s*status: (.+)$", line)
    if m:
        current["status"] = m.group(1).strip().strip('"')
        continue
    m = re.match(r"^\s*date: (\S+)", line)
    if m:
        current["date"] = m.group(1)
        continue
    m = re.match(r"^\s*(tags|related|blocks|blocked-by):\s*$", line)
    if m:
        list_key = m.group(1)
        current.setdefault(list_key, [])
        continue
    m = re.match(r"^\s*- (.+)$", line)
    if m and list_key:
        current[list_key].append(m.group(1).strip().strip('"'))
        continue
flush()

by_path = {e["path"]: e for _, e in entries if "path" in e}


def resolve(from_path, ref):
    base_dir = str(Path(from_path).parent)
    return normpath(f"{base_dir}/{ref}")


active = [(section, e) for section, e in entries if e.get("status") == "active"]

ready = []
blocked = []
for section, e in active:
    open_blockers = []
    for ref in e.get("blocked-by", []):
        target_path = resolve(e["path"], ref)
        target = by_path.get(target_path)
        if target is None:
            open_blockers.append(f"{ref} (not found in index at {target_path} -- treated as open)")
        elif target.get("status") != "done":
            open_blockers.append(f"{ref} (status: {target.get('status', 'unknown')})")
    if open_blockers:
        blocked.append((section, e, open_blockers))
    else:
        ready.append((section, e))

if mode == "ready":
    if not ready:
        print(
            "No active items are ready -- either nothing is active, or every "
            "active item has an open blocker (see: context-ready.sh --blocked)."
        )
    else:
        print(f"{len(ready)} ready item(s) (active, zero open blockers):")
        for section, e in sorted(ready, key=lambda x: x[1]["path"]):
            print(f"  [{section}] {e['path']}: \"{e.get('title', '')}\"")
else:
    if not blocked:
        print("No active items have an open blocker.")
    else:
        print(f"{len(blocked)} active item(s) with an open blocker:")
        for section, e, open_blockers in sorted(blocked, key=lambda x: x[1]["path"]):
            print(f"  [{section}] {e['path']}: \"{e.get('title', '')}\"")
            for ob in open_blockers:
                print(f"      blocked by: {ob}")
PYEOF
