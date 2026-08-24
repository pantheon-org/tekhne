#!/usr/bin/env bash
# shell: bash
# Validates a model-selection YAML against the model-selection schema.
# Usage: validate-model-selection.sh <file> [<file> ...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/model-selection.schema.json"

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
        errors.append(f"{f}: PyYAML not available, skipping")
        continue

    sel = data.get("model_selection")
    if not sel:
        errors.append(f"{f}: missing required top-level key 'model_selection'")
        continue

    required = ["environment", "question_text", "options"]
    for field in required:
        if field not in sel:
            errors.append(f"{f}: model_selection missing required field '{field}'")

    env = sel.get("environment")
    valid_envs = ["opencode-zen", "opencode-go", "claude-code", "byok"]
    if env and env not in valid_envs:
        errors.append(f"{f}: environment must be one of {valid_envs}, got '{env}'")

    options = sel.get("options", [])
    if not isinstance(options, list):
        errors.append(f"{f}: options must be an array")
    elif len(options) < 1:
        errors.append(f"{f}: options must have at least one entry")

if errors:
    print("Model selection validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against model-selection schema")
PYEOF
