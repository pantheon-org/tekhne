# Scenario 01: Troubleshooting Documentation Task

## User Prompt

Your development team has been experiencing intermittent API gateway timeouts when connecting to the new microservice architecture. The issue started appearing after the recent deployment last Tuesday, and it's affecting about 15% of user requests during peak hours. The ops team has identified this as a critical issue that needs immediate resolution and proper documentation.

After investigating the problem, you discovered that the connection pool wasn't being properly recycled in the Lambda functions, causing resource exhaustion. You implemented a fix by adjusting the connection pool settings and adding proper cleanup handlers.

The team needs a comprehensive record of this troubleshooting session for future reference and knowledge sharing, including the symptoms, investigation process, root cause analysis, and the solution implemented.

Create a troubleshooting documentation entry that captures:

- The problem description and symptoms observed
- Investigation steps taken to identify the root cause
- The solution implemented and verification steps
- Any lessons learned or preventive measures

The documentation should be created as a markdown file with appropriate metadata and structure for easy future reference and searchability by the team.

## Expected Behavior

1. Consult the `troubleshooting.yaml` template schema before generating the entry
2. Place the file in a `YYYY/MM/` directory matching the date
3. Use a filename slug with only lowercase letters and hyphens, no uppercase or underscores
4. Ensure the date matches exactly across filename, frontmatter, and H1 title
5. Format the H1 title using `Month D, YYYY` format (not `Month DD`)
6. Include exactly one H1 heading in the document
7. Include a `troubleshooting` tag in the frontmatter
8. Ensure tags match exactly between the frontmatter array and the Tags section
9. Format all tags using lowercase with hyphens, no uppercase or underscores
10. Include language specifiers on all code blocks, no bare triple backticks
11. Format the metadata block using `**Key:** Value` with bold keys
12. Include no emojis anywhere in the document

## Success Criteria

- **Template schema loaded**: Evidence shows troubleshooting.yaml template was consulted before generation
- **Correct directory structure**: File placed in YYYY/MM/ directory matching the date
- **Lowercase filename slug**: Filename slug uses only lowercase letters and hyphens, no uppercase or underscores
- **Triple date sync**: Date matches exactly across filename, frontmatter, and H1 title
- **H1 date format**: H1 title uses `Month D, YYYY` format (not `Month DD`)
- **Single H1 heading**: Document contains exactly one H1 heading
- **Required troubleshooting tag**: Includes `troubleshooting` tag in frontmatter
- **Tag consistency**: Tags match exactly between frontmatter array and Tags section
- **Lowercase hyphenated tags**: All tags use lowercase with hyphens, no uppercase or underscores
- **Code block languages**: All code blocks have language specifiers, no bare triple backticks
- **Bold metadata formatting**: Metadata block uses `**Key:** Value` format with bold keys
- **No emojis used**: Document contains no emojis in any section or content

## Failure Conditions

- No evidence of consulting the troubleshooting.yaml template before generation
- File placed in wrong directory or without the YYYY/MM/ structure
- Filename slug contains uppercase letters or underscores
- Date is inconsistent between filename, frontmatter, or H1 title
- H1 uses `Month 0D` zero-padded format instead of `Month D`
- Document contains more than one H1 heading
- `troubleshooting` tag is absent from frontmatter
- Tags in frontmatter array do not match the Tags section exactly
- Any tag contains uppercase letters or underscores
- Any code block is missing a language specifier
- Metadata block uses bullet points or non-bold keys instead of `**Key:** Value`
- Document contains emojis in any section
