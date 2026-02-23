# Dimension Analysis Template

Use this template when writing detailed findings in audit reports.

## D[N]: [Dimension Name]

Assessment: `[score]/[max]` (signal: strong/moderate/weak, priority: high/medium/low)

Inspect:

- [file/path] for [specific condition]
- Presence of [required patterns]

Fix steps:

1. [Actionable step 1]
2. [Actionable step 2]
3. [Actionable step 3]

Done criteria:

- [Measurable criterion 1]
- [Measurable criterion 2]

Re-check:

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json
```
