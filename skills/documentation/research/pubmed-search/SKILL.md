---
name: pubmed-search
description: "Search and analyze biomedical literature from PubMed using the free E-utilities API. Use when researching medical topics, discovering clinical papers, fetching article metadata by PMID, performing deep paper analysis, or downloading open-access PDFs from PubMed Central. Triggers: pubmed search, search biomedical literature, find medical papers, PMID lookup, pubmed metadata, clinical literature search, biomedical research, life sciences papers, PMC download, ncbi search."
allowed-tools: Read, Write, Bash
---

# PubMed Search

Search and analyze biomedical literature from PubMed using the free NCBI E-utilities API.

## When to Use

- Discovering biomedical or clinical papers by keyword, author, journal, or date range
- Fetching structured metadata (title, authors, abstract, DOI) for a known PMID
- Performing deep analysis of a paper when only the abstract and metadata are available
- Downloading open-access full-text PDFs from PubMed Central (PMC)
- Building a candidate list before running `triage-paper`

## When Not to Use

- The paper is already known (DOI, URL) — go straight to `triage-paper`
- A `semantic-scholar` MCP or PubTator MCP is configured — prefer the MCP; it returns structured data with no rate-limit risk
- The search is for general academic literature — use `google-scholar-search` or `semantic-scholar-search`
- A candidate JSON already exists at `/tmp/<topic>-candidates.json` — reuse it

### Recommended MCP Server

When available, prefer the PubTator MCP server over this script:

```json
{
  "mcpServers": {
    "pubtator": {
      "type": "stdio",
      "command": "uvx",
      "args": ["pubtator-mcp-server"]
    }
  }
}
```

## Mindset

Search is discovery, not analysis. The goal is a structured candidate list.

1. **Rate limits are a gotcha**: without an API key the limit is 3 req/s; a pitfall is issuing bulk PMID fetches without a delay, causing silent failures or 429s. ALWAYS add a short delay between batch calls.
2. **MCP first**: ALWAYS check whether a PubTator or PubMed MCP is configured before invoking the Python script. MCPs are faster, structured, and avoid rate-limit risk.
3. **Open access is not guaranteed**: a pitfall is assuming all PMC articles can be downloaded. NEVER attempt to download a PDF without first confirming PMC availability and open-access status.

## Workflow

### 1. Check MCP availability

ALWAYS check for a `pubtator` or `pubmed` MCP before running the script. If configured and reachable, prefer it.

### 2. Set up the environment (first run only)

See [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md) for venv creation and dependency installation.

### 3. Run the search

Activate the venv, then choose the appropriate subcommand. You may optionally add `--show-abstract` to keyword searches for a richer preview.

```bash
# Basic keyword search
./scripts/pubmed_search.py search --keywords "CRISPR gene editing" --results 10
# Advanced: filter by author, journal, and date range
./scripts/pubmed_search.py search --term "cancer immunotherapy" --author "Smith" \
  --journal "Nature" --start-date "2021" --end-date "2024" --results 20
# Fetch metadata for a known PMID
./scripts/pubmed_search.py metadata --pmid "33303479" --format json
# Deep paper analysis
./scripts/pubmed_search.py analyze --pmid "33303479" --output analysis.md
# Download open-access PDF
./scripts/pubmed_search.py download --pmid "33303479" --output-dir ./papers/
# Export candidate list to JSON
./scripts/pubmed_search.py search --keywords "Alzheimer disease biomarkers" \
  --results 50 --format json --output /tmp/candidates.json
```

### 4. Handle rate-limit errors

If the script returns HTTP 429, wait 30 s and retry once:

```bash
sleep 30 && ./scripts/pubmed_search.py search --keywords "<topic>" --results 10
```

NEVER retry in a tight loop. If still failing, set `PUBMED_API_KEY` in the environment.

### 5. Present candidates and hand off

NEVER triage automatically — ALWAYS confirm with the user first:

> Found N results. Would you like to triage any of these with `triage-paper`?

## Anti-Patterns

### NEVER auto-triage search results

**WHY:** Discovery and triage are separate quality gates. Auto-triaging bypasses user review.

**BAD** Pass every result to `triage-paper` immediately. → **GOOD** Present the list; wait for the user to choose.

### NEVER retry in a tight loop on 429

**WHY:** Repeated rapid retries worsen the block and extend the cooldown period.

**BAD** Loop `search` until it succeeds. → **GOOD** Retry once after 30 s; switch to MCP or API key on second failure.

### NEVER assume all PMC articles are downloadable

**WHY:** Many PMC articles are not open access; the download command will fail or return a redirect link.

**BAD** Call `download` for every PMID. → **GOOD** Check PMC availability and open-access status before downloading.

### NEVER skip the MCP availability check

**WHY:** The Python script is the fragile fallback. Skipping the check needlessly risks rate-limiting.

**BAD** Invoke the script without checking for a PubTator or PubMed MCP. → **GOOD** ALWAYS check MCP availability first; only fall back to the script if no MCP is configured.

### NEVER hardcode API keys in scripts or task files

**WHY:** API keys committed to source are a production security risk and will be rotated or revoked.

**BAD** Set `api_key = "abc123"` inside the script. → **GOOD** ALWAYS use environment variables (`PUBMED_API_KEY`) or a `.env` file that is gitignored.

### NEVER treat abstract-based analysis as a full-text review

**WHY:** PubMed metadata contains only the abstract. Deep analysis based solely on abstracts is incomplete and a pitfall for research quality.

**BAD** Mark a paper as "fully analysed" from `analyze` output alone. → **GOOD** Qualify analysis as "abstract-based" and recommend obtaining the full text for production use.

## References

- **Setup**: [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md)
- **Script**: [pubmed_search.py](scripts/pubmed_search.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/pubmed-search-skill](https://github.com/JackKuo666/pubmed-search-skill)
- **MCP alternative**: [PubTator-MCP-Server](https://github.com/JackKuo666/PubTator-MCP-Server)
