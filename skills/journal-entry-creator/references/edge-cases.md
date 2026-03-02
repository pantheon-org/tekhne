# Edge Case Handling Reference

Detailed resolution strategies for common edge cases when creating journal entries.

## File Already Exists

**Detection:** Check before writing file

**Resolution:**

1. Ask user: `Entry already exists at [path]. Overwrite? (y/N)`
2. If no: Suggest alternative filename with suffix `-v2`, `-alt`, or `-revised`
3. If yes: Backup original (rename to `.bak`) then write new file

## Date Mismatch Detected

**Problem:** User says "document yesterday's work" but filename uses today's date

**Resolution:** Ask user which date to use, then update ALL locations (filename, frontmatter, H1) to match and ensure correct directory placement.

## Schema Not Found

**Problem:** Template YAML file missing or unreadable

**Resolution:**

1. STOP immediately - do not proceed
2. Error message: `Template schema [type].yaml not found. Cannot generate entry without schema definition.`
3. List available schemas in template/ directory
4. Ask user to select available type or fix schema location

**Do NOT:** Generate entry with guessed structure - this violates compliance

## Validation Fails After Generation

**Common failures and auto-fixes:**

| Error                  | Auto-fix Strategy                               |
| ---------------------- | ----------------------------------------------- |
| Missing code language  | Add `bash` or `text` based on content          |
| Multiple blank lines   | Remove with prettier                            |
| Heading hierarchy skip | Adjust levels H1→H2→H3                          |
| Tag mismatch           | Sync Tags section with frontmatter             |
| Date format wrong      | Use `Month D, YYYY` format                      |

**Manual fixes required:** Missing sections (need content), incorrect entry type, ambiguous slug.

## User Wants Custom Structure

**Problem:** User asks to deviate from template structure

**Resolution:** Explain validation requirements and offer: (A) custom content within existing sections, (B) additional sections after required ones, or (C) different entry type. Warn that removing required sections fails validation.

## Ambiguous Entry Type

**Problem:** User intent doesn't clearly match any entry type

**Resolution:**

1. Present decision criteria from main skill
2. Ask clarifying questions:
   - "Are you resolving a problem?" (Troubleshooting)
   - "Are you documenting new knowledge?" (Learning)
   - "Are you summarizing external content?" (Article Summary)
3. Default to Journal Entry if still unclear
4. Allow user to override auto-detection

## Missing Required Information

**Problem:** User provides insufficient context for required sections

**Resolution:**

1. List missing required fields based on template schema
2. Ask specific questions for each missing field
3. Offer to use placeholder text with TODO comments
4. Document placeholders in commit message if applicable

## Multiple Topics in One Entry

**Problem:** User wants to document several unrelated topics

**Resolution:**

1. Suggest creating separate entries (preferred)
2. If user insists on single entry:
   - Use Journal Entry type
   - Create clear H2 sections for each topic
   - Use combined slug: `topic1-topic2-topic3`
   - Add multiple relevant tags
3. Warn about reduced discoverability

## Retroactive Entries (Past Dates)

**Problem:** User wants to create entries for past dates

**Resolution:**

1. Confirm the intended date explicitly
2. Ensure directory exists: `mkdir -p YYYY/MM`
3. Triple-check date consistency across all three locations
4. Add note in commit message: `(retroactive entry for YYYY-MM-DD)`

## Directory Permission Issues

**Problem:** Cannot create YYYY/MM directory or write file

**Resolution:**

1. Check current directory: `pwd`
2. Verify repository root
3. Test write permissions: `touch test.tmp && rm test.tmp`
4. Report specific error to user with suggested fix
5. Ask user to run with elevated permissions if needed
