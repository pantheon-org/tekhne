#!/usr/bin/env bash
# shell: bash
# Validates a generated tech-evaluation YAML against the tech-evaluation schema.
# Usage: validate-tech-evaluation.sh <file> [<file> ...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/tech-evaluation.schema.json"

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
eval_schema = schema["properties"]["evaluation"]
question_schema = eval_schema["properties"]["questions"]["items"]
evidence_schema = question_schema["properties"]["evidence"]["items"]
rec_schema = eval_schema["properties"]["recommendation"]
verdict_enum = rec_schema["properties"]["verdict"]["enum"]

errors = []

for f in files:
    p = Path(f)
    if not p.exists():
        errors.append(f"{f}: file not found")
        continue

    content = p.read_text()
    if not HAS_YAML:
        errors.append(f"{f}: PyYAML not available, cannot validate")
        continue

    try:
        data = yaml.safe_load(content)
    except yaml.YAMLError as e:
        errors.append(f"{f}: invalid YAML -- {e}")
        continue

    if not isinstance(data, dict):
        errors.append(f"{f}: root must be a mapping (got {type(data).__name__})")
        continue

    evaluation = data.get("evaluation")
    if not evaluation:
        errors.append(f"{f}: missing required top-level key 'evaluation'")
        continue

    for field in eval_schema["required"]:
        if field not in evaluation:
            errors.append(f"{f}: evaluation missing required field '{field}'")

    questions = evaluation.get("questions") or []
    if len(questions) < 1:
        errors.append(f"{f}: 'questions' must have at least 1 entry")

    for i, q in enumerate(questions):
        for field in question_schema["required"]:
            if field not in q:
                errors.append(f"{f}: questions[{i}] missing required field '{field}'")
        evidence = q.get("evidence") or []
        if len(evidence) < 1:
            errors.append(
                f"{f}: questions[{i}] ('{q.get('id', '?')}') has no evidence -- "
                "a verdict without a cited source is a guess, not a finding"
            )
        for j, ev in enumerate(evidence):
            for field in evidence_schema["required"]:
                if field not in ev:
                    errors.append(f"{f}: questions[{i}].evidence[{j}] missing required field '{field}'")

    recommendation = evaluation.get("recommendation") or {}
    for field in rec_schema["required"]:
        if field not in recommendation:
            errors.append(f"{f}: recommendation missing required field '{field}'")

    verdict = recommendation.get("verdict")
    if verdict is not None and verdict not in verdict_enum:
        errors.append(f"{f}: recommendation.verdict '{verdict}' is not one of {verdict_enum}")

    rationale = recommendation.get("rationale") or ""
    min_len = rec_schema["properties"]["rationale"]["minLength"]
    if len(rationale) < min_len:
        errors.append(f"{f}: recommendation.rationale is shorter than {min_len} characters -- too thin to be a rationale")

    action = recommendation.get("single_action") or ""
    action_min_len = rec_schema["properties"]["single_action"]["minLength"]
    if len(action) < action_min_len:
        errors.append(f"{f}: recommendation.single_action is shorter than {action_min_len} characters")

if errors:
    print("Tech-evaluation validation errors:")
    for e in errors:
        print(f"  {e}")
    sys.exit(1)

print(f"OK: {len(files)} file(s) validated against tech-evaluation schema")
PYEOF
