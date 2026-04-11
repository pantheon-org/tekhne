# Scenario 04: Create a Markdownlint Configuration File

## User Prompt

A team wants to enforce the Markdown authoring standards documented in the
markdown-authoring skill across their repository. They need a `.markdownlint.json`
configuration file that encodes the skill's four key anti-patterns as enforced rules:

1. Code fences must have explicit language tags (no bare fences).
2. Heading levels must not skip (sequential hierarchy required).
3. List markers must be consistent (one style across the document).
4. Rules must not be broadly disabled — only specific targeted rules can be configured.

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

Produce two files:

- `.markdownlint.json` — the linter configuration
- `markdownlint-notes.md` — explanations for each rule configured

## Expected Behavior

1. Set `"default": true` in `.markdownlint.json` to enable all standard rules
2. Include a configuration entry for rule MD040 (or equivalent) requiring language tags on fenced code blocks
3. Include a configuration entry for rule MD001 (or equivalent) enforcing sequential heading levels
4. Include a configuration entry for rule MD004 (or equivalent) enforcing a single list marker style
5. Set no rule to `false` — all configured rules are enabled or given specific option values
6. Write `markdownlint-notes.md` with one entry per configured rule explaining its purpose and which anti-pattern it prevents

## Success Criteria

- **`"default": true` is set**: The `.markdownlint.json` file contains `"default": true` so all standard rules are enabled unless explicitly overridden.
- **Fenced code language rule is configured**: The config includes a setting for rule MD040 (or equivalent) that requires language tags on fenced code blocks.
- **Heading level sequence rule is configured**: The config includes a setting for rule MD001 (or equivalent) that enforces sequential heading levels with no skips.
- **List marker consistency rule is configured**: The config includes a setting for rule MD004 (or equivalent) that enforces a single list marker style.
- **No rules are globally disabled**: No rule is set to `false` in the config. All configured rules are either enabled (`true`) or given a specific option value.
- **Notes file explains each configured rule**: The `markdownlint-notes.md` file includes one entry per configured rule explaining its purpose and which anti-pattern it prevents.

## Failure Conditions

- `.markdownlint.json` is missing `"default": true`
- No configuration entry for the fenced code language tag rule (MD040 or equivalent)
- No configuration entry for the heading level sequence rule (MD001 or equivalent)
- No configuration entry for the list marker consistency rule (MD004 or equivalent)
- Any rule is set to `false`, globally disabling it
- `markdownlint-notes.md` is missing, empty, or does not document the purpose of each configured rule
