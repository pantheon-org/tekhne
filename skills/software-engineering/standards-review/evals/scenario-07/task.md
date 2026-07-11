# Scenario 07: Mixed Format — Bullet List Without Headings

## Setup

The user's file `standards-checklist.md` contains:

```markdown
Project Standards Checklist

- All API endpoints must be versioned with a prefix (e.g. /v1/).
- Database migrations must be reviewed by a second developer.
- Every public function must have a JSDoc comment.
- Secrets must never be committed; use the secrets manager.
- All HTTP responses must include a request-id header.
```

No headings — just a paragraph followed by a bullet list.

## User Prompt

"Review my standards checklist."

## Expected Behavior

1. Read the file.
2. Detect that there are no headings but there is a bullet list.
3. Parse each bullet item as an individual standard.
4. Tell the user 5 standards were found.
5. Review each one at a time: one standard per question turn, with Accept/Revise/Reject/Other options.

## Success Criteria

- 5 standards are correctly parsed from the bullet list.
- The introductory paragraph ("Project Standards Checklist") is not treated as a standard.
- Each standard is presented one per question turn.

## Failure Conditions

- The paragraph text is included as a standard.
- Fewer than 5 standards are identified.
- Multiple standards are presented in a single question.
