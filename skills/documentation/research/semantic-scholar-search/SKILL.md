---
name: semantic-scholar-search
description: "Search Semantic Scholar for academic papers, fetch paper details by ID or DOI, retrieve author profiles, and analyse citation graphs. Preferred over google-scholar-search — uses the official API with no scraping. Triggers: search papers, find papers, semantic scholar, paper details, paper by DOI, paper by ID, author profile, citation analysis, references, literature search, paper discovery."
allowed-tools: Read, Write, Bash
---

# Semantic Scholar Search

Search Semantic Scholar for papers, fetch structured metadata by paper ID or DOI, profile authors, and traverse citation graphs. Uses the official Semantic Scholar API — no scraping, structured results, free to use.

## When to Use

- Discovering papers on a topic before running `triage-paper`
- Resolving an arxiv ID or DOI to full metadata during `triage-paper` step 1
- Fetching papers that cite or are cited by a known paper
- Profiling an author to find their key publications
- Preferred over `google-scholar-search` whenever both are available

## When Not to Use

- The `semantic-scholar` MCP server is configured — optionally prefer it over this script; the MCP returns the same data with less setup
- The paper is in a biomedical domain not covered by Semantic Scholar — consider PubMed instead
- You need Google Scholar–exclusive sources (patents, grey literature) — use `google-scholar-search`

## Mindset

This is a structured API, not a search engine — treat it accordingly.

1. **IDs over titles**: a gotcha is using a paper title as the `--paper-id` flag. The `paper` subcommand requires a Semantic Scholar paper ID, arxiv ID (prefix `arXiv:`), or DOI. Titles go to `search`.
2. **Author IDs, not names**: the `author` subcommand requires a Semantic Scholar author ID. A common pitfall is passing a name and getting "not found". Use `search` first to locate the author ID.
3. **Rate limits apply without a key**: 100 requests per 5 minutes without an API key. NEVER issue search loops that exhaust the quota; optionally set `SEMANTIC_SCHOLAR_API_KEY` to increase limits.

## Workflow

### 1. Choose the right subcommand

| Goal | Subcommand | Key flag |
|---|---|---|
| Keyword discovery | `search` | `--query` |
| Full metadata for a known paper | `paper` | `--paper-id` (SS ID, `arXiv:NNNN`, or DOI) |
| Author publications and h-index | `author` | `--author-id` (Semantic Scholar ID) |
| Papers citing or cited by a paper | `citations` | `--paper-id` + `--type` |

### 2. Set up the environment (first run only)

See [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md).

### 3. Run the subcommand

```bash
# Keyword discovery
./scripts/semantic-scholar-search.py search \
  --query "retrieval-augmented generation" --limit 10
```

```bash
# Full paper metadata — Semantic Scholar ID, arxiv ID, or DOI
./scripts/semantic-scholar-search.py paper --paper-id "arXiv:2005.11401"
./scripts/semantic-scholar-search.py paper --paper-id "10.18653/v1/2020.acl-main.740"
```

```bash
# Author profile — requires Semantic Scholar author ID
./scripts/semantic-scholar-search.py author --author-id "1741101"
```

```bash
# Citations and references for a known paper
./scripts/semantic-scholar-search.py citations --paper-id "arXiv:2005.11401" --type citations
./scripts/semantic-scholar-search.py citations --paper-id "arXiv:2005.11401" --type references
```

```bash
# Export any result to JSON
./scripts/semantic-scholar-search.py search \
  --query "chain-of-thought reasoning" --limit 10 --format json \
  --output /tmp/candidates.json
```

### 4. Find a Semantic Scholar author ID (if needed)

```bash
# Step 1: search for the author's papers to locate their ID
./scripts/semantic-scholar-search.py search --query "<author name> <topic>" --limit 5 --format json \
  | python3 -c "import json,sys; [print(a['authorId'], a['name']) for p in json.load(sys.stdin) for a in p.get('authors',[])]"
```

### 5. Present candidates and hand off

Show results to the user. NEVER triage automatically. For each promising paper offer:

> Found N results. Would you like to triage any of these with `triage-paper`?

For known papers, pass the arxiv ID or DOI directly to `triage-paper` step 1.

## Anti-Patterns

### NEVER pass a paper title to the paper subcommand

**WHY:** The `paper` subcommand resolves by ID only. Passing a title returns "not found" or a wrong paper.

**BAD** `--paper-id "Attention Is All You Need"` → **GOOD** `--paper-id "arXiv:1706.03762"` or the Semantic Scholar paper ID.

### NEVER pass an author name to the author subcommand

**WHY:** The `author` subcommand requires a Semantic Scholar author ID (numeric string). Names are not accepted.

**BAD** `--author-id "Yoshua Bengio"` → **GOOD** Locate the numeric author ID via `search` first, then use `--author-id "1794492"`.

### NEVER loop subcommands to exhaustion

**WHY:** Without an API key, the rate limit is 100 requests per 5 minutes. A tight loop will trigger a 429 and block further queries.

**BAD** Fetch citations recursively in a loop without rate limiting. → **GOOD** Batch fetches with `time.sleep(1)` or set `SEMANTIC_SCHOLAR_API_KEY` for higher quota.

### NEVER treat search ranking as completeness

**WHY:** Semantic Scholar search ranks by relevance, not publication date or citation count. The top 10 results are not "the 10 most important papers on X".

**BAD** Claim "here are the key papers on X" from a top-10 search. → **GOOD** Qualify results as "top results by Semantic Scholar relevance ranking."

### NEVER auto-triage search results

**WHY:** Discovery and triage are separate quality gates. Auto-triaging bypasses user review.

**BAD** Run `triage-paper` on every result. → **GOOD** Present the candidate list; confirm with the user which papers to triage.

### NEVER use DOI and Semantic Scholar ID interchangeably without the correct prefix

**WHY:** The API distinguishes ID types. An unprefixed DOI is treated as a Semantic Scholar paper ID and will return "not found".

**BAD** `--paper-id "10.18653/v1/2020.acl-main.740"` when the script expects an S2 ID. → **GOOD** Confirm the ID type; arxiv IDs need `arXiv:` prefix; DOIs are accepted as-is in most contexts but verify against the script's `--help`.

## References

- **Setup & troubleshooting**: [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md)
- **Script source**: [semantic-scholar-search.py](scripts/semantic-scholar-search.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/semanticScholar-search-skill](https://github.com/JackKuo666/semanticScholar-search-skill)
- **API key**: [semanticscholar.org/product/api](https://www.semanticscholar.org/product/api#api-key)
- **Preferred MCP alternative**: see `triage-paper` for `semantic-scholar` MCP config
