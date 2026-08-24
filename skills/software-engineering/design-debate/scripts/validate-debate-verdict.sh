#!/usr/bin/env bash
# shell: bash
# Validates a filled-in debate verdict YAML against the debate-verdict schema.
# Usage: validate-debate-verdict.sh <file> [<file> ...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/debate-verdict.schema.json"

python3 - "$SCHEMA" "$@" <<'PYEOF'
import sys
import json
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
required = schema.get("required", [])
role_enum = schema["properties"]["roles"]["items"]["properties"]["role"]["enum"]
verdict_enum = schema["properties"]["verdict"]["enum"]

errors = []

for f in files:
    p = Path(f)
    if not p.exists():
        continue
    content = p.read_text()
    if not HAS_YAML:
        errors.append(f"{f}: PyYAML not available, cannot validate")
        continue
    try:
        data = yaml.safe_load(content)
    except yaml.YAMLError as e:
        errors.append(f"{f}: invalid YAML — {e}")
        continue

    if not isinstance(data, dict):
        errors.append(f"{f}: root must be a mapping (got {type(data).__name__})")
        continue

    for field in required:
        if not data.get(field):
            errors.append(f"{f}: missing required field '{field}'")

    roles = data.get("roles", [])
    if len(roles) < 2:
        errors.append(f"{f}: 'roles' must have at least 2 entries (advocate + skeptic)")
    role_names = set()
    for r in roles:
        rn = r.get("role")
        if rn not in role_enum:
            errors.append(f"{f}: role must be one of {role_enum}, got '{rn}'")
        role_names.add(rn)
        for rf in ("summary", "key_argument"):
            if not r.get(rf):
                errors.append(f"{f}: role '{rn}' missing '{rf}'")
    if "advocate" not in role_names or "skeptic" not in role_names:
        errors.append(f"{f}: roles must include both 'advocate' and 'skeptic'")

    verdict = data.get("verdict")
    if verdict is not None and verdict not in verdict_enum:
        errors.append(f"{f}: verdict must be one of {verdict_enum}, got '{verdict}'")

    if verdict == "do_not_proceed_for_now" and not data.get("revisit_trigger"):
        errors.append(
            f"{f}: verdict 'do_not_proceed_for_now' must set 'revisit_trigger' "
            f"(a concrete condition that would reopen the decision)"
        )

if errors:
    print("Debate verdict validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against debate-verdict schema")
PYEOF
