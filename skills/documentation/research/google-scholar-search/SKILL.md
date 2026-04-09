---
name: google-scholar-search
description: "Search Google Scholar for academic papers and author profiles. Use when discovering papers to triage — by keyword, author, or year range. Returns titles, authors, abstracts, and links. Triggers: search papers, find papers, google scholar, search scholar, find author, author profile, literature search, paper discovery, search academic literature."
allowed-tools: Read, Write, Bash
---

# Google Scholar Search

Search Google Scholar for academic papers and author profiles to build a candidate list before triaging.

## When to Use

- Discovering papers on a topic before running `triage-paper`
- Finding papers by a specific author, optionally filtered by year range
- Scoping a research area and exporting a candidate list to JSON for batch review

## When Not to Use

- The paper is already known (arxiv ID, DOI, URL) — go straight to `triage-paper`
- The `semantic-scholar` MCP is configured — optionally prefer it; it returns structured data with no rate-limit risk
- The search is for biomedical literature — use PubMed or a domain-specific source
- A candidate JSON already exists at `/tmp/<topic>-candidates.json` — reuse it rather than re-running

## Mindset

Search is discovery, not triage. The goal is a candidate list, not a finished reference.

1. **Return candidates, not conclusions**: hand the list to the user; let them choose what to triage.
2. **Scraping is fragile**: a gotcha is that a blank result may be a silent CAPTCHA redirect, not an empty query. Always verify by checking the raw response length before reporting "no results".
3. **MCP first**: a common pitfall is invoking the script without checking whether a `semantic-scholar` or `google-scholar` MCP is available. MCPs are faster, structured, and avoid rate-limit risk.

## Workflow

### 1. Check MCP availability

Consider the `semantic-scholar` or `google-scholar` MCP server first. If either is configured and reachable, prefer it over this script.

### 2. Set up the environment (first run only)

See [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md) for venv creation and dependency installation.

### 3. Run the search

Activate the venv, then choose the appropriate subcommand:

```bash
# Basic keyword search
./scripts/google-scholar-search.py search --query "retrieval-augmented generation" --results 10
```

```bash
# Advanced: filter by author and year range
./scripts/google-scholar-search.py advanced \
  --query "LLM reasoning" --author "Yann LeCun" \
  --year-start 2020 --year-end 2024 --results 10
```

```bash
# Author profile
./scripts/google-scholar-search.py author --name "Geoffrey Hinton"
```

```bash
# Export candidate list to JSON
./scripts/google-scholar-search.py search \
  --query "prompt compression" --format json --output /tmp/candidates.json
```

### 4. Parse JSON output (optional)

```bash
# Preview titles from a saved candidate list
python3 -c "import json; [print(p['title']) for p in json.load(open('/tmp/candidates.json'))]"
```

### 5. Handle blocked responses

If the script returns HTTP 429 or an unusually short HTML body, wait 60 seconds and retry once:

```bash
sleep 60 && ./scripts/google-scholar-search.py search --query "<topic>" --results 10
```

NEVER retry in a tight loop. If still blocked, offer the `semantic-scholar` MCP as fallback and document the blocker.

### 6. Present candidates and hand off

Show the result list. For each promising paper, offer:

> Found N results. Would you like to triage any of these with `triage-paper`?

NEVER triage automatically — always confirm with the user first.

## Anti-Patterns

### NEVER auto-triage search results

**WHY:** Discovery and triage are separate quality gates. Auto-triaging bypasses user review.

**BAD** Run `triage-paper` on every result. → **GOOD** Present the list; wait for the user to choose.

### NEVER loop on blocked responses

**WHY:** Repeated retries worsen the block and may trigger a longer ban.

**BAD** Retry in a tight loop. → **GOOD** Retry once after 60 s; switch to MCP fallback on second failure.

### NEVER treat scraped abstracts as canonical

**WHY:** Scraped text is truncated and misformatted. Passing it to `triage-paper` corrupts the research record.

**BAD** Use the scraped abstract as the paper summary. → **GOOD** Use results for discovery only; re-fetch from arxiv or DOI during triage.

### NEVER skip the MCP availability check

**WHY:** The script is the fragile fallback. Skipping the check needlessly risks rate-limiting.

**BAD** Invoke the script without checking for an MCP first. → **GOOD** Check for `semantic-scholar`/`google-scholar` MCP; only fall back to the script if neither is available.

### NEVER treat year-range results as exhaustive

**WHY:** Google Scholar's year filter is a ranking hint, not a strict predicate. Papers outside the range may appear; papers inside may be missing.

**BAD** Claim "all 2020–2024 papers on X". → **GOOD** Qualify results as "a sample in the requested range, as returned by Google Scholar."

### NEVER report "no results" without checking for a silent block

**WHY:** A 200 response with a near-empty body is often a CAPTCHA redirect, not a genuine empty result set.

**BAD** Report "no papers found" when the HTML body is unusually short. → **GOOD** Check response length; if under ~1 KB, treat it as a potential block and retry once.

## References

- **Setup & troubleshooting**: [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md)
- **Script source**: [google-scholar-search.py](scripts/google-scholar-search.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/google-scholar-search-skill](https://github.com/JackKuo666/google-scholar-search-skill)
- **Preferred alternative**: see `triage-paper` for `semantic-scholar` MCP config
