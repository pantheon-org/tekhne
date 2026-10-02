# Recommended MCP Servers

Prefer these MCP servers over `WebFetch` for paper discovery and metadata resolution. They return structured data and avoid HTML scraping.

```json
{
  "mcpServers": {
    "semantic-scholar": {
      "type": "stdio",
      "command": "uvx",
      "args": ["semantic-scholar-fastmcp"]
    },
    "google-scholar": {
      "type": "stdio",
      "command": "uvx",
      "args": ["google_scholar_mcp_server"]
    }
  }
}
```

## Order of preference

1. `semantic-scholar` is the primary source: open, structured, and it covers most CS and ML papers.
2. `google-scholar` is the fallback for papers not indexed there.
3. `WebFetch` on the arxiv abstract page is the last resort, used only when neither MCP is configured or returns results.

## Which source for which input

| Input | Source |
|---|---|
| arxiv ID or URL | `semantic-scholar` MCP for title, authors, date, abstract and DOI |
| DOI | `semantic-scholar` or `google-scholar` MCP, not a raw HTTP fetch |
| PDF path | Read the file and extract the same fields |
