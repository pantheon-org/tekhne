# Setup and Troubleshooting

## Installation

### Recommended: uv

```bash
cd skills/documentation/research/sci-hub-search
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### Alternative: standard venv

```bash
cd skills/documentation/research/sci-hub-search
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Verify

```bash
python3 scripts/sci_hub_search.py --help
```

## Environment Variables (optional)

| Variable | Default | Description |
|---|---|---|
| `SCIHUB_BASE_URL` | auto-detect | Override the Sci-Hub mirror URL |
| `DOWNLOAD_TIMEOUT` | `30` | Download timeout in seconds |

Add to a `.env` file in the skill directory and load with `dotenvx run --`:

```bash
SCIHUB_BASE_URL=
DOWNLOAD_TIMEOUT=30
```

## Activating the environment

Every session requires activating the venv:

```bash
source skills/documentation/research/sci-hub-search/.venv/bin/activate
```

## Troubleshooting

### "scihub module not found"

```bash
pip install scihub
```

If running in a venv, ensure it is activated first.

### "Paper not found" or empty result

- Verify the DOI is correct at [doi.org](https://doi.org)
- The paper may not be indexed on the current Sci-Hub mirror
- Try again after a few minutes; mirrors rotate automatically
- If a specific mirror is known to work, set `SCIHUB_BASE_URL`

### SSL certificate errors

The script disables HTTPS verification warnings by default (`urllib3.disable_warnings`). If you see persistent SSL errors, check network proxy settings or try a different mirror.

### CrossRef API timeout (title/keyword search)

Title and keyword searches depend on the CrossRef API (`api.crossref.org`). If the API is slow or unreachable:

- Retry once after 30 seconds
- If you have the DOI, use `--doi` directly instead of `--title` or `--keyword`

### Download timeout

Increase the timeout for slow mirrors:

```bash
export DOWNLOAD_TIMEOUT=60
./scripts/sci_hub_search.py download --doi "10.xxxx/xxxxx" --output paper.pdf
```

## Legal Notice

Sci-Hub access may be restricted or illegal in some jurisdictions. Verify local laws before downloading papers. This skill is intended for research and educational use only.
