---
name: defuddle
description: Extract clean markdown content from web pages using Defuddle CLI, removing clutter and navigation to save tokens. Use instead of WebFetch when the user provides a URL to read or analyze, for online documentation, articles, blog posts, or any standard web page.
---

# Defuddle

Use Defuddle CLI to extract clean readable content from web pages. Prefer over WebFetch for standard web pages — it removes navigation, ads, and clutter, reducing token usage.

If not installed: `npm install -g defuddle`

## Mindset

Defuddle is a server-side HTML stripper, not a browser. Think of it as getting the printed article without the magazine wrapper — it fetches raw HTML and removes boilerplate, always faster and cheaper than WebFetch for standard pages.

**When to apply:** Any standard HTTP page — documentation, articles, blog posts, wikis.
**When NOT to apply:** SPAs requiring JavaScript, login-gated pages, JSON API endpoints, or pages behind auth redirects.

When in doubt, try defuddle first; if the output is empty or navigation-only, fall back to WebFetch.

## Quick Start

1. Confirm the URL is a standard web page (not a SPA or auth-required page)
2. Run `defuddle parse <url> --md` to extract clean markdown
3. If output is empty or only navigation text, fall back to WebFetch
4. Save output with `-o file.md` when persistence is needed

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

See [ANTI-PATTERNS.md](references/ANTI-PATTERNS.md) for full explanations of each mistake.

### 1. Using WebFetch when defuddle is available

**NEVER** use WebFetch for standard web pages when defuddle is available. Raw HTML commonly contains 5-10x more tokens than the extracted content.

BAD:

```bash
WebFetch("https://docs.example.com/guide")
```

GOOD:

```bash
defuddle parse https://docs.example.com/guide --md
```

### 2. Omitting the `--md` flag

**NEVER** call `defuddle parse` without `--md` when readable content is the goal. Without it, defuddle returns raw HTML — tag-heavy and token-wasteful.

BAD:

```bash
defuddle parse https://example.com/article
```

GOOD:

```bash
defuddle parse https://example.com/article --md
```

### 3. Using `-p` when full content is needed

**NEVER** use `-p` when the user wants the page body. It returns one metadata property and silently discards the entire body text.

BAD:

```bash
defuddle parse https://example.com/article -p title
```

GOOD:

```bash
defuddle parse https://example.com/article --md
```

### 4. Using shell redirection instead of `-o`

**NEVER** use `>` to save output — it can corrupt encoding on non-UTF-8 terminals. Always use `-o`.

BAD:

```bash
defuddle parse https://example.com/article --md > output.md
```

GOOD:

```bash
defuddle parse https://example.com/article --md -o output.md
```

## References

- [Defuddle npm](https://www.npmjs.com/package/defuddle)
- [Defuddle GitHub](https://github.com/kepano/defuddle)
