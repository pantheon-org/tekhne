---
name: defuddle
description: Extract clean markdown content from web pages using Defuddle CLI, removing clutter and navigation to save tokens. Use instead of WebFetch when the user provides a URL to read or analyze, for online documentation, articles, blog posts, or any standard web page.
---

# Defuddle

Use Defuddle CLI to extract clean readable content from web pages. Prefer over WebFetch for standard web pages — it removes navigation, ads, and clutter, reducing token usage.

If not installed: `npm install -g defuddle`

## When to Use

Use this decision framework before fetching any URL:

| Scenario | Tool | Reason |
|----------|------|--------|
| Article, blog post, documentation page | **defuddle** (ALWAYS) | Removes clutter, saves tokens |
| Standard web page with readable text | **defuddle** (ALWAYS) | Cleaner output than WebFetch |
| JavaScript-heavy SPA (React/Vue app shell) | WebFetch or inform user | defuddle cannot execute JS |
| Login-gated or auth-required page | WebFetch or inform user | defuddle fetches without credentials |
| API endpoint returning JSON | Direct fetch or inform user | defuddle is for HTML pages |
| User asks for metadata only (title, description) | `defuddle parse <url> -p <field>` | Lightweight extraction |

**Mental model:** defuddle is a server-side HTML parser — it downloads the HTML, strips boilerplate, and returns clean prose. It cannot authenticate, execute JavaScript, or follow redirects that require cookies. When in doubt about a URL, try defuddle first; if the output is empty or only navigation text, fall back to WebFetch.

## Usage

ALWAYS use `--md` for markdown output:

```bash
defuddle parse <url> --md
```

Save to file:

```bash
defuddle parse <url> --md -o content.md
```

Extract specific metadata:

```bash
defuddle parse <url> -p title
defuddle parse <url> -p description
defuddle parse <url> -p domain
```

## Output formats

| Flag | Format |
|------|--------|
| `--md` | Markdown (default choice) |
| `--json` | JSON with both HTML and markdown |
| (none) | HTML |
| `-p <name>` | Specific metadata property |

## Common Mistakes

### 1. Using WebFetch when defuddle is available

**NEVER** use WebFetch for standard web pages when defuddle is available. WebFetch returns the full raw HTML including navigation menus, sidebars, footers, and ads. This wastes tokens and produces noisy content.

**WHY:** Raw HTML pages commonly contain 5-10x more tokens than the extracted article content, inflating cost and degrading response quality.

BAD:

```bash
# WebFetch returns thousands of tokens of navigation clutter
WebFetch("https://docs.example.com/guide")
```

GOOD:

```bash
defuddle parse https://docs.example.com/guide --md
```

### 2. Omitting the `--md` flag

**NEVER** call `defuddle parse` without the `--md` flag when readable content is the goal. Without `--md`, defuddle returns raw HTML, which is much harder to read and costs more tokens than markdown.

**WHY:** HTML output retains tags, attributes, and inline styles that the agent must parse mentally, wasting context window space and increasing error risk.

BAD:

```bash
# Returns HTML — hard to read, token-heavy
defuddle parse https://example.com/article
```

GOOD:

```bash
defuddle parse https://example.com/article --md
```

### 3. Using `-p` metadata extraction when full content is needed

**NEVER** use the `-p` flag when the user wants page body content. The `-p` flag returns a single metadata property. Using it when the user wants the page content silently discards the body text.

**WHY:** The body of the article is not included in the output at all, so the agent will respond with only a title or description while the user expects the full article.

BAD:

```bash
# Returns only the title string, not the article content
defuddle parse https://example.com/article -p title
```

GOOD:

```bash
# Returns full article in markdown
defuddle parse https://example.com/article --md
```

### 4. Using shell redirection instead of the `-o` flag

**NEVER** use shell redirection (`>`) to save defuddle output to a file. Shell redirection bypasses defuddle's file-writing logic and can corrupt output encoding. Always use the `-o` flag to save output to a file.

**WHY:** Redirection can introduce encoding mismatches (especially on non-UTF-8 terminals) and may strip BOM or metadata that defuddle writes through its own file path.

BAD:

```bash
# Redirection may corrupt encoding or strip metadata
defuddle parse https://example.com/article --md > output.md
```

GOOD:

```bash
defuddle parse https://example.com/article --md -o output.md
```

## References

- [Defuddle npm](https://www.npmjs.com/package/defuddle)
- [Defuddle GitHub](https://github.com/kepano/defuddle)
