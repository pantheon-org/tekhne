#!/usr/bin/env bash
# shell: bash
# Scans a planning-document directory (default: .context) for decision
# indicators and cross-references them against the Source: links already
# recorded in existing ADRs (the provenance convention documented in
# references/context-extraction.md). Reports any planning document that
# looks like it contains a decision but has no ADR pointing back at it.
#
# Usage: check-undocumented-decisions.sh [--adr-dir DIR] [--source-dir DIR]
#   --adr-dir DIR      Where ADRs live (default: $ADR_DIR, else docs/adr,
#                       matching pantheon-adr's own resolve_dir default).
#   --source-dir DIR   Where planning documents live (default: .context).
set -euo pipefail

ADR_DIR="${ADR_DIR:-docs/adr}"
SOURCE_DIR=".context"

while [ $# -gt 0 ]; do
  case "$1" in
    --adr-dir)
      ADR_DIR="$2"
      shift 2
      ;;
    --source-dir)
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
import sys
import re
from pathlib import Path

root = Path(sys.argv[1])
adr_dir = root / sys.argv[2]
source_dir = root / sys.argv[3]

# --- Collect every source document already linked from an ADR's Source: line ---
referenced = set()

if adr_dir.is_dir():
    for adr_file in adr_dir.glob("*.md"):
        content = adr_file.read_text()
        for m in re.finditer(r"^\s*-\s*Source:\s*(.+)$", content, re.MULTILINE):
            path_val = m.group(1).strip().strip('"').strip("'")
            resolved = (root / path_val).resolve()
            if resolved.exists():
                referenced.add(str(resolved))

# --- Decision-indicating signals, matching the guidance in
# references/context-extraction.md's "Recognizing a binding decision" table ---
DECISION_KEYWORDS = [
    r"^##\s*Decision\b",
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
    resolved = str(md_file.resolve())
    if resolved in referenced:
        continue  # already linked from an ADR

    content = md_file.read_text()

    found_keyword = None
    for kw in DECISION_KEYWORDS:
        if re.search(kw, content, re.MULTILINE):
            found_keyword = kw
            break

    if found_keyword:
        rel = str(md_file.relative_to(root))
        undocumented.append((rel, found_keyword))

if not undocumented:
    print("All planning documents with decision indicators are linked from an ADR.")
    sys.exit(0)

print("WARNING: the following documents contain decision indicators but are")
print("not linked as a Source: from any ADR. Consider extracting an ADR for each")
print("(see references/context-extraction.md):")
print()
for path, keyword in undocumented:
    print(f"  {path}")
    print(f"    Indicator: {keyword}")
    print()

print(f"Total: {len(undocumented)} undocumented decision(s)")
print("Run `pantheon-adr list` to see existing ADRs before creating new ones.")
sys.exit(2)
PYEOF
