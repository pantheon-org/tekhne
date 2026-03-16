# Scenario 1: Fix Markdownlint Violations

## Context

A contributor submitted a documentation file with several markdownlint violations.
The violations must be fixed in the source content. Disabling rules globally or adding
broad suppressions is not acceptable.

## Input Document (`guide.md`)

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

## Task

Produce a corrected file `guide.md` that:

1. Fixes the heading level skip (H1 → H3 skipping H2).
2. Adds language tags to all untagged fenced code blocks.
3. Makes list markers consistent throughout the document.
4. Fixes the heading level skip from H2 to H4.
5. Normalizes the table column spacing for consistency.
6. Does not add any markdownlint disable comments or change the document's content meaning.

## Output Specification

Produce a single file `guide.md` with all violations corrected.
