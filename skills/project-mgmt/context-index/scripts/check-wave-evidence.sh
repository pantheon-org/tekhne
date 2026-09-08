#!/usr/bin/env bash
# shell: bash
# check-wave-evidence.sh - advisory-only nudge: flag a wave/task proof-of-work
# line that claims a test-count or score metric but carries no quoted command
# output backing it up. Mirrors check-plan-staleness.sh in spirit and shape:
# a mechanical backstop for AGENTS.md's Wave-Based Implementation Rules ("Update
# the plan on completion, with evidence, not just a checkmark"), not a
# substitute for actually reading the line.
#
# A completed line is one containing the ✅ marker. It is flagged only when it
# also claims a numeric test/score metric (e.g. "20 tests pass", "score 97")
# without a quoted substring anywhere on the line - a bare commit SHA with no
# metric claim is not flagged, since the rule is about unbacked *numbers*, not
# about every checkmark needing a quote.
#
# Always exits 0. A flagged line may still be genuinely evidenced by a nearby
# line, a linked commit, or a claim that just doesn't need a quote (e.g. an
# ABANDONED line) - this is a prompt to check, not proof of a violation.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
PLANS_DIR="$ROOT/.context/plans"

if [ ! -d "$PLANS_DIR" ]; then
  exit 0
fi

python3 - "$PLANS_DIR" <<'PYEOF'
import re
import sys
from pathlib import Path

plans_dir = Path(sys.argv[1])

MARKER = "✅"
CLAIM_PATTERNS = [
    re.compile(r"\b\d+\s*(tests?|pass(?:ed)?|fail(?:ed)?|skip(?:ped)?)\b", re.IGNORECASE),
    re.compile(r"\bscore[:\s]*\d+\b", re.IGNORECASE),
]

flagged = []
for path in sorted(plans_dir.rglob("*.md")):
    rel = path.relative_to(plans_dir.parent.parent)
    try:
        lines = path.read_text().splitlines()
    except OSError:
        continue
    for lineno, line in enumerate(lines, start=1):
        if MARKER not in line:
            continue
        if '"' in line:
            continue
        if not any(p.search(line) for p in CLAIM_PATTERNS):
            continue
        flagged.append((str(rel), lineno, line.strip()))

if flagged:
    print(f"NOTICE: {len(flagged)} proof-of-work line(s) claim a metric with no quoted evidence (advisory, non-blocking):")
    for rel, lineno, line in flagged:
        print(f"  {rel}:{lineno}")
        print(f"    {line}")
    print()
    print("A ✅ line stating a test count or score should carry a quoted excerpt of the")
    print('command output backing it (e.g. (bun test: "20 pass, 0 fail")), not just the')
    print("number restated from memory. Check whether this one actually has evidence")
    print("nearby before treating it as unmet.")

sys.exit(0)
PYEOF
