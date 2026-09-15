#!/usr/bin/env bash
# shell: bash
# Scans a planning-document directory (default: .context) for decision
# indicators and cross-references them against the `related` lists in existing
# ADRs' YAML frontmatter (the provenance convention documented in
# references/context-extraction.md). Reports any planning document that looks
# like it contains a decision but has no record pointing back at it.
#
# Usage: check-undocumented-decisions.sh [--adr-dir DIR] [--source-dir DIR]
#   --adr-dir DIR      Where records live (default: $ADR_DIR, else docs/adr,
#                       matching pantheon-adr's own resolution order). This
#                       directory is excluded from the planning-document scan.
#   --source-dir DIR   Where planning documents live (default: .context).
set -euo pipefail

ADR_DIR="${ADR_DIR:-docs/adr}"
SOURCE_DIR=".context"

while [ $# -gt 0 ]; do
  case "$1" in
    --adr-dir)
      if [ $# -lt 2 ]; then
        echo "--adr-dir requires a value" >&2
        exit 2
      fi
      ADR_DIR="$2"
      shift 2
      ;;
    --source-dir)
      if [ $# -lt 2 ]; then
        echo "--source-dir requires a value" >&2
        exit 2
      fi
      SOURCE_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

ROOT="$(git rev-parse --show-toplevel)"

if [ ! -d "$ROOT/$SOURCE_DIR" ]; then
  echo "No $SOURCE_DIR directory found under $ROOT - nothing to scan."
  exit 0
fi

python3 - "$ROOT" "$ADR_DIR" "$SOURCE_DIR" <<'PYEOF'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
adr_dir = (root / sys.argv[2]).resolve()
source_dir = root / sys.argv[3]

FRONTMATTER = re.compile(r"\A---\r?\n(.*?)\r?\n---\r?\n", re.DOTALL)
RELATED_ITEM = re.compile(r"^\s*-\s*(.+?)\s*$")


def read_utf8(path):
    """Read a markdown file as UTF-8.

    `Path.read_text()` with no encoding uses the locale's, which on Windows is
    the ANSI code page and raises UnicodeDecodeError on any non-ASCII character.
    Markdown here is always UTF-8, so say so.
    """
    return path.read_text(encoding="utf-8")


def related_paths(record):
    """Every existing path listed under a record's frontmatter `related` key.

    Parsed line by line rather than with a YAML library so the script stays
    dependency-free; `related` is a flat list of scalars by construction.
    """
    match = FRONTMATTER.match(read_utf8(record))
    if not match:
        return
    in_related = False
    for line in match.group(1).splitlines():
        if re.match(r"^related:\s*(\[\s*\])?\s*$", line):
            in_related = True
            continue
        if in_related:
            item = RELATED_ITEM.match(line)
            if item:
                value = item.group(1).strip().strip('"').strip("'")
                resolved = (record.parent / value).resolve()
                if resolved.exists():
                    yield str(resolved)
                continue
            # Any other key at column zero ends the list.
            if line and not line[0].isspace():
                in_related = False


referenced = set()
if adr_dir.is_dir():
    for record in adr_dir.glob("*.md"):
        referenced.update(related_paths(record))

# Decision-indicating signals, matching the guidance in
# references/context-extraction.md's "Recognising a binding decision" table.
DECISION_KEYWORDS = [
    r"^##\s*Decision\b",
    r"^###\s*Chosen Solution\b",
    r"^##\s*Recommendation\b",
    r"^##\s*Recommended Approach\b",
    r"^##\s*Proposed Approach\b",
    r"\*\*Decision:\*\*",
    r"\*\*Recommendation:\*\*",
    r"\bWe will\b",
    r"\bAdopt Option\b",
    r"\bGoing forward, we\b",
]

undocumented = []

for md_file in sorted(source_dir.rglob("*.md")):
    resolved = md_file.resolve()
    if str(resolved) in referenced:
        continue  # already linked from a record
    if adr_dir == resolved.parent or adr_dir in resolved.parents:
        continue  # the records themselves are not planning documents

    content = read_utf8(md_file)
    found = next(
        (kw for kw in DECISION_KEYWORDS if re.search(kw, content, re.MULTILINE)),
        None,
    )
    if found:
        undocumented.append((str(md_file.relative_to(root)), found))

if not undocumented:
    print("All planning documents with decision indicators are linked from a record.")
    sys.exit(0)

print("WARNING: the following documents contain decision indicators but are not")
print("listed under any record's `related` frontmatter. Consider extracting an ADR")
print("for each (see references/context-extraction.md):")
print()
for path, keyword in undocumented:
    print(f"  {path}")
    print(f"    Indicator: {keyword}")
    print()

print(f"Total: {len(undocumented)} undocumented decision(s)")
print("Run `pantheon-adr list` to see existing records before creating new ones.")
sys.exit(2)
PYEOF
