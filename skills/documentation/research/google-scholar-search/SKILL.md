---
name: google-scholar-search
description: "Search Google Scholar for academic papers and author profiles. Use when discovering papers to triage — by keyword, author, or year range. Returns titles, authors, abstracts, and links. Triggers: search papers, find papers, google scholar, search scholar, find author, author profile, literature search, paper discovery."
allowed-tools: Read, Write, Bash
---

# Google Scholar Search

Search Google Scholar for academic papers and author profiles to support literature discovery before triaging.

## When to Use

- Discovering papers on a topic before running `triage-paper`
- Finding all papers by a specific author
- Scoping a research area with year-range filtering
- Exporting a candidate list to JSON for batch triage

## When Not to Use

- The paper is already known (arxiv ID, DOI, URL) — go straight to `triage-paper`
- You need stable, structured metadata — prefer the `semantic-scholar` MCP if configured (no scraping, no rate-limit risk)
- The search is for biomedical literature — use PubMed instead

## Mindset

Search is discovery, not triage. The goal is a candidate list, not a finished reference.

1. **Return candidates, not conclusions**: hand the list to the user and let them choose what to triage.
2. **Scraping is fragile**: Google Scholar blocks scrapers aggressively. A blocked response is not a bug — set expectations and offer the MCP fallback.
3. **Prefer MCP when available**: if `semantic-scholar` or `google-scholar` MCP servers are configured, use them instead of this script — they are more reliable and return richer structured data.

## Setup

Install Python dependencies once before first use:

```bash
# Recommended: uv (fastest)
cd skills/documentation/research/google-scholar-search
uv venv && source .venv/bin/activate
uv pip install -r requirements.txt

# Alternative: standard venv
python3 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
```

## Workflow

### 1. Confirm MCP availability

Check whether a `semantic-scholar` or `google-scholar` MCP server is configured. If yes, prefer the MCP — stop here and use it directly.

### 2. Run the search

```bash
# Activate the venv if not already active
source skills/documentation/research/google-scholar-search/.venv/bin/activate

# Basic keyword search
python3 scripts/google-scholar-search.py search --query "<topic>" --results 10

# Advanced: filter by author and year range
python3 scripts/google-scholar-search.py advanced \
  --query "<topic>" \
  --author "<author name>" \
  --year-start <YYYY> --year-end <YYYY> \
  --results 10

# Author profile
python3 scripts/google-scholar-search.py author --name "<author name>"

# Export to JSON for review
python3 scripts/google-scholar-search.py search --query "<topic>" \
  --format json --output /tmp/candidates.json
```

### 3. Handle blocked responses

If the script returns an HTTP 429 or CAPTCHA response:

- Wait 60 seconds and retry once.
- If still blocked, report to the user and offer the `semantic-scholar` MCP as fallback.
- Do not retry in a loop.

### 4. Present candidates

Show the result list to the user. For each promising paper, offer:

> Found N results for your query. Would you like to triage any of these with `triage-paper`?
> Provide the paper URL or title and I will run the full triage workflow.

Do not triage automatically — always confirm with the user first.

## Quick Commands

```bash
# Install dependencies
cd skills/documentation/research/google-scholar-search && uv venv && source .venv/bin/activate && uv pip install -r requirements.txt

# Basic search
python3 scripts/google-scholar-search.py search --query "attention mechanism" --results 10

# Advanced search
python3 scripts/google-scholar-search.py advanced --query "LLM reasoning" --year-start 2022 --year-end 2024 --results 10

# Author profile
python3 scripts/google-scholar-search.py author --name "Ilya Sutskever"

# JSON export
python3 scripts/google-scholar-search.py search --query "RAG retrieval" --format json --output /tmp/candidates.json
```

## Anti-Patterns

### NEVER triage without user confirmation

**WHY:** Discovery and triage are separate steps. Auto-triaging a search result bypasses the quality gate.

**BAD** Run `triage-paper` on every search result automatically. → **GOOD** Present the candidate list and ask which papers to triage.

### NEVER loop on blocked responses

**WHY:** Retrying aggressively after a 429 or CAPTCHA worsens the block and can cause a longer ban.

**BAD** Retry in a tight loop until results appear. → **GOOD** Retry once after 60 seconds; if still blocked, switch to the `semantic-scholar` MCP fallback.

### NEVER treat search results as verified metadata

**WHY:** Web-scraped titles, authors, and abstracts can be truncated or misformatted. Always re-resolve from the source (arxiv, DOI) during triage.

**BAD** Use the scraped abstract as the triage summary. → **GOOD** Use search results as discovery only; re-fetch the full paper during `triage-paper`.

## References

- **Script**: [google-scholar-search.py](scripts/google-scholar-search.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/google-scholar-search-skill](https://github.com/JackKuo666/google-scholar-search-skill)
- **Preferred alternative**: `semantic-scholar` MCP — see `triage-paper` for MCP config
