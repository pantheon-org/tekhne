# Prompt-Cache Audit Checklist

Reference checklist. Core guidance is in `SKILL.md`.

- Stable content (system prompt, tool definitions) sits before volatile conversation.
- A cache breakpoint follows the largest stable prefix.
- Tool list and system prompt are byte-stable across turns (no reordering or timestamps).
- Cache hit rate is measured; misses are attributed to specific churn.
