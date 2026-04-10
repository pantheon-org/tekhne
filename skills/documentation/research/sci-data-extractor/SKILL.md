---
name: sci-data-extractor
description: "Extract structured data from scientific literature PDFs using AI-powered OCR and LLM analysis. Supports enzyme kinetics, experimental results, and literature review templates. Use when researchers need to parse tables, charts, or text from paper PDFs into Markdown or CSV. Triggers: extract data from PDF, scientific data extraction, parse paper tables, enzyme kinetics extraction, batch PDF extraction, literature data, research data from paper, OCR scientific paper, convert PDF to structured data."
allowed-tools: Read, Write, Bash
---

# Sci-Data-Extractor

AI-powered tool for extracting structured data from scientific literature PDFs. Converts tables, figures, and text from research papers into Markdown tables or CSV files using PyMuPDF (free) or Mathpix OCR (high-precision) plus an LLM extraction layer.

## When to Use

- Extracting enzyme kinetics parameters (Km, Kcat, Kcat/Km) from biochemistry papers
- Converting experimental result tables from PDFs to structured CSV or Markdown
- Building a literature review dataset from multiple paper PDFs
- Batch-processing a folder of PDFs into a unified dataset
- Parsing complex tables or chart data points that require AI assistance

## When Not to Use

- The paper is already in a machine-readable format (XML, JSON, HTML) — parse it directly
- You only need the abstract or title — use `semantic-scholar-search` instead
- The PDF is scanned at low resolution with no embedded text and Mathpix is unavailable — results will be unreliable
- You need real-time citation counts or DOI resolution — use `semantic-scholar-search`

## Mindset

Data extraction from PDFs is a pipeline with multiple failure points — always validate each stage.

1. **OCR first, extraction second**: a gotcha is assuming PyMuPDF handles all PDFs equally. Scanned images, complex multi-column layouts, and formula-heavy pages are a known pitfall — switch to Mathpix for high-precision needs or pre-inspect output before running AI extraction.
2. **Prompts determine quality**: the AI extraction layer is only as good as the extraction prompt. ALWAYS review the raw LLM output before saving; consider that ambiguous column headers or merged table cells can silently corrupt extracted values.
3. **Batch is not atomic**: a production gotcha is treating batch runs as pass/fail. ALWAYS check the per-file success log; failed files are listed at the end and must optionally be re-run individually.

## Workflow

### 1. Install dependencies (first run only)

See [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md).

### 2. Configure environment variables

```bash
export EXTRACTOR_API_KEY="your-api-key-here"
export EXTRACTOR_BASE_URL="https://api.anthropic.com"
export EXTRACTOR_MODEL="claude-sonnet-4-5-20250929"
# Optionally for Mathpix OCR:
export MATHPIX_APP_ID="your-mathpix-app-id"
export MATHPIX_APP_KEY="your-mathpix-app-key"
```

### 3. Run with a preset template

```bash
# Enzyme kinetics data — extracts Km, Kcat, organism, mutant, etc.
./scripts/extractor.py paper.pdf --template enzyme -o results.md

# Experimental results — extracts conditions, values, p-values, sample size
./scripts/extractor.py paper.pdf --template experiment -o results.md

# Literature review — extracts authors, year, DOI, key findings
./scripts/extractor.py paper.pdf --template review -o references.md
```

### 4. Run with a custom prompt

```bash
./scripts/extractor.py paper.pdf \
  -p "Extract all protein structure data: PDB ID, resolution, R-value, R_free" \
  -o custom.md
```

### 5. Output as CSV

```bash
./scripts/extractor.py paper.pdf --template enzyme -o results.csv --format csv
```

### 6. Use Mathpix OCR for high-precision extraction

```bash
./scripts/extractor.py paper.pdf --template enzyme -o results.md --ocr mathpix
```

### 7. Batch-process a folder of PDFs

```bash
./scripts/batch_extract.py ./literature/ ./output/ --template enzyme --format csv
```

### 8. Print extracted results to terminal

```bash
./scripts/extractor.py paper.pdf --template review -o references.md --print
```

## Anti-Patterns

### NEVER skip validating extracted output before saving to a database

**WHY:** AI extraction silently produces incorrect values when table headers are ambiguous, units are merged into cells, or multi-row headers span columns. A production gotcha is writing raw LLM output to a database without review.

**BAD** Pipe output directly to a database insert script. → **GOOD** Print results with `--print`, spot-check at least 3 rows against the source PDF, then save.

### NEVER pass a scanned image-only PDF to PyMuPDF without warning the user

**WHY:** PyMuPDF extracts embedded text; it cannot OCR pixel images. A pitfall is getting empty or near-empty extraction with no error, leading to silently wrong results.

**BAD** Run `./scripts/extractor.py scanned.pdf --template enzyme -o out.md` and trust the output. → **GOOD** Check that extracted text length is reasonable; if content is empty or very short, switch to `--ocr mathpix`.

### NEVER use a custom prompt that omits output format instructions

**WHY:** Without explicit format instructions, the LLM may return prose summaries instead of tables, breaking downstream CSV conversion.

**BAD** `-p "Extract protein data"` → **GOOD** `-p "Extract protein data into a Markdown table with columns: Name | PDB ID | Resolution | R-value. Use | as delimiter."`

### NEVER treat batch output as complete without reviewing the failed-files list

**WHY:** `batch_extract.py` continues on errors and reports failures at the end. A production pitfall is assuming a completed batch run means 100% extraction success.

**BAD** Count output files to verify completeness. → **GOOD** ALWAYS read the final summary printed by `batch_extract.py`; re-run failed files individually with `--print` to diagnose issues.

### NEVER store `EXTRACTOR_API_KEY` in plain text in the script or a committed file

**WHY:** API keys in source control are a security production gotcha — they will be exposed in git history even if later removed.

**BAD** Hardcode `api_key = "sk-ant-..."` in the script. → **GOOD** ALWAYS use environment variables or a `.env` file excluded from version control.

### NEVER assume Mathpix is available without checking credentials first

**WHY:** Mathpix requires paid credentials. Attempting OCR without `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` set causes a hard failure mid-pipeline, wasting API quota already consumed for PDF upload.

**BAD** Use `--ocr mathpix` without verifying credentials. → **GOOD** ALWAYS confirm both `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` are set before invoking Mathpix; optionally fall back to PyMuPDF.

## References

- **Setup**: [setup-and-troubleshooting.md](references/setup-and-troubleshooting.md)
- **Script**: [extractor.py](scripts/extractor.py)
- **Dependencies**: [requirements.txt](requirements.txt)
- **Upstream**: [JackKuo666/sci-data-extractor](https://github.com/JackKuo666/sci-data-extractor)
