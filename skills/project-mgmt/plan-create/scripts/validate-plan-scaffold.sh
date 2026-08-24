#!/usr/bin/env bash
# shell: bash
# Validates a plan scaffold YAML against the plan-scaffold schema.
# Usage: validate-plan-scaffold.sh <file> [<file> ...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/plan-scaffold.schema.json"

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
plan_props = schema.get("properties", {}).get("plan", {}).get("properties", {})

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
        errors.append(f"{f}: PyYAML not available, skipping")
        continue

    plan = data.get("plan")
    if not plan:
        errors.append(f"{f}: missing required top-level key 'plan'")
        continue

    required = ["filename", "frontmatter", "goal", "phases", "open_questions"]
    for field in required:
        if field not in plan:
            errors.append(f"{f}: plan missing required field '{field}'")

    fm = plan.get("frontmatter", {})
    for field in ["title", "type", "status", "date"]:
        if field not in fm:
            errors.append(f"{f}: frontmatter missing '{field}'")

    phases = plan.get("phases", [])
    if not phases:
        errors.append(f"{f}: must have at least 1 phase")
    for i, phase in enumerate(phases):
        if not phase.get("name"):
            errors.append(f"{f}: phase {i+1} missing 'name'")
        tasks = phase.get("tasks", [])
        if not tasks:
            errors.append(f"{f}: phase '{phase.get('name', i+1)}' has no tasks")
        if len(tasks) > 8:
            errors.append(f"{f}: phase '{phase.get('name', i+1)}' has {len(tasks)} tasks (max 8)")

    oq = plan.get("open_questions", [])
    if not oq:
        errors.append(f"{f}: open_questions is empty — add at least 1")

if errors:
    print("Plan scaffold validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against plan-scaffold schema")
PYEOF
