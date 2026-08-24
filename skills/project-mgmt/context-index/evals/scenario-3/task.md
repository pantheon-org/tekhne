# Scenario 3: Answer "What Context Files Exist?"

## User Prompt

"What context files exist right now? Give me a summary."

## Input (`.context/index.yaml` content)

```yaml
plans:
  - path: ".context/plans/2026-06-30-add-structured-logging.md"
    title: "Plan: Add Structured Logging"
    status: active
    date: 2026-06-30
  - path: ".context/plans/2026-06-28-improve-test-coverage.md"
    title: "Plan: Improve Test Coverage"
    status: done
    date: 2026-06-28
findings:
  - path: ".context/findings/2026-06-30-logging-library-evaluation.md"
    title: "Finding: Logging Library Evaluation"
    status: active
    date: 2026-06-30
decisions:
  - path: ".context/decisions/2026-06-29-adopt-structured-logging.md"
    title: "Decision: Adopt Structured Logging"
    status: active
    date: 2026-06-29
tickets:
  - path: ".context/tickets/2026-06-30-proj-1234-refinement.md"
    title: "Ticket: PROJ-1234 Refinement"
    status: done
    date: 2026-06-30
```

## Expected Behavior

1. Read `.context/index.yaml` to answer the question.
2. Summarise context files grouped by typology (plans, findings, decisions,
   tickets).
3. Include a status breakdown (how many active, how many done) for each
   typology.
4. Do NOT edit `.context/index.yaml` — it is regenerated and should not be
   hand-edited.
5. State that the source of truth is the frontmatter in each `.md` file, not
   the index.

## Success Criteria

- Index read to produce the summary.
- Summary grouped by typology section.
- Status breakdown included (e.g., 1 active, 1 done for plans).
- States that the index is regenerated and should not be hand-edited.
- Source of truth correctly identified as frontmatter in `.md` files.

## Failure Conditions

- Files listed individually without typology grouping.
- No status breakdown.
- Index edited or modified in any way.
- States that the index is the source of truth.
- Index not consulted (files listed from memory or a directory listing
  instead).
