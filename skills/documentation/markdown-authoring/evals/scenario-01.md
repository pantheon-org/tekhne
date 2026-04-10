# Scenario 01: Fix Markdownlint Violations

## User Prompt

A contributor submitted a documentation file with several markdownlint violations.
The violations must be fixed in the source content. Disabling rules globally or adding
broad suppressions is not acceptable.

Here is the input document (`guide.md`):

````markdown
# Getting Started
### Installation

Run the following to install:

```
npm install my-tool
```

Then configure it:

```
# config.yaml
host: localhost
port: 8080
```

## Usage

- Use the `--verbose` flag for detailed output
* Use the `--dry-run` flag to preview changes
+ Use `--help` to see all options

#### Advanced Options

For power users, the following options are available:

| Option | Description |
|--------|-------------|
|`--timeout` | Sets the request timeout  |
| `--retries`| Number of retries |
````

Produce a corrected file `guide.md` that:

1. Fixes the heading level skip (H1 → H3 skipping H2).
2. Adds language tags to all untagged fenced code blocks.
3. Makes list markers consistent throughout the document.
4. Fixes the heading level skip from H2 to H4.
5. Normalizes the table column spacing for consistency.
6. Does not add any markdownlint disable comments or change the document's content meaning.

Produce a single file `guide.md` with all violations corrected.

## Expected Behavior

1. Fix all heading level skips so H1 is followed by H2, H2 by H3, with no jumps
2. Add an explicit language identifier to every fenced code block (e.g., `bash`, `yaml`)
3. Make all bullet list markers consistent throughout, using a single marker character
4. Remove any markdownlint disable comments rather than suppressing violations
5. Preserve all original sections, code samples, and table content unchanged

## Success Criteria

- **Heading levels are sequential with no skips**: The corrected file has no heading level jumps: H1 is followed by H2, H2 by H3, etc. There is no H1→H3 or H2→H4 skip.
- **All fenced code blocks have explicit language tags**: Every triple-backtick fence in the output file includes a language identifier (e.g., `bash`, `yaml`). No untagged fences remain.
- **List markers are consistent throughout**: All bullet list items use the same marker character. No mixing of `-`, `*`, and `+` within the document.
- **No markdownlint disable comments or suppressions added**: The output contains no `<!-- markdownlint-disable -->`, `<!-- markdownlint-disable-line -->`, or `<!-- markdownlint-disable-next-line -->` comments.
- **Document content and meaning unchanged**: All original sections, code samples, and table content are preserved. No text was deleted or reworded to avoid fixing the structural issues.

## Failure Conditions

- Output still contains any heading level jump (H1→H3 or H2→H4)
- Any fenced code block is missing a language identifier
- List markers are still mixed (`-`, `*`, and `+` used within the same document)
- Any markdownlint disable comment is added to suppress violations
- Original content, code samples, or table data is deleted or reworded
