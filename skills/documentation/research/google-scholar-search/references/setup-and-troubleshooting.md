# Setup and Troubleshooting

Reference for installing dependencies and diagnosing common failures.

## Installation

### Recommended: uv

```bash
cd skills/documentation/research/google-scholar-search
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### Alternative: standard venv

```bash
cd skills/documentation/research/google-scholar-search
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Verify

```bash
python3 scripts/google-scholar-search.py --help
```

## Activating the environment

Every shell session requires activating the venv before running the script:

```bash
source skills/documentation/research/google-scholar-search/.venv/bin/activate
```

## Troubleshooting

### HTTP 429 / rate limit

Google Scholar rate-limits scraping aggressively. If you receive a 429 or the response body is unusually short (~under 1 KB):

1. Wait 60 seconds and retry once.
2. If still blocked, switch to the `semantic-scholar` MCP (see `triage-paper` SKILL.md for config).
3. Do not retry in a loop — it worsens and extends the block.

### Empty result set on a valid query

A 200 response with zero results may be a silent CAPTCHA redirect. Check response length:

```bash
curl -s "https://scholar.google.com/scholar?q=machine+learning" | wc -c
```

If the byte count is under ~2000, the request was likely intercepted. Treat as a block; see above.

### `scholarly` import error (author search only)

Author profile lookup requires the `scholarly` library. If it fails to import:

```bash
pip install scholarly
```

If `scholarly` triggers its own block, use Semantic Scholar's author API via MCP instead.

### Wrong Python version

The script requires Python 3.8+. Check with:

```bash
python3 --version
```

If below 3.8, use `mise` or `pyenv` to install a compatible version.
