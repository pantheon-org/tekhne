# Set Up Markdownlint CI Pipeline

A team uses a GitHub repository with Markdown documentation across multiple directories.
There are currently no automated checks for documentation quality.

Set up a GitHub Actions CI workflow that runs `markdownlint-cli2` on every pull request
and push to `main`. The workflow must:

1. Run `markdownlint-cli2` across all `**/*.md` files, excluding `node_modules/`.
2. Use Node.js 20 with npm caching.
3. Fail the build if any markdownlint errors are found.
4. Use a configuration file `.markdownlint.json` that:
   - Enables all rules by default.
   - Configures `MD013` (line length) to 120 characters, excluding code blocks and tables.
   - Configures `MD033` (inline HTML) to allow `details`, `summary`, `br`, and `img`.
   - Configures `MD024` (no-duplicate-heading) to `siblings_only: true`.
5. Include a `lint:md` script in `package.json` that runs the linter.

Produce two files:
- `.github/workflows/markdown-lint.yml` — the CI workflow
- `package.json` — the root package.json with the lint script (only the relevant fields)
