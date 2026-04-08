# Eval Scenario Format

All skills must use `evals/scenario-NN.md` — one Markdown file per scenario, numbered from `01`.

## Required Structure

```markdown
# Scenario NN: Title

## User Prompt

"Exact trigger phrase the user would type."

## Expected Behavior

1. Step the agent takes
2. Next step
3. ...

## Success Criteria

- Measurable outcome 1
- Measurable outcome 2

## Failure Conditions

- What a bad agent response looks like
- Another failure mode
```

All four sections are required. Success criteria must be measurable (files created, commands run, specific output verified) — never vague ("agent does well").

## Quantity

Minimum 5 scenarios per skill. Target 7–9 for skills with broad trigger surfaces.

Cover:
- Primary happy path
- Edge cases and partial inputs
- Failure / anti-pattern detection
- At least one scenario where the skill should refuse or warn

## File Naming

`evals/scenario-01.md`, `evals/scenario-02.md`, … `evals/scenario-09.md`

Zero-padded two digits. No gaps in numbering.

## Tile.json

List each scenario file in the `files` array:

```json
{
  "files": [
    "evals/scenario-01.md",
    "evals/scenario-02.md"
  ]
}
```

## Non-Standard Formats (do not use)

| Format | Problem |
|--------|---------|
| `evals/*.yaml` | Not linkable from tile.json `files`; diverges from markdown-first convention |
| `evals.md` (single file) | Cannot reference individual scenarios; does not scale beyond 3–4 scenarios |
| `evals/instructions.json` | Meta-artifact from a retired eval framework; remove if present |
| `evals/summary.json` | Retired; remove if present |
