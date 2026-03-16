# Scenario 4: Create a Markdownlint Configuration File

## Context

A team wants to enforce the Markdown authoring standards documented in the
markdown-authoring skill across their repository. They need a `.markdownlint.json`
configuration file that encodes the skill's four key anti-patterns as enforced rules:

1. Code fences must have explicit language tags (no bare fences).
2. Heading levels must not skip (sequential hierarchy required).
3. List markers must be consistent (one style across the document).
4. Rules must not be broadly disabled — only specific targeted rules can be configured.

## Task

Produce a `.markdownlint.json` configuration file that:

1. Enables all rules by default (`"default": true`).
2. Configures the rule that enforces language tags on fenced code blocks.
3. Configures the rule that prevents heading level skips.
4. Configures the rule that enforces consistent list marker style.
5. Does not globally disable any rules (no rules set to `false` unless a specific,
   justified exception is documented in an accompanying comment file).
6. Is valid JSON that would be accepted by `markdownlint-cli2`.

Also produce a short `markdownlint-notes.md` file that explains why each rule
was configured and which anti-pattern it prevents.

## Output Specification

Produce two files:

- `.markdownlint.json` — the linter configuration
- `markdownlint-notes.md` — explanations for each rule configured
