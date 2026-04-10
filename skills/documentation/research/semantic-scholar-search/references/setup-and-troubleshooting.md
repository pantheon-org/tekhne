# Setup and Troubleshooting

## Installation

### Recommended: uv

```bash
cd skills/documentation/research/semantic-scholar-search
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### Alternative: standard venv

```bash
cd skills/documentation/research/semantic-scholar-search
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Verify

```bash
python3 scripts/semantic-scholar-search.py --help
```

## API Key (optional but recommended)

The Semantic Scholar API is free without a key, but limited to 100 requests per 5 minutes. For heavier use, set an API key:

```bash
# Obtain a free key at https://www.semanticscholar.org/product/api#api-key
export SEMANTIC_SCHOLAR_API_KEY="your-key-here"
```

Or add to a `.env` file in the skill directory and load with `dotenvx run --`.

## Activating the environment

Every session requires activating the venv:

```bash
source skills/documentation/research/semantic-scholar-search/.venv/bin/activate
```

## Troubleshooting

### HTTP 429 — rate limit exceeded

Without an API key, you are limited to 100 requests/5 minutes. Wait 5 minutes before retrying. Set `SEMANTIC_SCHOLAR_API_KEY` to raise the limit.

### "Paper not found"

Check the ID format:

- **Semantic Scholar ID**: plain alphanumeric string (e.g. `204e3073870fae3d05bcbc2f6a8e263d9b72e776`)
- **arxiv ID**: must be prefixed with `arXiv:` (e.g. `arXiv:1706.03762`)
- **DOI**: accepted as-is (e.g. `10.18653/v1/2020.acl-main.740`)

If the paper is very recent (< 2 weeks), it may not yet be indexed.

### "Author not found"

The `author` subcommand requires a Semantic Scholar author ID (a numeric string like `1794492`), not a name. Locate the ID by running a `search` for the author's papers and extracting `authorId` from the response.

### Import error for `semanticscholar`

```bash
pip install semanticscholar
```

If running in a venv, ensure it is activated first.
