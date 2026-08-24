#!/usr/bin/env bash
# shell: bash
# Validate a ticket-detail journal entry (journal-entry-import Mode B).
#
# Runs the base journal validator, then validates the entry's frontmatter and
# structure against
# skills/documentation/journal-entry-import/assets/schemas/ticket-detail-frontmatter.schema.json
# (schema-driven: required / enum / pattern / x-forbidden-fields / x-required-sections /
# x-h1-pattern / x-decisive-contribution-pattern).
#
# Usage: validate-ticket-detail.sh <file.md> [more.md ...]
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$SCRIPT_DIR/../../../.." 2>/dev/null && pwd)}"
SCHEMA="$ROOT/skills/documentation/journal-entry-import/assets/schemas/ticket-detail-frontmatter.schema.json"
BASE_VALIDATOR="$ROOT/skills/documentation/journal-entry-creator/scripts/validate-journal-entry.sh"

if [[ $# -eq 0 ]]; then
  echo "Usage: $(basename "$0") <file.md> [more.md ...]" >&2
  exit 2
fi

rc=0

# 1) Base journal validation (single dated H1, Session Overview/Compliance/Tags, tags match, code fences).
if [[ -f "$BASE_VALIDATOR" ]]; then
  for f in "$@"; do
    if ! bash "$BASE_VALIDATOR" "$f" >/dev/null 2>&1; then
      bash "$BASE_VALIDATOR" "$f" >&2 || true
      echo "Invalid: base validator failed for $f" >&2
      rc=1
    fi
  done
fi

# 2) Ticket-detail frontmatter + structure, driven by the JSON schema.
if ! python3 - "$SCHEMA" "$@" <<'PYEOF'
import sys, json, re
from pathlib import Path

schema = json.loads(Path(sys.argv[1]).read_text())
files = sys.argv[2:]
required = schema.get("required", [])
props = schema.get("properties", {})
enum_fields = {k: v["enum"] for k, v in props.items() if "enum" in v}
pattern_fields = {k: re.compile(v["pattern"]) for k, v in props.items() if "pattern" in v}
forbidden = schema.get("x-forbidden-fields", [])
req_sections = schema.get("x-required-sections", [])
h1_pattern = re.compile(schema["x-h1-pattern"]) if "x-h1-pattern" in schema else None
dc_pattern = re.compile(schema["x-decisive-contribution-pattern"]) if "x-decisive-contribution-pattern" in schema else None

errors = []
for f in files:
    p = Path(f)
    if not p.exists():
        errors.append(f"{f}: file not found")
        continue
    content = p.read_text()
    lines = content.splitlines()

    # Frontmatter block.
    if not content.startswith("---\n"):
        errors.append(f"{f}: missing frontmatter")
        continue
    try:
        end = content.index("\n---", 4)
    except ValueError:
        errors.append(f"{f}: unclosed frontmatter")
        continue
    fm_text = content[4:end]
    fm = {}
    for line in fm_text.splitlines():
        if ": " in line and not line.startswith(" ") and not line.startswith("-"):
            k, _, v = line.partition(": ")
            fm[k.strip()] = v.strip().strip('"')

    for field in required:
        if field not in fm and not re.search(rf"^{re.escape(field)}:", fm_text, re.M):
            errors.append(f"{f}: missing required frontmatter field '{field}'")
    for field, values in enum_fields.items():
        if field in fm and fm[field] not in values:
            errors.append(f"{f}: '{field}' must be one of {values}, got '{fm[field]}'")
    for field, pat in pattern_fields.items():
        if field in fm and not pat.search(fm[field]):
            errors.append(f"{f}: '{field}' does not match {pat.pattern!r}, got '{fm[field]}'")
    for field in forbidden:
        if re.search(rf"^{re.escape(field)}:", fm_text, re.M):
            errors.append(f"{f}: forbidden frontmatter field '{field}' present")

    # H1 shape.
    h1s = [ln for ln in lines if ln.startswith("# ")]
    if h1_pattern:
        if len(h1s) != 1:
            errors.append(f"{f}: expected exactly one H1, found {len(h1s)}")
        elif not h1_pattern.match(h1s[0]):
            errors.append(f"{f}: H1 must match '# <KEY> - <summary> - <Month D, YYYY>' (got: {h1s[0]})")

    # Required sections.
    heading_set = set(ln.rstrip() for ln in lines)
    for sec in req_sections:
        if sec not in heading_set:
            errors.append(f"{f}: missing required section '{sec}'")

    # Decisive-contribution callout form (only if such a line exists).
    if dc_pattern and "Decisive contribution" in content:
        if not any(dc_pattern.match(ln) for ln in lines):
            errors.append(f"{f}: decisive-contribution note must use '> **Decisive contribution (<name>):**' form")

if errors:
    print("Ticket-detail validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)
print(f"OK: {len(files)} file(s) pass ticket-detail checks")
PYEOF
then
  rc=1
fi

exit "$rc"
