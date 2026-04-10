# Sci-Data-Extractor — Scanned PDF Detection and Fallback

## Task

A researcher runs extraction on `scanned_paper.pdf` using PyMuPDF. The extraction returns near-empty text (fewer than 50 characters).

Detect that PyMuPDF has failed to extract meaningful content, warn the user about the likely scanned-image PDF issue, and recommend switching to Mathpix OCR. The user has `MATHPIX_APP_ID` and `MATHPIX_APP_KEY` set in their environment.

Re-run the extraction using Mathpix OCR with the `experiment` template and save results to `experiment_results.md`.
