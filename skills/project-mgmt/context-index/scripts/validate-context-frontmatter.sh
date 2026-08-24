#!/usr/bin/env sh
# validate-context-frontmatter.sh - validate YAML frontmatter in
# .context/**/*.md files against the schema create-context-file's generator
# writes: title, type (enum), date (YYYY-MM-DD), status (enum), tags (array).
#
# Deliberately standalone (no dependency on create-context-file's schema
# file) so this skill validates correctly even when create-context-file
# isn't installed alongside it. Keep the enums below in sync with
# create-context-file's KNOWN_TYPES / singular_of() if that set changes.
#
# Usage: validate-context-frontmatter.sh <file> [<file> ...]
set -eu

python3 - "$@" <<'PYEOF'
import sys
import re
from pathlib import Path

files = sys.argv[1:]
if not files:
    sys.exit(0)

TYPE_ENUM = {
    "finding", "plan", "guide", "follow-up", "merge-request",
    "ticket", "decision", "note", "research",
}
STATUS_ENUM = {"active", "done"}
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
REQUIRED = ["title", "type", "date", "status"]

errors = []

for f in files:
    p = Path(f)
    if not p.exists():
        continue
    content = p.read_text()
    if not content.startswith("---\n"):
        errors.append(f"{f}: missing frontmatter (file must begin with ---)")
        continue
    try:
        end = content.index("---\n", 4)
    except ValueError:
        errors.append(f"{f}: unclosed frontmatter (no closing ---)")
        continue
    fm_text = content[4:end]
    fm = {}
    for line in fm_text.splitlines():
        if ": " in line and not line.startswith(" "):
            k, _, v = line.partition(": ")
            fm[k.strip()] = v.strip().strip('"')

    for field in REQUIRED:
        if not fm.get(field):
            errors.append(f"{f}: missing required field '{field}'")

    if "type" in fm and fm["type"] not in TYPE_ENUM:
        errors.append(
            f"{f}: 'type' must be one of {sorted(TYPE_ENUM)}, got '{fm['type']}' "
            "(a one-off typology from --allow-new-type intentionally falls outside this list)"
        )

    if "status" in fm and fm["status"] not in STATUS_ENUM:
        errors.append(f"{f}: 'status' must be one of {sorted(STATUS_ENUM)}, got '{fm['status']}'")

    if "date" in fm and fm["date"] and not DATE_RE.match(fm["date"]):
        errors.append(f"{f}: 'date' must match YYYY-MM-DD, got '{fm['date']}'")

    if "tags:" not in fm_text:
        errors.append(f"{f}: missing 'tags' key (use 'tags: []' when there are none)")

if errors:
    print("Frontmatter validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against schema")
PYEOF
