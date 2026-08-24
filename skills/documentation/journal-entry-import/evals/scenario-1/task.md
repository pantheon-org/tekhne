# Scenario 1: Bulk-import a legacy work log (Mode A)

## User Prompt

"We're retiring our old Confluence space. There's a page called 'Weekly Journal' with a couple of weeks of notes
on it - import it into the repo's journal format."

## Input

The exported page body (`source.md`) contains:

```markdown
2024

### Week ending on March 8

- Paired with the platform team on the outage retro; agreed to add a synthetic canary for the checkout path.
- Reviewed PROJ-771 - flaky integration test in the billing service.

### Week ending on March 15

### Week ending on March 22

- Deployed the canary from last week's retro; caught a real regression within a day.
```

The source URL is `https://example.atlassian.net/wiki/spaces/TEAM/pages/98765/Weekly+Journal`.

## Expected Behavior

1. Split the source into its dated sections, treating "Week ending on <date>" as the section boundary, deriving
   the year (2024) from the standalone year line above the first heading.
2. Create `2024/03/2024-03-08-imported-entry.md` and `2024/03/2024-03-22-imported-entry.md`.
3. Skip the March 15 section entirely - it has no content, so no file is written for it.
4. Each generated file has `journal-entry-creator`-compatible frontmatter (`title`, `date`, `authors`, `tags`
   including `imported`, `legacy-import`, and `"2024"`, `source` set to the page URL, `status: published`), a
   single dated H1, and preserves the bullets verbatim - no summarising, no added interpretation.
5. Validate, format, and lint the generated files; treat the import as one atomic unit before any Mode B
   enrichment is attempted on the tickets it mentions.

## Success Criteria

- Exactly two files created, at the correct `YYYY/MM/YYYY-MM-DD-imported-entry.md` paths, dated by each section's
  week-ending date.
- The empty March 15 week produces no file.
- Frontmatter passes `journal-entry-creator`'s base validator; tags match the `## Tags` section.
- Bullet content matches the source verbatim (formatting normalised only).
- The entries are validated (and formatting/lint run) before being presented as done.

## Failure Conditions

- A file created for the empty March 15 week.
- Content rewritten, summarised, or embellished beyond verbatim transcription plus formatting normalisation.
- Missing or incorrect frontmatter (wrong tags, missing `source`, wrong `status`).
- Entries never run through the base validator.
