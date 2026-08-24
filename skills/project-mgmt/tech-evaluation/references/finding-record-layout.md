# Finding Record Layout

Once the structured YAML has passed validation (Workflow step 3), render it into a human-readable finding with the
`context-file` skill.

## Record shape

Path: `.context/findings/YYYY-MM-DD-<topic>.md`, `type: FINDING`.

```markdown
> One-sentence blockquote: the recommendation's rationale, compressed.

## Summary

| Question | Verdict |
| --- | --- |
| correctness | ... |
| integration_fit | ... |
| footprint | ... |
| maturity | ... |
| practical_fit | ... |

## Detail

### Correctness
{expanded verdict with citations}

### Integration fit
{expanded verdict with citations}

...

## Recommended Action

{recommendation.single_action verbatim, plus the rationale}
```

## Section-by-section rules

- **Blockquote summary** -- one sentence, no hedging. This is the line a reader skims first; it must contain the
  actual recommendation, not "further investigation may be warranted."
- **`## Summary` table** -- exactly one row per question from the question set (Workflow step 1), columns
  `Question | Verdict`. This is a scannable index into the `## Detail` section, not a place to restate the
  evidence.
- **`## Detail`** -- one `###`-level subsection per question. Every verdict here MUST carry its citation (file
  path + line, URL, or doc section) inline, not as a trailing footnote list.
- **`## Recommended Action`** -- the `recommendation.single_action` field verbatim, plus its rationale. Never a
  menu of options; this section is what makes the record actionable rather than descriptive.

## Linking from a decision

If the evaluation feeds a plan or an ADR, link the record via `related:` in that document's frontmatter -- the
record is the evidence trail, not the decision record itself. Confirm the link resolves (the record file exists
at the path referenced) before treating the plan or ADR as grounded.
