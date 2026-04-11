# Scenario 04: General Documentation Task

## User Prompt

Your team just completed the quarterly planning session where several important decisions were made about the upcoming product roadmap. The session covered technology stack decisions, resource allocation across different projects, and alignment on development priorities for the next three months.

Key decisions included adopting a new CI/CD pipeline tool, restructuring the development team into feature-focused squads, and establishing new code review processes. There were also discussions about technical debt priorities and infrastructure modernization efforts.

The engineering leadership wants to maintain a record of these strategic sessions for future reference and to track how decisions evolve over time. This documentation will be valuable for onboarding new team members and for quarterly retrospectives.

Create a general documentation entry that captures:

- Key decisions and their rationale
- Action items and ownership assignments
- Timeline and milestone information
- Any unresolved issues or follow-up items
- Context that will be helpful for future reference

The documentation should be well-organized and include appropriate metadata for easy searching and categorization within the team's knowledge base.

## Expected Behavior

1. Show evidence of loading the `journal-entry.yaml` template before generation
2. Create the `YYYY/MM/` directory structure using `mkdir -p` if needed
3. Use a filename following `YYYY-MM-DD-slug.md` with no spaces and all-lowercase slug
4. Ensure the date matches exactly across filename, frontmatter, and H1 title
5. Format the H1 title using exact `Month D, YYYY` format (e.g., `February 3, 2025`, not `February 03`)
6. Include exactly one H1 heading, no duplicates
7. Format all tags in lowercase-hyphenated style (e.g., `ci-cd` not `CI_CD` or `ciCd`)
8. Ensure the frontmatter tags array perfectly matches the Tags section format
9. Include language specifiers on all code blocks, no bare triple backticks
10. Include no emojis in any section or content area
11. Format the metadata block using `**Key:**` with bold keys, no bullet points
12. Show evidence of running the validation script and fixing any issues

## Success Criteria

- **Journal-entry schema**: Shows evidence of loading journal-entry.yaml template before generation
- **Directory creation**: Creates YYYY/MM/ directory structure using mkdir -p if needed
- **Filename constraints**: Filename follows YYYY-MM-DD-slug.md with no spaces, all-lowercase slug
- **Date triple sync**: Date consistency across filename, frontmatter, and H1 exactly matching
- **H1 date precision**: H1 title uses exact `Month D, YYYY` format (February 3, not February 03)
- **Single H1 constraint**: Document contains only one H1 heading, no duplicates
- **Tag case compliance**: All tags use lowercase-hyphenated format (ci-cd not CI_CD or ciCd)
- **Tag matching**: Frontmatter tags array perfectly matches Tags section format
- **Code language requirement**: All code blocks include language specifiers, no bare ``` blocks
- **No emoji constraint**: Document contains no emojis in any section or content area
- **Bold metadata keys**: Metadata block uses `**Key:**` format with bold keys, no bullet points
- **Validation execution**: Shows evidence of running validation script and fixing any issues

## Failure Conditions

- No evidence of loading the journal-entry.yaml template before generation
- File placed in wrong directory or YYYY/MM/ structure not created
- Filename contains spaces, uppercase letters, or underscores in the slug
- Date is inconsistent between filename, frontmatter, or H1 title
- H1 uses zero-padded day (`February 03`) instead of `February 3`
- Document contains more than one H1 heading
- Any tag contains uppercase letters, underscores, or camelCase
- Frontmatter tags array does not match the Tags section exactly
- Any code block is missing a language specifier
- Document contains any emojis
- Metadata block uses bullet points or non-bold keys
- No evidence of running the validation script
