# Inspecting and Validating pins.json

Read-only checks for the pin state file. These never modify the board, which matches the rule that `/pin show` must not rewrite the file.

## Derive the file path

```bash
GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
REL_PATH="${PWD#$GIT_ROOT/}"
SLUG=$(echo "$REL_PATH" | tr '/' '-')
PINS_FILE="$PRAXIS_DIR/.session-logs/$SLUG/pins.json"
```

## Count items against the limits

The limits are 5 items per type and 20 in total.

```bash
jq '.items | length' "$PINS_FILE"
jq '.items | group_by(.type) | map({type: .[0].type, count: length})' "$PINS_FILE"
```

## Check for an existing pin before appending

The duplicate check is an exact match on the `content` field.

```bash
jq --arg c "use bun everywhere" '[.items[] | select(.content == $c)] | length' "$PINS_FILE"
```

A result above zero means respond `⚠️ Already pinned.` and stop.

## Confirm numbering stays stable

`next_id` must be greater than every `id` in the file, including after removals.

```bash
jq '{max_id: ([.items[].id] | max), next_id: .next_id}' "$PINS_FILE"
```

## Type and emoji mapping

| Emoji | Type | Cleared by `/pin clear triage` |
| --- | --- | --- |
| ✅ | approved | Yes |
| ❓ | pending | Yes |
| ❌ | killed | No |
| 📌 | scope | No |
| 🔧 | correction | No |
