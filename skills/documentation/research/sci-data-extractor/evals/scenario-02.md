# Scenario 02: Sci-Data-Extractor — Scanned PDF Detection and Fallback

## User Prompt

A researcher runs extraction on `scanned_paper.pdf` using PyMuPDF. The extraction returns near-empty text (fewer than 50 characters).

Detect that PyMuPDF has failed to extract meaningful content, warn the user about the likely scanned-image PDF issue, and recommend switching to Mathpix OCR. The user has `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` set in their environment.

Re-run the extraction using Mathpix OCR with the `experiment` template and save results to `experiment_results.md`.

## Expected Behavior

1. Identify that PyMuPDF returned fewer than 50 characters of content and report this as a likely scanned-image PDF issue
2. Explicitly explain the PyMuPDF limitation (embedded text only, cannot OCR pixel images) before switching to Mathpix
3. Confirm that `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` are set before attempting the Mathpix OCR run
4. Invoke `./scripts/extractor.py scanned_paper.pdf --template experiment --ocr mathpix -o experiment_results.md`

## Success Criteria

- **empty extraction detected and reported**: The agent identifies that PyMuPDF returned fewer than 50 characters of content and reports this as a likely scanned-image PDF issue.
- **Mathpix credentials verified before re-running**: The agent confirms that MATHPIX_APP_ID and MATHPIX_APP_KEY are set before attempting the Mathpix OCR run.
- **re-run uses --ocr mathpix and experiment template**: The agent invokes ./scripts/extractor.py scanned_paper.pdf --template experiment --ocr mathpix -o experiment_results.md.
- **user is informed of the limitation before fallback**: The agent explicitly explains the PyMuPDF limitation (embedded text only, cannot OCR pixel images) before switching to Mathpix.

## Failure Conditions

- Agent does not detect or report the near-empty extraction output
- Agent proceeds to the Mathpix fallback without verifying `MATHPIX_APP_ID` and `MATHPIX_APP_KEY`
- Agent omits `--ocr mathpix` or uses the wrong template in the re-run command
- Agent switches to Mathpix without explaining the PyMuPDF limitation to the user
