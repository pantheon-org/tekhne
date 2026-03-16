# Scenario 2: Author a README from Scratch

## Context

You are creating a `README.md` for an open-source CLI tool called `snapcheck`. The tool:

- Is written in TypeScript and distributed via npm.
- Accepts a config file path and a URL, then checks whether a webpage's visual snapshot
  matches a stored baseline.
- Has three commands: `init`, `check`, and `update`.
- Requires Node 18+ and can be installed globally via `npm install -g snapcheck`.

## Task

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

## Output Specification

Produce a single file `README.md` with the complete documentation.
