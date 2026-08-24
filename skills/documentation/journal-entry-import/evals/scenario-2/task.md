# Scenario 2: Enrich a ticket reference into a detail entry (Mode B)

## User Prompt

"This entry mentions PROJ-4001. Reconstruct a self-contained detail entry from the ticket and cross-link it to
the entry."

## Input

Entry `2020/09/2020-09-25-weekly-journal.md` contains the bullet:

```markdown
- [PROJ-4001](https://example.atlassian.net/browse/PROJ-4001): Spent the week getting the nightly export Lambda to
  stop timing out.
```

The issue tracker resolves `PROJ-4001` (fetched with comments) as:

- **summary:** "Nightly export Lambda timing out"
- **status:** Done, **created** 2020-09-21, **resolutiondate** 2020-09-25
- **reporter:** Dana Whitfield, **assignee:** Alex Rivera
- **description:** "The nightly export Lambda times out intermittently. Investigate and make it reliable."
- **comments:** Alex Rivera traced it to a synchronous S3 list on a growing prefix; switched to paginated
  listing and raised the Lambda timeout; Sam Okafor reviewed and confirmed the fix.

## Expected Behavior

1. Create `2020/09/2020-09-25-proj-4001-<slug>.md`, dated by the resolution date (2020-09-25).
2. Single dated H1 `# PROJ-4001 - Nightly export Lambda timing out - September 25, 2020`.
3. Frontmatter per the ticket-detail schema: title starts with the key; tags include `proj-4001`, topic tags,
   `ticket-detail`, `"2020"`; source is the tracker's ticket URL; status published. NO `refinement_ticket` field.
4. Sections in order: metadata block, `## Related Entries` (link to the entry), `## Session Overview`,
   `## Task and Context`, `## Ticket Description` (verbatim blockquote), `## Decisions and Resolution`
   (synthesised, not a comment transcript), `## Compliance`, `## Tags`.
5. Include the decisive-contribution callout (Alex Rivera was the pivotal actor here).
6. Add the reciprocal back-link: a nested `- Detailed entry: [...](2020-09-25-proj-4001-...md)` under the
   PROJ-4001 bullet in the original entry.
7. Entry passes `scripts/validate-ticket-detail.sh`.

## Success Criteria

- Detail file created at the correct path, dated by the resolution date.
- Frontmatter valid; `refinement_ticket` not set.
- All required sections present and in order, including verbatim `## Ticket Description`.
- Decisions section is synthesised prose, with a `> **Decisive contribution (Alex Rivera):**` callout.
- Bidirectional cross-link present (Related Entries -> weekly entry; nested bullet weekly entry -> detail).
- Passes the ticket-detail validator.

## Failure Conditions

- `refinement_ticket` set in frontmatter, or a fabricated supersession field used for the cross-link.
- Missing `## Ticket Description` or any required section.
- Decisions section written as a per-comment transcript.
- Decisive-contribution callout added when unwarranted, or wrong blockquote format.
- No back-link, or dated by the mention's date instead of the ticket's resolution date.
