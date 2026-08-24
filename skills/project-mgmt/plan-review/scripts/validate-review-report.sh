#!/usr/bin/env bash
# shell: bash
# Validates a generated review report YAML against the review-report schema.
# Usage: validate-review-report.sh <file> [<file> ...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/review-report.schema.json"

python3 - "$SCHEMA" "$@" <<'PYEOF'
import sys
import json
import yaml
from pathlib import Path

schema_path = Path(sys.argv[1])
files = sys.argv[2:]

if not files:
    sys.exit(0)

try:
    import yaml
    HAS_YAML = True
except ImportError:
    HAS_YAML = False

schema = json.loads(schema_path.read_text())

errors = []

for f in files:
    p = Path(f)
    if not p.exists():
        continue
    content = p.read_text()
    if HAS_YAML:
        try:
            data = yaml.safe_load(content)
        except yaml.YAMLError as e:
            errors.append(f"{f}: invalid YAML — {e}")
            continue
    else:
        errors.append(f"{f}: PyYAML not available, skipping YAML parse")
        continue

    if not isinstance(data, dict):
        errors.append(f"{f}: root must be a mapping (got {type(data).__name__})")
        continue

    report = data.get("report")
    if not report:
        errors.append(f"{f}: missing required top-level key 'report'")
        continue

    required = schema.get("properties", {}).get("report", {}).get("required", [])
    props = schema.get("properties", {}).get("report", {}).get("properties", {})

    for field in required:
        if field not in report:
            errors.append(f"{f}: report missing required field '{field}'")

    sections = report.get("sections", {})
    if not sections:
        errors.append(f"{f}: report missing 'sections'")

    sec_required = props.get("sections", {}).get("required", [])
    for field in sec_required:
        if field not in sections:
            errors.append(f"{f}: sections missing required field '{field}'")

if errors:
    print("Review report validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against review-report schema")
PYEOF
