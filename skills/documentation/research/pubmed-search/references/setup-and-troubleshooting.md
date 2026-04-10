# Setup and Troubleshooting

Reference for installing dependencies and diagnosing common failures.

## Installation

### Recommended: uv

```bash
cd skills/documentation/research/pubmed-search
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### Alternative: standard venv

```bash
cd skills/documentation/research/pubmed-search
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Verify

```bash
python3 scripts/pubmed_search.py --help
```

## Activating the environment

Every shell session requires activating the venv before running the script:

```bash
source skills/documentation/research/pubmed-search/.venv/bin/activate
```

## Optional: API Key Configuration

The script works without an API key using the free PubMed E-utilities API. For higher rate limits, obtain a free key at [ncbi.nlm.nih.gov/account](https://www.ncbi.nlm.nih.gov/account/) and set these environment variables (or add to a `.env` file in the skill root):

```bash
export PUBMED_API_KEY="your-api-key-here"
export PUBMED_EMAIL="your-email@example.com"
```

Rate limits:
- Without API key: 3 requests per second
- With API key: up to 10 requests per second

## Troubleshooting

### HTTP 429 / rate limit

PubMed rate-limits unauthenticated requests. If you receive a 429:

1. Wait 30 seconds and retry once.
2. If still failing, set `PUBMED_API_KEY` (see above).
3. Alternatively, configure the PubTator MCP server (see SKILL.md `When Not to Use`).
4. Do not retry in a loop — it extends the cooldown.

### Empty result set on a valid query

If the search URL is correct but returns zero PMIDs, verify the query manually:

```bash
curl -s "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term=CRISPR&retmax=5&retmode=xml"
```

If the response contains `<Count>0</Count>`, the query genuinely returns no results. Broaden your search terms.

### PDF download returns "not open access"

Only articles with a PubMed Central (PMC) ID and an open-access licence can be downloaded automatically. For other articles, the script returns a direct PubMed URL where you can check access options.

### `requests` import error

If the script fails with `ModuleNotFoundError: No module named 'requests'`, the venv is not activated or dependencies are not installed:

```bash
source .venv/bin/activate
pip install -r requirements.txt
```

### Wrong Python version

The script requires Python 3.8+. Check with:

```bash
python3 --version
```

If below 3.8, use `mise` or `pyenv` to install a compatible version.
