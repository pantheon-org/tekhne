# REVIEWED.md and REFERENCE_INDEX.md Formats

## REVIEWED.md summary row

Add a row to the summary table at the top, in reverse-chronological order:

```text
| <today's date> | <slug> | paper | pending | <one-line description> |
```

## REVIEWED.md detailed section

Add a detailed section below the table:

```markdown
## <slug> — <Full paper title>

- **arxiv**: <ID>
- **Authors**: <list>
- **Date**: <YYYY-MM-DD>
- **Tags**: <tags>
- **Summary**: <2–3 sentences>
- **Disposition**: pending — awaiting user decision on promotion
```

## REFERENCE_INDEX.md

Add a row under the most relevant category table. If no category fits, add it to the closest one and note it for the user.

## Universal tags

| Tag | Meaning |
|---|---|
| `survey` | Overview paper covering the topic broadly |
| `benchmark` | Evaluation dataset or framework |
| `empirical` | Study with experimental evaluation |
| `theoretical` | Formal or theoretical contribution |
| `system` | System or implementation paper |

## Promotion

When the user confirms promotion, create `analysis/ANALYSIS-arxiv-<id>-<slug>.md` from `assets/templates/ANALYSIS-paper.yaml` and change the disposition in REVIEWED.md from `pending` to `analysis`.
