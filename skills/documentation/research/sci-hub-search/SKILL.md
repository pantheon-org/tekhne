---
name: sci-hub-search
description: "Search and download academic papers through Sci-Hub by DOI, title, or keyword. Supports PDF download, metadata extraction, and automatic mirror detection via CrossRef integration. Use when a paper is behind a paywall and you have the DOI or title. Triggers: sci-hub, download paper, fetch paper, academic paper download, paper by DOI, paper access, paywall bypass, retrieve paper, get full text, paper download."
allowed-tools: Read, Write, Bash
---

# Sci-Hub Search

Search and download academic papers through Sci-Hub by DOI, title, or keyword. Supports direct PDF download, metadata extraction, and automatic mirror detection.

## When to Use

- Fetching a full-text PDF when only a DOI, title, or keyword is known
- Downloading papers that are behind a paywall and unavailable via open-access channels
- Extracting structured metadata (title, author, year, PDF URL) for a known DOI
- Automating paper retrieval in a research pipeline after `triage-paper` identifies a target

## When Not to Use

- The paper is freely available on arxiv or via DOI — prefer the direct source
- The `sci-hub` MCP server is configured — ALWAYS prefer the MCP; no Python setup required

### Recommended MCP Server

When available, prefer the [Sci-Hub-MCP-Server](https://github.com/JackKuo666/Sci-Hub-MCP-Server):

```json
{
  "mcpServers": {
    "sci-hub": {
      "type": "stdio",
      "command": "uvx",
      "args": ["sci-hub-mcp-server"]
    }
  }
}
```

- Sci-Hub availability is restricted or blocked in your region — check legal status first
- You need citation graphs or author profiles — use `semantic-scholar-search` instead

## Mindset

Sci-Hub is a last-resort retrieval tool, not a primary discovery tool.

1. **DOI first, always**: a gotcha is passing a keyword when you have the DOI. DOI lookup is deterministic; keyword lookup goes through CrossRef and may return the wrong paper. ALWAYS use `--doi` when the DOI is known.
2. **Mirror instability is a pitfall**: Sci-Hub mirrors change frequently. NEVER hardcode a mirror URL in scripts or configs; let the library auto-detect a working mirror via the `SCIHUB_BASE_URL` environment variable or auto-detection.
3. **Verify legality before use**: Sci-Hub access is illegal in some jurisdictions. ALWAYS inform the user to verify local laws before downloading. This skill is for research and educational use only.

## Workflow

### 1. Check MCP availability

Consider the `sci-hub` MCP server first. If configured and reachable, optionally prefer it over this script.

### 2. Set up the environment (first run only)

See [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md).

### 3. Run the subcommand

```bash
# Search by DOI (most accurate)
./scripts/sci_hub_search.py search --doi "10.1038/nature09492"
```

```bash
# Search by title (CrossRef lookup)
./scripts/sci_hub_search.py search --title "CRISPR gene editing"
```

```bash
# Search by keyword (returns multiple results)
./scripts/sci_hub_search.py search --keyword "neural machine translation" --results 10
```

```bash
# Get metadata for a known DOI
./scripts/sci_hub_search.py metadata --doi "10.1002/jcad.12075"
```

```bash
# Download a paper PDF by DOI
./scripts/sci_hub_search.py download --doi "10.1002/jcad.12075" --output paper.pdf
```

```bash
# Download using a direct PDF URL
./scripts/sci_hub_search.py download --url "https://sci-hub.se/xxxxx" --output paper.pdf
```

```bash
# Export search results to JSON
./scripts/sci_hub_search.py search --keyword "transformer attention" --results 5 --format json \
  --output /tmp/candidates.json
```

### 4. Present results and hand off

Show results to the user. NEVER download automatically without confirmation. For each paper found, offer:

> Found paper: `{title}`. Would you like to download the PDF?

NEVER triage or download automatically — always confirm with the user first.
## Anti-Patterns

### NEVER use keyword search when a DOI is known

**WHY:** Keyword search routes through CrossRef and may resolve to a different paper. DOI lookup is exact and deterministic.

**BAD** `./scripts/sci_hub_search.py search --keyword "Attention Is All You Need"` → **GOOD** `./scripts/sci_hub_search.py search --doi "10.48550/arXiv.1706.03762"`

### NEVER hardcode a Sci-Hub mirror URL

**WHY:** Sci-Hub mirrors change frequently. A hardcoded URL will fail without notice and silently return nothing.

**BAD** Setting `SCIHUB_BASE_URL=https://sci-hub.se` permanently in code → **GOOD** Leave `SCIHUB_BASE_URL` unset and let the library auto-detect a working mirror.

### NEVER download papers without user confirmation

**WHY:** Auto-downloading is invasive, may create unwanted files, and bypasses legal-review responsibility.

**BAD** Automatically download every result from a keyword search → **GOOD** Present the result list; confirm with the user before each download.

### NEVER treat "not found" as a definitive absence

**WHY:** A paper not found on the current mirror may be accessible on another mirror or after a delay. ALWAYS report the error and suggest retrying or checking the DOI directly.

**BAD** Report "paper unavailable" after a single failed fetch → **GOOD** Retry once; suggest verifying the DOI on CrossRef; offer the `semantic-scholar-search` fallback for metadata.

### NEVER skip the legal disclaimer

**WHY:** Sci-Hub access is prohibited in some jurisdictions. ALWAYS remind the user to verify local laws before downloading.

**BAD** Download papers without any legal notice → **GOOD** Add a note: "Verify that downloading this paper is legal in your jurisdiction before proceeding."

### NEVER pass raw keyword results directly to triage-paper

**WHY:** Keyword results via CrossRef may include unrelated papers due to fuzzy matching. Let the user review before triaging.

**BAD** Auto-run `triage-paper` on all keyword results → **GOOD** Show candidates and ALWAYS require explicit user selection before triaging.

## References

- **Setup**: [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md)
- **Script**: [sci_hub_search.py](scripts/sci_hub_search.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/sci-hub-search-skill](https://github.com/JackKuo666/sci-hub-search-skill)
- **MCP alternative**: [Sci-Hub-MCP-Server](https://github.com/JackKuo666/Sci-Hub-MCP-Server)
