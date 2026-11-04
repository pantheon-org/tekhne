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
import re
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

    # The fields the plans index check reads. Enforced here, not left to the schema alone,
    # because this script only reads the schema's top-level required list.
    plan = report.get("plan")
    if "plan" in report and (not isinstance(plan, str) or not plan.strip()):
        errors.append(f"{f}: report 'plan' must be a non-empty string")

    lenses_allowed = props.get("lenses_run", {}).get("items", {}).get("enum", [])
    lenses = report.get("lenses_run")
    if "lenses_run" in report:
        if not isinstance(lenses, list) or not lenses:
            errors.append(f"{f}: report 'lenses_run' must be a non-empty list")
        else:
            bad = [x for x in lenses if x not in lenses_allowed]
            if bad:
                errors.append(f"{f}: report 'lenses_run' has {bad}, allowed {lenses_allowed}")
            if len(set(map(str, lenses))) != len(lenses):
                errors.append(f"{f}: report 'lenses_run' lists a lens more than once")
            if len(lenses) > len(lenses_allowed):
                errors.append(f"{f}: report 'lenses_run' has more than {len(lenses_allowed)} entries")

    plan_hash = report.get("plan_hash")
    if "plan_hash" in report and not (isinstance(plan_hash, str) and re.fullmatch(r"[0-9a-f]{64}", plan_hash)):
        errors.append(f"{f}: report 'plan_hash' must be 64 lowercase hex characters (shasum -a 256)")

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
