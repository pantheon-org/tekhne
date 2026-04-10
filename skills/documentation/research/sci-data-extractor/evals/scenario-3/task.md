# Sci-Data-Extractor — Batch Processing with Failure Review

## Task

A researcher has a folder `./literature/` containing 5 PDF files and wants all of them extracted using the `review` template into CSV format, saved to `./output/`.

Run `batch_extract.py` on the folder. After the batch completes, the summary shows that 2 files failed (e.g., `paper3.pdf` and `paper5.pdf`).

Report the results to the user, explain what steps to take to diagnose and re-run the failed files individually, and show the exact command for re-running one of the failed files with `--print` to inspect output.
