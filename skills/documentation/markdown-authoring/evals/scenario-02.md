# Scenario 02: Author a README from Scratch

## User Prompt

You are creating a `README.md` for an open-source CLI tool called `snapcheck`. The tool:

- Is written in TypeScript and distributed via npm.
- Accepts a config file path and a URL, then checks whether a webpage's visual snapshot
  matches a stored baseline.
- Has three commands: `init`, `check`, and `update`.
- Requires Node 18+ and can be installed globally via `npm install -g snapcheck`.

Produce a `README.md` for `snapcheck` that:

1. Starts with a single H1 title.
2. Uses the document structure: Overview → Installation → Usage → Commands → Configuration → Contributing.
3. Never skips a heading level (all sub-sections are H3 under their H2 parent sections,
   if deeper nesting is needed).
4. Shows all shell commands in fenced code blocks tagged with `bash`.
5. Shows all config file examples in fenced code blocks tagged with the appropriate
   language (`yaml`, `json`, etc.).
6. Uses a consistent list marker style throughout.
7. Includes a table for the Commands section listing each command, its syntax, and a
   brief description.

Produce a single file `README.md` with the complete documentation.

## Expected Behavior

1. Include H2 sections for Overview, Installation, Usage, Commands, Configuration, and Contributing, in that order
2. Maintain sequential heading levels with no skips throughout the document
3. Tag all code blocks with explicit language identifiers (`bash`, `yaml`, `json`, etc.)
4. Use a single consistent list marker style throughout the document
5. Include a formatted Markdown table in the Commands section with command name, syntax, and description columns

## Success Criteria

- **Document structure matches required section order**: The README contains H2 sections for Overview, Installation, Usage, Commands, Configuration, and Contributing, in that order.
- **Heading levels are sequential with no skips**: There are no heading level jumps in the document. Every H3 appears under an H2 parent, and no H4 appears without an H3 parent.
- **All code blocks have explicit language tags**: Every fenced code block in the document has a language tag. Shell commands use `bash`, config examples use `yaml` or `json`.
- **List markers are consistent throughout**: All bullet list items use the same marker style. No mixing of `-`, `*`, and `+` within or across lists.
- **Commands section includes a formatted table**: The Commands section contains a Markdown table with columns for command name, syntax, and description. The table is properly formatted with aligned pipe characters.

## Failure Conditions

- Any required H2 section is missing or appears out of order (Overview, Installation, Usage, Commands, Configuration, Contributing)
- Any heading level is skipped (e.g., H2 directly to H4, or H3 without an H2 parent)
- Any fenced code block is missing a language tag, or shell commands are not tagged `bash`
- List markers are mixed within or across lists
- Commands section is missing a Markdown table, or the table omits command name, syntax, or description columns
