#!/usr/bin/env bash
# shell: bash
# regenerate-context-index.sh - rebuild .context/index.yaml from the YAML
# frontmatter of every .context/**/*.md file.
#
# The index is a generated cache grouped by typology (the plural directory
# name a file lives under: findings/, plans/, guides/, follow-ups/,
# merge-requests/, tickets/, decisions/, notes/, research/). The source of
# truth is always the frontmatter in each .md file, never this output.
#
# Usage:
#   regenerate-context-index.sh          # write .context/index.yaml
#   regenerate-context-index.sh --check  # verify it is already up to date
set -euo pipefail

CHECK_MODE=false
if [ "${1:-}" = "--check" ]; then
  CHECK_MODE=true
fi

ROOT="$(git rev-parse --show-toplevel)"
INDEX="$ROOT/.context/index.yaml"

python3 - "$ROOT" "$INDEX" "$CHECK_MODE" <<'PYEOF'
import sys
import json
import re
from pathlib import Path
from datetime import date

root = Path(sys.argv[1])
index_path = Path(sys.argv[2])
check_mode = sys.argv[3] == "true"
context_dir = root / ".context"


def unquote(v):
    # Frontmatter values may use either YAML quoting style. Stripping only '"'
    # left single-quoted values carrying their quotes plus any inner '"', which
    # then broke the emitted index (see emit_scalar).
    v = v.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        inner = v[1:-1]
        if v[0] == "'":
            return inner.replace("''", "'")
        return inner
    return v


def emit_scalar(v):
    # A double-quoted YAML scalar built by naive f-string interpolation breaks
    # on any value containing '"'. JSON string syntax is a valid YAML 1.2
    # double-quoted scalar and escapes what needs escaping; ensure_ascii=False
    # keeps em dashes and other non-ASCII readable.
    return json.dumps(v, ensure_ascii=False)


def parse_list_block(raw, key):
    # Matches a block-style YAML list under `key:`, e.g.:
    #   tags:
    #     - auth
    #     - oauth
    # Does not match flow-style `key: []` or `key: [a, b]` - callers treat an
    # absent block as "no items" either way.
    m = re.search(rf"^{re.escape(key)}:\n((?:  - .+\n?)+)", raw, re.MULTILINE)
    if not m:
        return []
    return [ln.strip()[2:].strip() for ln in m.group(1).splitlines() if ln.strip().startswith("- ")]


def parse_frontmatter(text):
    if not text.startswith("---\n"):
        return None
    try:
        end = text.index("---\n", 4)
    except ValueError:
        return None
    fm_text = text[4:end]
    fm = {}
    for line in fm_text.splitlines():
        if ": " in line and not line.startswith(" "):
            k, _, v = line.partition(": ")
            fm[k.strip()] = unquote(v)
    fm["_raw"] = fm_text
    return fm


entries = []
missing = []

for md in sorted(context_dir.rglob("*.md")):
    rel = str(md.relative_to(root))
    content = md.read_text()
    fm = parse_frontmatter(content)
    if fm is None:
        missing.append(rel)
        continue
    required = ["title", "type", "status", "date"]
    absent = [f for f in required if not fm.get(f)]
    if absent:
        missing.append(f"{rel} (missing: {', '.join(absent)})")
        continue
    entry = {k: fm[k] for k in required}
    entry["path"] = rel
    tags = parse_list_block(fm["_raw"], "tags")
    if tags:
        entry["tags"] = tags
    related = parse_list_block(fm["_raw"], "related")
    if related:
        entry["related"] = related
    entries.append(entry)

if missing:
    print(
        "WARNING: files with missing frontmatter (excluded from index):", file=sys.stderr
    )
    for f in missing:
        print(f"  {f}", file=sys.stderr)

# The plural directory name a file lives under is the typology; the file's
# own `type:` frontmatter field is the singular form of that same typology
# (findings/ -> finding, follow-ups/ -> follow-up, research/ -> research).
# This mapping must stay in sync with singular_of() in create-context-file's
# scripts/create-context-file.sh.
type_group_key = {
    "finding": "findings",
    "plan": "plans",
    "guide": "guides",
    "follow-up": "follow-ups",
    "merge-request": "merge-requests",
    "ticket": "tickets",
    "decision": "decisions",
    "note": "notes",
    "research": "research",
}
type_order = [
    "findings",
    "plans",
    "guides",
    "follow-ups",
    "merge-requests",
    "tickets",
    "decisions",
    "notes",
    "research",
]
type_label = {
    "findings": "Findings",
    "plans": "Plans",
    "guides": "Guides",
    "follow-ups": "Follow-ups",
    "merge-requests": "Merge Requests",
    "tickets": "Tickets",
    "decisions": "Decisions",
    "notes": "Notes",
    "research": "Research",
}

# A file's `type:` should always match the typology its own directory
# implies - the directory already says what it should be, so any
# disagreement is a mistake in the file, not a legitimate new typology.
# Silently routing it into the `other` catch-all is how such mistakes go
# unnoticed. Warn loudly instead of swallowing it.
dir_to_type = {v: k for k, v in type_group_key.items()}
mismatches = []
for e in entries:
    parts = e["path"].split("/")
    if len(parts) < 3 or parts[0] != ".context":
        continue
    actual_dir = parts[1]
    expected_type = dir_to_type.get(actual_dir)
    if expected_type is None:
        # Directories outside the curated typology set (e.g. one-off folders
        # created with create-context-file's --allow-new-type, or content
        # belonging to an unrelated tool) are not an error - they simply fall
        # into the "other" catch-all below.
        continue
    if e["type"] != expected_type:
        mismatches.append(
            f"{e['path']}: type: {e['type']!r} does not match its directory '{actual_dir}/' (expected type: {expected_type!r})"
        )

if mismatches:
    print("WARNING: type/directory mismatches (entry kept, but check the source file):", file=sys.stderr)
    for m in mismatches:
        print(f"  {m}", file=sys.stderr)

grouped = {}
for e in entries:
    key = type_group_key.get(e["type"], "other")
    grouped.setdefault(key, []).append(e)
other = grouped.pop("other", [])
for t in list(grouped):
    if t not in type_order:
        other.extend(grouped.pop(t))
if other:
    grouped["other"] = other
    type_order.append("other")
    type_label["other"] = "Other"

status_counts = {}
for e in entries:
    s = e["status"]
    status_counts[s] = status_counts.get(s, 0) + 1
status_summary = ", ".join(f"{n} {s}" for s, n in sorted(status_counts.items()))

lines = [
    "# Auto-generated by context-index skill. Do not edit manually.",
    f"# Last updated: {date.today().isoformat()}",
    f"# {len(entries)} entries: {status_summary}",
    "",
]
for t in type_order:
    lst = grouped.get(t)
    if not lst:
        continue
    lines.append(f"# ── {type_label[t]} ({len(lst)}) ──")
    lines.append(f"{t}:")
    for e in lst:
        lines.append(f"  - path: {emit_scalar(e['path'])}")
        lines.append(f"    title: {emit_scalar(e['title'])}")
        lines.append(f"    status: {emit_scalar(e['status'])}")
        lines.append(f"    date: {e['date']}")
        if e.get("tags"):
            lines.append("    tags:")
            for tag in e["tags"]:
                lines.append(f"      - {emit_scalar(tag)}")
        if e.get("related"):
            lines.append("    related:")
            for r in e["related"]:
                lines.append(f"      - {emit_scalar(r)}")
    lines.append("")

output = "\n".join(lines) + "\n"
if check_mode:
    current = index_path.read_text() if index_path.exists() else ""
    if output != current:
        print("ERROR: .context/index.yaml is stale — regenerate it and commit the result", file=sys.stderr)
        sys.exit(1)
    print("context index is fresh")
else:
    index_path.parent.mkdir(parents=True, exist_ok=True)
    index_path.write_text(output)
    print(f"Generated {len(entries)} entries -> {index_path}")
PYEOF
