# Setup and Troubleshooting

## Installation

### Recommended: uv

```bash
cd skills/documentation/research/sci-data-extractor
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### Alternative: standard venv

```bash
cd skills/documentation/research/sci-data-extractor
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Verify

```bash
python3 scripts/extractor.py --help
```

## Environment Variables

Create a `.env` file in the skill directory (excluded from git):

```bash
EXTRACTOR_API_KEY=your-api-key-here
EXTRACTOR_BASE_URL=https://api.anthropic.com
EXTRACTOR_MODEL=claude-sonnet-4-5-20250929
EXTRACTOR_TEMPERATURE=0.1
EXTRACTOR_MAX_TOKENS=16384

# Optional: Mathpix OCR credentials
MATHPIX_APP_ID=your-mathpix-app-id
MATHPIX_APP_KEY=your-mathpix-app-key
```

Load with `dotenvx run --` before any command, or export variables directly.

### Obtain API Keys

- **Anthropic Claude**: [console.anthropic.com](https://console.anthropic.com/)
- **OpenAI-compatible**: [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Mathpix OCR**: [api.mathpix.com](https://api.mathpix.com/)

## Activating the Environment

Every session requires activating the venv:

```bash
source skills/documentation/research/sci-data-extractor/.venv/bin/activate
```

## Troubleshooting

### Empty or very short extracted text

PyMuPDF extracts embedded text only. If the PDF contains scanned images rather than embedded text, the extracted content will be empty or very short. Switch to Mathpix OCR:

```bash
./scripts/extractor.py paper.pdf --template enzyme -o results.md --ocr mathpix
```

### "API key not set" warning

Ensure `EXTRACTOR_API_KEY` is exported or present in `.env`. The script reads both `EXTRACTOR_API_KEY` and `API_KEY` environment variables.

### Mathpix upload fails with HTTP 4xx

Verify that both `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` are set and match your Mathpix account credentials at [api.mathpix.com](https://api.mathpix.com/).

### LLM returns prose instead of a table

Add explicit output format instructions to your custom prompt. For example:

```
-p "Extract data into a Markdown table using | as delimiter. Include columns: Name | Value | Unit"
```

### `batch_extract.py` imports fail

The batch script imports from `extractor` using a relative import. Always run it from the `scripts/` directory or ensure `extractor.py` is on the Python path:

```bash
cd skills/documentation/research/sci-data-extractor
python3 scripts/batch_extract.py ./literature/ ./output/ --template enzyme
```

### Long documents exceed token limits

Increase the token limit via the environment variable:

```bash
export EXTRACTOR_MAX_TOKENS=32768
```

The script automatically segments long documents; increasing `EXTRACTOR_MAX_TOKENS` reduces the chance of truncated table output.

### ImportError for `fitz` (PyMuPDF)

```bash
pip install pymupdf
```

Note: the package name is `pymupdf` but the import is `fitz`.
